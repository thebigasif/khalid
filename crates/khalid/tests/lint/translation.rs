//! The missing-translation warning: a post with no version in one of the
//! languages the site declares, and the quiet once each language has one.

use std::fs;

use serde_json::Value;

use super::{khalid, site, text, within};

/// The demo site declares English and German. This English post names its
/// translation key and has no German version yet.
const TRANSLATED: &str = "---
title: Why internal links matter
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
translation: internal-links
---

An internal link points to [how to pick link text](/blog/choosing-link-text/).
";

#[test]
fn lint_warns_about_a_post_with_no_version_in_a_language_the_site_has() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", TRANSLATED)]);
    let output = khalid(site.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    let missing: Vec<&Value> = findings
        .iter()
        .filter(|f| f["rule"] == "missing-translation")
        .collect();
    assert_eq!(missing.len(), 1, "{json:#}");
    assert_eq!(missing[0]["severity"], "warn");
    assert_eq!(missing[0]["slug"], "why-internal-links-matter");
    assert_eq!(missing[0]["subject"], "de");
    let said = text(&khalid(site.path(), &["lint"]).stdout);
    assert!(
        said.contains("This post has no version in one of your site's languages."),
        "{said}"
    );
    assert!(said.contains("  About: de\n"), "{said}");
}

/// The German version of [`TRANSLATED`], sharing its translation key.
const TRANSLATED_DE: &str = "---
title: Interne Links
slug: interne-links
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
translation: internal-links
---

An internal link points to [the English version](/blog/why-internal-links-matter/).
";

#[test]
fn lint_is_quiet_about_translation_once_each_language_has_a_version() {
    let site = site(&[("2026-09-14-why-internal-links-matter.md", TRANSLATED)]);
    let german = within(site.path(), "content/de/posts");
    fs::create_dir_all(&german).unwrap();
    fs::write(
        within(&german, "2026-09-14-interne-links.md"),
        TRANSLATED_DE,
    )
    .unwrap();
    let output = khalid(site.path(), &["lint", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let json: Value = serde_json::from_str(&text(&output.stdout)).unwrap();
    let findings = json["findings"].as_array().unwrap();
    assert!(
        findings.iter().all(|f| f["rule"] != "missing-translation"),
        "{json:#}"
    );
}
