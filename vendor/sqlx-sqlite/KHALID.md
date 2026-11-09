# Why Khalid carries a copy of sqlx-sqlite

This folder is the crates.io source of `sqlx-sqlite` 0.9.0, copied on
2026-11-09. Its licence files are unchanged: `LICENSE-MIT` and
`LICENSE-APACHE`.

## What changed

One line. The `libsqlite3-sys` requirement in `Cargo.toml` (and the same line
in `Cargo.toml.orig`) went from `>=0.30.1, <0.38.0` to `>=0.30.1, <0.40.0`.
No Rust source changed: the crate builds against `libsqlite3-sys` 0.39 as
published.

The workspace `Cargo.toml` points at this copy with:

```toml
[patch.crates-io]
sqlx-sqlite = { path = "vendor/sqlx-sqlite" }
```

## Why

RUSTSEC-2026-0459 (https://rustsec.org/advisories/RUSTSEC-2026-0459) is a
flaw in the SQLite that `libsqlite3-sys` bundles. It is fixed in
`libsqlite3-sys` 0.39.0 and 0.39.1, which bundle SQLite 3.54. The newest
sqlx, 0.9.0, only accepts `libsqlite3-sys` below 0.38, so without this copy
Khalid could not take the fix, and `cargo deny` failed on every pull request.

`crates/khalid-db/tests/sqlite_version.rs` checks that the linked SQLite is
3.54 or newer.

## When to remove it

Remove this vendored copy and the patch entry when sqlx releases a version
that accepts libsqlite3-sys 0.39. Written 2026-11-09.

This folder is third-party code: it is not a workspace member, so the
workspace lints, `cargo fmt` and the file-size rule do not apply, and
`typos.toml` excludes `vendor/`.
