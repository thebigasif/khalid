//! The Encore template end to end (docs/templates/encore.md): `khalid
//! build` of the demo site with `[template] name = "encore"`, tour dates
//! and two posts given a release's facts (format, release date, label,
//! where to listen), then the header and main part of its front page, a
//! release, a list and the band page.

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

const SETTINGS: &str = "\n[template]\nname = \"encore\"\n[template.options]\ntagline = [\"A band from Glasgow.\"]\nlisten_on = [\"Bandcamp | https://www.example.com/bandcamp/\", \"Nowhere | not an address\"]\ntour_dates = [\"2026-11-14 | Glasgow | The Old Granary | https://www.example.com/tickets/\", \"2026-11-18 | Inverness | Riverside Hall | Sold out\", \"Spring 2027 | Belfast | To be announced\"]\n";

/// Two posts that become releases: their facts, in the order Encore
/// reads them.
const RELEASES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Format\n    value: \"Album\"\n  - label: Released\n    value: \"5 October 2026\"\n  - label: Label\n    value: \"Fieldnote Records\"\n  - label: Listen on Bandcamp\n    value: \"https://www.example.com/listen/\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Format\n    value: \"Single\"\n  - label: Released\n    value: \"21 September 2026\"\n",
    ),
];

/// The demo site built with Encore, with two posts made releases.
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
    for (rel, facts) in RELEASES {
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
fn encore_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest release leads as Out now, every other release sits on the
/// shelf, and the notes (posts without facts) show as cards, so no post
/// shows twice.
#[test]
fn encore_front_page_splits_releases_from_notes() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".out-now h3"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(texts(&page, ".out-now .button"), ["Listen on Bandcamp"]);
    assert_eq!(
        texts(&page, ".shelf .sleeve-title"),
        ["How to write a title tag"]
    );
    assert_eq!(texts(&page, ".shelf .sleeve-meta"), ["Single 2026"]);
    let notes = texts(&page, ".notes .note h3");
    assert_eq!(notes.len(), 3, "{notes:?}");
    assert!(
        notes.iter().all(|title| !title.contains("title tag")),
        "{notes:?}"
    );
    assert_eq!(texts(&page, ".listen-on a"), ["Bandcamp"]);
    assert!(page.contains("A band from Glasgow."));
}

/// Each tour line becomes a show: a date written as 2026-11-14 shows its
/// day and month, a ticket address becomes a link, any other last part a
/// note, and a date in other words shows as written.
#[test]
fn the_tour_list_reads_one_show_a_line() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".tour-city"),
        ["Glasgow", "Inverness", "Belfast"]
    );
    assert_eq!(texts(&page, ".tour-day"), ["14", "18", "Spring 2027"]);
    assert_eq!(texts(&page, ".tour-month"), ["Nov", "Nov"]);
    assert!(page.contains("<time class=\"tour-when\" datetime=\"2026-11-14\">"));
    assert!(page.contains("<a class=\"tour-tickets\" href=\"https://www.example.com/tickets/\""));
    assert_eq!(texts(&page, ".tour-note"), ["Sold out"]);
    assert_eq!(texts(&page, ".tour-tickets").len(), 1);
}

#[test]
fn encore_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A release shows its liner notes from its facts, and each fact whose
/// value is an address as a button with the fact's label. A note has
/// none.
#[test]
fn a_release_shows_its_liner_notes_and_listen_button() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&page, ".liner dd"),
        ["Album", "5 October 2026", "Fieldnote Records"]
    );
    assert!(page.contains("<a class=\"button\" href=\"https://www.example.com/listen/\" rel=\"noopener\">Listen on Bandcamp</a>"));
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&note, ".release-head").is_empty(),
        "a note has no liner notes"
    );
}

#[test]
fn encore_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn encore_band_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
