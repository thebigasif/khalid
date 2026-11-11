//! No migration ever changes once it is on `main` (RULES.md BL-DB-2).
//!
//! `migrate!` records each migration's checksum in every database it runs
//! on. Edit a migration that a site already ran, and that site refuses to
//! start, because its record no longer matches the file. This test pins each
//! checksum in `fixtures/migration_checksums.txt`, so an edit fails here
//! instead of on someone's server. A new migration adds one line there.

use std::fmt::Write as _;

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

/// The pinned lines, one `<version> <sha384 hex>` per migration.
const PINNED: &str = include_str!("fixtures/migration_checksums.txt");

/// One `<version> <sha384 hex>` line for each migration this crate builds in.
#[cfg(test)]
fn built_in_lines() -> Vec<String> {
    MIGRATOR
        .iter()
        .map(|migration| {
            let mut hex = String::new();
            for byte in migration.checksum.iter() {
                write!(hex, "{byte:02x}").unwrap();
            }
            format!("{} {hex}", migration.version)
        })
        .collect()
}

#[test]
fn every_migration_matches_its_pinned_checksum() {
    let built_in = built_in_lines();
    let pinned: Vec<&str> = PINNED.lines().filter(|line| !line.is_empty()).collect();
    let changed: Vec<&&str> = pinned
        .iter()
        .filter(|line| !built_in.iter().any(|b| b == **line))
        .collect();
    assert!(
        changed.is_empty(),
        "these migrations were edited or deleted after they landed: {changed:?}. \
         A site that already ran them would refuse to start. Put the old file back \
         and make the change in a new migration."
    );
    let unpinned: Vec<&String> = built_in
        .iter()
        .filter(|line| !pinned.contains(&line.as_str()))
        .collect();
    assert!(
        unpinned.is_empty(),
        "these migrations have no pinned checksum yet. Add each line to \
         crates/khalid-db/tests/fixtures/migration_checksums.txt:\n{}",
        unpinned
            .iter()
            .map(|line| line.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
