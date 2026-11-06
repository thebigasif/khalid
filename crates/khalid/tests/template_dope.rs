//! The Dope template end to end: `khalid build` of the demo site with
//! `[template] name = "dope"` and a logo, then the header and main part of
//! its front page, a post and an author page.

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

/// The demo site built with Dope, a logo from the main site, and `options`
/// under `[template.options]`.
#[cfg(test)]
fn built(options: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str("\n[template]\nname = \"dope\"\n[template.options]\n");
    toml.push_str(options);
    fs::write(&settings, toml).unwrap();
    let tokens = at(temp.path(), "theme/tokens.toml");
    fs::create_dir_all(tokens.parent().unwrap()).unwrap();
    fs::write(
        tokens,
        "[brand]\nlogo = \"https://www.example.com/logo.svg\"\n",
    )
    .unwrap();
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

#[test]
fn dope_front_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "index.html"));
}

#[test]
fn dope_post() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn dope_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}

#[test]
fn the_front_page_shows_one_tile_per_topic_linked_to_its_page() {
    let (_temp, out) = built("");
    let page = shown(&out, "index.html");
    let shelf = page.split_once("<ul class=\"shelf-list\"").unwrap().1;
    let shelf = shelf.split_once("</ul>").unwrap().0;
    assert_eq!(
        shelf.matches("<li class=\"shelf-tile\">").count(),
        3,
        "{shelf}"
    );
    for topic in ["technical-seo", "on-page-seo", "content-strategy"] {
        let link = format!("<h2><a href=\"/blog/category/{topic}/\">");
        assert!(shelf.contains(&link), "{topic}: {shelf}");
    }
    assert!(page.contains("<section class=\"feed\""), "{page}");
}

#[test]
fn a_post_opens_on_its_picture_then_its_topic_and_title() {
    let (_temp, out) = built("");
    let page = shown(&out, "fixing-slow-pages/index.html");
    let media = page.find("<div class=\"post-media\">").unwrap();
    let topic = page.find("<p class=\"post-tags\">").unwrap();
    let title = page.find("<h1>").unwrap();
    assert!(media < topic && topic < title, "{page}");
}
