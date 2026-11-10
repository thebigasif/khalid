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

/// The four checks the 1.0.0 decision left for after launch, by heading.
const AFTER_LAUNCH: [&str; 4] = [
    "## Ahrefs and Semrush Site Audit on the demo site",
    "## HTTPS with a live Let's Encrypt certificate",
    "## The Astro and Next.js starter kits, deployed",
    "## SerpDad's own blog on Khalid",
];

/// The status line under each after-launch check, in heading order.
#[cfg(test)]
fn after_launch_statuses() -> Vec<String> {
    let page = repository("docs/releases/after-launch.md");
    AFTER_LAUNCH
        .iter()
        .map(|heading| {
            let (_, rest) = page
                .split_once(&format!("{heading}\n"))
                .unwrap_or_else(|| panic!("after-launch.md has no `{heading}`"));
            let status = rest
                .lines()
                .find(|line| !line.is_empty())
                .unwrap_or_default();
            assert!(
                status.starts_with("Status: open.") || status.starts_with("Status: done, "),
                "`{heading}` starts with `{status}`, not `Status: open.` or `Status: done, <date>.`"
            );
            status.to_owned()
        })
        .collect()
}

/// The `major.minor.patch` numbers of `version`, or `None` for a file such
/// as `after-launch.md`.
#[cfg(test)]
fn numbers(version: &str) -> Option<Vec<u32>> {
    version.split('.').map(|part| part.parse().ok()).collect()
}

#[test]
fn every_after_launch_check_has_a_status() {
    assert_eq!(after_launch_statuses().len(), AFTER_LAUNCH.len());
}

/// The 1.0.0 notes promised to report the after-launch checks in the next
/// release's notes, and 1.1.0 to 1.8.0 did not. From 1.9.0 on, a
/// release's notes link to the status page while any check is open.
#[test]
fn releases_after_1_8_0_link_the_open_after_launch_checks() {
    if after_launch_statuses()
        .iter()
        .all(|status| status.starts_with("Status: done, "))
    {
        return;
    }
    let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    folder.push("../../docs/releases");
    for entry in fs::read_dir(&folder).unwrap() {
        let name = entry.unwrap().file_name().into_string().unwrap();
        let Some(version) = name.strip_suffix(".md").and_then(numbers) else {
            continue;
        };
        if version > vec![1, 8, 0] {
            let notes = repository(&format!("docs/releases/{name}"));
            assert!(
                notes.contains("(after-launch.md)"),
                "docs/releases/{name} does not link after-launch.md, \
                 but an after-launch check is still open"
            );
        }
    }
}

#[test]
fn the_sdk_and_the_admin_carry_the_same_version() {
    assert_eq!(npm_version("sdk/typescript/package.json"), VERSION);
    assert_eq!(npm_version("admin/package.json"), VERSION);
}
