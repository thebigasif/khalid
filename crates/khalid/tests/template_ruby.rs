//! The Ruby template end to end, based on Ghost's Ruby theme: `khalid
//! build` of the demo site with `[template] name = "ruby"` and a logo, then
//! the header and main part of its front page, a post and an author page,
//! and the card grid on the front page and after a post.

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

const SETTINGS: &str = "\n[template]\nname = \"ruby\"\n";

/// The demo site built with Ruby, a logo from the main site and `options`
/// as its `[template.options]`.
#[cfg(test)]
fn built_with(options: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(SETTINGS);
    if !options.is_empty() {
        toml.push_str("\n[template.options]\n");
        toml.push_str(options);
    }
    fs::write(&settings, toml).unwrap();
    let tokens = at(temp.path(), "theme/tokens.toml");
    fs::create_dir_all(tokens.parent().unwrap()).unwrap();
    fs::write(
        tokens,
        "[brand]\nlogo = \"https://www.example.com/logo.svg\"\n",
    )
    .unwrap();
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

/// The demo site built with Ruby's defaults.
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    built_with("")
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

#[test]
fn ruby_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

#[test]
fn ruby_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn ruby_author_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}

#[test]
fn the_front_page_lists_every_post_as_a_ruby_card() {
    let (_temp, out) = built();
    let home = shown(&out, "index.html");
    assert!(
        home.contains("<ul class=\"rb-feed\" role=\"list\">"),
        "{home}"
    );
    let cards = home.matches("<article class=\"rb-card\">").count();
    let titles = home.matches("<h3 class=\"rb-title\">").count();
    assert!(cards > 0, "{home}");
    assert_eq!(cards, titles, "one title per card");
    assert_eq!(cards, home.matches("<p class=\"rb-foot\">").count());
}

#[test]
fn a_post_puts_its_meta_line_over_the_title() {
    let (_temp, out) = built();
    let post = shown(&out, "fixing-slow-pages/index.html");
    let meta = post.find("<div class=\"post-meta rb-meta\">").unwrap();
    let title = post.find("<h1>").unwrap();
    assert!(meta < title, "{post}");
}

#[test]
fn no_ruby_page_needs_a_script_of_its_own() {
    let (_temp, out) = built_with("contents = \"side-end\"\n");
    for rel in [
        "index.html",
        "fixing-slow-pages/index.html",
        "authors/asif/index.html",
    ] {
        let main = shown(&out, rel);
        assert!(!main.contains("<script"), "{rel}");
    }
}
