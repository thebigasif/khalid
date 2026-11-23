//! `khalid lint` on ordinary markdown links to the site's own addresses
//! (backlog 2026-11-12 row 18): a link to an address with no page is
//! `broken-internal-link`, and a link to an address moved by a stored
//! redirect is `redirecting-link`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

/// The post the others link to.
const TARGET: &str = "---
title: Choosing link text
date: 2026-09-10
author: asif
description: Good link text names the page it opens, so readers know where they go.
---

Good link text names the page it opens, like [why links matter](/blog/why-internal-links-matter/).
";

/// A post with a link to a post that exists, one to an address that has
/// no page, one to an old address, a file and the main site beside the
/// blog.
const LINKS: &str = "---
title: Why internal links matter
date: 2026-09-14
author: asif
description: Internal links help readers and search engines find the rest of your site.
---

Read [how to pick link text](/blog/choosing-link-text/), the
[guide we never wrote](/blog/no-such-guide/), the
[old tips](/blog/link-text-tips/), the [checklist](/blog/checklist.pdf)
and [our pricing](/pricing/).
";

/// A site folder with the demo settings and the two posts.
#[cfg(test)]
fn site() -> tempfile::TempDir {
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
    fs::write(within(&folder, "2026-09-10-choosing-link-text.md"), TARGET).unwrap();
    fs::write(
        within(&folder, "2026-09-14-why-internal-links-matter.md"),
        LINKS,
    )
    .unwrap();
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

/// The subjects of the findings of `rule` in `khalid lint --json`.
#[cfg(test)]
fn subjects(site: &Path, rule: &str) -> Vec<String> {
    let output = khalid(site, &["lint", "--json"]);
    let said = String::from_utf8_lossy(&output.stdout).into_owned();
    assert_eq!(output.status.code(), Some(0), "{said}");
    let json: Value = serde_json::from_str(&said).unwrap();
    let findings = json["findings"].as_array().unwrap();
    let found = findings.iter().filter(|f| f["rule"] == rule);
    found
        .map(|f| f["subject"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn a_link_to_an_address_with_no_page_is_named() {
    let site = site();
    let broken = subjects(site.path(), "broken-internal-link");
    assert_eq!(broken, ["/blog/no-such-guide/", "/blog/link-text-tips/"]);
    assert!(!within(site.path(), "data").exists(), "lint makes no data/");
}

#[test]
fn a_stored_redirect_turns_a_broken_link_into_a_redirecting_one() {
    let site = site();
    let csv = within(site.path(), "moves.csv");
    fs::write(
        &csv,
        "from,to,status,match\n/blog/link-text-tips/,/blog/choosing-link-text/,301,exact\n",
    )
    .unwrap();
    let csv = csv.to_str().unwrap();
    let output = khalid(site.path(), &["redirects", "import", csv]);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let broken = subjects(site.path(), "broken-internal-link");
    assert_eq!(broken, ["/blog/no-such-guide/"]);
    let moved = subjects(site.path(), "redirecting-link");
    assert_eq!(moved, ["/blog/link-text-tips/"]);
}
