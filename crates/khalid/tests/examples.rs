//! The example sites in `fixtures/examples/`, end to end: each one builds,
//! every post passes `khalid lint` with no finding, and the built site
//! passes `khalid audit` with no warning.
//!
//! Each example names the template that suits it. A template that is not
//! built into Khalid yet is left out of the copy the test builds, so the
//! content is checked with the classic look until the template lands.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;
use toml_edit::DocumentMut;

/// Rules that depend on the day the test runs, not on the content: a post
/// written today is due for review in six months and stale in twelve.
const DATED: [&str; 3] = ["review-due", "stale-post", "stale-page"];

#[cfg(test)]
fn examples_folder() -> PathBuf {
    let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    folder.push("../../fixtures/examples");
    folder
}

/// Every example's folder name, in order, so a new example is tested
/// without a change here. A folder is an example once it has a
/// `site.toml`, as `khalid` itself decides.
#[cfg(test)]
fn examples() -> Vec<String> {
    let mut found: Vec<String> = fs::read_dir(examples_folder())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            let mut settings = path.clone();
            settings.push("site.toml");
            settings.is_file()
        })
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_owned())
        .collect();
    found.sort();
    assert_ne!(found, Vec::<String>::new());
    found
}

#[cfg(test)]
fn example(slug: &str) -> PathBuf {
    let mut site = examples_folder();
    site.push(slug);
    site
}

/// A copy of one example, so the build and the audit's `data/` stay out of
/// the fixture, with its `[template]` table left out when Khalid does not
/// have that template yet.
#[cfg(test)]
fn example_copy(slug: &str) -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&example(slug), temp.path());
    let mut settings = temp.path().to_path_buf();
    settings.push("site.toml");
    let mut document: DocumentMut = fs::read_to_string(&settings).unwrap().parse().unwrap();
    let name = document
        .get("template")
        .and_then(|template| template.get("name"))
        .and_then(|name| name.as_str())
        .unwrap_or_else(|| panic!("{slug} names no template"))
        .to_owned();
    if !khalid_theme::packs().iter().any(|pack| pack.name == name) {
        document.remove("template");
        fs::write(&settings, document.to_string()).unwrap();
    }
    temp
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        if ["data", "public"].contains(&entry.file_name().to_str().unwrap()) {
            continue;
        }
        let mut target = into.to_path_buf();
        target.push(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

#[cfg(test)]
fn khalid(site: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site)
        .args(args)
        .output()
        .unwrap()
}

#[cfg(test)]
fn json(output: &Output) -> Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}"))
}

#[test]
fn every_example_has_a_readme() {
    for slug in examples() {
        let mut readme = example(&slug);
        readme.push("README.md");
        assert!(readme.is_file(), "{slug}");
    }
}

#[test]
fn every_example_builds_with_an_about_page() {
    for slug in examples() {
        let site = example_copy(&slug);
        let output = khalid(site.path(), &["build"]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(0), "{slug}: {stderr}");
        let mut about = site.path().to_path_buf();
        about.push("public/about/index.html");
        assert!(about.is_file(), "{slug}");
    }
}

#[test]
fn every_example_post_passes_lint_with_no_finding() {
    for slug in examples() {
        let site = example_copy(&slug);
        let output = khalid(site.path(), &["lint", "--json"]);
        let body = json(&output);
        let findings: Vec<&Value> = body["findings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|finding| !DATED.contains(&finding["rule"].as_str().unwrap()))
            .collect();
        assert_eq!(findings, Vec::<&Value>::new(), "{slug}: {body}");
        assert_eq!(output.status.code(), Some(0), "{slug}");
        let posts = body["scores"].as_array().unwrap().len();
        assert!((6..=10).contains(&posts), "{slug} has {posts} posts");
    }
}

#[test]
fn every_built_example_passes_the_audit_with_no_warning() {
    for slug in examples() {
        let site = example_copy(&slug);
        let output = khalid(site.path(), &["audit", "--offline", "--json"]);
        let body = json(&output);
        let problems: Vec<&Value> = body["issues"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|issue| ["error", "warning"].contains(&issue["severity"].as_str().unwrap()))
            .filter(|issue| !DATED.contains(&issue["check"].as_str().unwrap()))
            .collect();
        assert_eq!(problems, Vec::<&Value>::new(), "{slug}: {body}");
        assert_eq!(body["score"], 100, "{slug}: {body}");
        assert_eq!(output.status.code(), Some(0), "{slug}");
    }
}
