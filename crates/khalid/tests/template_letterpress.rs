//! The Letterpress template end to end (docs/templates/letterpress.md):
//! `khalid build` of the demo site with `[template] name =
//! "letterpress"`, the newest post given an issue number and three parts
//! as facts and an older one an issue number only, then the header and
//! main part of its front page, an issue, a list and the people page.

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

const SETTINGS: &str = "\n[template]\nname = \"letterpress\"\n[template.options]\ntagline = [\"A weekly letter about search.\"]\n";

/// The facts of two issues, in the order Letterpress reads them: the
/// issue number, then the parts in this issue.
const ISSUES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-12-planning-a-topic-cluster.md",
        "facts:\n  - label: Issue\n    value: \"No. 12\"\n  - label: Feature\n    value: \"A cluster in an afternoon\"\n  - label: The tool\n    value: \"A sheet of paper\"\n  - label: Letters\n    value: \"How many posts is enough\"\n",
    ),
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Issue\n    value: \"No. 11\"\n",
    ),
];

/// The demo site built with Letterpress, with two posts given their
/// issue facts.
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

#[test]
fn letterpress_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest issue is the cover, with its number in the nameplate's
/// folio line and its contents; every other post is a past issue, and
/// the cover is not one of them.
#[test]
fn letterpress_front_page_shows_the_cover_and_past_issues() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let cover = "Planning a topic cluster in an afternoon";
    assert_eq!(texts(&page, ".lp-cover h2"), [cover]);
    assert_eq!(texts(&page, ".hero .lp-folio-number"), ["No. 12"]);
    assert_eq!(
        texts(&page, ".lp-cover .lp-inside-part"),
        ["Feature", "The tool", "Letters"]
    );
    let past = texts(&page, ".lp-sheet h3");
    assert!(!past.is_empty(), "expected something here");
    assert!(!past.contains(&cover.to_owned()), "{past:?}");
    assert!(
        past.contains(&"Fixing slow pages, starting with the biggest win".to_owned()),
        "{past:?}"
    );
    assert_eq!(texts(&page, ".lp-sheet-number"), ["No. 11"]);
    assert!(page.contains("A weekly letter about search."));
}

#[test]
fn letterpress_issue() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "planning-a-topic-cluster/index.html"));
}

/// An issue opens with its number in the folio line, lists its parts
/// and closes with an end mark that names it. An issue without facts
/// shows neither the number nor the contents.
#[test]
fn an_issue_opens_with_its_number_and_closes_with_an_end_mark() {
    let (_temp, out) = built();
    let issue = shown(&out, "planning-a-topic-cluster/index.html");
    assert_eq!(texts(&issue, ".post-header .lp-folio-number"), ["No. 12"]);
    assert_eq!(
        texts(&issue, ".lp-inside-title"),
        [
            "A cluster in an afternoon",
            "A sheet of paper",
            "How many posts is enough"
        ]
    );
    assert_eq!(texts(&issue, ".lp-end-number"), ["No. 12"]);
    let plain = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&plain, ".lp-folio-number").is_empty(),
        "expected nothing here"
    );
    assert!(
        texts(&plain, ".lp-inside").is_empty(),
        "expected nothing here"
    );
    assert_eq!(texts(&plain, ".lp-end").len(), 1);
}

#[test]
fn letterpress_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn letterpress_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
