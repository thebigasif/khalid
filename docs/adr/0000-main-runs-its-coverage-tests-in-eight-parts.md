# 0000. Main runs its coverage tests in eight parts

**Number:** next free. **Status:** Proposed, 2026-11-09. Main CI speed task (BL-TST-3, BL-TST-6).

## Context

On main, the nightly run and manual runs, CI ran every Rust test once, built for coverage, in one job, so the coverage floor (BL-TST-6) read one whole run. That job took 89 to 118 minutes (runs 42564080407 and 42655464150): about 5 minutes to build, then 4,543 tests that kept a runner's four cores busy for 26,839 test-seconds. Every other job of a main run was done by minute 16.

All main runs shared one concurrency group. A group holds one running run and one waiting run, and a newer push replaces the waiting one, so a merge that landed while another run was going often never got a result. On 2026-11-09 alone, nine main runs ended cancelled, among them 1ce13dd0, the v1.7.1 commit.

## Decision

- **Eight parts.** On main, the tests run as eight jobs, `cargo nextest run --partition hash:N/8`, each built for coverage as the one run was. The partitions hold every test exactly once, and main still runs every test, the showcase demos included.
- **Each part hands over one LCOV file.** `scripts/ci/coverage-part.sh` runs `cargo llvm-cov report --lcov` in the part's own job, against the binaries that part built and ran, and the part uploads the file. It fails if llvm-cov reports mismatched data.
- **One job joins them.** `rust-coverage` downloads the eight files and joins them by source file and line (`scripts/ci/coverage-report.sh`): a line counts once, and is covered when any part ran it. It fails under the floor in `coverage.toml`. It builds nothing.
- **Rust needs it.** The required `Rust` check passes on main only when `rust-coverage` passed. On a pull request it must be skipped.
- **No main run is replaced.** Pull requests keep one group per branch, with a newer push stopping the older run. Every other run gets a group of its own (its run id), so no main run is ever stopped or replaced.

## Why the number stays the same

The parts hold every test exactly once, and line coverage only asks whether a line ran. A line ran in the whole run exactly when it ran in some part, so joining the parts' files by line gives the number one run of every test gives.

## Amended 2026-11-23: report in each part

The first version joined the parts' raw profiles and reported them against a fresh build in `rust-coverage`. That build never matched the parts' builds exactly: llvm-cov found 81 to 450 functions with mismatched data on every main run from 10 to 22 November, dropped their counts, and the mismatch check failed the job each time, though the total stayed near 91.7% against the 80% floor. Profiles name functions by a hash of their build; LCOV names source lines, which do not change between builds of one commit. So each part now reports against its own build, and the join needs no build at all, which also saves the job's build time.

## Why eight

A simulation over run 42655464150's test times, with PR 1124's one-test-per-template split, gives the slowest part about 32 minutes of tests at four parts, 21 at six, 17 at eight, 14 at ten. Eight keeps the whole run near 32 minutes with room under the 40-minute goal; ten saves two more minutes for two more runners.

## Consequences

- Main's CI gives its full result in about 32 minutes instead of about two hours: about 6 minutes to build, about 17 for the slowest part, 1 to hand over its profile, then about a minute for `rust-coverage` to join the reports.
- A main run takes eight runners for its tests instead of one. Runs on main can now overlap; GitHub queues jobs past the account's limit instead of dropping runs.
- The demo layout jobs may start a few minutes later when the parts hold the runners; they still finish long before the coverage report.
