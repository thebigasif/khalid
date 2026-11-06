//! The Versions template end to end (docs/templates/versions.md): `khalid
//! build` of the demo site with `[template] name = "versions"` and three
//! posts given a version as their first fact, then the header and main
//! part of its front page, a change and a type of change: the version
//! rail, the type marks, the filter row and the feeds.

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

const SETTINGS: &str = "\n[template]\nname = \"versions\"\n[template.options]\ntagline = [\"Every change to the demo.\"]\nchange_types = [\"technical-seo\", \"on-page-seo\"]\n";

/// Three posts that become changes: their version first, in the order
/// Versions reads the facts. Two share a version, so they form one
/// release.
const CHANGES: [(&str, &str); 3] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Version\n    value: \"2.1.0\"\n  - label: Platforms\n    value: \"Web\"\n",
    ),
    (
        "content/en/posts/2026-09-28-what-a-sitemap-does.md",
        "facts:\n  - label: Version\n    value: \"2.1.0\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Version\n    value: \"2.0.0\"\n",
    ),
];

/// The demo site built with Versions, with three posts made changes.
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
    for (rel, facts) in CHANGES {
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

/// The value of `attribute` on every element `css` selects in `html`.
#[cfg(test)]
fn attributes(html: &str, css: &str, attribute: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| {
            element
                .value()
                .attr(attribute)
                .unwrap_or_default()
                .to_owned()
        })
        .collect()
}

#[test]
fn versions_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Posts that follow one another with the same first fact form one
/// release under that version; a post without facts forms a release of
/// its own date. Each release has an anchor, and the versions on the page
/// link to them.
#[test]
fn the_rail_groups_changes_by_version() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let releases = attributes(&page, ".release-log .release", "id");
    assert_eq!(releases.first().map(String::as_str), Some("d2026-10-12"));
    assert!(
        releases.contains(&"v2-1-0".to_owned()) && releases.contains(&"v2-0-0".to_owned()),
        "{releases:?}"
    );
    assert_eq!(
        texts(&page, ".release-log .release-version"),
        ["2.1.0", "2.0.0"]
    );
    let shipped = texts(&page, "#v2-1-0 .change-title");
    assert_eq!(
        shipped,
        [
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not",
        ]
    );
    assert_eq!(
        attributes(&page, ".release-index a", "href"),
        ["#v2-1-0", "#v2-0-0"]
    );
    assert!(page.contains("Every change to the demo."));
}

/// Each change's type chip carries the place of its category in
/// `change_types`, and a category not listed carries none. The filter row
/// starts with every change, then each type, as plain links.
#[test]
fn types_are_marked_by_their_place_in_change_types() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let fixing = attributes(&page, "#v2-1-0 .change-type", "data-mark");
    assert_eq!(fixing, ["1", "1"]);
    let title = attributes(&page, "#v2-0-0 .change-type", "data-mark");
    assert_eq!(title, ["2"]);
    let cluster = attributes(&page, "#d2026-10-12 .change-type", "data-mark");
    assert_eq!(cluster, [""]);
    assert_eq!(
        texts(&page, ".filter-row a"),
        [
            "All changes",
            "On-page SEO",
            "Technical SEO",
            "Content strategy"
        ]
    );
    assert_eq!(texts(&page, ".filter-row a[aria-current]"), ["All changes"]);
    let feeds = attributes(&page, ".feed-links a", "href");
    assert_eq!(feeds.first().map(String::as_str), Some("/blog/feed.xml"));
    assert!(
        feeds.contains(&"/blog/category/technical-seo/feed.xml".to_owned()),
        "{feeds:?}"
    );
}

#[test]
fn versions_change_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A change shows its type and version above the title, and every fact
/// in the facts row.
#[test]
fn a_change_shows_its_type_version_and_facts() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".change-head .change-type"), ["Technical SEO"]);
    assert_eq!(texts(&page, ".change-head .release-version"), ["2.1.0"]);
    assert_eq!(texts(&page, ".change-facts dt"), ["Version", "Platforms"]);
}

#[test]
fn versions_type_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

/// A type of change links its own feed, and its filter row marks it.
#[test]
fn a_type_of_change_links_its_feed_and_marks_its_chip() {
    let (_temp, out) = built();
    let page = shown(&out, "category/technical-seo/index.html");
    assert_eq!(
        attributes(&page, ".list-feed a", "href"),
        ["/blog/category/technical-seo/feed.xml"]
    );
    assert_eq!(
        texts(&page, ".filter-row a[aria-current]"),
        ["Technical SEO"]
    );
}
