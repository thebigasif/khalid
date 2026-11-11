//! No two `ToSchema` types in the workspace share a name. The API
//! document keys its schemas by type name, so a second type with the same
//! name silently replaces the first, and every route that answers with the
//! first is then typed wrong in the SDK (gap audit 4 row 40).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Every `.rs` file under `folder`, in name order.
#[cfg(test)]
fn rust_files(folder: &Path, found: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(folder)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            rust_files(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            found.push(path);
        }
    }
}

/// The names of the types in `source` that derive `ToSchema` under their
/// own name, that is without a `#[schema(as = ...)]` rename.
#[cfg(test)]
fn schema_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut deriving = false;
    let mut renamed = false;
    for line in source.lines().map(str::trim) {
        if line.starts_with("#[derive(") && line.contains("ToSchema") {
            deriving = true;
            renamed = false;
        } else if deriving && line.starts_with("#[schema(as") {
            renamed = true;
        } else if deriving && !line.starts_with("#[") && !line.starts_with("///") {
            let mut words = line.split_whitespace();
            let kind = words.find(|word| *word == "struct" || *word == "enum");
            if let (Some(_), Some(name), false) = (kind, words.next(), renamed) {
                let ends = |c: char| matches!(c, '<' | '{' | '(' | ';');
                names.push(name.split(ends).next().unwrap_or(name).to_owned());
            }
            deriving = false;
        }
    }
    names
}

#[test]
fn no_two_schema_types_share_a_name() {
    let mut crates = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crates.pop();
    let mut files = Vec::new();
    let mut folders: Vec<PathBuf> = fs::read_dir(&crates)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    folders.sort();
    for mut folder in folders {
        folder.push("src");
        if folder.is_dir() {
            rust_files(&folder, &mut files);
        }
    }
    let mut seen: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
    for file in &files {
        for name in schema_names(&fs::read_to_string(file).unwrap()) {
            seen.entry(name).or_default().push(file.clone());
        }
    }
    assert!(seen.len() > 100, "found only {} schema types", seen.len());
    let twins: Vec<_> = seen.iter().filter(|(_, files)| files.len() > 1).collect();
    assert!(
        twins.is_empty(),
        "these schema names are used twice, so the document keeps only one \
         of each; rename one or add #[schema(as = ...)]: {twins:#?}"
    );
}

#[test]
fn the_name_reader_skips_renamed_types_and_reads_generics() {
    let source = "#[derive(Debug, ToSchema)]\n/// Doc.\n#[serde(rename_all = \"snake_case\")]\npub(crate) struct Page<T> {\n}\n\
                  #[derive(ToSchema)]\n#[schema(as = Other)]\nstruct Hidden;\n\
                  #[derive(Serialize, ToSchema)]\npub enum Kind {\n}\n";
    assert_eq!(schema_names(source), ["Page", "Kind"]);
}
