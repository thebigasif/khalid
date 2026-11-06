//! The templates Khalid ships, read from the packs folder as the scripts
//! read them (`scripts/template-packs.sh`): each folder in
//! `crates/khalid-theme/packs/` that holds a `pack.toml`. A test-only pack,
//! such as `trial`, has none. Tests count and list templates from here, so
//! a new template changes no test.

use std::fs;
use std::path::PathBuf;

/// The name of every shipped template, `classic` among them, in name order.
#[cfg(test)]
pub(crate) fn templates() -> Vec<String> {
    let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    folder.push("../khalid-theme/packs");
    let mut names = Vec::new();
    for entry in fs::read_dir(&folder).unwrap() {
        let entry = entry.unwrap();
        let mut file = entry.path();
        file.push("pack.toml");
        if file.is_file() {
            names.push(entry.file_name().into_string().unwrap());
        }
    }
    names.sort();
    assert!(!names.is_empty(), "no pack.toml in {}", folder.display());
    names
}
