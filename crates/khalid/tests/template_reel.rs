//! The Reel template end to end (docs/templates/reel.md): `khalid build`
//! of the demo site with `[template] name = "reel"` and two posts given a
//! review's facts (rating, release, cast, where to watch), then the header and main part of its front page, a review, a list
//! and the critics page, with the stars, the ticket and the score sheet.

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

const SETTINGS: &str = "\n[template]\nname = \"reel\"\n[template.options]\ntagline = [\"New films, reviewed every week.\"]\n";

/// Two posts that become reviews: their facts, in the order Reel reads
/// them.
const REVIEWS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Rating\n    value: \"3.5\"\n  - label: Year and length\n    value: \"2026, 118 min\"\n  - label: Starring\n    value: \"Ada Lindqvist, Teo Marsh\"\n  - label: Where to watch\n    value: \"In cinemas, Lanternbox\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Score\n    value: \"8/10\"\n  - label: Season\n    value: \"Season 2, 8 episodes\"\n",
    ),
];

/// The demo site built with Reel, with two posts made reviews.
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
    for (rel, facts) in REVIEWS {
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
fn reel_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest review leads as Now showing, the other review is a poster
/// card, the score sheet ranks both by rating, and a post without facts
/// is a feature, never a review.
#[test]
fn reel_front_page_splits_reviews_from_features() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".lead-review h3"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(
        texts(&page, ".latest-reviews .poster h3"),
        ["How to write a title tag"]
    );
    assert_eq!(
        texts(&page, ".score-what h3"),
        [
            "How to write a title tag",
            "Fixing slow pages, starting with the biggest win",
        ]
    );
    let features = texts(&page, ".features .feed-item h3");
    assert!(!features.is_empty());
    assert!(
        features.iter().all(|title| !title.contains("title tag")),
        "{features:?}"
    );
    assert!(page.contains("New films, reviewed every week."));
}

/// A rating out of 5 fills its stars up to the score, with a half star
/// for a half; a rating with its own scale, 8/10, draws four of five.
/// Screen readers hear each score once, in words.
#[test]
fn a_rating_shows_as_five_stars_and_in_words() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    let html = scraper::Html::parse_fragment(&page);
    let count = |css: &str| html.select(&scraper::Selector::parse(css).unwrap()).count();
    assert_eq!(count(".ticket .rating-star"), 5);
    assert_eq!(count(".ticket .rating-star-full"), 3);
    assert_eq!(count(".ticket .rating-star-half"), 1);
    assert_eq!(
        texts(&page, ".ticket .visually-hidden"),
        ["Rated 3.5 out of 5"]
    );
    let home = shown(&out, "index.html");
    assert!(home.contains("Rated 8 out of 10"));
}

/// The ticket holds the facts after the rating: each name in the cast
/// and each service in Where to watch is its own item. A post without
/// facts has no ticket.
#[test]
fn a_review_shows_its_cast_and_services_on_the_ticket() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".watch-list li"), ["In cinemas", "Lanternbox"]);
    assert_eq!(
        texts(&page, ".ticket-facts dt"),
        ["Year and length", "Starring", "Where to watch"]
    );
    assert_eq!(
        texts(&page, ".cast-names li"),
        ["Ada Lindqvist", "Teo Marsh"]
    );
    let feature = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&feature, ".ticket").is_empty(),
        "a feature has no ticket"
    );
}

#[test]
fn reel_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn reel_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn reel_critics_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
