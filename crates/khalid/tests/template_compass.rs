//! The Compass template (templates spec 3.4), end to end: `khalid build`
//! of the demo site with `[template] name = "compass"` gives breadcrumbs,
//! an open contents box, the author's name without a photo, share links
//! and the author box after the post, picture cards on the lists, and a
//! site's own options and `[reader]` keys win.

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

/// Builds a copy of the demo site in Compass with `table` added to its
/// `site.toml`, and expects success.
#[cfg(test)]
fn built(table: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str("\n[template]\nname = \"compass\"\n");
    toml.push_str(table);
    fs::write(&settings, toml).unwrap();
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

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

/// The page's `main` element, the part a template changes.
#[cfg(test)]
fn main_of(page: &str) -> &str {
    let (_, main) = page.split_once("<main").unwrap();
    let (main, _) = main.split_once("</main>").unwrap();
    main
}

#[test]
fn the_compass_index_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(main_of(&page(&out, "index.html")));
}

#[test]
fn the_compass_front_page_lists_the_newest_posts_as_one_titled_section() {
    let (_temp, out) = built("");
    let home = page(&out, "index.html");
    assert!(
        home.contains("<section class=\"home-section\" id=\"latest\" aria-labelledby=\"latest-title\">\n<h2 id=\"latest-title\">Recently published</h2>\n<ul class=\"cards\" role=\"list\">"),
        "{home}"
    );
    assert!(!home.contains("class=\"jump-links\""), "{home}");
}

#[test]
fn the_compass_post_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(main_of(&page(&out, "what-a-sitemap-does/index.html")));
}

#[test]
fn the_compass_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(main_of(&page(&out, "authors/lena-vogt/index.html")));
}

#[test]
fn a_compass_post_opens_with_its_trail_and_contents_and_ends_with_its_author() {
    let (_temp, out) = built("");
    let post = page(&out, "what-a-sitemap-does/index.html");
    let crumbs = post.find("<nav class=\"breadcrumbs\"").unwrap();
    let contents = post.find("<nav class=\"toc-box\"").unwrap();
    let body = post.find("<div class=\"post-body\">").unwrap();
    let share = post.rfind("<div class=\"share\">").unwrap();
    let author = post.find("<section class=\"author-box\">").unwrap();
    assert!(crumbs < contents && contents < body, "{post}");
    assert!(body < share && share < author, "{post}");
    assert!(post.contains("<nav class=\"toc-box\" aria-label=\"Contents\">\n<p class=\"toc-title\">Contents</p>\n<ol>"), "{post}");
    assert!(!post.contains("<ul class=\"people\""), "{post}");
    assert!(!post.contains("class=\"related"), "{post}");
    assert!(!post.contains("class=\"reading-time\""), "{post}");
    let (_, sheet) = post
        .split_once("<link rel=\"stylesheet\" href=\"/blog/")
        .unwrap();
    let (sheet, _) = sheet.split_once('"').unwrap();
    assert!(page(&out, sheet).contains(".jump-links"), "{sheet}");
}

#[test]
fn a_site_changes_compass_options_and_its_reader_keys_win() {
    let (_temp, out) = built(
        "[template.options]\nbreadcrumbs = false\ncontents = \"none\"\n\n[reader]\nbyline = true\n",
    );
    let post = page(&out, "what-a-sitemap-does/index.html");
    assert!(!post.contains("class=\"breadcrumbs\""), "{post}");
    assert!(!post.contains("class=\"toc-"), "{post}");
    assert!(post.contains("<ul class=\"people\""), "{post}");
}
