//! The Ticker template end to end (docs/templates/ticker.md): `khalid
//! build` of the demo site with `[template] name = "ticker"` and two posts
//! given an issue's facts (a market and its move each), then the header
//! and main part of its front page, an issue and a list, and the bar
//! chart and the shaded table built from the facts.

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

const SETTINGS: &str = "\n[template]\nname = \"ticker\"\n[template.options]\ntagline = [\"The week in markets.\"]\ndisclaimer = [\"Not investment advice.\"]\n";

/// Two posts that become issues: each fact a market and its move, in
/// the same order, plus one value that is not a number.
const ISSUES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Stocks\n    value: \"+1.8%\"\n  - label: Bonds\n    value: \"-0.6%\"\n  - label: Copper\n    value: \"+0.9%\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Stocks\n    value: \"-2.4%\"\n  - label: Bonds\n    value: \"+0.3%\"\n  - label: Copper\n    value: \"flat\"\n",
    ),
];

/// The demo site built with Ticker, with two posts made issues.
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
    for (rel, facts) in ISSUES {
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

/// The text of every element `css` selects in `html`, with the value of
/// its `attribute`.
#[cfg(test)]
fn attributes(html: &str, css: &str, attribute: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .filter_map(|element| element.value().attr(attribute))
        .map(str::to_owned)
        .collect()
}

#[test]
fn ticker_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest issue leads beside its chart, the tape shows its facts when
/// no `hero_stats` are set, and every other post sits in the table, so
/// the lead never shows twice.
#[test]
fn the_front_page_leads_with_the_newest_issue_and_its_tape() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let lead = texts(&page, ".lead-issue h2");
    assert_eq!(lead, ["Fixing slow pages, starting with the biggest win"]);
    assert_eq!(texts(&page, ".tape-figure dd"), ["+1.8%", "-0.6%", "+0.9%"]);
    assert_eq!(
        attributes(&page, ".tape-figure", "data-move"),
        ["up", "down", "up"]
    );
    let rows = texts(&page, ".fact-table tbody th a");
    assert!(!rows.is_empty());
    assert!(!rows.contains(&lead[0]), "{rows:?}");
    assert!(page.contains("The week in markets."));
    assert_eq!(texts(&page, ".disclaimer"), ["Not investment advice."]);
}

/// The chart scales each bar to the largest value in twenty steps, grows
/// both ways when a value is below zero, and draws no bar for a value
/// that is not a number.
#[test]
fn an_issue_draws_its_facts_as_a_bar_chart() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert!(page.contains("<figure class=\"bar-chart\" data-both-ways>"));
    assert_eq!(attributes(&page, ".bar", "data-w"), ["20", "7", "10"]);
    assert_eq!(
        attributes(&page, ".bar-row", "data-move"),
        ["up", "down", "up"]
    );
    assert_eq!(texts(&page, ".bar-note"), ["Not investment advice."]);
    let flat = shown(&out, "how-to-write-a-title-tag/index.html");
    assert_eq!(texts(&flat, ".bar-value"), ["-2.4%", "+0.3%", "flat"]);
    assert_eq!(attributes(&flat, ".bar", "data-w").len(), 2);
    let plain = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&plain, ".bar-chart").is_empty(), "no facts, no chart");
}

/// A list is a table with one column per fact, headed by the first
/// issue's labels, and each figure cell shaded by its size in its column.
#[test]
fn a_list_is_a_table_shaded_by_each_move() {
    let (_temp, out) = built();
    let page = shown(&out, "archive/index.html");
    assert_eq!(
        texts(&page, ".fact-table thead th"),
        ["Post", "Date", "Stocks", "Bonds", "Copper"]
    );
    assert_eq!(
        attributes(&page, ".fact-table-figure[data-heat]", "data-heat"),
        ["3", "4", "4", "4", "2"]
    );
}

#[test]
fn ticker_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn ticker_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}
