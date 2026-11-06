//! The Locale template end to end (docs/templates/locale.md): `khalid
//! build` of the demo site with `[template] name = "locale"` and two posts
//! given a place's facts (kind, price level, hours, address), then the
//! header and main part of its front page, a place, a list and the
//! writers page, the price meter drawn from the price level and the
//! place's name read from its title.

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

const SETTINGS: &str = "\n[template]\nname = \"locale\"\n[template.options]\ntagline = [\"A guide to the city, area by area.\"]\n";

/// Two posts that become places: their facts, in the order Locale reads
/// them. The first price is two signs, the second a number.
const PLACES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Kind\n    value: Bakery\n  - label: Price\n    value: \"££\"\n  - label: Hours\n    value: \"Tue to Sun, 7:30 to 15:00\"\n  - label: Address\n    value: 14 Kiln Yard\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Kind\n    value: Park\n  - label: Price\n    value: \"3\"\n",
    ),
];

/// The first place's title, given a colon so its name is the part before.
const OLD_TITLE: &str = "title: Fixing slow pages, starting with the biggest win";
const NEW_TITLE: &str = "title: \"Fixing slow pages: starting with the biggest win\"";

/// The demo site built with Locale, with two posts made places and the
/// first given a title with a colon.
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
    for (rel, facts) in PLACES {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        let text = text.replacen('\n', &format!("\n{facts}"), 1);
        fs::write(path, text.replacen(OLD_TITLE, NEW_TITLE, 1)).unwrap();
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
fn locale_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Places (posts with facts) show under their area, named by the part of
/// the title before a colon, and every other post shows as a note card,
/// so no post shows twice. Each area with places gets a chip.
#[test]
fn locale_front_page_lists_places_by_area() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".lc-area h3"),
        ["On-page SEO", "Technical SEO"]
    );
    let places = texts(&page, ".lc-area .lc-line h4");
    assert_eq!(places, ["How to write a title tag", "Fixing slow pages"]);
    let notes = texts(&page, ".lc-note h3");
    assert!(!notes.is_empty(), "expected something here");
    assert!(
        notes
            .iter()
            .all(|title| !title.starts_with("Fixing slow pages")),
        "{notes:?}"
    );
    assert!(!notes.contains(&places[0]), "{notes:?}");
    assert_eq!(texts(&page, ".lc-chips .lc-count"), ["1", "1"]);
    assert!(page.contains("A guide to the city, area by area."));
}

/// Two signs fill two of four marks with that sign; a number fills as
/// many round coins; both read out as a level of 4. The row's key line
/// shows the kind and the hours.
#[test]
fn the_price_meter_reads_signs_and_numbers() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let html = scraper::Html::parse_fragment(&page);
    let count = |css: &str| {
        let selector = scraper::Selector::parse(css).unwrap();
        html.select(&selector).count()
    };
    assert_eq!(count(".price-mark"), 4);
    assert_eq!(count(".price-mark.price-on"), 2);
    assert_eq!(count(".price-coin"), 4);
    assert_eq!(count(".price-coin.price-on"), 3);
    assert_eq!(
        texts(&page, ".price-meter .visually-hidden"),
        ["Price level 3 of 4", "Price level 2 of 4"]
    );
    assert_eq!(texts(&page, ".lc-kind"), ["Park", "Bakery"]);
    assert_eq!(
        texts(&page, ".lc-hours"),
        ["Hours: Tue to Sun, 7:30 to 15:00"]
    );
}

#[test]
fn locale_place() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A place opens with its name, the rest of its title under it, and the
/// At a glance card: the price meter, then every other fact with its
/// label. A post without facts has no card and keeps its whole title.
#[test]
fn a_place_shows_its_name_and_the_card() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&page, "h1"),
        ["Fixing slow pages: starting with the biggest win"]
    );
    assert_eq!(
        texts(&page, ".lc-title-rest"),
        [": starting with the biggest win"]
    );
    assert_eq!(texts(&page, ".lc-glance dt"), ["Kind", "Hours", "Address"]);
    assert_eq!(
        texts(&page, ".lc-glance-price .price-mark.price-on"),
        ["£", "£"]
    );
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&note, ".lc-glance").is_empty(), "a note has no card");
    assert!(
        texts(&note, ".lc-title-rest").is_empty(),
        "expected nothing here"
    );
}

#[test]
fn locale_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn locale_writers_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
