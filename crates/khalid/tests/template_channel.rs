//! The Channel template end to end (docs/templates/channel.md): `khalid
//! build` of the demo site with `[template] name = "channel"`, one older
//! post given a video and a running time, then the header and main part
//! of its front page, the video post, a list and the hosts page.

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

const SETTINGS: &str = "\n[template]\nname = \"channel\"\n[template.options]\nhero_stats = [\"9 | videos\", \"4 | playlists\"]\ntagline = [\"Short videos on writing for search.\"]\nshelves = 2\n";

/// The post that becomes a video post: its facts, the first the running
/// time, and a video as the first block of its body. It is not the newest
/// post, so Now showing must find it by its video.
const VIDEO_POST: &str = "content/en/posts/2026-09-21-how-to-write-a-title-tag.md";
const VIDEO_FACTS: &str =
    "facts:\n  - label: Length\n    value: \"4:05\"\n  - label: Level\n    value: \"Beginner\"\n";
const VIDEO_BLOCK: &str = "::video{src=\"/media/title-tags.mp4\" title=\"Writing a title tag\" poster=\"/media/title-tags.jpg\" duration=\"4:05\"}\n\n";

/// The demo site built with Channel, with one post made a video post.
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
    let path = at(temp.path(), VIDEO_POST);
    let text = fs::read_to_string(&path).unwrap();
    let text = text.replacen('\n', &format!("\n{VIDEO_FACTS}"), 1);
    let text = text.replacen("---\n\n", &format!("---\n\n{VIDEO_BLOCK}"), 1);
    fs::write(path, text).unwrap();
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
fn channel_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Now showing is the newest post with a video, found as the video row
/// finds it, not the newest post; it shows once, and every card with a
/// running time carries it at the end of its play bar.
#[test]
fn now_showing_is_the_newest_post_with_a_video() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".lead-video h3"), ["How to write a title tag"]);
    let latest = texts(&page, ".latest-videos .video-card h3");
    assert!(!latest.is_empty(), "expected something here");
    assert!(
        !latest.contains(&"How to write a title tag".to_owned()),
        "{latest:?}"
    );
    assert!(
        texts(&page, ".lead-bar .play-time")
            .concat()
            .contains("4:05")
    );
    assert_eq!(texts(&page, ".figure dd"), ["9", "4"]);
    assert!(texts(&page, ".playlists .shelf").len() <= 2);
}

/// A video post opens on its video and leaves its lead picture out; a
/// post without one keeps the lead picture.
#[test]
fn a_video_post_opens_on_its_video() {
    let (_temp, out) = built();
    let page = shown(&out, "how-to-write-a-title-tag/index.html");
    let body = page.split("<div class=\"post-body\">").nth(1).unwrap();
    assert!(body.trim_start().starts_with("<figure class=\"video\">"));
    assert_eq!(texts(&page, ".video-facts dd"), ["4:05", "Beginner"]);
    assert!(
        texts(&page, ".post-header .image").is_empty(),
        "expected nothing here"
    );
}

#[test]
fn channel_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "how-to-write-a-title-tag/index.html"));
}

#[test]
fn channel_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/on-page-seo/index.html"));
}

#[test]
fn channel_hosts_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
