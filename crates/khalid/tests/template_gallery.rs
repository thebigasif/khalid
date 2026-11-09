//! The Gallery template end to end (docs/templates/gallery.md): `khalid
//! build` of the demo site with `[template] name = "gallery"` and two
//! posts given project details as facts (client, year, medium, edition),
//! then the header and main part of its front page, a post, a list and
//! the people page, the wall labels and the list of works read from the
//! facts by position, and the project sheet.

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

const SETTINGS: &str = "\n[template]\nname = \"gallery\"\n[template.options]\nstatement = [\"Prints and posters for small museums.\"]\n[template.cta]\nlabel = \"Ask about a commission\"\nhref = \"https://www.example.com/studio/\"\n";

/// Two posts given project details, in the order Gallery reads them.
const PROJECTS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Client\n    value: \"Harbour Science Museum\"\n  - label: Year\n    value: \"2025\"\n  - label: Medium\n    value: \"Giclée on cotton rag\"\n  - label: Edition\n    value: \"6 prints, 50 of each\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Publisher\n    value: \"Tidewater Books\"\n  - label: Year\n    value: \"2024\"\n",
    ),
];

/// The demo site built with Gallery, with two posts given project details.
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
    for (rel, facts) in PROJECTS {
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
fn gallery_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Every post on the page hangs on the wall, newest first, each under a
/// two-digit number, and the list of works repeats them under the same
/// numbers, so a reader can match a label to its row.
#[test]
fn the_wall_and_the_list_of_works_share_their_numbers() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let titles = [
        "Planning a topic cluster in an afternoon",
        "Fixing slow pages, starting with the biggest win",
        "What a sitemap does, and what it does not",
        "How to write a title tag",
        "Why internal links matter",
    ];
    assert_eq!(texts(&page, ".wall-title"), titles);
    assert_eq!(texts(&page, ".checklist-rows a"), titles);
    let numbers = ["No. 01", "No. 02", "No. 03", "No. 04", "No. 05"];
    assert_eq!(texts(&page, ".wall-number"), numbers);
    assert_eq!(texts(&page, ".checklist-number"), numbers);
    assert!(page.contains("Prints and posters for small museums."));
}

/// A wall label shows the medium (the third fact) and the year (the
/// second), or the category and the post's own year without them. The
/// list of works shows the client (the first fact) and the year.
#[test]
fn the_labels_read_the_facts_by_position() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".wall-meta"),
        [
            "Content strategy2026",
            "Giclée on cotton rag2025",
            "Technical SEO2026",
            "On-page SEO2024",
            "On-page SEO2026",
        ]
    );
    assert_eq!(
        texts(&page, ".checklist-meta"),
        [
            "Content strategy2026",
            "Harbour Science Museum2025",
            "Technical SEO2026",
            "Tidewater Books2024",
            "On-page SEO2026",
        ]
    );
}

/// A project page shows its facts as the project sheet, with the
/// writer's own labels, beside its title. A post without facts has none.
#[test]
fn the_project_sheet_shows_every_fact() {
    let (_temp, out) = built();
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&post, ".project-sheet dt"),
        ["Client", "Year", "Medium", "Edition"]
    );
    assert_eq!(
        texts(&post, ".project-sheet dd"),
        [
            "Harbour Science Museum",
            "2025",
            "Giclée on cotton rag",
            "6 prints, 50 of each",
        ]
    );
    let plain = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&plain, ".project-sheet").is_empty(),
        "a post without facts has no sheet"
    );
}

#[test]
fn gallery_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn gallery_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn gallery_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
