#!/usr/bin/env python3
"""The line coverage floor (RULES.md BL-TST-6) over a test suite run in parts.

CI runs the tests in parallel partitions. Each partition builds every test
binary with coverage, runs its share of the tests and writes an LCOV report
from `cargo llvm-cov report --lcov`. Every report lists every line of the
workspace that coverage maps, with the times this partition ran it.

A line counts as covered when any partition ran it, which is what one
merged run would report, since counts only add up. The total is every
mapped line in any report. The run fails when the covered share is under
the floor in coverage.toml, as `cargo llvm-cov --fail-under-lines` does.

Each report's own share is printed too, to compare with the summary that
partition's job printed from cargo llvm-cov.

Usage: coverage-floor.py COVERAGE_TOML REPORT.lcov...
"""

import re
import sys
from pathlib import Path


def floor_from(path):
    """The `lines` floor in coverage.toml, or 0 when it sets none."""
    for line in Path(path).read_text().splitlines():
        found = re.match(r"^lines\s*=\s*([0-9.]+)", line)
        if found:
            return float(found.group(1))
    return 0.0


def lines_of(path):
    """{(file, line): ran} for one LCOV report."""
    lines = {}
    source = None
    for row in Path(path).read_text().splitlines():
        if row.startswith("SF:"):
            source = row[3:]
        elif row.startswith("DA:") and source is not None:
            number, count = row[3:].split(",")[:2]
            key = (source, int(number))
            lines[key] = lines.get(key, False) or int(count) > 0
        elif row == "end_of_record":
            source = None
    return lines


def share(lines):
    return 100.0 * sum(lines.values()) / len(lines) if lines else 0.0


def main(coverage_toml, reports):
    floor = floor_from(coverage_toml)
    merged = {}
    for report in reports:
        lines = lines_of(report)
        print(f"{report}: {sum(lines.values())} of {len(lines)} lines, {share(lines):.2f}%")
        for key, ran in lines.items():
            merged[key] = merged.get(key, False) or ran
    if not merged:
        print("No coverage report held a line, so the floor cannot be checked.")
        return 1
    percent = share(merged)
    print(f"All partitions: {sum(merged.values())} of {len(merged)} lines, {percent:.2f}%, floor {floor:g}%")
    if percent < floor:
        print(
            f"Line coverage is {percent:.2f}%, under the floor of {floor:g}% in coverage.toml."
            " Add tests for the code this change left untested."
        )
        return 1
    return 0


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: coverage-floor.py COVERAGE_TOML REPORT.lcov...", file=sys.stderr)
        sys.exit(2)
    sys.exit(main(sys.argv[1], sys.argv[2:]))
