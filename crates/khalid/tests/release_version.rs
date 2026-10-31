//! The version `khalid --version` prints is ready to ship: `CHANGELOG.md`
//! has its section, `docs/releases/` has its notes, and the SDK and the
//! admin carry the same number. The release workflow builds whatever tag is
//! pushed, so this is what keeps a tag and the files behind it in step.

use std::fs;
use std::path::PathBuf;

/// The version every crate but khalid-core and khalid-lint inherits.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The text of the file at `relative` from the root of the repository.
#[cfg(test)]
fn repository(relative: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The `version` field of the npm package at `relative`.
#[cfg(test)]
fn npm_version(relative: &str) -> String {
    let json: serde_json::Value = serde_json::from_str(&repository(relative)).unwrap();
    json["version"].as_str().unwrap().to_owned()
}

#[test]
fn the_version_is_a_release_not_the_placeholder() {
    assert_ne!(VERSION, "0.0.0");
}

#[test]
fn the_changelog_has_a_section_for_the_version() {
    let heading = format!("## {VERSION}");
    let changelog = repository("CHANGELOG.md");
    assert!(
        changelog
            .lines()
            .any(|line| line == heading || line.starts_with(&format!("{heading} "))),
        "CHANGELOG.md has no `{heading}` section"
    );
}

#[test]
fn the_release_notes_exist_for_the_version() {
    let notes = repository(&format!("docs/releases/{VERSION}.md"));
    assert!(notes.starts_with(&format!("# Khalid {VERSION}")), "{notes}");
}

#[test]
fn the_sdk_and_the_admin_carry_the_same_version() {
    assert_eq!(npm_version("sdk/typescript/package.json"), VERSION);
    assert_eq!(npm_version("admin/package.json"), VERSION);
}
