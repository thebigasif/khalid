//! The Vital template end to end (docs/templates/vital.md): `khalid
//! build` of the demo site with `[template] name = "vital"` and two posts
//! given a routine's facts (time, level, how often, what you need), then
//! the header and main part of its front page, a routine, a list and the
//! coaches page, and the session panel, care note and routine track.

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

const SETTINGS: &str = "\n[template]\nname = \"vital\"\n[template.options]\nhero_stats = [\"8 | routines\", \"3 | programmes\"]\ntagline = [\"Gentle routines for every day.\"]\ncare_note = [\"General wellbeing ideas, not medical advice.\"]\n[template.cta]\nlabel = \"See the timetable\"\nhref = \"https://www.example.com/timetable/\"\n";

/// Two posts that become routines: their facts, in the order Vital
/// reads them.
const ROUTINES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Time\n    value: \"10 min\"\n  - label: Level\n    value: \"Gentle\"\n  - label: How often\n    value: \"Daily\"\n  - label: You need\n    value: \"A mat\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Time\n    value: \"5 min\"\n  - label: Level\n    value: \"Steady\"\n",
    ),
];

/// The demo site built with Vital, with two posts made routines.
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
    for (rel, facts) in ROUTINES {
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
fn vital_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Routines (posts with facts) form the session cards, each with its
/// time in the ring, and every other post the notes feed, so no post
/// shows twice. The care note shows beside the notes.
#[test]
fn vital_front_page_splits_routines_from_notes() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let routines = texts(&page, ".routines .session-card h3");
    assert_eq!(
        routines,
        [
            "Fixing slow pages, starting with the biggest win",
            "How to write a title tag",
        ]
    );
    let rings = texts(&page, ".routines .session-ring");
    assert_eq!(rings, ["Time: 10 min", "Time: 5 min"]);
    let notes = texts(&page, ".notes .feed-item h3");
    assert!(!notes.is_empty(), "expected something here");
    assert!(
        notes.iter().all(|title| !routines.contains(title)),
        "{notes:?}"
    );
    assert_eq!(texts(&page, ".figure dd"), ["8", "3"]);
    assert!(page.contains("Gentle routines for every day."));
    assert!(
        texts(&page, ".care-note-home")
            .concat()
            .contains("not medical advice")
    );
}

/// The programmes list the categories that hold routines. A category
/// whose posts are all notes, here Content strategy, is left out by the
/// same rule as the notes feed, not by its name.
#[test]
fn the_programmes_leave_out_a_category_of_notes_only() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let programmes = texts(&page, ".programme-chips a");
    assert!(
        programmes.contains(&"Technical SEO".to_owned()),
        "{programmes:?}"
    );
    assert!(
        programmes.contains(&"On-page SEO".to_owned()),
        "{programmes:?}"
    );
    assert!(
        !programmes.contains(&"Content strategy".to_owned()),
        "{programmes:?}"
    );
}

#[test]
fn vital_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A routine opens with the session panel: its time in the large ring,
/// its other facts beside it and the care note under it. Its numbered
/// list is the routine track. A note has none of these.
#[test]
fn a_routine_opens_with_its_session_panel() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    let ring = texts(&page, ".session-ring-large").concat();
    assert!(ring.contains("Time") && ring.contains("10 min"), "{ring}");
    assert_eq!(
        texts(&page, ".session-facts dd"),
        ["Gentle", "Daily", "A mat"]
    );
    assert!(
        texts(&page, ".care-note")
            .concat()
            .contains("not medical advice")
    );
    assert_eq!(texts(&page, ".routine-track > ol").len(), 1);
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&note, ".session-panel").is_empty(),
        "a note has no panel"
    );
    assert!(
        texts(&note, ".care-note").is_empty(),
        "a note has no care note"
    );
    assert!(
        texts(&note, ".routine-track").is_empty(),
        "a note has no track"
    );
}

#[test]
fn vital_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn vital_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
