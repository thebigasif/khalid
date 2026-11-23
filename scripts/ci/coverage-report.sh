#!/usr/bin/env bash
# Reports line coverage for the whole test run from the LCOV files its
# parts wrote (scripts/ci/coverage-part.sh), and fails under the floor in
# coverage.toml (BL-TST-6). It builds nothing and needs no Rust toolchain.
#
# The parts hold every test exactly once, so a line is covered in the whole
# run when any part ran it. The files join by source file and line: a line
# counts once, and counts as covered when any part's count for it is above
# zero. That is what one run of every test would report.
#
# Usage: scripts/ci/coverage-report.sh <folder of part LCOV files> <parts>
set -euo pipefail

parts_dir="${1:?Name the folder that holds the part LCOV files.}"
parts="${2:?Say how many parts the tests ran in.}"

shopt -s nullglob
found=("$parts_dir"/*.lcov)
if [ "${#found[@]}" -ne "$parts" ]; then
  echo "Found ${#found[@]} part LCOV files in $parts_dir, expected $parts. Coverage of the whole run cannot be told."
  exit 1
fi

floor=$(sed -n 's/^lines *= *\([0-9.]*\).*/\1/p' coverage.toml 2>/dev/null || echo 0)

# DA:<line>,<count>[,<checksum>] under SF:<file>.
awk -v floor="$floor" '
  /^SF:/ { file = substr($0, 4); next }
  /^DA:/ {
    split(substr($0, 4), field, ",")
    key = file SUBSEP field[1]
    if (!(key in hit)) { hit[key] = 0; lines[file]++ }
    if (field[2] + 0 > 0 && hit[key] == 0) { hit[key] = 1; covered[file]++ }
  }
  END {
    total = 0; done = 0
    for (f in lines) {
      total += lines[f]; done += covered[f]
      printf "%7.2f%%  %6d lines  %s\n", 100 * covered[f] / lines[f], lines[f], f | "sort -n"
    }
    close("sort -n")
    if (total == 0) { print "The LCOV files hold no lines."; exit 1 }
    percent = 100 * done / total
    printf "\nTOTAL  %d of %d lines covered, %.2f%%. Floor %s%%.\n", done, total, percent, floor
    if (percent < floor) { print "Line coverage is under the floor in coverage.toml (BL-TST-6)."; exit 1 }
  }
' "${found[@]}"
