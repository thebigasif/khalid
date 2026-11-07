//! The Dialogue template end to end (docs/templates/dialogue.md): `khalid
//! build` of the demo site with `[template] name = "dialogue"` and two
//! posts given an episode's facts (the guest, who they are, the length and
//! one line they said), then the header and main part of its front page,
//! an episode, a list and the hosts page, and the latest guest, the guest
//! rows, the topics and the name plate.

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

const SETTINGS: &str = "\n[template]\nname = \"dialogue\"\n[template.options]\ntagline = [\"Conversations with people who make things.\"]\nlisten_links = [\"Podcast feed | https://www.example.com/feed.xml\", \"no address here\"]\n[template.cta]\nlabel = \"Suggest a guest\"\nhref = \"https://www.example.com/guests/\"\n";

/// Two posts that become episodes: their facts, in the order Dialogue
/// reads them. The second has no line of the guest's.
const EPISODES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Guest\n    value: \"Mira Castell\"\n  - label: Work\n    value: \"Potter, Porto\"\n  - label: Length\n    value: \"52 min\"\n  - label: In one line\n    value: \"Clay remembers every hurried minute.\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Guest\n    value: \"Tomasz Wren\"\n  - label: Work\n    value: \"Bookbinder\"\n  - label: Length\n    value: \"47 min\"\n",
    ),
];

/// The demo site built with Dialogue, with two posts made episodes.
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
    for (rel, facts) in EPISODES {
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
fn dialogue_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest episode leads as the latest guest, with the guest's name,
/// who they are and their line; the other episode is a guest row, which
/// shows the post's summary since it has no line; every post without
/// facts is a note, so no post shows twice. A listen link without an
/// address is left out.
#[test]
fn dialogue_front_page_leads_with_the_latest_guest() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".latest-guest-name"), ["Mira Castell"]);
    assert_eq!(texts(&page, ".latest-guest-role"), ["Potter, Porto"]);
    assert_eq!(
        texts(&page, ".latest-guest .quote-line"),
        ["Clay remembers every hurried minute."]
    );
    assert_eq!(
        texts(&page, ".conversations .guest-row h3"),
        ["How to write a title tag"]
    );
    assert_eq!(
        texts(&page, ".conversations .guest-row-name"),
        ["Guest: Tomasz Wren"]
    );
    assert!(texts(&page, ".conversations .quote-line").is_empty());
    assert_eq!(texts(&page, ".conversations .guest-row-summary").len(), 1);
    let notes = texts(&page, ".show-notes .feed-item h3");
    assert!(!notes.is_empty(), "expected some notes");
    assert!(
        notes
            .iter()
            .all(|title| title != "How to write a title tag"),
        "{notes:?}"
    );
    assert_eq!(texts(&page, ".listen-links a"), ["Podcast feed"]);
    assert!(page.contains("Conversations with people who make things."));
}

/// The topics list the categories that hold episodes, and leave out a
/// category whose posts are all notes, here Content strategy.
#[test]
fn the_topics_leave_out_a_category_of_notes_only() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let topics = texts(&page, ".topic-links a");
    assert!(topics.contains(&"Technical SEO".to_owned()), "{topics:?}");
    assert!(topics.contains(&"On-page SEO".to_owned()), "{topics:?}");
    assert!(
        !topics.contains(&"Content strategy".to_owned()),
        "{topics:?}"
    );
}

#[test]
fn dialogue_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// An episode opens with the name plate, host beside guest, then the
/// length beside the date and the guest's line as the quote line. A note
/// has neither in its own article, and shows its author in its meta line;
/// its related posts may still show an episode's line in a guest row.
#[test]
fn an_episode_opens_with_its_name_plate() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".name-plate-label"), ["Host", "Guest"]);
    assert_eq!(texts(&page, ".name-plate-name"), ["Mira Castell"]);
    assert_eq!(texts(&page, ".post-length"), ["Length: 52 min"]);
    assert!(
        texts(&page, ".post-header > .quote-line")
            .concat()
            .contains("Clay remembers every hurried minute.")
    );
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&note, "article.post .name-plate").is_empty(),
        "a note has no plate"
    );
    assert!(
        texts(&note, "article.post .quote-line").is_empty(),
        "a note has no quote"
    );
    assert_eq!(texts(&note, ".post-meta address").len(), 1);
}

#[test]
fn dialogue_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn dialogue_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
