#!/usr/bin/env bash
# Reports line coverage for the whole test run from the profiles its parts
# wrote (scripts/ci/coverage-part.sh), and fails under the floor in
# coverage.toml (BL-TST-6). Run it after a coverage build of the tests,
# made as the parts made theirs, so the build's coverage map matches.
# CI only: never run coverage on the shared build server.
#
# Each part's profile goes where the tests write raw profiles, and then
# `cargo llvm-cov report` runs exactly as it ran after one whole run: it
# merges every profile there with llvm-profdata, which reads a profile by
# its contents, not its name. Counters only add up, so the table and the
# total are the ones one run of every test gives.
#
# Usage: scripts/ci/coverage-report.sh <folder of part profiles> <parts>
set -euo pipefail

parts_dir="${1:?Name the folder that holds the part profiles.}"
parts="${2:?Say how many parts the tests ran in.}"

shopt -s nullglob
found=("$parts_dir"/*.profdata)
if [ "${#found[@]}" -ne "$parts" ]; then
  echo "Found ${#found[@]} part profiles in $parts_dir, expected $parts. Coverage of the whole run cannot be told."
  exit 1
fi

eval "$(cargo llvm-cov show-env --sh)"
pattern="${LLVM_PROFILE_FILE:?cargo llvm-cov show-env set no LLVM_PROFILE_FILE.}"
dir="${pattern%/*}"
name="${pattern##*/}"
prefix="${name%%-%p*}"

# Only the parts' profiles count, never a raw profile left from a build.
rm -f "$dir/$prefix"-*.profraw
for part in "${found[@]}"; do
  base="${part##*/}"
  cp "$part" "$dir/$prefix-${base%.profdata}.profraw"
done

floor=$(sed -n 's/^lines *= *\([0-9.]*\).*/\1/p' coverage.toml 2>/dev/null || echo 0)
warnings="$(mktemp)"
status=0
cargo llvm-cov report --summary-only --fail-under-lines "$floor" 2> "$warnings" || status=$?
cat "$warnings" >&2

# A function whose build here differs from the parts' build loses its
# counts, and the total would drop without a test changing. llvm-cov says
# so as "mismatched data"; then the number is not the whole run's.
if grep -q "mismatched data" "$warnings"; then
  echo "The coverage build here differs from the parts' build, so some counts were dropped. Rebuild with the same toolchain, flags and cache as the rust job."
  exit 1
fi
exit "$status"
