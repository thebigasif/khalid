# Why Khalid carries a copy of htmd

This folder is the crates.io source of `htmd` 0.5.6, copied on 2026-11-10.
Its licence file is unchanged: `LICENSE` (Apache-2.0).

## What changed

Only the manifests. In `Cargo.toml` and `Cargo.toml.orig` the requirements
went from `html5ever` 0.38, `markup5ever_rcdom` 0.38 and `phf` 0.13 to 0.40,
0.40 and 0.14. The tests, benches and examples folders were left out, and the
manifests lost their dev-dependency, bench and example entries to match. No
Rust source changed: every file under `src/` is as published and builds
against html5ever 0.40.

The workspace `Cargo.toml` points at this copy with:

```toml
[patch.crates-io]
htmd = { path = "vendor/htmd" }
```

## Why

`ammonia` and `scraper` use html5ever 0.40, and htmd 0.5.6 pins 0.38, so the
`khalid` binary carried two HTML parsers, two `markup5ever` and two
`web_atoms` tables. One parser makes the release binary 558 KB smaller
(31,524,568 to 30,966,744 bytes on the build server), which brought the server
log work (task/p1-mail-trace-7) back under the 30 MB size budget.

The `khalid-import` and `khalid-api` tests, which turn imported HTML into
Markdown through htmd, pass against this copy.

## When to remove it

Remove this vendored copy and the patch entry when htmd releases a version
that uses html5ever 0.40 or newer. Written 2026-11-10.

This folder is third-party code: it is not a workspace member, so the
workspace lints, `cargo fmt` and the file-size rule do not apply, and
`typos.toml` excludes `vendor/`.
