//! The pictures of the showcase site and the example sites: every post has
//! its own cover, every author a portrait and every brand a logo, each a
//! plain SVG at the size the templates expect, and the built home page
//! shows them on its cards.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The collections whose entries show on cards, so each needs a cover.
const CARDS: [&str; 3] = ["posts", "updates", "work"];

/// What no picture here may hold: anything that runs, loads another file
/// or needs a font. A cover holds no words either, so one picture serves
/// every language of a post.
const REFUSED: [&str; 6] = [
    "<script",
    "<style",
    "<foreignObject",
    "href=",
    "<image",
    "<text",
];

#[cfg(test)]
fn fixtures() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../fixtures");
    path
}

#[cfg(test)]
fn under(base: &Path, parts: &[&str]) -> PathBuf {
    let mut path = base.to_path_buf();
    for part in parts {
        path.push(part);
    }
    path
}

/// Copies the folder `from` into `into`, leaving out what a build made.
#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        if ["data", "public"].contains(&entry.file_name().to_str().unwrap()) {
            continue;
        }
        let target = under(into, &[entry.file_name().to_str().unwrap()]);
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

/// The showcase site and every example site.
#[cfg(test)]
fn sites() -> Vec<PathBuf> {
    let mut sites = vec![under(&fixtures(), &["showcase-site"])];
    let examples = under(&fixtures(), &["examples"]);
    for entry in fs::read_dir(examples).unwrap() {
        let path = entry.unwrap().path();
        if under(&path, &["site.toml"]).is_file() {
            sites.push(path);
        }
    }
    sites.sort();
    sites
}

/// Every `content/<language>/<collection>/<entry>/` folder of `site`, with
/// its collection's name.
#[cfg(test)]
fn entries(site: &Path) -> Vec<(String, PathBuf)> {
    let mut found = Vec::new();
    for language in fs::read_dir(under(site, &["content"])).unwrap() {
        for collection in fs::read_dir(language.unwrap().path()).unwrap() {
            let collection = collection.unwrap().path();
            let name = collection.file_name().unwrap().to_str().unwrap().to_owned();
            for entry in fs::read_dir(&collection).unwrap() {
                let entry = entry.unwrap().path();
                if entry.is_dir() {
                    found.push((name.clone(), entry));
                }
            }
        }
    }
    found.sort();
    found
}

/// The SVG at `path`, checked to be plain and `width` by `height`.
#[cfg(test)]
fn plain_svg(path: &Path, width: u32, height: u32, refused: &[&str]) {
    let svg =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let root = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\""
    );
    assert!(
        svg.starts_with(&root),
        "{} starts {:?}",
        path.display(),
        svg.get(..120)
    );
    assert!(svg.trim_end().ends_with("</svg>"), "{}", path.display());
    assert!(
        svg.len() < 8_000,
        "{} is {} bytes",
        path.display(),
        svg.len()
    );
    for word in refused {
        assert!(!svg.contains(word), "{} holds {word}", path.display());
    }
}

#[test]
fn every_post_has_a_cover_and_every_author_a_portrait() {
    for site in sites() {
        let entries = entries(&site);
        assert_ne!(entries, Vec::new(), "{}", site.display());
        for (collection, entry) in entries {
            let text = fs::read_to_string(under(&entry, &["index.md"])).unwrap();
            let front = text.split("\n---").next().unwrap();
            if CARDS.contains(&collection.as_str()) {
                assert!(
                    front.contains("\nimage:\n  src: cover.svg\n  alt: "),
                    "{}",
                    entry.display()
                );
                plain_svg(&under(&entry, &["cover.svg"]), 1200, 675, &REFUSED);
            } else if collection == "authors" {
                assert!(
                    front.contains("\nphoto: portrait.svg"),
                    "{}",
                    entry.display()
                );
                plain_svg(&under(&entry, &["portrait.svg"]), 160, 160, &REFUSED);
            }
        }
    }
}

#[test]
fn every_brand_has_a_logo_in_its_header_tokens() {
    for site in sites() {
        let tokens = fs::read_to_string(under(&site, &["theme", "tokens.toml"])).unwrap();
        assert!(
            tokens.contains("/blog/about/logo.svg\""),
            "{}",
            site.display()
        );
        let logo = under(&site, &["content", "en", "pages", "about", "logo.svg"]);
        // A logo's name is text, so only what runs or loads is refused.
        plain_svg(&logo, 320, 80, &REFUSED[..5]);
    }
}

#[test]
fn the_showcase_home_page_shows_covers_and_portraits_on_its_cards() {
    // A copy, so the build's `data/` stays out of the fixture.
    let site = tempfile::tempdir().unwrap();
    copy_dir(&under(&fixtures(), &["showcase-site"]), site.path());
    let out = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site.path())
        .args(["build", "--out"])
        .arg(out.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    let home = fs::read_to_string(under(out.path(), &["index.html"])).unwrap();
    assert!(home.matches("/cover.svg\" srcset=").count() >= 6, "{home}");
    assert!(home.contains("/portrait.svg\""), "{home}");
}
