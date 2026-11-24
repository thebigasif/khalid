#!/usr/bin/env bash
# Everything CI checks, in the same order, so a green local run means a green CI.
# Each step prints its name. The first failure stops the run.
#
# --no-nextest leaves out the one nextest run and does everything else. The
# release workflow runs that, and runs the nextest suite beside it in parts,
# so the two add up to this whole script.
set -euo pipefail

nextest=true
case "${1:-}" in
  "") ;;
  --no-nextest) nextest=false ;;
  *) echo "Unknown option $1. The one option is --no-nextest."; exit 2 ;;
esac

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

step() { printf '\n==> %s\n' "$1"; }

step "No em dashes in any tracked text (RULES.md BL-TXT-1)"
if git grep -nI -- $'\xe2\x80\x94' -- ':!*.lock' ':!.claude/skills/rust-best-practices/references/*' >/dev/null; then
  git grep -nI -- $'\xe2\x80\x94' -- ':!*.lock' ':!.claude/skills/rust-best-practices/references/*'
  echo "Replace each em dash with a full stop, a comma or a colon."
  exit 1
fi

step "Spelling (typos)"
typos

if ls crates/*/Cargo.toml >/dev/null 2>&1; then
  step "Format (cargo fmt --check)"
  cargo fmt --all -- --check

  step "Clippy, warnings as errors"
  cargo clippy --workspace --all-targets --all-features -- -D warnings

  if [ "$nextest" = true ]; then
    step "Tests (nextest + doc tests)"
    cargo nextest run --workspace --all-features
  else
    step "Doc tests (nextest runs elsewhere)"
  fi
  cargo test --workspace --doc

  step "Docs, warnings as errors"
  RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

  step "Dependencies (cargo deny)"
  cargo deny check

  step "Unused dependencies (cargo machete)"
  cargo machete

  step "Published crates package and build alone (cargo publish --dry-run, KH-LNT-6)"
  # --allow-dirty because this runs before the commit. Nothing is uploaded.
  cargo publish --dry-run --allow-dirty --locked -p khalid-core -p khalid-lint

  step "Lint table proven to fire"
  ./scripts/check-lints.sh

  ./scripts/check-gates.sh

  step "Source searches proven to fire"
  ./scripts/prove-check-gates.sh
else
  echo "No crates yet. Rust checks start with the first crate (Phase 0)."
fi

if [ -f admin/package.json ]; then
  step "Admin: Biome, TypeScript, Vitest"
  # Run only the tools pinned in admin/node_modules. A bare npx would download and run
  # whatever unrelated package carries the same name when node_modules is missing.
  if [ -d admin/node_modules ]; then
    (cd admin && npm exec --no -- biome ci . && npm exec --no -- tsc --noEmit && npm exec --no -- vitest run)
  else
    echo "admin/node_modules is missing, so the admin checks are left to CI. Run npm ci in admin/ to run them here."
  fi
fi

step "TypeScript SDK: Biome and TypeScript (G27)"
if [ -d sdk/typescript/node_modules ]; then
  (cd sdk/typescript && npm exec --no -- biome ci . && npm exec --no -- tsc)
else
  echo "sdk/typescript/node_modules is missing, so the SDK checks are left to CI. Run npm ci in sdk/typescript/ to run them here."
fi

printf '\nAll checks passed.\n'
