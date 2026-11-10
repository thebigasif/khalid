//! A post's Updates list, end to end (plan 4.12): `khalid build` shows the
//! dated notes on the post page, newest first, and the newest date becomes
//! the page's `dateModified`, its visible updated date and its sitemap
//! `lastmod`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Copies the folder `from` into `into`, file by file.
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

/// A copy of the demo site whose sitemap post carries `front`.
#[cfg(test)]
fn site_with(front: &str) -> tempfile::TempDir {
    let site = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, site.path());
    let post = at(
        site.path(),
        "content/en/posts/2026-09-28-what-a-sitemap-does.md",
    );
    let text = fs::read_to_string(&post).unwrap();
    fs::write(&post, text.replacen("---\n", &format!("---\n{front}"), 1)).unwrap();
    site
}

/// Builds [`site_with`] `front`, and returns the post page and the English
/// posts sitemap.
#[cfg(test)]
fn built_with(front: &str) -> (String, String) {
    let site = site_with(front);
    let out = at(site.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let page = fs::read_to_string(at(&out, "what-a-sitemap-does/index.html")).unwrap();
    let sitemap = fs::read_to_string(at(&out, "sitemaps/posts-en-1.xml")).unwrap();
    (page, sitemap)
}

const UPDATES: &str = "updates:\n  - date: 2026-10-02\n    note: Added the part on image sitemaps.\n  \
                       - date: 2026-11-02\n    note: Refreshed the crawl numbers.\n";

#[test]
fn the_post_page_lists_its_updates_newest_first() {
    let (page, _) = built_with(UPDATES);
    let newest = page
        .find(
            "<li><time datetime=\"2026-11-02\">2 November 2026</time>: Refreshed the crawl numbers.</li>",
        )
        .unwrap_or_else(|| panic!("{page}"));
    let older = page
        .find("<li><time datetime=\"2026-10-02\">2 October 2026</time>: Added the part on image sitemaps.</li>")
        .unwrap_or_else(|| panic!("{page}"));
    assert!(newest < older);
    assert!(page.contains("<h2 id=\"post-updates\">Updates</h2>"));
}

#[test]
fn the_newest_update_is_the_date_modified_the_updated_date_and_the_lastmod() {
    let (page, sitemap) = built_with(UPDATES);
    assert!(
        page.contains("\"dateModified\":\"2026-11-02T00:00:00Z\""),
        "{page}"
    );
    assert!(
        page.contains("Updated <time datetime=\"2026-11-02\">2 November 2026</time>"),
        "{page}"
    );
    let (_, entry) = sitemap
        .split_once("<loc>https://www.example.com/blog/what-a-sitemap-does/</loc>")
        .unwrap();
    assert!(
        entry
            .trim_start()
            .starts_with("<lastmod>2026-11-02T00:00:00Z</lastmod>"),
        "{sitemap}"
    );
}

#[test]
fn the_linter_reads_the_newest_update_as_the_updated_date() {
    let lint = |front: &str| {
        let site = site_with(front);
        let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
            .args(["--site", site.path().to_str().unwrap(), "lint"])
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    // An update dated before the post itself is the wrong date; one after
    // it is fine.
    let early = lint("updates:\n  - date: 2026-01-05\n    note: Fixed a typo.\n");
    assert!(early.contains("updated-date-wrong"), "{early}");
    let later = lint("updates:\n  - date: 2026-10-05\n    note: New numbers.\n");
    assert!(!later.contains("updated-date-wrong"), "{later}");
}

#[test]
fn a_post_without_updates_has_no_updates_section() {
    let (page, _) = built_with("");
    assert!(!page.contains("class=\"updates\""), "{page}");
}
