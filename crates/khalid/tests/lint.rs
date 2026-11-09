//! `khalid lint`: every rule over a site folder, in sentences or JSON, with
//! exit code 1 on a block (KH-LNT-3, RULES.md BL-CLI-1 to BL-CLI-3).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

#[path = "lint/fix.rs"]
mod fix;
#[path = "lint/translation.rs"]
mod translation;

/// A post that passes every block: title, date, author and an internal link.
const GOOD: &str = "---
title: Why internal links matter
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
---

An internal link points to [how to pick link text](/blog/choosing-link-text/).
";

/// The same post with no date, which blocks publishing.
const UNDATED: &str = "---
title: Choosing link text
author: asif
focus_keyword: link text
---

Good link text names the page it opens, like [why links matter](/blog/why-internal-links-matter/).

![Link text that names the page it opens](https://example.org/link-text.png)
";

/// A site folder with the demo settings and `posts` as `(file name, text)`.
#[cfg(test)]
fn site(posts: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let settings = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/demo-site/site.toml"
    );
    fs::copy(settings, within(dir.path(), "site.toml")).unwrap();
    let authors = within(dir.path(), "content/en/authors");
    fs::create_dir_all(&authors).unwrap();
    fs::write(
        within(&authors, "asif.md"),
        "---\nname: Asif\nslug: asif\n---\n",
    )
    .unwrap();
    let folder = within(dir.path(), "content/en/posts");
    fs::create_dir_all(&folder).unwrap();
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

#[test]
fn lint_passes_a_site_with_only_warnings() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", GOOD)]);
    let output = khalid(site.path(), &["lint"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let said = text(&output.stdout);
    assert!(said.contains("0 blocks"), "{said}");
}

#[test]
fn lint_blocks_a_post_with_no_date_and_says_why() {
    let site = site(&[
        ("2026-09-14-why-internal-links-matter.md", GOOD),
        ("choosing-link-text.md", UNDATED),
    ]);
    let output = khalid(site.path(), &["lint"]);
    assert_eq!(output.status.code(), Some(1));
    let said = text(&output.stdout);
    assert!(
        said.contains("en/choosing-link-text: blocks publishing"),
        "{said}"
    );
    assert!(said.contains("This post has no date."), "{said}");
    assert!(said.contains("Add `date:` to the header."), "{said}");
}

#[test]
fn lint_json_prints_the_report_and_still_exits_1() {
    let site = site(&[("choosing-link-text.md", UNDATED)]);
    let output = khalid(site.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(1));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    assert_eq!(json["blocks"], true);
    let first = &json["findings"][0];
    assert_eq!(first["rule"], "no-date");
    assert_eq!(first["slug"], "choosing-link-text");
    assert_eq!(first["guide"], "post-dates");
}

#[test]
fn lint_prints_each_post_below_100_with_its_score() {
    let site = site(&[
        ("2026-09-14-why-internal-links-matter.md", GOOD),
        ("choosing-link-text.md", UNDATED),
    ]);
    let output = khalid(site.path(), &["lint"]);
    let said = text(&output.stdout);
    assert!(said.contains("Scores out of 100:"), "{said}");
    assert!(said.contains("  en/choosing-link-text: 40\n"), "{said}");
    let one = "content/en/posts/choosing-link-text.md";
    let said = text(&khalid(site.path(), &["lint", one]).stdout);
    assert!(said.contains("Score: 40 out of 100.\n"), "{said}");
    assert!(!said.contains("Scores out of 100:"), "{said}");
}

#[test]
fn lint_json_scores_every_post_and_one_post_alone() {
    let site = site(&[
        ("2026-09-14-why-internal-links-matter.md", GOOD),
        ("choosing-link-text.md", UNDATED),
    ]);
    let output = khalid(site.path(), &["lint", "--json"]);
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let scores = json["scores"].as_array().unwrap();
    assert_eq!(scores.len(), 2, "{json}");
    let undated = scores.iter().find(|s| s["slug"] == "choosing-link-text");
    let undated = undated.unwrap();
    assert_eq!(undated["locale"], "en");
    assert_eq!(undated["value"], 40);
    assert_eq!(undated["lost"][0]["rule"], "no-date");

    let one = "content/en/posts/choosing-link-text.md";
    let output = khalid(site.path(), &["lint", one, "--json"]);
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    assert_eq!(json["score"]["value"], 40, "{json}");
    assert_eq!(json.get("scores"), None);
}

#[test]
fn lint_checks_only_the_post_it_is_given() {
    let site = site(&[
        ("2026-09-14-why-internal-links-matter.md", GOOD),
        ("choosing-link-text.md", UNDATED),
    ]);
    let good = "content/en/posts/2026-09-14-why-internal-links-matter.md";
    let output = khalid(site.path(), &["lint", good, "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    assert!(
        findings
            .iter()
            .all(|f| f["slug"] == "why-internal-links-matter")
    );
}

/// A good post whose address has an accented letter.
const ACCENTED: &str = "---
title: Café menus that sell
slug: café-menus
date: 2026-09-14
author: asif
description: How a short menu on the café wall helps people choose and order faster.
---

Short menus work like [good link text](/blog/choosing-link-text/).
";

#[test]
fn lint_warns_about_an_accented_address_on_a_plain_address_site() {
    let site = site(&[("2026-09-14-cafe-menus.md", ACCENTED)]);
    let output = khalid(site.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    let found = findings.iter().find(|f| f["rule"] == "unicode-slug");
    let found = found.unwrap_or_else(|| panic!("no unicode-slug in {json}"));
    assert_eq!(found["severity"], "warn");
    assert_eq!(found["subject"], "cafe-menus");
}

/// A good post that links to its own site over `http://`; the demo site's
/// address is `https://www.example.com`.
const INSECURE: &str = "---
title: Why internal links matter
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
---

An internal link points to [how to pick link text](/blog/choosing-link-text/), and
an old one to [pruning posts](http://example.com/blog/pruning-old-posts/).
";

#[test]
fn lint_warns_about_a_link_to_the_own_site_over_http() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", INSECURE)]);
    let output = khalid(site.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    let found = findings
        .iter()
        .find(|f| f["rule"] == "insecure-internal-link");
    let found = found.unwrap_or_else(|| panic!("no insecure-internal-link in {json}"));
    assert_eq!(found["severity"], "warn");
    assert_eq!(
        found["subject"],
        "http://example.com/blog/pruning-old-posts/"
    );
}

/// A good post whose search title holds its focus keyword and whose
/// headline does not.
const SPLIT_TITLE: &str = "---
title: Why links between your posts matter
seo_title: \"Internal links, a guide {sep} {site}\"
focus_keyword: internal links
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
---

Internal links point to [how to pick link text](/blog/choosing-link-text/).
";

#[test]
fn lint_warns_when_the_headline_lacks_the_focus_keyword_the_search_title_has() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", SPLIT_TITLE)]);
    let output = khalid(site.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    let rules: Vec<&Value> = findings.iter().map(|f| &f["rule"]).collect();
    assert!(
        !rules.contains(&&Value::from("focus-not-in-title")),
        "{json}"
    );
    let found = findings.iter().find(|f| f["rule"] == "focus-not-in-h1");
    let found = found.unwrap_or_else(|| panic!("no focus-not-in-h1 in {json}"));
    assert_eq!(found["severity"], "warn");
    assert_eq!(found["subject"], "internal links");
}

/// A good post written in three very long sentences of long words, under
/// a heading with no full stop.
const DENSE: &str = "---
title: Why internal links matter
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
---

## Documentation at scale

Organizations implementing comprehensive internationalization strategies frequently underestimate the considerable administrative complexity introduced whenever multilingual documentation requires synchronized maintenance across numerous independently managed publication environments, particularly where contributors collaborate without [standardized terminology](/blog/choosing-link-text/).

Consequently, editorial departments responsible for authoritative reference material increasingly demand sophisticated verification procedures, systematic translation memories, comprehensive revision histories, and meticulously documented approval workflows before anything reaches publicly accessible destinations or downstream syndication partners.

Unfortunately, organizations attempting simultaneous modernization of infrastructure, governance, localization, accessibility, and performance commonly discover that interdependent requirements multiply exponentially, transforming seemingly straightforward improvements into lengthy, expensive, politically complicated undertakings that exhaust available engineering capacity and postpone measurable benefits indefinitely.
";

#[test]
fn lint_warns_about_a_post_that_is_hard_to_read() {
    let dense = site(&[("2026-09-14-why-internal-links-matter.md", DENSE)]);
    let output = khalid(dense.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    let found = findings.iter().find(|f| f["rule"] == "hard-to-read");
    let found = found.unwrap_or_else(|| panic!("no hard-to-read in {json}"));
    assert_eq!(found["severity"], "warn");
    let easy = site(&[("2026-09-14-why-internal-links-matter.md", GOOD)]);
    let output = khalid(easy.path(), &["lint", "--json"]);
    assert!(!text(&output.stdout).contains("hard-to-read"));
}

/// A good post that moved: its old path redirects to it.
const MOVED: &str = "---
title: Choosing link text
date: 2026-09-10
author: asif
description: Good link text names the page it opens, so readers know where they go.
aliases:
  - /blog/link-text-tips/
---

Good link text names the page it opens, like [why links matter](/blog/why-internal-links-matter/).

![Link text that names the page it opens](https://example.org/link-text.png)
";

/// A good post that still links to the moved post's old path.
const OLD_LINK: &str = "---
title: Why internal links matter
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
---

An internal link points to [how to pick link text](/blog/link-text-tips/).
";

#[test]
fn lint_warns_about_a_link_to_an_old_address_of_a_moved_post() {
    let site = site(&[
        ("2026-09-10-choosing-link-text.md", MOVED),
        ("2026-09-14-why-internal-links-matter.md", OLD_LINK),
    ]);
    let output = khalid(site.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    let found = findings.iter().find(|f| f["rule"] == "redirecting-link");
    let found = found.unwrap_or_else(|| panic!("no redirecting-link in {json}"));
    assert_eq!(found["slug"], "why-internal-links-matter");
    assert_eq!(found["subject"], "/blog/link-text-tips/");
}

#[test]
fn lint_names_a_post_it_cannot_find() {
    let site = site(&[("choosing-link-text.md", UNDATED)]);
    let output = khalid(site.path(), &["lint", "content/en/posts/missing.md"]);
    assert_eq!(output.status.code(), Some(1));
    let said = text(&output.stderr);
    assert!(said.contains("content/en/posts/missing.md"), "{said}");
}

#[test]
fn a_mistake_in_lint_toml_names_its_line() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", GOOD)]);
    fs::write(
        within(site.path(), "lint.toml"),
        "[rules]\nno-dates = \"off\"\n",
    )
    .unwrap();
    let output = khalid(site.path(), &["lint"]);
    assert_eq!(output.status.code(), Some(1));
    let said = text(&output.stderr);
    assert!(said.contains("line 2"), "{said}");
    assert!(said.contains("no-dates"), "{said}");
}

#[test]
fn lint_toml_turns_a_block_into_a_warning() {
    let site = site(&[("choosing-link-text.md", UNDATED)]);
    fs::write(
        within(site.path(), "lint.toml"),
        "[rules]\nno-date = \"warn\"\n",
    )
    .unwrap();
    let output = khalid(site.path(), &["lint"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
}

#[test]
fn lint_needs_a_site_folder() {
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("lint")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(text(&output.stderr).contains("needs a site folder"));
}
