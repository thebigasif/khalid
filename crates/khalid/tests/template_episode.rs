//! The Episode template end to end: `khalid build` of the demo site with
//! `[template] name = "episode"`, a tagline, places to listen and one post
//! with a sound player, then the header and main part of its front page,
//! that post and an author page.

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

/// The template, a tagline, and two places to listen, the second of which
/// is not a web address and so must not show.
const SETTINGS: &str = "\n[template]\nname = \"episode\"\n[template.options]\ntagline = \"A weekly show about search.\"\nlisten_on = [\"Spotify | https://open.spotify.com/show/1\", \"Nowhere | javascript:alert(1)\"]\n";

/// The post that gets a sound player, a bundle folder with its own files.
const EPISODE: &str = "content/en/posts/2026-10-05-fixing-slow-pages";

/// The demo site built with Episode, a logo from the main site, and a
/// sound player in the post at [`EPISODE`].
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    built_with(true)
}

/// The demo site built with Episode, with the sound player when `episode`
/// is set and none at all when it is not.
#[cfg(test)]
fn built_with(episode: bool) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    copy_dir(&at(repo, "../../fixtures/demo-site"), temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(SETTINGS);
    fs::write(&settings, toml).unwrap();
    let tokens = at(temp.path(), "theme/tokens.toml");
    fs::create_dir_all(tokens.parent().unwrap()).unwrap();
    fs::write(
        tokens,
        "[brand]\nlogo = \"https://www.example.com/logo.svg\"\n",
    )
    .unwrap();
    if episode {
        let folder = at(temp.path(), EPISODE);
        fs::copy(
            at(
                repo,
                "../../fixtures/showcase-site/content/en/posts/2026-10-23-podcast-answer-first/episode-1-sample.wav",
            ),
            at(&folder, "episode.wav"),
        )
        .unwrap();
        let post = at(&folder, "index.md");
        let text = fs::read_to_string(&post).unwrap();
        let player =
            "---\n\n::audio{src=\"episode.wav\" title=\"Episode one: fixing slow pages\"}\n\n";
        fs::write(&post, text.replacen("---\n\n", player, 1)).unwrap();
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

/// The text of each element of `html` that `css` selects.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .collect()
}

#[test]
fn episode_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest post with a sound player leads the front page with its
/// player, which plays the post's own file from the front page, and the
/// list below leaves it out. Only a web address becomes a place to listen.
#[test]
fn episode_front_page_leads_with_the_newest_episode_and_its_player() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let lead = texts(&page, ".episode-latest h3");
    assert_eq!(lead, ["Fixing slow pages, starting with the biggest win"]);
    assert!(
        page.contains("src=\"/blog/fixing-slow-pages/episode.wav\""),
        "{page}"
    );
    assert!(at(&out, "fixing-slow-pages/episode.wav").is_file());
    let listed = texts(&page, ".episodes .card h3");
    assert!(!listed.is_empty(), "{page}");
    assert!(!listed.contains(&lead[0]), "{listed:?}");
    assert_eq!(
        texts(&page, ".episode-tagline"),
        ["A weekly show about search."]
    );
    assert_eq!(texts(&page, ".listen-on b"), ["Spotify"]);
    assert!(page.contains("href=\"https://open.spotify.com/show/1\""));
    assert!(!page.contains("javascript:"), "{page}");
}

/// With no post that carries a sound player, the front page has no lead
/// and lists every post.
#[test]
fn episode_front_page_without_an_episode_lists_every_post() {
    let (_temp, out) = built_with(false);
    let page = shown(&out, "index.html");
    assert!(!page.contains("episode-lead"), "{page}");
    assert!(!page.contains("<audio"), "{page}");
    let listed = texts(&page, ".episodes .card h3");
    assert!(
        listed.contains(&"Fixing slow pages, starting with the biggest win".to_owned()),
        "{listed:?}"
    );
}

#[test]
fn episode_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn episode_author_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}
