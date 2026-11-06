//! The Respawn template end to end (docs/templates/respawn.md): `khalid
//! build` of the demo site with `[template] name = "respawn"`, a release
//! calendar, and two posts given a review's facts (score, platforms,
//! developer), then the header and main part of its front page, a review,
//! a list and the writers page, and the score meter drawn from the score.

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

const SETTINGS: &str = "\n[template]\nname = \"respawn\"\n[template.options]\ntagline = [\"Games news and honest reviews.\"]\nrelease_calendar = [\"2026-11-20 | Hollowmere | PC, consoles | /blog/fixing-slow-pages/\", \"Spring 2027 | Kitebound: Updraft | PC\"]\n[template.cta]\nlabel = \"How we score\"\nhref = \"https://www.example.com/scores/\"\n";

/// Two posts that become reviews: their facts, in the order Respawn reads
/// them. The second score is a word, not a number.
const REVIEWS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Score\n    value: \"8.5\"\n  - label: Platforms\n    value: \"PC, consoles\"\n  - label: Developer\n    value: \"Ashfield Games\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Verdict\n    value: \"Recommended\"\n  - label: Platforms\n    value: \"PC\"\n",
    ),
];

/// The demo site built with Respawn, with two posts made reviews.
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
fn respawn_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Reviews (posts with facts) show as score cards and every other post in
/// the news feed, so no post shows twice. The newest post leads.
#[test]
fn respawn_front_page_splits_reviews_from_news() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let reviews = texts(&page, ".rs-reviews .rs-card h3");
    assert_eq!(
        reviews,
        [
            "Fixing slow pages, starting with the biggest win",
            "How to write a title tag",
        ]
    );
    let lead = texts(&page, ".rs-lead h2");
    assert_eq!(lead, ["Planning a topic cluster in an afternoon"]);
    let news = texts(&page, ".rs-news .feed-item h3");
    assert!(!news.is_empty(), "expected something here");
    assert!(
        news.iter().all(|title| !reviews.contains(title)),
        "{news:?}"
    );
    assert!(!news.contains(&lead[0]), "{news:?}");
    assert!(page.contains("Games news and honest reviews."));
}

/// The release calendar draws a dated line as a month and a day, links the
/// game when the line names a link, and shows any other date as written.
#[test]
fn the_release_calendar_reads_each_line() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".rs-month"), ["Nov"]);
    assert_eq!(texts(&page, ".rs-day-number"), ["20"]);
    assert_eq!(texts(&page, ".rs-day-text"), ["Spring 2027"]);
    assert_eq!(
        texts(&page, ".rs-date h3"),
        ["Hollowmere", "Kitebound: Updraft"]
    );
    assert_eq!(texts(&page, ".rs-date h3 a"), ["Hollowmere"]);
    assert_eq!(
        texts(&page, ".rs-date .rs-platforms"),
        ["PC, consoles", "PC"]
    );
    assert!(page.contains("<time datetime=\"2026-11-20\">"));
}

#[test]
fn respawn_review() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A score of 8.5 fills eight cells and half of the ninth, in the score
/// strip and again in the verdict, and reads out as a score out of 10. A
/// word in place of a number shows alone, with no bar. A post without
/// facts has neither strip nor verdict.
#[test]
fn a_review_draws_its_score_as_a_health_bar() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    let html = scraper::Html::parse_fragment(&page);
    let count = |css: &str| {
        let selector = scraper::Selector::parse(css).unwrap();
        html.select(&selector).count()
    };
    assert_eq!(count(".rs-strip .score-cell"), 10);
    assert_eq!(count(".rs-strip .score-full"), 8);
    assert_eq!(count(".rs-strip .score-half"), 1);
    assert_eq!(count(".rs-verdict .score-full"), 8);
    assert_eq!(
        texts(&page, ".rs-strip .score-tile"),
        ["Score 8.5 out of 10"]
    );
    assert_eq!(texts(&page, ".rs-facts dt"), ["Platforms", "Developer"]);
    let word = shown(&out, "how-to-write-a-title-tag/index.html");
    assert_eq!(
        texts(&word, ".rs-strip .score-tile"),
        ["Verdict Recommended"]
    );
    assert!(texts(&word, ".score-bar").is_empty(), "a word has no bar");
    let news = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&news, ".rs-strip").is_empty(),
        "news has no score strip"
    );
    assert!(
        texts(&news, ".rs-verdict").is_empty(),
        "news has no verdict"
    );
}

#[test]
fn respawn_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn respawn_writers_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
