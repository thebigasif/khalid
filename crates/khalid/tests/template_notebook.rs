//! The Notebook template end to end (docs/templates/notebook.md): `khalid
//! build` of the demo site with `[template] name = "notebook"`, one post
//! made a link note by a first fact that is a web address and one given a
//! plain fact, then the header and main part of its front page, a post, a
//! list and the people page, the month headings of the stream and the
//! link notes read from the facts.

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

const SETTINGS: &str = "\n[template]\nname = \"notebook\"\n[template.options]\ntagline = [\"Short notes on search, kept as I go.\"]\n";

/// A link note, whose first fact is an address, and a note with a plain
/// fact, in the order Notebook reads them.
const FACTS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Search Notes\n    value: \"https://www.example.com/sn/titles\"\n  - label: Read in\n    value: \"5 minutes\"\n",
    ),
    (
        "content/en/posts/2026-09-28-what-a-sitemap-does.md",
        "facts:\n  - label: Status\n    value: \"Draft idea\"\n",
    ),
];

/// The demo site built with Notebook, with two posts given facts.
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
    for (rel, facts) in FACTS {
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
fn notebook_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The stream lists every note newest first under a heading for each
/// month, with each note's day and month in the margin.
#[test]
fn the_stream_groups_notes_by_month() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".nb-month-title"),
        ["October 2026", "September 2026"]
    );
    assert_eq!(
        texts(&page, ".nb-stream .nb-title"),
        [
            "Planning a topic cluster in an afternoon",
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not",
            "How to write a title tag",
            "Why internal links matter",
        ]
    );
    assert_eq!(texts(&page, ".nb-stream .nb-day")[0], "12");
    assert!(page.contains("Short notes on search, kept as I go."));
}

/// A first fact that is an address makes a link note: the stream and the
/// post link to it under the fact's label, and the post lists its other
/// facts. A first fact that is not an address is a plain fact.
#[test]
fn a_first_fact_that_is_an_address_makes_a_link_note() {
    let (_temp, out) = built();
    let home = shown(&out, "index.html");
    assert_eq!(texts(&home, ".nb-source a"), ["Read it at Search Notes"]);
    let post = shown(&out, "how-to-write-a-title-tag/index.html");
    assert_eq!(texts(&post, ".nb-link-out a"), ["Read it at Search Notes"]);
    assert!(post.contains("href=\"https://www.example.com/sn/titles\""));
    assert_eq!(texts(&post, ".nb-fact dt"), ["Read in"]);
    let plain = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&plain, ".nb-link-out").is_empty(), "no link note");
    assert_eq!(texts(&plain, ".nb-fact dd"), ["Draft idea"]);
}

#[test]
fn notebook_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn notebook_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn notebook_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
