//! `khalid lint --fix`: the safe repairs it writes into a post, how it
//! reports them, and what it leaves alone.

use std::fs;

use serde_json::Value;

use super::{GOOD, INSECURE, khalid, site, text, within};

/// A post whose `####` heading skips a level, with the two blank lines
/// before `## Setup` that a repair must leave as they are.
const JUMPY: &str = "---
title: Setting up
date: 2026-09-14
author: asif
---

Read [why links matter](/blog/why-internal-links-matter/) first.


## Setup

#### Pick a folder

Any empty folder works.

## Writing

###### Last words
";

#[test]
fn lint_fix_puts_headings_in_order_and_leaves_the_rest() {
    let site = site(&[("setting-up.md", JUMPY)]);
    let file = within(site.path(), "content/en/posts/setting-up.md");
    let before = text(&khalid(site.path(), &["lint"]).stdout);
    assert!(before.contains("Rule heading-order"), "{before}");
    assert_eq!(
        fs::read_to_string(&file).unwrap(),
        JUMPY,
        "lint alone wrote"
    );

    let output = khalid(site.path(), &["lint", "--fix"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let in_order = JUMPY
        .replace("#### Pick", "### Pick")
        .replace("###### Last", "### Last");
    assert_eq!(fs::read_to_string(&file).unwrap(), in_order);
    let said = text(&output.stdout);
    assert!(
        said.contains("Fixed content/en/posts/setting-up.md"),
        "{said}"
    );
    assert!(
        said.contains("\"Pick a folder\" is now a level 3"),
        "{said}"
    );
    assert!(said.contains("\"Last words\" is now a level 3"), "{said}");
    assert!(!said.contains("Rule heading-order"), "{said}");
    assert!(said.contains("Fixed 2 problems."), "{said}");
}

/// A post with a figure that has a caption but no description.
const BARE_FIGURE: &str = "---
title: Lisbon door
date: 2026-09-14
author: asif
---

Read [why links matter](/blog/why-internal-links-matter/) first.

:::figure{src=\"/media/door.jpg\" alt=\"\"}
A *blue* door in Lisbon at noon.
:::
";

#[test]
fn lint_fix_describes_an_image_with_its_caption() {
    let site = site(&[("lisbon-door.md", BARE_FIGURE)]);
    let output = khalid(site.path(), &["lint", "--fix", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let file = within(site.path(), "content/en/posts/lisbon-door.md");
    let described = BARE_FIGURE.replace("alt=\"\"}", "alt=\"A blue door in Lisbon at noon.\"}");
    assert_eq!(fs::read_to_string(file).unwrap(), described);
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    assert_eq!(json["fixed"][0]["rule"], "missing-alt");
    assert_eq!(json["fixed"][0]["subject"], "/media/door.jpg");
    let findings = json["findings"].as_array().unwrap();
    assert!(
        findings.iter().all(|f| f["rule"] != "missing-alt"),
        "{json}"
    );
}

#[test]
fn lint_fix_moves_a_link_to_the_own_site_onto_https() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", INSECURE)]);
    let output = khalid(site.path(), &["lint", "--fix", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let file = within(
        site.path(),
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
    );
    let secure = INSECURE.replace("(http://example.com/", "(https://example.com/");
    assert_eq!(fs::read_to_string(file).unwrap(), secure);
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    assert_eq!(json["fixed"][0]["rule"], "insecure-internal-link");
    assert_eq!(
        json["fixed"][0]["subject"],
        "http://example.com/blog/pruning-old-posts/"
    );
    let findings = json["findings"].as_array().unwrap();
    assert!(
        findings
            .iter()
            .all(|f| f["rule"] != "insecure-internal-link"),
        "{json}"
    );
}

#[test]
fn lint_fix_json_lists_repairs_and_still_exits_1_on_a_block() {
    let undated_jumpy = JUMPY.replace("date: 2026-09-14\n", "");
    let site = site(&[("setting-up.md", &undated_jumpy)]);
    let output = khalid(site.path(), &["lint", "--fix", "--json"]);
    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    assert_eq!(json["blocks"], true);
    let fixed = json["fixed"].as_array().unwrap();
    assert_eq!(fixed.len(), 2, "{json}");
    assert_eq!(fixed[0]["path"], "content/en/posts/setting-up.md");
    assert_eq!(fixed[0]["rule"], "heading-order");
    assert_eq!(fixed[0]["subject"], "Pick a folder");
    let findings = json["findings"].as_array().unwrap();
    assert!(findings.iter().any(|f| f["rule"] == "no-date"), "{json}");
    assert!(
        findings.iter().all(|f| f["rule"] != "heading-order"),
        "{json}"
    );
}

#[test]
fn lint_fix_leaves_a_rule_that_is_off() {
    let site = site(&[("setting-up.md", JUMPY)]);
    fs::write(
        within(site.path(), "lint.toml"),
        "[rules]\nheading-order = \"off\"\n",
    )
    .unwrap();
    let output = khalid(site.path(), &["lint", "--fix", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    assert_eq!(json["fixed"], Value::Array(Vec::new()));
    let file = within(site.path(), "content/en/posts/setting-up.md");
    assert_eq!(fs::read_to_string(file).unwrap(), JUMPY);
}

#[test]
fn lint_fix_says_when_there_is_nothing_to_fix() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", GOOD)]);
    let output = khalid(site.path(), &["lint", "--fix"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let said = text(&output.stdout);
    assert!(said.contains("Nothing needed a safe fix."), "{said}");
}

/// A post with no `slug:`, so its address comes from a title full of
/// filler words and a year.
const WORDY: &str = "---
title: Start a blog in 2026
date: 2026-09-14
author: asif
---

Read [why links matter](/blog/why-internal-links-matter/) first.
";

#[test]
fn lint_fix_cleans_the_address_and_keeps_the_old_one_working() {
    let site = site(&[("start-a-blog-in-2026.md", WORDY)]);
    let output = khalid(site.path(), &["lint", "--fix", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let file = within(site.path(), "content/en/posts/start-a-blog-in-2026.md");
    let cleaned = WORDY.replace(
        "author: asif\n",
        "author: asif\nslug: start-blog\naliases:\n  - /blog/start-a-blog-in-2026/\n",
    );
    assert_eq!(fs::read_to_string(file).unwrap(), cleaned);
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let rules: Vec<&Value> = json["fixed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| &f["rule"])
        .collect();
    assert_eq!(rules, ["slug-stopwords", "slug-date"], "{json}");
    let findings = json["findings"].as_array().unwrap();
    assert!(findings.iter().all(|f| f["slug"] == "start-blog"), "{json}");
}

#[test]
fn lint_fix_adds_to_aliases_and_replaces_the_slug_as_written() {
    let post = WORDY.replace(
        "author: asif\n",
        "author: asif\nslug: the-guide-2026\naliases:\n- /blog/old-guide/\n",
    );
    let site = site(&[("guide.md", &post)]);
    let output = khalid(site.path(), &["lint", "--fix"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let file = within(site.path(), "content/en/posts/guide.md");
    let cleaned = WORDY.replace(
        "author: asif\n",
        "author: asif\nslug: guide\naliases:\n- /blog/the-guide-2026/\n- /blog/old-guide/\n",
    );
    assert_eq!(fs::read_to_string(file).unwrap(), cleaned);
    let said = text(&output.stdout);
    assert!(
        said.contains("/blog/the-guide-2026/ now redirects"),
        "{said}"
    );
}

#[test]
fn lint_fix_leaves_an_address_another_post_already_has() {
    let taken = WORDY.replace("Start a blog in 2026", "Start blog");
    let site = site(&[
        ("start-a-blog-in-2026.md", WORDY),
        ("start-blog.md", &taken),
    ]);
    let output = khalid(site.path(), &["lint", "--fix", "--json"]);
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    assert_eq!(json["fixed"], Value::Array(Vec::new()), "{json}");
    let file = within(site.path(), "content/en/posts/start-a-blog-in-2026.md");
    assert_eq!(fs::read_to_string(file).unwrap(), WORDY);
}
