//! The publish gate in `khalid build`: a post the linter blocks is left
//! out and named, `lint.toml` is read, and drafts stay with the writer.

use std::fs;

use serde_json::Value;

use super::{at, demo_site, khalid, page, path, text};

/// A post that passes every block.
const READY: &str = "---
title: Why internal links matter
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
---

An internal link points to [how to pick link text](/blog/choosing-link-text/).
";

/// A dated post with no author and no internal link: two blocks.
const BLOCKED: &str = "---
title: Choosing link text
date: 2026-09-21
---

Good link text names the page it opens.
";

/// A site folder with the demo settings and `posts` as `(file name, text)`.
#[cfg(test)]
fn site(posts: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::copy(at(&demo_site(), "site.toml"), at(dir.path(), "site.toml")).unwrap();
    let folder = at(dir.path(), "content/en/posts");
    fs::create_dir_all(&folder).unwrap();
    for (name, text) in posts {
        fs::write(at(&folder, name), text).unwrap();
    }
    dir
}

#[test]
fn build_leaves_out_a_post_the_gate_blocks_and_builds_the_rest() {
    let site = site(&[
        ("2026-09-14-why-internal-links-matter.md", READY),
        ("2026-09-21-choosing-link-text.md", BLOCKED),
    ]);
    let output = khalid(&["--site", path(site.path()), "build"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let said = text(&output.stdout);
    assert!(
        said.contains("en/choosing-link-text: blocks publishing"),
        "{said}"
    );
    assert!(said.contains("This post has no author."), "{said}");
    assert!(said.contains("Rule no-internal-link"), "{said}");
    assert!(!said.contains("en/why-internal-links-matter"), "{said}");
    assert!(said.contains("Left out 1 post"), "{said}");
    let public = at(site.path(), "public");
    assert!(at(&public, "why-internal-links-matter/index.html").exists());
    assert!(!at(&public, "choosing-link-text").exists());
    let home = page(&public, "index.html");
    assert!(!home.contains("Choosing link text"), "{home}");
}

#[test]
fn build_json_lists_the_posts_it_left_out() {
    let site = site(&[("2026-09-21-choosing-link-text.md", BLOCKED)]);
    let output = khalid(&["--site", path(site.path()), "build", "--json"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty(), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout).lines().count(), 1, "one JSON value");
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let left_out = json["left_out"].as_array().unwrap();
    assert_eq!(left_out.len(), 1, "{json}");
    assert_eq!(left_out[0]["blocks"], true);
    let rules: Vec<_> = left_out[0]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["severity"] == "block")
        .map(|f| f["rule"].as_str().unwrap())
        .collect();
    assert_eq!(rules, ["no-author", "no-internal-link"]);
}

#[test]
fn build_reads_lint_toml_and_stops_on_a_mistake_in_it() {
    let site = site(&[("2026-09-21-choosing-link-text.md", BLOCKED)]);
    let settings = at(site.path(), "lint.toml");
    fs::write(&settings, "[rules]\nno-authors = \"off\"\n").unwrap();
    let output = khalid(&["--site", path(site.path()), "build"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(text(&output.stderr).contains("line 2"));

    let opened = "[rules]\nno-author = \"warn\"\nno-internal-link = \"off\"\n";
    fs::write(&settings, opened).unwrap();
    let output = khalid(&["--site", path(site.path()), "build"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
}

#[test]
fn build_leaves_drafts_to_the_writer() {
    let draft = BLOCKED.replace("date: 2026-09-21\n", "");
    let site = site(&[
        ("2026-09-14-why-internal-links-matter.md", READY),
        ("choosing-link-text.md", &draft),
    ]);
    let output = khalid(&["--site", path(site.path()), "build", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let public = at(site.path(), "public");
    assert!(at(&public, "why-internal-links-matter/index.html").exists());
    assert!(!at(&public, "choosing-link-text").exists());
}
