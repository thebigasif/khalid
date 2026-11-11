//! Case studies and changelog entries (plan 5.13): `khalid lint` holds each
//! to the fields its collection needs.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

/// A case study with its client, outcome and date.
const STUDY: &str = "---
title: How a bakery doubled its search visits
date: 2026-10-20
client: Northfield Bakery
outcome: Search visits doubled in four months.
---

The bakery wrote one topic cluster and linked it together.
";

/// A case study with a client and nothing else it needs.
const BARE_STUDY: &str = "---
title: How a plumber got steady calls
client: Fieldwork Plumbing
---

The firm wrote about the jobs people search for.
";

/// A changelog entry with a date and a version.
const CHANGE: &str = "---
title: Series and case studies
date: 2026-11-01
version: \"2.4\"
---

Posts can now form a series.
";

/// A changelog entry with a version and no date.
const UNDATED_CHANGE: &str = "---
version: \"2.5\"
---

A glossary from A to Z.
";

/// A site folder with the demo settings, no post, and the `studies` and
/// `changes` as `(file name, text)`.
#[cfg(test)]
fn site(studies: &[(&str, &str)], changes: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let settings = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/demo-site/site.toml"
    );
    fs::copy(settings, within(dir.path(), "site.toml")).unwrap();
    for (folder, files) in [("case-studies", studies), ("changelog", changes)] {
        let folder = within(dir.path(), &format!("content/en/{folder}"));
        fs::create_dir_all(&folder).unwrap();
        for (name, text) in files {
            fs::write(within(&folder, name), text).unwrap();
        }
    }
    dir
}

/// `name` inside `dir`.
#[cfg(test)]
fn within(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path
}

#[cfg(test)]
fn lint(site: &Path, args: &[&str]) -> (Option<i32>, String) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_khalid"));
    command.arg("--site").arg(site).arg("lint").args(args);
    let Output { status, stdout, .. } = command.output().unwrap();
    (status.code(), String::from_utf8_lossy(&stdout).into_owned())
}

/// The `(rule, slug, subject)` of every finding in a JSON report.
#[cfg(test)]
fn findings(said: &str) -> Vec<(String, String, String)> {
    let json: Value = serde_json::from_str(said).unwrap();
    let field = |finding: &Value, name: &str| finding[name].as_str().unwrap_or_default().to_owned();
    let all = json["findings"].as_array().unwrap().iter();
    all.map(|f| (field(f, "rule"), field(f, "slug"), field(f, "subject")))
        .collect()
}

#[test]
fn complete_entries_pass() {
    let site = site(&[("bakery.md", STUDY)], &[("2026-11.md", CHANGE)]);
    let (code, said) = lint(site.path(), &["--json"]);
    assert_eq!(code, Some(0), "{said}");
    assert_eq!(findings(&said), [], "{said}");
}

#[test]
fn lint_blocks_a_study_without_its_fields_and_an_undated_change() {
    let studies = [("bakery.md", STUDY), ("plumber.md", BARE_STUDY)];
    let changes = [("2026-11.md", CHANGE), ("next.md", UNDATED_CHANGE)];
    let site = site(&studies, &changes);
    let (code, said) = lint(site.path(), &["--json"]);
    assert_eq!(code, Some(1), "{said}");
    let hit = |rule: &str, slug: &str, subject: &str| {
        (rule.to_owned(), slug.to_owned(), subject.to_owned())
    };
    let plumber = "how-a-plumber-got-steady-calls";
    let want = [
        hit("case-study-incomplete", plumber, "outcome"),
        hit("case-study-incomplete", plumber, "date"),
        hit("changelog-entry-undated", "2-5", ""),
    ];
    assert_eq!(findings(&said), want, "{said}");
}

#[test]
fn build_leaves_out_the_entries_lint_blocks_and_builds_the_rest() {
    let studies = [("bakery.md", STUDY), ("plumber.md", BARE_STUDY)];
    let changes = [("2026-11.md", CHANGE), ("next.md", UNDATED_CHANGE)];
    let site = site(&studies, &changes);
    let mut command = Command::new(env!("CARGO_BIN_EXE_khalid"));
    let Output { status, stdout, .. } = command
        .arg("--site")
        .arg(site.path())
        .arg("build")
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&stdout);
    assert_eq!(status.code(), Some(0), "{said}");
    assert!(said.contains("Rule case-study-incomplete"), "{said}");
    assert!(said.contains("Rule changelog-entry-undated"), "{said}");
    assert!(said.contains("Left out 2 pages"), "{said}");
    let public = within(site.path(), "public");
    let built = |rest: &str| within(&public, rest).exists();
    assert!(built(
        "case-studies/how-a-bakery-doubled-its-search-visits/index.html"
    ));
    assert!(!built("case-studies/how-a-plumber-got-steady-calls"));
    assert!(built("changelog/series-and-case-studies/index.html"));
    assert!(!built("changelog/2-5"));
    let list = fs::read_to_string(within(&public, "case-studies/index.html")).unwrap();
    assert!(!list.contains("Fieldwork Plumbing"), "{list}");
}

#[test]
fn lint_says_which_field_to_add_in_sentences() {
    let site = site(&[("plumber.md", BARE_STUDY)], &[]);
    let (code, said) = lint(site.path(), &[]);
    assert_eq!(code, Some(1), "{said}");
    assert!(
        said.contains("leaves out its client, its outcome or its date"),
        "{said}"
    );
    assert!(said.contains("About: outcome"), "{said}");
    assert!(said.contains("Rule case-study-incomplete"), "{said}");
}
