//! Every `.sql` file this crate compiles in keeps plain LF line endings.
//!
//! `query_file!` names its offline entry in `.sqlx/` by a hash of the file's
//! bytes, and `migrate!` checksums each migration. A Windows checkout that
//! turns LF into CRLF changes both, so the release build cannot find its
//! offline query data and a database made on one system looks edited on the
//! other. `.gitattributes` pins `*.sql` to LF; this test fails on a checkout
//! that ignores it.

use std::path::PathBuf;

/// The `.sql` files in one folder of this crate, in name order.
#[cfg(test)]
fn sql_files(folder: &str) -> Vec<PathBuf> {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push(folder);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "sql"))
        .collect();
    files.sort();
    files
}

#[test]
fn query_and_migration_files_have_no_carriage_returns() {
    let mut files = sql_files("queries");
    files.extend(sql_files("migrations"));
    assert!(files.len() > 2, "no .sql files found");
    let with_cr: Vec<String> = files
        .iter()
        .filter(|path| std::fs::read(path).unwrap().contains(&b'\r'))
        .map(|path| path.display().to_string())
        .collect();
    assert!(
        with_cr.is_empty(),
        "these files have CRLF line endings, so .sqlx no longer matches them: {with_cr:?}"
    );
}
