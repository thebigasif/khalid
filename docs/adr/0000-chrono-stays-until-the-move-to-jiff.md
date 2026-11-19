# 0000. chrono stays until the move to jiff, with a dated advisory ignore

**Number:** next free. **Status:** Accepted, 2026-11-19. CI repair task kw-ci-main (issue #1088).

## Context

In mid-November 2026 RustSec marked chrono unmaintained (RUSTSEC-2026-0543), and humantime with it, because humantime wraps chrono (RUSTSEC-2026-0542). Neither advisory names a vulnerability, and no fixed release exists. `cargo deny check` refuses every advisory that is not listed in `deny.toml`, so main's "Rust lints, docs and dependencies" job has been red on every run since 17 November.

chrono is not a corner dependency here. Every crate that stores or shows a time uses it: khalid-core, khalid-content, khalid-db, khalid-api, khalid-seo, khalid-render and most of the others. sqlx's SQLite driver encodes `DateTime<Utc>` through it, and the feed crates the tests read (atom_syndication, rss) use it too. humantime arrives only through trycmd, a test-only dependency of the CLI.

Two ways were open:

1. **Move now.** Replace chrono with jiff or time across the workspace, sqlx's column types included, before main can be green again.
2. **Ignore with a reason and a date.** List both advisories in `deny.toml` with a reason and a review date, as the file already does for bincode, paste, ttf-parser and rustybuzz, and plan the move as its own task.

## Decision

- **Option 2.** Both advisory IDs go in `deny.toml` with a one-line reason each and a review date of 2027-02-19.
- The ignore is dropped at once if RustSec files a vulnerability against chrono, since `cargo deny` then raises a new ID that the ignore does not cover.
- The move to jiff is a task of its own: it touches the shared types in khalid-core, every crate's tests that build times, and sqlx's column encoding, so it needs its own stack of pull requests.

## Consequences

- Main's dependency check is green again, and a new advisory against any other crate still turns it red.
- chrono keeps working as it does today: Khalid's times come from the `Clock`, the content files and the database, never from a stranger's string parsed without our own error type around it.
- Someone has to look again by 2027-02-19. The review date sits next to the ignore in `deny.toml`, where the other ignores keep theirs.
