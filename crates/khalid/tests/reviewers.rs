//! A reviewer per category (plan 4.12, backlog row 31): `lint.toml` names
//! the categories whose posts must say who checked them, and `khalid lint`
//! and `khalid build` hold every post in one to it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

/// A health post that names no reviewer.
const UNCHECKED: &str = "---
title: How much water to drink each day
date: 2026-09-14
author: asif
category: health
description: How much water an adult needs each day, and the signs that you need more of it.
---

Most adults need about two litres a day. See [the checked guide to sleep](/blog/how-long-to-sleep/).
";

/// A health post that names the author who checked it.
const CHECKED: &str = "---
title: How long to sleep each night
date: 2026-09-14
author: asif
reviewed_by: [asif]
category: health
description: How many hours of sleep an adult needs, and how to tell when you get too few.
---

Most adults need seven to nine hours. See [how much water to drink](/blog/how-much-water-to-drink-each-day/).
";

/// The site's one category.
const CATEGORIES: &str = "[[categories]]
slug = \"health\"
en = \"Health\"
";

/// A site folder with the demo settings, a health category, the demo's
/// author `asif`, and the two health posts.
#[cfg(test)]
fn site() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let demo = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/demo-site");
    fs::copy(
        within(Path::new(demo), "site.toml"),
        within(dir.path(), "site.toml"),
    )
    .unwrap();
    fs::write(within(dir.path(), "categories.toml"), CATEGORIES).unwrap();
    let authors = within(dir.path(), "content/en/authors");
    fs::create_dir_all(&authors).unwrap();
    fs::copy(
        within(Path::new(demo), "content/en/authors/asif.md"),
        within(&authors, "asif.md"),
    )
    .unwrap();
    let folder = within(dir.path(), "content/en/posts");
    fs::create_dir_all(&folder).unwrap();
    let posts = [
        ("2026-09-14-how-much-water-to-drink-each-day.md", UNCHECKED),
        ("2026-09-14-how-long-to-sleep.md", CHECKED),
    ];
    for (name, text) in posts {
        fs::write(within(&folder, name), text).unwrap();
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
fn khalid(site: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site)
        .args(args)
        .output()
        .unwrap()
}

#[cfg(test)]
fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// The slug of every post a JSON report finds `rule` on.
#[cfg(test)]
fn found(json: &Value, rule: &str) -> Vec<String> {
    let findings = json["findings"].as_array().unwrap();
    findings
        .iter()
        .filter(|finding| finding["rule"] == rule)
        .map(|finding| finding["slug"].as_str().unwrap_or_default().to_owned())
        .collect()
}

#[test]
fn lint_blocks_only_the_health_post_with_no_reviewer() {
    let site = site();
    let rules = "[reviewers]\nrequired-in = [\"health\"]\n";
    fs::write(within(site.path(), "lint.toml"), rules).unwrap();
    let output = khalid(site.path(), &["lint", "--json"]);
    let said = text(&output.stdout);
    assert_eq!(output.status.code(), Some(1), "{said}");
    let json: Value = serde_json::from_str(&said).unwrap();
    assert_eq!(
        found(&json, "no-reviewer"),
        ["how-much-water-to-drink-each-day"],
        "{json}"
    );
}

#[test]
fn a_site_that_lists_no_category_needs_no_reviewer() {
    let site = site();
    let output = khalid(site.path(), &["lint", "--json"]);
    let said = text(&output.stdout);
    let json: Value = serde_json::from_str(&said).unwrap();
    assert!(found(&json, "no-reviewer").is_empty(), "{json}");
}

#[test]
fn build_leaves_out_the_post_with_no_reviewer() {
    let site = site();
    let rules = "[reviewers]\nrequired-in = [\"health\"]\n";
    fs::write(within(site.path(), "lint.toml"), rules).unwrap();
    let output = khalid(site.path(), &["build"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let said = text(&output.stdout);
    assert!(said.contains("Left out 1 post"), "{said}");
    assert!(said.contains("needs a reviewer"), "{said}");
}

#[test]
fn a_misspelt_reviewer_category_stops_lint_and_build() {
    let site = site();
    let rules = "[reviewers]\nrequired-in = [\"heath\"]\n";
    fs::write(within(site.path(), "lint.toml"), rules).unwrap();
    for command in ["lint", "build"] {
        let output = khalid(site.path(), &[command]);
        let said = text(&output.stderr);
        assert_eq!(output.status.code(), Some(1), "{command}: {said}");
        assert!(said.contains("`heath`, on line 2"), "{command}: {said}");
        assert!(
            said.contains("one of the site's categories: health"),
            "{said}"
        );
    }
}

#[test]
fn a_reviewer_category_only_a_post_names_is_known() {
    let site = site();
    fs::remove_file(within(site.path(), "categories.toml")).unwrap();
    let rules = "[reviewers]\nrequired-in = [\"health\"]\n";
    fs::write(within(site.path(), "lint.toml"), rules).unwrap();
    let output = khalid(site.path(), &["lint", "--json"]);
    let said = text(&output.stdout);
    let json: Value = serde_json::from_str(&said).unwrap();
    assert_eq!(found(&json, "no-reviewer").len(), 1, "{json}");
}

#[test]
fn the_publish_door_refuses_a_health_draft_with_no_reviewer() {
    let site = site();
    let rules = "[reviewers]\nrequired-in = [\"health\"]\n";
    fs::write(within(site.path(), "lint.toml"), rules).unwrap();
    let drafts = within(site.path(), "drafts/en");
    fs::create_dir_all(&drafts).unwrap();
    let draft = UNCHECKED.replace("date: 2026-09-14\n", "slug: salt-each-day\n");
    fs::write(within(&drafts, "salt-each-day.md"), &draft).unwrap();
    let output = khalid(site.path(), &["publish", "salt-each-day", "--json"]);
    let said = text(&output.stdout);
    assert_eq!(output.status.code(), Some(1), "{said}");
    let answer: Value = serde_json::from_str(&said).unwrap();
    assert_eq!(found(&answer["lint"], "no-reviewer"), ["salt-each-day"]);
}
