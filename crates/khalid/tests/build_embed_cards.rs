//! `khalid build` with a provider embed: the page shows a card that loads
//! nothing until the reader asks, and the feeds and `llms.txt` carry the
//! embed's title and link and no player (privacy plan U5, R1, R12, R13).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// A post with a video embed, built, and the file at `rel` in `public/`.
#[cfg(test)]
fn built(folder: &Path, rel: &str) -> String {
    let site = folder.to_str().unwrap();
    let khalid = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_khalid"))
            .args(args)
            .output()
            .unwrap()
    };
    let post = at(folder, "content/en/posts/tour");
    fs::create_dir_all(&post).unwrap();
    fs::write(
        at(&post, "index.md"),
        "---\ntitle: Tour\nslug: tour\ndate: 2026-09-14\nauthor: asif\n---\n\n\
         Watch the tour, then read [the home page](/).\n\n\
         ::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"How search works\"}\n",
    )
    .unwrap();
    khalid(&["init", site, "--address", "https://www.example.com"]);
    let output = khalid(&["--site", site, "build"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    fs::read_to_string(at(folder, &format!("public/{rel}")))
        .unwrap_or_else(|error| panic!("{rel}: {error}"))
}

/// The page with each inert `template` left out.
#[cfg(test)]
fn shown(html: &str) -> String {
    let mut kept = String::new();
    let mut rest = html;
    while let Some((before, after)) = rest.split_once("<template>") {
        kept.push_str(before);
        rest = after
            .split_once("</template>")
            .map_or("", |(_, after)| after);
    }
    kept.push_str(rest);
    kept
}

#[test]
fn a_post_page_holds_the_player_until_the_reader_asks() {
    let temp = tempfile::tempdir().unwrap();
    let page = built(temp.path(), "tour/index.html");
    assert!(
        page.contains("youtube-nocookie.com/embed/dQw4w9WgXcQ"),
        "{page}"
    );
    assert!(!shown(&page).contains("<iframe"), "{page}");
    assert!(shown(&page).contains(">Load media</a>"), "{page}");
}

#[test]
fn feeds_and_llms_txt_carry_the_title_and_link_and_no_player() {
    let temp = tempfile::tempdir().unwrap();
    built(temp.path(), "tour/index.html");
    for rel in ["feed.xml", "atom.xml", "feed.json"] {
        let feed = fs::read_to_string(at(temp.path(), &format!("public/{rel}"))).unwrap();
        assert!(feed.contains("How search works"), "{rel}: {feed}");
        assert!(
            feed.contains("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            "{rel}: {feed}"
        );
        for part in ["iframe", "template", "Load media"] {
            assert!(!feed.contains(part), "{part} in {rel}: {feed}");
        }
    }
    let llms = fs::read_to_string(at(temp.path(), "public/llms.txt")).unwrap();
    assert!(!llms.contains("iframe"), "{llms}");
}
