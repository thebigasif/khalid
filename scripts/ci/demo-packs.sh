#!/usr/bin/env bash
# Which template packs the demo layout check covers in this CI run, and how
# the work is split into parallel jobs. Used by .github/workflows/ci.yml.
#
# A push to main, the nightly run and a run by hand check every pack. A pull
# request checks only the packs it touches:
#
#   crates/khalid-theme/packs/<pack>/       the pack itself
#   fixtures/showcase-brands/<pack>/        its sample brand
#   fixtures/showcase-publications/<pack>/  its demo publication
#   crates/<crate>/tests/build_<pack>.rs    its own tests
#
# A change to anything the packs share checks every pack: the rest of
# crates/ (the theme outside one pack, packs/_sections, the renderer, the
# media crate and the binary), the Cargo files, the demo fixtures, the demo
# scripts, scripts/ci and the workflows. A path this script does not know
# also checks every pack, so a new kind of file is never missed. Only the
# paths in `no_packs` below, which no demo build reads, check none.
#
# The packs are the templates the release binary lists, as the layout check
# reads them, so a folder that is not a template is never selected.
#
# Usage: scripts/ci/demo-packs.sh <khalid binary>
# Reads GITHUB_EVENT_NAME and, on pull requests, GITHUB_BASE_REF, and
# needs the base branch fetched. Writes to GITHUB_OUTPUT, or prints when
# that is unset:
#   packs   the selected packs, comma separated
#   count   how many
#   matrix  the jobs, as {"include": [{"chunk", "of", "packs", "browser"}]}
# DEMO_CHUNK_PACKS sets the most packs one job checks (default 12), which
# keeps the slowest browser, WebKit, under about 10 minutes a job.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"
bin="${1:?Name the khalid binary, which lists the template packs.}"
per_job="${DEMO_CHUNK_PACKS:-12}"
browsers=(chromium firefox webkit)

# shellcheck source=../demo-check/showcase.sh
source "$root/scripts/demo-check/showcase.sh"
probe="$(mktemp -d)"
trap 'rm -rf "$probe"' EXIT
# shellcheck disable=SC2207 # one pack per word, no spaces in pack names
every=($(showcase_packs "$bin" "$probe" | tr ' ' '\n' | sed '/^$/d' | sort -u))
[ "${#every[@]}" -gt 0 ] || stop "$bin did not list its templates, so the demo check cannot be planned. Check that it runs."

is_pack() {
  local pack
  for pack in "${every[@]}"; do [ "$pack" = "$1" ] && return 0; done
  return 1
}

# Paths no demo build reads.
no_packs() {
  case "$1" in
    docs/* | admin/* | starters/* | changelog.d/* | perf/* | lints/* | .claude/* | site/*) return 0 ;;
    .github/ISSUE_TEMPLATE/* | .github/PULL_REQUEST_TEMPLATE.md | .github/CODEOWNERS | .github/dependabot.yml) return 0 ;;
    fixtures/showcase-brands/README.md | fixtures/showcase-publications/README.md) return 0 ;;
    scripts/check-lints.sh | scripts/prove-lint-table.sh | scripts/install.sh | scripts/maintainer/*) return 0 ;;
    scripts/ghost-client* | scripts/starters-check.* | scripts/perf.sh | scripts/perf-gate.py) return 0 ;;
    scripts/docs-bundle.sh | scripts/docs-pack.md | scripts/pages-forward.sh) return 0 ;;
    */*) return 1 ;;
    *.md | LICENSE* | .gitignore | .typos.toml | _typos.toml | typos.toml | deny.toml | clippy.toml | coverage.toml) return 0 ;;
  esac
  return 1
}

# The pack a path belongs to on its own, or nothing.
own_pack() {
  local path="$1" pack=""
  case "$path" in
    crates/khalid-theme/packs/_*) ;;
    crates/khalid-theme/packs/*/*) pack="${path#crates/khalid-theme/packs/}" ;;
    fixtures/showcase-brands/*/*) pack="${path#fixtures/showcase-brands/}" ;;
    fixtures/showcase-publications/*/*) pack="${path#fixtures/showcase-publications/}" ;;
    crates/*/tests/build_*.rs) pack="${path##*/build_}"; pack="${pack%.rs}" ;;
  esac
  pack="${pack%%/*}"
  [ -n "$pack" ] && echo "$pack"
  return 0
}

all=false
selected=()
if [ "${GITHUB_EVENT_NAME:-}" != "pull_request" ]; then
  all=true
else
  base="${GITHUB_BASE_REF:?On a pull request GITHUB_BASE_REF names the base branch.}"
  changed="$(git diff --name-only "origin/$base...HEAD")"
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    pack="$(own_pack "$path")"
    if [ -n "$pack" ]; then
      # A folder named like a pack that is not a template this binary has,
      # such as a test-only pack, has no demo to check.
      is_pack "$pack" && selected+=("$pack")
      continue
    fi
    case "$path" in crates/*/tests/*) continue ;; esac
    no_packs "$path" && continue
    echo "$path is shared by every pack, so every pack is checked."
    all=true
    break
  done <<<"$changed"
fi

if [ "$all" = true ]; then
  packs=("${every[@]}")
elif [ "${#selected[@]}" -gt 0 ]; then
  # shellcheck disable=SC2207 # one pack per word
  packs=($(printf '%s\n' "${selected[@]}" | sort -u))
else
  packs=()
fi

count="${#packs[@]}"
matrix='{"include":[]}'
if [ "$count" -gt 0 ]; then
  jobs=$(((count + per_job - 1) / per_job))
  include=""
  for ((chunk = 0; chunk < jobs; chunk++)); do
    # Even chunks: pack i goes to chunk i * jobs / count.
    members=""
    for ((i = 0; i < count; i++)); do
      [ $((i * jobs / count)) -eq "$chunk" ] && members="${members:+$members,}${packs[i]}"
    done
    for browser in "${browsers[@]}"; do
      include="${include:+$include,}{\"chunk\":$((chunk + 1)),\"of\":$jobs,\"packs\":\"$members\",\"browser\":\"$browser\"}"
    done
  done
  matrix="{\"include\":[$include]}"
fi

list="$(IFS=,; echo "${packs[*]-}")"
if [ "$count" -eq 0 ]; then
  echo "This change touches no template pack and nothing the packs share, so the layout check has nothing to check."
else
  echo "The layout check covers $count of ${#every[@]} packs: $list"
fi
out="${GITHUB_OUTPUT:-/dev/stdout}"
{
  echo "packs=$list"
  echo "count=$count"
  echo "all=$all"
  echo "matrix=$matrix"
} >>"$out"
