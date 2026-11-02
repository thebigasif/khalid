//! The Meridian template (templates spec 3.9), end to end: `khalid build`
//! of the demo site with `[template] name = "meridian"`, and the main part
//! of its home, post and author pages as snapshots.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = at(into, &entry.file_name().to_string_lossy());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// The settings every test here adds to the demo's `site.toml`.
const MERIDIAN: &str = "\n[template]\nname = \"meridian\"\n\n[template.cta]\nlabel = \"Book a demo\"\nhref = \"https://www.example.com/demo/\"\nnote = \"See your own blog in Meridian.\"\n";

/// Builds a copy of the demo site in Meridian, with `extra` added to its
/// `site.toml`, one post marked featured, and each of `files` written.
#[cfg(test)]
fn built(extra: &str, files: &[(&str, &str)]) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let toml = fs::read_to_string(&settings).unwrap();
    fs::write(&settings, format!("{toml}{MERIDIAN}{extra}")).unwrap();
    let post = at(temp.path(), FEATURED);
    let text = fs::read_to_string(&post).unwrap();
    fs::write(&post, text.replacen('\n', "\nfeatured: true\n", 1)).unwrap();
    for (rel, text) in files {
        fs::write(at(temp.path(), rel), text).unwrap();
    }
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    (temp, out)
}

/// The page at `rel`, from the site header to the end of `<main>`.
#[cfg(test)]
fn shown(out: &Path, rel: &str) -> String {
    let page = fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"));
    let start = page.find("<header class=\"site-header\">").unwrap();
    let end = page.find("</main>").unwrap();
    page.get(start..end).unwrap().to_owned()
}

const FEATURED: &str = "content/en/posts/2026-09-21-how-to-write-a-title-tag.md";

/// A long guide in Meridian's reference layout, with sections and parts.
const GUIDE: (&str, &str) = (
    "content/en/posts/2026-10-19-robots-txt-guide.md",
    "---
title: The robots.txt guide
slug: robots-txt-guide
date: 2026-10-19
updated: 2026-10-26
author: lena-vogt
category: technical-seo
template: reference
description: What robots.txt does, what it cannot do, and how to write one that keeps crawlers out of the pages you do not want crawled.
---

:::callout{kind=summary}
- robots.txt asks crawlers to stay out of some paths.
- It does not hide a page from search results.
:::

## What robots.txt does

It is a plain text file at the root of your site that crawlers read first.

### Who reads it

Search engine crawlers that follow the rules read it before anything else.

### Who ignores it

Scrapers that do not follow the rules ignore it.

## How to write one

Start with one rule per path you want left alone, then list your sitemap, as [what a sitemap does](/blog/what-a-sitemap-does/) explains.

## How to check it

Open the file in a browser at /robots.txt and read each rule aloud.

:::callout{kind=note}
Test every change before you publish it. A wrong rule can hide your whole blog.
:::
",
);

#[test]
fn the_meridian_home_page() {
    let (_temp, out) = built("", &[]);
    insta::assert_snapshot!(shown(&out, "index.html"));
}

#[test]
fn a_meridian_post() {
    let (_temp, out) = built("", &[]);
    insta::assert_snapshot!(shown(&out, "why-internal-links-matter/index.html"));
}

#[test]
fn a_meridian_author_page() {
    let (_temp, out) = built("", &[]);
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}

#[test]
fn a_meridian_reference_post() {
    let (_temp, out) = built("", &[GUIDE]);
    insta::assert_snapshot!(shown(&out, "robots-txt-guide/index.html"));
}

#[test]
fn meridian_ships_its_own_rules_after_the_shared_ones() {
    let (_temp, out) = built("", &[]);
    let assets = at(&out, "assets");
    let css = fs::read_dir(&assets)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("site.") && name.ends_with(".css")
        })
        .unwrap();
    let css = fs::read_to_string(css).unwrap();
    assert!(css.contains("--text-h1: clamp(2.5rem,"), "{css}");
    assert!(css.contains("--measure: 42rem;"), "{css}");
    assert!(css.contains("--font-heading: \"Iowan Old Style\""), "{css}");
    assert!(css.contains("--color-accent: #2c3f9c;"), "{css}");
    assert!(css.contains("minmax(0, 38%)"), "{css}");
}

/// The home page opens with the newest post as the lead story, then the
/// editor's picks as text, then the rest of the posts as rows.
#[test]
fn the_meridian_home_page_leads_with_the_newest_post() {
    let (_temp, out) = built("", &[]);
    let home = shown(&out, "index.html");
    let lead = home
        .find("<div class=\"lead\">")
        .unwrap_or_else(|| panic!("{home}"));
    let picks = home
        .find("<section class=\"picks\">")
        .unwrap_or_else(|| panic!("{home}"));
    assert!(lead < picks, "{home}");
    assert!(home.contains("<ul class=\"cards cards-text\""), "{home}");
    assert_eq!(home.matches("<div class=\"lead\">").count(), 1, "{home}");
}

/// The masthead shows a home intro the site wrote, and never the built-in
/// description every blog gets.
#[test]
fn the_meridian_masthead_shows_only_the_site_s_own_intro() {
    let (_temp, out) = built("", &[]);
    let home = shown(&out, "index.html");
    assert!(home.contains("<header class=\"masthead\">"), "{home}");
    assert!(!home.contains("The newest posts on"), "{home}");
}

/// A post ends with its author and three related posts with pictures.
#[test]
fn a_meridian_post_ends_with_its_author_and_related_posts() {
    let (_temp, out) = built("", &[]);
    let post = shown(&out, "why-internal-links-matter/index.html");
    assert!(post.contains("<section class=\"author-box\">"), "{post}");
    assert!(
        post.contains("<section class=\"related related-scroll\">"),
        "{post}"
    );
}

#[test]
fn a_site_option_wins_over_meridian_s_default() {
    let (_temp, out) = built("[template.options]\ncards = \"text\"\n", &[]);
    let home = shown(&out, "index.html");
    assert!(home.contains("<ul class=\"cards cards-text\""), "{home}");
}
