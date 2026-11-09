//! The Itinerary template end to end (docs/templates/itinerary.md):
//! `khalid build` of the demo site with `[template] name = "itinerary"`
//! and two posts given a trip's facts (days, route, when, cost), then the
//! header and main part of its front page, a trip, a country's list and
//! the writers page, and the boarding pass and day route on a trip.

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

const SETTINGS: &str = "\n[template]\nname = \"itinerary\"\n[template.options]\nhero_stats = [\"5 | countries\", \"56 | days on the road\"]\ntagline = [\"Two of us, one bag each, day by day.\"]\n[template.cta]\nlabel = \"Get the packing list\"\nhref = \"https://www.example.com/packing/\"\n";

/// Two posts that become trips: their facts, in the order Itinerary
/// reads them.
const TRIPS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Days\n    value: \"10\"\n  - label: Route\n    value: \"Tbilisi to Kazbegi\"\n  - label: When\n    value: \"May 2026\"\n  - label: Cost for two\n    value: \"£1,180\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Nights\n    value: \"4\"\n  - label: Route\n    value: \"Mestia to Ushguli\"\n",
    ),
];

/// A packing list, added to the end of the first trip as a task list.
const PACKING: &str = "\n\n- [x] Rain jacket\n- [ ] Walking poles\n";

/// The demo site built with Itinerary, with two posts made trips.
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
    for (rel, facts) in TRIPS {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', &format!("\n{facts}"), 1)).unwrap();
    }
    let first = at(temp.path(), TRIPS[0].0);
    let mut text = fs::read_to_string(&first).unwrap();
    text.push_str(PACKING);
    fs::write(first, text).unwrap();
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
fn itinerary_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Trips (posts with facts) are tickets, the newest one wide, and every
/// other post a note in the dated feed, so no post shows twice. A ticket's
/// stub carries the first fact, the length of the trip.
#[test]
fn itinerary_front_page_splits_trips_from_notes() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".trip-lead .ticket-wide .ticket-title"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(
        texts(&page, ".trip-list .ticket-title"),
        ["How to write a title tag"]
    );
    assert_eq!(texts(&page, ".stub-number"), ["10", "4"]);
    assert_eq!(texts(&page, ".stub-label"), ["Days", "Nights"]);
    let notes = texts(&page, ".trip-notes .feed-item h3");
    assert_ne!(notes, Vec::<String>::new(), "the notes are empty");
    assert!(
        notes.iter().all(|title| !title.contains("title tag")),
        "{notes:?}"
    );
    assert_eq!(texts(&page, ".figure dd"), ["5", "56"]);
    assert!(page.contains("Two of us, one bag each, day by day."));
}

/// Each country with a trip gets a stamp with its count. A category whose
/// posts are all notes, here Content strategy, gets none, by the same
/// rule as the notes, not by its name.
#[test]
fn the_country_stamps_leave_out_a_category_of_notes_only() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let stamps = texts(&page, ".stamp-name");
    assert!(stamps.contains(&"Technical SEO".to_owned()), "{stamps:?}");
    assert!(stamps.contains(&"On-page SEO".to_owned()), "{stamps:?}");
    assert!(
        !stamps.contains(&"Content strategy".to_owned()),
        "{stamps:?}"
    );
    assert_eq!(texts(&page, ".stamp-count"), ["1 trip", "1 trip"]);
}

#[test]
fn itinerary_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A trip shows its facts as a boarding pass, the days large on the stub
/// and the rest in cells, and its text runs down the day route with the
/// packing list as checkboxes. A note has neither.
#[test]
fn a_trip_has_a_boarding_pass_and_a_day_route() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".pass-stub .stub-number"), ["10"]);
    assert_eq!(
        texts(&page, ".pass-facts dd"),
        ["Tbilisi to Kazbegi", "May 2026", "£1,180"]
    );
    let route = texts(&page, ".day-route").concat();
    assert!(route.contains("Walking poles"), "the packing list");
    assert!(page.contains("type=\"checkbox\""), "a checklist");
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&note, ".pass").is_empty(), "a note has no pass");
    assert!(texts(&note, ".day-route").is_empty(), "nor a route");
}

#[test]
fn itinerary_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn itinerary_writers_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
