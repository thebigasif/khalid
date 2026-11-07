#!/usr/bin/env bash
# The page-weight check: the bytes of pictures each template's demo home
# page downloads, at 390 and 1440 px in Chromium, held to the budgets in
# docs/standards/performance.md ("Page weight"). scripts/demo-check/weight.mjs
# does the measuring and says what each budget means.
#
# Each pack is built as its demo is published: its ready demo publication
# where it has one, in its sample brand, as a static site.
#
# Needs Node 22 or later and the pinned tools, with Chromium:
#   npm install --prefix scripts/demo-check
#   npx --prefix scripts/demo-check playwright install --with-deps chromium
#
# Usage: scripts/page-weight.sh [--sites <folder>] [--packs keystone,wire]
#   --sites   measures the sites already built there, such as
#             target/layout-check/sites after scripts/layout-check.sh, in
#             place of building them again
#   --packs   measures only these packs
# The numbers go to target/page-weight/report.json.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
tools="$root/scripts/demo-check"
work="$root/target/page-weight"
# shellcheck source=demo-check/showcase.sh
source "$tools/showcase.sh"

sites=""
packs=""
while [ $# -gt 0 ]; do
  case "$1" in
    --sites) sites="${2:?Name the folder of built sites.}"; shift 2 ;;
    --packs) packs="${2:?Name the packs, such as keystone,wire.}"; shift 2 ;;
    *) stop "This check takes --sites and --packs, not $1." ;;
  esac
done

command -v node >/dev/null || stop "Node is missing. The browser check runs on it. Install Node 22 or later, then run this again."
(cd "$tools" && node -e 'require("playwright")' 2>/dev/null) || stop "Playwright is not installed. The browser comes from it. Run: npm install --prefix scripts/demo-check"

rm -rf "$work"
mkdir -p "$work/sites"
if [ -z "$sites" ]; then
  # CI hands over the release binary it already built as KHALID_BIN.
  if [ -n "${KHALID_BIN:-}" ]; then
    [ -x "$KHALID_BIN" ] || stop "KHALID_BIN is $KHALID_BIN, which is not a program this check can run. Point it at a built khalid, or unset it to build one."
    bin="$KHALID_BIN"
  else
    cargo build --release -p khalid
    bin="${CARGO_TARGET_DIR:-$root/target}/release/khalid"
  fi
  all="$(showcase_packs "$bin" "$work")"
  [ -n "$all" ] || stop "This Khalid did not list its templates, so there is nothing to measure. Check that $bin runs."
  for pack in $all; do
    if [ -z "$packs" ] || [[ ",$packs," == *",$pack,"* ]]; then
      showcase_start "$bin" "$work" "$pack" "$pack" brand "" publication
    fi
  done
  showcase_finish
  sites="$work/sites"
fi

args=(--sites "$sites" --report "$work/report.json")
[ -z "$packs" ] || args+=(--packs "$packs")
node "$tools/weight.mjs" "${args[@]}"
