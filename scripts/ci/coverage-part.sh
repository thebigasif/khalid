#!/usr/bin/env bash
# Joins the coverage of one part of the test run into one profile file, so
# the part can hand it to the job that reports coverage for the whole run.
# Run it after the part's tests, in the checkout they ran in. CI only: never
# run coverage on the shared build server.
#
# The tests write many raw profiles (one per test process). This file
# merges them with llvm-profdata, as `cargo llvm-cov report` would. Merging
# only adds counters together, so merging the parts later gives the same
# counts as one run of every test.
#
# Usage: scripts/ci/coverage-part.sh <profile to write>
set -euo pipefail

out="${1:?Name the profile file to write.}"

# The folder and name the coverage build gave each raw profile.
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

bin="$(rustc --print sysroot)/lib/rustlib/$(rustc --print host-tuple)/bin"
mkdir -p "$(dirname "$out")"
printf '%s\n' "${raw[@]}" > "$out.list"
"$bin/llvm-profdata" merge -sparse -f "$out.list" -o "$out"
rm "$out.list"
echo "${#raw[@]} raw profiles joined into $out ($(stat -c %s "$out") bytes)."
