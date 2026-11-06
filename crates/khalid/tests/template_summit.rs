//! The Summit template end to end (docs/templates/summit.md): `khalid
//! build` of the demo site with `[template] name = "summit"` and two posts
//! given a session's facts (day, time, room, format), then the header and
//! main part of its front page, a session, a venue (category) and the
//! speakers page, and the timetable's order by day and time.

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

const SETTINGS: &str = "\n[template]\nname = \"summit\"\n[template.options]\nhero_stats = [\"2 | days\", \"2 | sessions\"]\nevent_line = [\"13 to 14 May 2027, Easthaven\"]\ntagline = [\"Two days on how the web is read.\"]\n[template.cta]\nlabel = \"Book a ticket\"\nhref = \"https://www.example.com/tickets/\"\n";

/// Two posts that become sessions: their facts, in the order Summit
/// reads them. The older post runs on the first day, so a timetable in
/// post order, newest first, would be wrong.
const SESSIONS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Day\n    value: \"2027-05-13\"\n  - label: Time\n    value: \"09:30\"\n  - label: Room\n    value: \"Main hall\"\n  - label: Format\n    value: \"Keynote\"\n",
    ),
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Day\n    value: \"2027-05-14\"\n  - label: Time\n    value: \"14:00\"\n  - label: Room\n    value: \"Studio 2\"\n  - label: Format\n    value: \"Workshop\"\n",
    ),
];

/// The demo site built with Summit, with two posts made sessions.
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
    for (rel, facts) in SESSIONS {
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
fn summit_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Sessions (posts with facts) form the programme, grouped by day and
/// sorted by day and time, with the weekday worked out from the date.
/// Every other post is an update, so no post shows twice.
#[test]
fn the_programme_runs_by_day_and_time() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".summit-programme .slot-title"),
        [
            "How to write a title tag",
            "Fixing slow pages, starting with the biggest win",
        ]
    );
    assert_eq!(
        texts(&page, ".summit-programme .schedule-heading"),
        ["Thursday 13 May 2027", "Friday 14 May 2027"]
    );
    assert_eq!(texts(&page, ".slot-time time"), ["09:30", "14:00"]);
    assert!(page.contains("<time datetime=\"2027-05-13T09:30\">"));
    assert!(page.contains("href=\"#day-2027-05-14\""));
    let sessions = texts(&page, ".slot-title");
    let updates = texts(&page, ".summit-updates .feed-item h3");
    assert!(!updates.is_empty());
    assert!(
        updates.iter().all(|title| !sessions.contains(title)),
        "{updates:?}"
    );
    assert_eq!(texts(&page, ".figure dd"), ["2", "2"]);
    assert!(page.contains("13 to 14 May 2027, Easthaven"));
}

/// The venues are the categories that hold sessions, each with its count.
#[test]
fn the_venues_count_their_sessions() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let venues = texts(&page, ".venue-index .venue-name");
    assert!(venues.contains(&"Technical SEO".to_owned()), "{venues:?}");
    assert!(venues.contains(&"On-page SEO".to_owned()), "{venues:?}");
    assert!(
        !venues.contains(&"Content strategy".to_owned()),
        "{venues:?}"
    );
    assert!(
        texts(&page, ".venue-count")
            .iter()
            .all(|count| count == "1 session")
    );
}

#[test]
fn summit_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A session shows its facts on the pass; an update has none.
#[test]
fn a_session_shows_its_pass() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".pass-day"), ["Friday 14 May 2027"]);
    assert_eq!(
        texts(&page, ".pass-facts dd"),
        ["14:00", "Studio 2", "Workshop"]
    );
    let update = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&update, ".session-pass").is_empty());
}

#[test]
fn summit_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn summit_speakers_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
