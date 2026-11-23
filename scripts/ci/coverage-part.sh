#!/usr/bin/env bash
# Writes the line coverage of one part of the test run as an LCOV file, so
# the part can hand it to the job that reports coverage for the whole run.
# Run it after the part's tests, in the checkout and build they ran in. CI
# only: never run coverage on the shared build server.
#
# The report is made here, against the binaries this part built and ran,
# so every count matches its function. A later job on another runner
# cannot rebuild those binaries exactly: its build gave some functions
# other hashes, and llvm-cov dropped their counts ("mismatched data"). An
# LCOV file names source lines, not functions, so the parts' files join by
# line (scripts/ci/coverage-report.sh).
#
# Usage: scripts/ci/coverage-part.sh <LCOV file to write>
set -euo pipefail

out="${1:?Name the LCOV file to write.}"

eval "$(cargo llvm-cov show-env --sh)"
pattern="${LLVM_PROFILE_FILE:?cargo llvm-cov show-env set no LLVM_PROFILE_FILE.}"
dir="${pattern%/*}"
name="${pattern##*/}"
prefix="${name%%-%p*}"

shopt -s nullglob
raw=("$dir/$prefix"-*.profraw)
if [ "${#raw[@]}" -eq 0 ]; then
  echo "No raw coverage profile in $dir. The tests did not run with coverage."
  exit 1
fi

mkdir -p "$(dirname "$out")"
warnings="$(mktemp)"
status=0
cargo llvm-cov report --lcov --output-path "$out" 2> "$warnings" || status=$?
cat "$warnings" >&2

# The profiles and the binaries come from this one build, so a mismatch
# here means the build changed under the tests.
if grep -q "mismatched data" "$warnings"; then
  echo "Some counts do not match this part's own build, so they were dropped."
  exit 1
fi
if [ "$status" -ne 0 ]; then
  exit "$status"
fi
echo "${#raw[@]} raw profiles reported as $out ($(stat -c %s "$out") bytes)."
