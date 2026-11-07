//! The Gazette template end to end (docs/templates/gazette.md): `khalid
//! build` of the demo site with `[template] name = "gazette"` and two
//! posts given a place as their first fact, then the header and main
//! part of its front page, a story, a section and the reporters page: the
//! top story, the latest news rail grouped by day, a block per section
//! under its flag, and the place as a label and as the dateline.

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

const SETTINGS: &str = "\n[template]\nname = \"gazette\"\n[template.options]\ntagline = [\"News for the whole valley\"]\n";

/// Two posts given a place, the first fact Gazette reads.
const PLACES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Where\n    value: Mill Lane\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Where\n    value: Market Square\n",
    ),
];

/// The demo site built with Gazette, with two posts given a place.
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
    for (rel, facts) in PLACES {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', &format!("\n{facts}"), 1)).unwrap();
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
fn gazette_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest post is the top story and does not show again in its
/// section, so Content strategy, which holds only that post, gets no
/// block and no link in the index. Each other section shows its newest
/// story, then its other posts as headlines.
#[test]
fn gazette_front_page_has_a_block_per_section() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".gz-top h2"),
        ["Planning a topic cluster in an afternoon"]
    );
    assert_eq!(
        texts(&page, ".gz-flag-name"),
        ["Technical SEO", "On-page SEO"]
    );
    assert_eq!(
        texts(&page, ".gz-index a"),
        ["Technical SEO", "On-page SEO"]
    );
    assert_eq!(
        texts(&page, ".gz-story h3"),
        [
            "Fixing slow pages, starting with the biggest win",
            "How to write a title tag"
        ]
    );
    assert_eq!(
        texts(&page, ".gz-sections .headline-row h3"),
        [
            "What a sitemap does, and what it does not",
            "Why internal links matter"
        ]
    );
    assert!(page.contains("News for the whole valley"));
}

/// The rail lists the newest posts under a heading for each day, and a
/// post with a place carries it as a label, with its fact's label for
/// screen readers.
#[test]
fn the_rail_groups_the_latest_news_by_day() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".gz-rail .headline-day").len(), 5);
    assert_eq!(texts(&page, ".gz-rail .headline-row h4").len(), 5);
    assert_eq!(
        texts(&page, ".gz-rail .headline-place"),
        ["Where: Mill Lane", "Where: Market Square"]
    );
}

#[test]
fn gazette_story() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A story opens with its section as the flag, and a story with a place
/// starts its text with the dateline. A story without one has none.
#[test]
fn a_story_shows_its_section_and_dateline() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".gz-kicker"), ["Technical SEO"]);
    assert_eq!(texts(&page, ".gz-dateline"), ["Where: Mill Lane"]);
    let other = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&other, ".gz-dateline").is_empty(),
        "no place, no dateline"
    );
}

#[test]
fn gazette_section_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn gazette_reporters_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
