# 0000. Main runs its coverage tests in eight parts

**Number:** next free. **Status:** Proposed, 2026-11-09. Main CI speed task (BL-TST-3, BL-TST-6).

## Context

On main, the nightly run and manual runs, CI ran every Rust test once, built for coverage, in one job, so the coverage floor (BL-TST-6) read one whole run. That job took 89 to 118 minutes (runs 42564080407 and 42655464150): about 5 minutes to build, then 4,543 tests that kept a runner's four cores busy for 26,839 test-seconds. Every other job of a main run was done by minute 16.

All main runs shared one concurrency group. A group holds one running run and one waiting run, and a newer push replaces the waiting one, so a merge that landed while another run was going often never got a result. On 2026-11-09 alone, nine main runs ended cancelled, among them 1ce13dd0, the v1.7.1 commit.

## Decision

- **Eight parts.** On main, the tests run as eight jobs, `cargo nextest run --partition hash:N/8`, each built for coverage as the one run was. The partitions hold every test exactly once, and main still runs every test, the showcase demos included.
- **Each part hands over one profile.** `scripts/ci/coverage-part.sh` joins the part's raw profiles into one `.profdata` file with `llvm-profdata merge`, and the part uploads it.
- **One job joins them.** `rust-coverage` builds the tests for coverage from the parts' cache (same toolchain, flags and cache key), puts the eight profiles where raw profiles go, and runs the same `cargo llvm-cov report --summary-only --fail-under-lines` as before (`scripts/ci/coverage-report.sh`). It fails if llvm-cov reports mismatched data, which is what a differing build would cause, so a dropped count cannot lower the number unseen.
- **Rust needs it.** The required `Rust` check passes on main only when `rust-coverage` passed. On a pull request it must be skipped.
- **No main run is replaced.** Pull requests keep one group per branch, with a newer push stopping the older run. Every other run gets a group of its own (its run id), so no main run is ever stopped or replaced.

## Why the number stays the same

Coverage counters only add up. Merging eight parts' profiles gives each line the same count one process running every test would give, and line coverage only asks whether that count is above zero. The report runs against binaries built from the same source, toolchain, flags and paths as the parts' binaries. No build script in the workspace embeds a time or a commit, so those builds match, and the mismatch check proves it on every run.

## Why eight

A simulation over run 42655464150's test times, with PR 1124's one-test-per-template split, gives the slowest part about 32 minutes of tests at four parts, 21 at six, 17 at eight, 14 at ten. Eight keeps the whole run near 32 minutes with room under the 40-minute goal; ten saves two more minutes for two more runners.

## Consequences

- Main's CI gives its full result in about 32 minutes instead of about two hours: about 6 minutes to build, about 17 for the slowest part, 1 to hand over its profile, then about 7 for `rust-coverage` to build and report.
- A main run takes eight runners for its tests instead of one. Runs on main can now overlap; GitHub queues jobs past the account's limit instead of dropping runs.
- The demo layout jobs may start a few minutes later when the parts hold the runners; they still finish long before the coverage report.
