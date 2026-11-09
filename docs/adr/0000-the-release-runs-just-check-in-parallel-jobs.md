# 0000. The release runs `just check` as parallel jobs on the tagged commit

**Number:** next free. **Status:** Proposed, 2026-11-09. Release speed task (KH-OPS-6).

## Context

The v1.7.1 release spent 70 minutes in one `just check` job, while its five platform builds were done in 8 to 16 minutes. The release must not publish anything that has not passed every check, and it should finish in under 30 minutes. Two ways were open:

1. **Reuse.** Skip the release's own check when main's CI run on the tagged commit passed.
2. **Split.** Run the release's own check as parallel jobs, as pull requests already split their tests.

Reuse does not hold, for four reasons found on 2026-11-09:

- **Main's CI does not run all of `just check`.** It leaves out `cargo publish --dry-run` for khalid-core and khalid-lint (KH-LNT-6), the `anyhow` check (BL-ERR-1), the check that disallowed methods are suppressed only where allowed (BL-RUN-1, BL-RUN-2, BL-RUN-8, BL-SEC-4) and the 600-line file size check (BL-SIZE-1). It also runs the tests without the admin bundle, where the release's serve test needs `/admin/` to answer with the app.
- **Main's run on a release commit is often missing.** A run waiting on main is replaced by the next merge. Main's run on the v1.7.1 commit, 1ce13dd0, was cancelled, so reuse would have refused that release.
- **Main's runs fail on jobs `just check` does not hold.** Main's runs on 98fa7481 and d0f0df0a failed on a WebKit demo layout job alone. Reuse would tie a release to those.
- **Main's run is slow.** Its one coverage test run takes about two hours. A tag pushed right after a merge would wait that long for an answer.

## Decision

- **The release checks the tagged commit itself, in parallel.** `rust-lints` runs `scripts/check.sh --no-nextest`: every step of `just check` but the one nextest run. `rust` runs that nextest run in six hash partitions (`--partition hash:N/6`), which hold every test exactly once. The draft release and the Docker image need both jobs and every build.
- **One list of commands.** `--no-nextest` is an option of `scripts/check.sh` itself, so a step added to `just check` reaches the release without a change to the workflow.
- **A warm test build.** The `rust` job has the same id, `RUSTFLAGS`, `RUSTDOCFLAGS` and toolchain as the `rust` job in `ci.yml`, so a tag restores the dependency build that main's CI caches.
- **No single test holds a part back.** The three tests that looped over every template (85, 22 and 22 minutes) are one test per template, so their builds spread over the partitions.

## Consequences

- The release's checks take about 25 minutes instead of 70: about 5 to build the tests from main's cache, then about 20 for the slowest of six parts. The builds, at 8 to 16 minutes, run beside them.
- A release uses nine Linux runners at once for its checks and builds instead of four.
- Pull requests still leave the showcase demo tests out by name; the release and main run them.
