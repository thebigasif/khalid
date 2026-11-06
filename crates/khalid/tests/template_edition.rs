//! The Edition template end to end: `khalid build` of the demo site with
//! `[template] name = "edition"` and one featured post with a picture,
//! then the header and main part of its front page, that post and an
//! author page.

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

/// The post with a picture, which the tests mark featured.
const FEATURED: &str = "content/en/posts/2026-10-05-fixing-slow-pages/index.md";

/// The demo site built with Edition and `options` under
/// `[template.options]`, a logo from the main site, and the post at
/// [`FEATURED`] marked featured.
#[cfg(test)]
fn built_with(options: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    copy_dir(&at(repo, "../../fixtures/demo-site"), temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str("\n[template]\nname = \"edition\"\n[template.options]\n");
    toml.push_str(options);
    fs::write(&settings, toml).unwrap();
    let tokens = at(temp.path(), "theme/tokens.toml");
    fs::create_dir_all(tokens.parent().unwrap()).unwrap();
    fs::write(
        tokens,
        "[brand]\nlogo = \"https://www.example.com/logo.svg\"\n",
    )
    .unwrap();
    let post = at(temp.path(), FEATURED);
    let text = fs::read_to_string(&post).unwrap();
    fs::write(&post, text.replacen("---\n", "---\nfeatured: true\n", 1)).unwrap();
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

/// The demo site built with Edition's defaults.
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    built_with("")
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
fn edition_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The cover takes the featured post's picture, the featured row shows
/// that post, and the latest posts still list it with the others.
#[test]
fn edition_cover_takes_the_featured_posts_picture() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert!(
        page.contains("<div class=\"cover cover-pictured\">"),
        "{page}"
    );
    let cover = page.split("<div class=\"cover-content\">").next().unwrap();
    assert!(
        cover.contains("alt=\"A bar chart of page weight by part\""),
        "{cover}"
    );
    let featured = texts(&page, ".featured-post h3");
    assert_eq!(
        featured,
        ["Fixing slow pages, starting with the biggest win"]
    );
    let latest = texts(&page, ".feed .card h3");
    assert!(latest.contains(&featured[0]), "{latest:?}");
    assert!(latest.len() > 1, "{latest:?}");
}

/// The cover_text option is the cover's large line, and the blog's name
/// stays the page's one heading for screen readers. A half cover and the
/// minimal feed with its day and month block follow the options.
#[test]
fn edition_front_page_follows_its_options() {
    let (_temp, out) = built_with(
        "cover = \"half\"\ncover_text = \"A weekly letter on search.\"\nfeed_layout = \"minimal\"\n",
    );
    let page = shown(&out, "index.html");
    assert!(page.contains("cover-pictured cover-half"), "{page}");
    assert_eq!(texts(&page, ".cover-text"), ["A weekly letter on search."]);
    assert_eq!(texts(&page, "h1.visually-hidden"), ["Field Notes on SEO"]);
    let dates = texts(&page, ".feed-calendar");
    assert_eq!(
        dates.first().map(String::as_str),
        Some("12 Oct"),
        "{dates:?}"
    );
    assert!(!page.contains("class=\"cards"), "{page}");
}

/// With the cover off, the front page starts with its title.
#[test]
fn edition_front_page_without_a_cover_starts_with_its_title() {
    let (_temp, out) = built_with("cover = \"none\"\n");
    let page = shown(&out, "index.html");
    assert!(!page.contains("class=\"cover"), "{page}");
    assert_eq!(texts(&page, "main > h1"), ["Field Notes on SEO"]);
}

#[test]
fn edition_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn edition_author_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}
