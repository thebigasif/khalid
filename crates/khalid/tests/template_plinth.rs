//! The Plinth template end to end (docs/templates/plinth.md): `khalid
//! build` of the demo site with `[template] name = "plinth"` and two posts
//! given a project's facts (location, year, floor area, stage), then the
//! header and main part of its front page, a project, a list and the
//! people page, and the title block built from the project's facts.

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

const SETTINGS: &str = "\n[template]\nname = \"plinth\"\n[template.options]\nhero_stats = [\"38 | buildings completed\", \"21 | years in practice\"]\ntagline = [\"Houses and libraries by the sea.\"]\n[template.cta]\nlabel = \"Start a project\"\nhref = \"https://www.example.com/contact/\"\n";

/// Two posts that become projects: their facts, in the order Plinth
/// shows them.
const PROJECTS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Location\n    value: \"Harrowfield\"\n  - label: Year\n    value: \"2026\"\n  - label: Floor area\n    value: \"640 m²\"\n  - label: Stage\n    value: \"Completed\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Location\n    value: \"Bow Lane\"\n  - label: Year\n    value: \"2025\"\n",
    ),
];

/// The demo site built with Plinth, with two posts made projects.
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
fn plinth_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Projects (posts with facts) form the spreads, each with a numbered
/// title block, and every other post the studio notes after them, so no
/// post shows twice.
#[test]
fn plinth_front_page_splits_projects_from_notes() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let projects = texts(&page, ".spread h3");
    assert_eq!(
        projects,
        [
            "Fixing slow pages, starting with the biggest win",
            "How to write a title tag",
        ]
    );
    assert_eq!(texts(&page, ".spread .title-block-sheet dd"), ["01", "02"]);
    let notes = texts(&page, ".notes .feed-item h3");
    assert!(!notes.is_empty());
    assert!(
        notes.iter().all(|title| !projects.contains(title)),
        "{notes:?}"
    );
    assert_eq!(texts(&page, ".figure dd"), ["38", "21"]);
    assert!(page.contains("Houses and libraries by the sea."));
}

/// The types of work list the categories that hold projects. A category
/// whose posts are all notes, here Content strategy, is left out by the
/// same rule as the notes, not by its name.
#[test]
fn the_types_of_work_leave_out_a_category_of_notes_only() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let kinds = texts(&page, ".kind-index a");
    assert!(kinds.contains(&"Technical SEO".to_owned()), "{kinds:?}");
    assert!(kinds.contains(&"On-page SEO".to_owned()), "{kinds:?}");
    assert!(!kinds.contains(&"Content strategy".to_owned()), "{kinds:?}");
}

#[test]
fn plinth_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A project's title block holds each of its facts with its own label,
/// then the author and the date. A note, without facts, has none.
#[test]
fn a_project_shows_its_facts_in_a_title_block() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    let labels = texts(&page, ".project-data .title-block dt");
    assert_eq!(
        labels.get(..4).unwrap_or_default(),
        ["Location", "Year", "Floor area", "Stage"]
    );
    let values = texts(&page, ".project-data .title-block dd");
    assert!(values.contains(&"640 m²".to_owned()), "{values:?}");
    assert_eq!(labels.len(), 6, "four facts, the author and the date");
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&note, ".title-block").is_empty(),
        "a post without facts has no title block"
    );
}

#[test]
fn plinth_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn plinth_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
