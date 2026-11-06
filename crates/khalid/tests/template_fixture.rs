//! The Fixture template end to end (docs/templates/fixture.md): `khalid
//! build` of the demo site with `[template] name = "fixture"` and four
//! posts given a match's facts (opponent, result or kick-off, ground,
//! competition), then the header and main part of its front page, a match
//! report, a list and the squad page, and the next match, the form guide
//! and the scoreboard built from the posts' own facts.

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

const SETTINGS: &str = "\n[template]\nname = \"fixture\"\n[template.options]\nhero_stats = [\"2 | played\", \"1 | won\"]\ntagline = [\"Amateur football since 1974.\"]\n[template.cta]\nlabel = \"Join the club\"\nhref = \"https://www.example.com/join/\"\n";

/// Four posts that become matches: their facts, in the order Fixture
/// reads them. Two are results and two, published after both, are
/// previews, so the newer preview is the next match.
const MATCHES: [(&str, &str); 4] = [
    (
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
        "facts:\n  - label: Opponent\n    value: \"Harbour Town\"\n  - label: Result\n    value: \"W 3-1\"\n  - label: Ground\n    value: \"Home\"\n  - label: Competition\n    value: \"League\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Opponent\n    value: \"Millbrook\"\n  - label: Result\n    value: \"L 0-2\"\n  - label: Ground\n    value: \"Away\"\n  - label: Competition\n    value: \"Cup\"\n",
    ),
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Opponent\n    value: \"Eastfield\"\n  - label: Kick-off\n    value: \"Sat 10 Oct, 3pm\"\n  - label: Ground\n    value: \"Home\"\n",
    ),
    (
        "content/en/posts/2026-10-12-planning-a-topic-cluster.md",
        "facts:\n  - label: Opponent\n    value: \"Westbury\"\n  - label: Kick-off\n    value: \"Sat 17 Oct, 3pm\"\n  - label: Ground\n    value: \"Away\"\n  - label: Competition\n    value: \"League\"\n",
    ),
];

/// The demo site built with Fixture, with four posts made matches.
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
    for (rel, facts) in MATCHES {
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
fn fixture_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The previews published after the newest result are the fixtures: the
/// newest is the next match, the other is listed under Fixtures. The
/// results form the scoreboard and the form guide, oldest first, and a
/// post without facts is club news.
#[test]
fn the_front_page_splits_the_next_match_results_and_news() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".next-versus a"), ["Westbury"]);
    assert_eq!(
        texts(&page, ".next-kick-off").concat(),
        "Kick-off Sat 17 Oct, 3pm"
    );
    assert_eq!(texts(&page, ".club-fixtures .match h3"), ["Eastfield"]);
    assert_eq!(
        texts(&page, ".club-results .match h3"),
        ["Millbrook", "Harbour Town"]
    );
    assert_eq!(texts(&page, ".club-results .match-mark"), ["L", "W"]);
    assert_eq!(texts(&page, ".form-chip span[aria-hidden]"), ["W", "L"]);
    let news = texts(&page, ".club-news .feed-item h3");
    assert!(
        news.contains(&"What a sitemap does, and what it does not".to_owned()),
        "{news:?}"
    );
    assert_eq!(texts(&page, ".figure dd"), ["2", "1"]);
    assert!(page.contains("Amateur football since 1974."));
}

#[test]
fn fixture_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "how-to-write-a-title-tag/index.html"));
}

/// An away result puts the opponent first as the home side, with each
/// side's goals from the score, the club's goals first in the fact. A
/// preview shows "v" and its kick-off, and a news post no scoreboard.
#[test]
fn the_scoreboard_puts_the_home_side_first() {
    let (_temp, out) = built();
    let page = shown(&out, "how-to-write-a-title-tag/index.html");
    assert_eq!(texts(&page, ".score-home .score-name"), ["Millbrook"]);
    assert_eq!(texts(&page, ".score-home .score-goals"), ["2"]);
    assert_eq!(texts(&page, ".score-away .score-goals"), ["0"]);
    assert_eq!(
        texts(&page, ".score-away .score-name"),
        ["Field Notes on SEO"]
    );
    assert_eq!(texts(&page, ".score-result"), ["Defeat"]);
    let preview = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&preview, ".score-v"), ["v"]);
    assert_eq!(
        texts(&preview, ".score-home .score-name"),
        ["Field Notes on SEO"]
    );
    assert!(texts(&preview, ".score-goals").is_empty());
    let news = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&news, ".scoreboard").is_empty(),
        "news has no scoreboard"
    );
}

/// A list groups its posts under a heading for each month, newest first.
#[test]
fn fixture_category_page() {
    let (_temp, out) = built();
    let page = shown(&out, "category/technical-seo/index.html");
    assert_eq!(
        texts(&page, ".results-month h2"),
        ["October 2026", "September 2026"]
    );
    insta::assert_snapshot!(page);
}

#[test]
fn fixture_squad_page() {
    let (_temp, out) = built();
    let page = shown(&out, "authors/index.html");
    assert_eq!(texts(&page, "h1"), ["Squad and staff"]);
    insta::assert_snapshot!(page);
}
