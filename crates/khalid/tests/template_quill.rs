//! The Quill template end to end (docs/templates/quill.md): `khalid
//! build` of the demo site with `[template] name = "quill"`, then the
//! header and main part of its front page, a post and a list, the one-line
//! list with no pictures, and the short stylesheet with its nib.

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

const SETTINGS: &str = "\n[template]\nname = \"quill\"\n";

/// The demo site built with Quill.
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(SETTINGS);
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

/// How many elements `css` selects in `html`.
#[cfg(test)]
fn count(html: &str, css: &str) -> usize {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector).count()
}

#[test]
fn quill_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The front page lists every post as one line, its title and its date,
/// with no picture and no summary, though a post has a lead picture.
#[test]
fn the_front_page_lists_one_line_a_post_with_no_pictures() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let cards = count(&page, ".cards-compact .card");
    assert!(cards > 1, "{page}");
    assert_eq!(count(&page, ".cards-compact .card h2 a"), cards);
    assert_eq!(count(&page, ".cards-compact .card time"), cards);
    assert_eq!(count(&page, "main img"), 0, "{page}");
    assert_eq!(count(&page, ".card-meta"), 0, "{page}");
}

#[test]
fn quill_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A post keeps its lead picture and its description under the title.
#[test]
fn a_post_shows_its_picture_and_its_dek() {
    let (_temp, out) = built();
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(count(&post, ".post-header img"), 1, "{post}");
    assert_eq!(count(&post, ".post-header .dek"), 1, "{post}");
}

#[test]
fn quill_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

/// The site's stylesheet carries Quill's few rules, the nib among them.
#[test]
fn the_stylesheet_draws_the_nib() {
    let (_temp, out) = built();
    let assets = at(&out, "assets");
    let css: String = fs::read_dir(&assets)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "css"))
        .map(|path| fs::read_to_string(path).unwrap())
        .collect();
    assert!(css.contains(".post-body::after"), "the nib after a post");
    assert!(css.contains(".cards-compact .card"), "the one-line list");
}
