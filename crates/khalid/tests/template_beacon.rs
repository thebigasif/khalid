//! The Beacon template end to end (docs/templates/beacon.md): `khalid
//! build` of the demo site with `[template] name = "beacon"`, a donate call
//! and three posts given a campaign's facts (raised, goal, then two of the
//! writer's own), then the header and main part of its front page, a
//! campaign, a list and the team page, and the goal meter built from each
//! campaign's own facts.

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

const SETTINGS: &str = "\n[template]\nname = \"beacon\"\n[template.options]\nhero_stats = [\"42 km | of river reopened\", \"1,240 | volunteers\"]\ntagline = [\"We bring the river back to life.\"]\n[template.cta]\nlabel = \"Donate\"\nhref = \"https://www.example.com/donate/\"\nnote = \"Every gift goes to the river.\"\n";

/// Three posts that become campaigns: their facts, in the order Beacon
/// reads them. The third has no numbers, so it shows no goal meter.
const CAMPAIGNS: [(&str, &str); 3] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Raised\n    value: \"£6,150\"\n  - label: Goal\n    value: \"£10,000\"\n  - label: Supporters\n    value: \"212\"\n  - label: Closes\n    value: \"31 March 2027\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Given\n    value: \"750\"\n  - label: Needed\n    value: \"3,000\"\n",
    ),
    (
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
        "facts:\n  - label: Status\n    value: \"Open\"\n  - label: Goal\n    value: \"To be set\"\n",
    ),
];

/// The demo site built with Beacon, with three posts made campaigns.
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
    for (rel, facts) in CAMPAIGNS {
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
fn beacon_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest campaign leads beside the donate button, the other
/// campaigns follow as cards, and posts without facts are the news, so no
/// post shows twice. The impact figures and the tagline show too.
#[test]
fn beacon_front_page_leads_with_the_newest_campaign() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let lead = texts(&page, ".bc-appeal .bc-card h3");
    assert_eq!(lead, ["Fixing slow pages, starting with the biggest win"]);
    let cards = texts(&page, ".bc-section .bc-card h3");
    assert_eq!(
        cards,
        ["How to write a title tag", "Why internal links matter"]
    );
    let news = texts(&page, ".bc-news .feed-item h3");
    assert!(!news.is_empty());
    assert!(
        news.iter()
            .all(|title| !cards.contains(title) && !lead.contains(title)),
        "{news:?}"
    );
    assert_eq!(texts(&page, ".figure dd"), ["42 km", "1,240"]);
    assert!(page.contains("We bring the river back to life."));
    assert!(
        page.contains("<a class=\"button\" href=\"https://www.example.com/donate/\">Donate</a>")
    );
}

/// The goal meter reads the digits of the first two facts, so "£6,150" of
/// "£10,000" is 61 per cent, and shows nothing when they hold no number.
#[test]
fn the_goal_meter_is_built_from_the_campaign_facts() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".goal-share"),
        ["61% of the goal", "25% of the goal"]
    );
    assert!(page.contains("<meter class=\"goal-bar\" min=\"0\" max=\"10000\" value=\"6150\""));
    assert!(page.contains("<meter class=\"goal-bar\" min=\"0\" max=\"3000\" value=\"750\""));
}

#[test]
fn beacon_campaign() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A campaign's panel holds its goal meter, facts 3 and 4 and the donate
/// button; a news post has no panel.
#[test]
fn a_campaign_shows_its_panel_and_a_news_post_does_not() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".bc-panel .goal-done dd"), ["£6,150"]);
    assert_eq!(texts(&page, ".bc-facts dt"), ["Supporters", "Closes"]);
    assert_eq!(texts(&page, ".bc-panel .bc-give a"), ["Donate"]);
    assert!(
        texts(&page, ".cta-band .cta-note")
            .concat()
            .contains("Every gift")
    );
    let news = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&news, ".bc-panel").is_empty(),
        "a news post has no panel"
    );
}

#[test]
fn beacon_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn beacon_team_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
