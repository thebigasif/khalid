//! The Masthead template end to end (docs/templates/masthead.md): `khalid
//! build` of the demo site with `[template] name = "masthead"`, then the
//! header and main part of its front page, a post, a list and the
//! writers page, and the lead story, the latest posts in the grid or list
//! front, and the headlines by section.

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

/// The settings every build here adds to the demo's `site.toml`, before
/// the options a test gives.
const SETTINGS: &str = "\n[template]\nname = \"masthead\"\n[template.options]\ntagline = [\"Notes on search, written slowly.\"]\nlatest_count = 3\n";

/// The demo's post that a test marks featured.
const TITLE_TAG: &str = "content/en/posts/2026-09-21-how-to-write-a-title-tag.md";

/// The demo site built with Masthead and `options` added under
/// `[template.options]`, with each post in `featured` marked featured.
#[cfg(test)]
fn built(options: &str, featured: &[&str]) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(SETTINGS);
    toml.push_str(options);
    fs::write(&settings, toml).unwrap();
    for rel in featured {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', "\nfeatured: true\n", 1)).unwrap();
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

/// The texts of every element `css` selects in `html`.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>())
        .collect()
}

#[test]
fn masthead_front_page() {
    let (_temp, out) = built("", &[]);
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// With no post featured, the newest post leads. The next three, as
/// `latest_count` sets, show as picture cards in the grid front, and the
/// one post left shows as a headline under its section. No post shows
/// twice.
#[test]
fn the_front_page_leads_then_lists_the_latest_then_the_sections() {
    let (_temp, out) = built("", &[]);
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".lead h2"),
        ["Planning a topic cluster in an afternoon"]
    );
    assert_eq!(
        texts(&page, ".front-grid .cards .card h3"),
        [
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not",
            "How to write a title tag",
        ]
    );
    assert_eq!(texts(&page, ".section-column h3"), ["On-page SEO"]);
    assert_eq!(
        texts(&page, ".section-column .feed-item h4"),
        ["Why internal links matter"]
    );
    assert!(page.contains("Notes on search, written slowly."));
    assert_eq!(texts(&page, "h1.visually-hidden").len(), 1);
}

/// A post marked featured leads in place of the newest, and shows only
/// once on the page.
#[test]
fn a_featured_post_leads_the_front_page() {
    let (_temp, out) = built("", &[TITLE_TAG]);
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".lead h2"), ["How to write a title tag"]);
    let cards = texts(&page, ".cards .card h3");
    assert!(
        !cards.contains(&"How to write a title tag".to_owned()),
        "{cards:?}"
    );
    assert!(
        cards.contains(&"Planning a topic cluster in an afternoon".to_owned()),
        "{cards:?}"
    );
}

/// The list front puts one story a row on the front page, on lists and
/// under a post; the grid front, the default, puts picture cards.
#[test]
fn the_list_front_puts_one_story_a_row_everywhere() {
    let (_temp, out) = built("front = \"list\"\n", &[]);
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".front-list .cards-row .card h3").len(), 3);
    assert!(texts(&page, ".front-grid").is_empty());
    let list = shown(&out, "category/technical-seo/index.html");
    assert_eq!(texts(&list, ".front-list .cards-row .card h2").len(), 2);
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert!(!texts(&post, ".related .cards-row").is_empty());
}

#[test]
fn masthead_post() {
    let (_temp, out) = built("", &[]);
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A post opens with its section as a label over a centred title, and
/// its related stories sit under a rule label.
#[test]
fn a_post_opens_with_its_section_and_ends_with_more_stories() {
    let (_temp, out) = built("", &[]);
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".post-header .kicker"), ["Technical SEO"]);
    assert_eq!(texts(&page, ".related .rule-label"), ["More stories"]);
}

#[test]
fn masthead_category_page() {
    let (_temp, out) = built("", &[]);
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn masthead_people_page() {
    let (_temp, out) = built("", &[]);
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
