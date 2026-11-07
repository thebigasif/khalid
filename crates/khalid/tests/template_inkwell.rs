//! The Inkwell template end to end (docs/templates/inkwell.md): `khalid
//! build` of the demo site with `[template] name = "inkwell"`, two posts
//! given an issue number and parts as facts and one a first fact too long
//! to be a number, then the header and main part of its front page, an
//! issue, a list and the people page.

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

const SETTINGS: &str = "\n[template]\nname = \"inkwell\"\n[template.options]\ntagline = [\"A monthly letter since 2019\"]\nsignup_note = [\"One letter a month.\"]\n";

/// The facts of three posts, in the order Inkwell reads them: two issues
/// with their number and parts, and one whose first fact is no number.
const FACTS: [(&str, &str); 3] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Issue\n    value: \"12\"\n  - label: Lead\n    value: \"The biggest win first\"\n  - label: Reading\n    value: \"Three posts on page speed\"\n  - label: Letters\n    value: \"Your slowest pages\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Issue\n    value: \"11\"\n  - label: Lead\n    value: \"Writing a title tag\"\n",
    ),
    (
        "content/en/posts/2026-09-28-what-a-sitemap-does.md",
        "facts:\n  - label: Client\n    value: \"Calder Street Library\"\n",
    ),
];

/// The demo site built with Inkwell, with three posts given facts.
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

/// The texts of every element `css` selects in `html`, trimmed.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .collect()
}

#[test]
fn inkwell_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The latest issue is the newest post with a number, not the newest
/// post. The archive keeps every post in its year, numbers the issues
/// and shows their parts, and lists a post whose first fact is no number
/// as an extra.
#[test]
fn inkwell_front_page_leads_with_the_latest_numbered_issue() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".iw-latest h2"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(texts(&page, ".iw-stamp-figure"), ["12"]);
    assert_eq!(
        texts(&page, ".iw-latest .iw-part"),
        ["Lead", "Reading", "Letters"]
    );
    assert_eq!(texts(&page, ".iw-spine h3"), ["2026"]);
    assert_eq!(texts(&page, ".iw-issue .iw-no-figure"), ["12", "11"]);
    assert_eq!(texts(&page, ".iw-tally"), ["2 issues"]);
    assert_eq!(texts(&page, ".iw-issue h4").len(), 5);
    assert!(!page.contains("Calder Street Library"));
    assert!(page.contains("A monthly letter since 2019"));
}

#[test]
fn inkwell_issue() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// An issue shows its number as a stamp and its parts in the box; a post
/// whose first fact is no number shows neither.
#[test]
fn an_issue_shows_its_stamp_and_parts_and_an_extra_does_not() {
    let (_temp, out) = built();
    let issue = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&issue, ".iw-stamp-small"), ["Issue 12"]);
    assert_eq!(
        texts(&issue, ".iw-contents .iw-item"),
        [
            "The biggest win first",
            "Three posts on page speed",
            "Your slowest pages"
        ]
    );
    let extra = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&extra, ".iw-stamp-small").is_empty());
    assert!(texts(&extra, ".iw-contents").is_empty());
}

#[test]
fn inkwell_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn inkwell_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
