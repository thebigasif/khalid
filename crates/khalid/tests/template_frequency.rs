//! The Frequency template end to end (docs/templates/frequency.md):
//! `khalid build` of the demo site with `[template] name = "frequency"`,
//! listen links and two posts made episodes, each with a player at the
//! top of its notes and an episode's facts (the number, the running time,
//! the guest, then one of the writer's own). Then the header and main part
//! of its front page, an episode, a list and the hosts page, and the
//! running-time dial built from each episode's own facts.

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

const SETTINGS: &str = "\n[template]\nname = \"frequency\"\n[template.options]\ntagline = [\"A weekly show about search.\"]\nlisten_on = [\"Apple Podcasts | https://www.example.com/apple/\", \"RSS feed | /blog/feed.xml\"]\nhero_stats = [\"12 | episodes\"]\n[template.cta]\nlabel = \"Follow the show\"\nhref = \"https://www.example.com/follow/\"\nnote = \"A new episode every Thursday.\"\n";

/// Two posts that become episodes: their facts, in the order Frequency
/// reads them, and the player put first in their notes. The second runs
/// past the 90-minute scale, so its dial widens to fit.
const EPISODES: [(&str, &str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Episode\n    value: \"12\"\n  - label: Length\n    value: \"48 min\"\n  - label: Guest\n    value: \"Lena Vogt\"\n  - label: Season\n    value: \"2\"\n",
        "::audio{src=\"episode-12.mp3\" title=\"Episode 12: fixing slow pages\"}\n\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Episode\n    value: \"11\"\n  - label: Length\n    value: \"112 min\"\n",
        "::audio{src=\"/media/episode-11.mp3\" title=\"Episode 11: title tags\"}\n\n",
    ),
];

/// The demo site built with Frequency, with two posts made episodes.
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
    for (rel, facts, player) in EPISODES {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        let text = text.replacen('\n', &format!("\n{facts}"), 1);
        let text = text.replacen("\n---\n\n", &format!("\n---\n\n{player}"), 1);
        fs::write(path, text).unwrap();
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
fn frequency_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest episode with a player leads in the console and plays there,
/// from its own folder; every other post is a row, so none shows twice.
/// A post without facts is a row with no number.
#[test]
fn frequency_front_page_plays_the_newest_episode() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let lead = texts(&page, ".fq-air h3");
    assert_eq!(lead, ["Fixing slow pages, starting with the biggest win"]);
    assert!(
        page.contains("src=\"/blog/fixing-slow-pages/episode-12.mp3\""),
        "the console plays the newest episode"
    );
    let rows = texts(&page, ".fq-row h3");
    assert!(
        rows.contains(&"How to write a title tag".to_owned()),
        "{rows:?}"
    );
    assert!(!rows.contains(&lead[0]), "{rows:?}");
    assert_eq!(texts(&page, ".fq-row .fq-figure"), ["11"]);
    assert_eq!(
        texts(&page, ".hero .link-label"),
        ["Apple Podcasts", "RSS feed"]
    );
    assert!(page.contains("A weekly show about search."));
}

/// The dial reads the digits of the second fact on a 90-minute scale, and
/// a longer episode widens the scale to fit.
#[test]
fn the_dial_is_built_from_the_running_time() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert!(page.contains("<meter class=\"fq-needle\" min=\"0\" max=\"90\" value=\"48\""));
    assert!(page.contains("<meter class=\"fq-needle\" min=\"0\" max=\"112\" value=\"112\""));
}

#[test]
fn frequency_episode() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// An episode's plate holds every fact, the player opens its notes right
/// after it, and the listen links follow the notes; a post without facts
/// has no plate.
#[test]
fn an_episode_shows_its_plate_and_a_note_does_not() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&page, ".fq-plate dt"),
        ["Episode", "Length", "Guest", "Season"]
    );
    assert_eq!(
        texts(
            &page,
            ".fq-plate + .post-body > figure.audio:first-child figcaption"
        ),
        ["Episode 12: fixing slow pages"],
        "the player opens the notes right after the plate"
    );
    assert_eq!(
        texts(&page, ".fq-listen .link-label"),
        ["Apple Podcasts", "RSS feed"]
    );
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&note, ".fq-plate").is_empty(), "a note has no plate");
}

#[test]
fn frequency_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn frequency_hosts_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
