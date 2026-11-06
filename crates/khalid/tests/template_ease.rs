//! The Ease template end to end: `khalid build` of the demo site with
//! `[template] name = "ease"`, then the header and main part of its front
//! page, a post, a category page and an author page.

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

/// The demo site built with Ease. It runs as a server, so it has a
/// search page for the cover's search box.
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    copy_dir(&at(repo, "../../fixtures/demo-site"), temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str("\n[template]\nname = \"ease\"\n");
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

/// The page at `rel`, checked for one `h1` and unique ids, cut to its
/// header and main part: what a template changes.
#[cfg(test)]
fn shown(out: &Path, rel: &str) -> String {
    let page = fs::read_to_string(at(out, rel)).unwrap();
    let html = scraper::Html::parse_document(&page);
    assert!(html.errors.is_empty(), "{rel}: {:?}", html.errors);
    let select = |css: &str| {
        let selector = scraper::Selector::parse(css).unwrap();
        html.select(&selector)
            .filter_map(|element| element.value().attr("id").or(Some("")))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(select("h1").len(), 1, "{rel}: one h1");
    let mut ids = select("[id]");
    let count = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), count, "{rel}: unique ids");
    let start = page.find("<header class=\"site-header\">").unwrap();
    let end = page.find("</main>").unwrap();
    page.get(start..end).unwrap().to_owned()
}

/// The text of each element of `html` that `css` selects.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .collect()
}

#[test]
fn ease_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Each category is a section of the front page, in the order of its
/// newest post, listing its posts newest first, then a section of the
/// newest posts. The cover's search box sends the words to the search
/// page.
#[test]
fn ease_front_page_has_a_section_per_category_and_a_search_box() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".ease-topic > h2"),
        [
            "Content strategy",
            "Technical SEO",
            "On-page SEO",
            "Latest posts"
        ]
    );
    let technical = texts(&page, ".ease-topic:nth-child(2) .ease-links span");
    assert_eq!(
        technical,
        [
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not"
        ]
    );
    assert!(
        page.contains(
            "<form class=\"ease-search\" role=\"search\" action=\"/blog/search/\" method=\"get\">"
        ),
        "{page}"
    );
    assert!(page.contains("name=\"q\""), "{page}");
}

#[test]
fn ease_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A category lists its posts as plain rows, each title a link.
#[test]
fn ease_category_lists_its_posts_as_rows() {
    let (_temp, out) = built();
    let page = shown(&out, "category/technical-seo/index.html");
    assert_eq!(texts(&page, ".ease-term h1"), ["Technical SEO"]);
    assert_eq!(texts(&page, ".ease-row h2").len(), 2, "{page}");
}

#[test]
fn ease_author_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}
