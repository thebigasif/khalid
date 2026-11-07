//! The Brief template end to end (docs/templates/brief.md): `khalid
//! build` of the demo site with `[template] name = "brief"` and two posts
//! given an issue's facts (its number and how many items it holds), then
//! the header and main part of its front page, an issue and a list, and
//! the stamp line, item ticks and dated rows built from them.

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

const SETTINGS: &str = "\n[template]\nname = \"brief\"\n[template.options]\ntagline = [\"Five things, every morning.\"]\n";

/// Two posts that become issues: the newest with its number and five
/// items, and an older one whose item count holds no digits.
const ISSUES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-12-planning-a-topic-cluster.md",
        "facts:\n  - label: Issue\n    value: \"No. 12\"\n  - label: Items\n    value: \"5\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Issue\n    value: \"11\"\n  - label: Items\n    value: \"seven\"\n",
    ),
];

/// The demo site built with Brief, with two posts made issues.
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
fn brief_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest post leads with its number and a tick for each of its five
/// items, and every other post is a dated row, so the lead never shows
/// twice.
#[test]
fn the_front_page_leads_with_the_newest_issue_and_its_ticks() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let lead = texts(&page, ".lead-title");
    assert_eq!(lead, ["Planning a topic cluster in an afternoon"]);
    assert_eq!(texts(&page, ".lead-issue .brief-number"), ["No. 12"]);
    assert_eq!(texts(&page, ".lead-issue .progress-label"), ["5 items"]);
    assert_eq!(texts(&page, ".lead-issue .progress-step").len(), 5);
    let rows = texts(&page, ".brief-issue h3 a");
    assert!(!rows.is_empty(), "expected earlier issues");
    assert!(!rows.contains(&lead[0]), "{rows:?}");
    assert!(page.contains("Five things, every morning."));
}

/// Each row's date is a stamp of weekday, day and month, and its counts
/// show only what holds digits: "seven" items shows no count.
#[test]
fn a_list_dates_each_issue_and_counts_only_digits() {
    let (_temp, out) = built();
    let page = shown(&out, "archive/index.html");
    assert_eq!(
        attributes(&page, ".brief-day", "datetime")[..2],
        ["2026-10-12", "2026-10-05"]
    );
    assert_eq!(texts(&page, ".brief-weekday")[1], "Mon");
    assert_eq!(texts(&page, ".brief-month")[1], "Oct");
    let metas = texts(&page, ".brief-meta");
    let title_tag = metas.iter().find(|meta| meta.contains("No. 11")).unwrap();
    assert!(!title_tag.contains("items"), "{title_tag}");
}

/// An issue ends on the sign-off line; a post without facts has no ticks
/// and no sign-off.
#[test]
fn an_issue_signs_off_and_a_plain_post_does_not() {
    let (_temp, out) = built();
    let issue = shown(&out, "planning-a-topic-cluster/index.html");
    assert_eq!(
        texts(&issue, ".brief-end"),
        ["That's the brief. See you next issue."]
    );
    assert_eq!(texts(&issue, ".post-header .progress-step").len(), 5);
    let plain = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&plain, ".brief-end").is_empty(),
        "no facts, no sign-off"
    );
    assert!(
        texts(&plain, ".brief-ticks").is_empty(),
        "no facts, no ticks"
    );
}

#[test]
fn brief_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "planning-a-topic-cluster/index.html"));
}

#[test]
fn brief_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}
