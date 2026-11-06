//! The Bulletin template end to end: `khalid build` of the demo site with
//! `[template] name = "bulletin"`, a logo and a `[mail]` table, so the
//! front page's sign-up panel holds the newsletter form, then the header
//! and main part of its front page, a post and an author page.

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

/// `[mail]`, so the blog takes sign-ups. Nothing is sent while building.
const MAIL: &str = "\n[mail]\nfrom = \"news@example.com\"\nname = \"Field Notes\"\n\n\
     [mail.smtp]\nhost = \"127.0.0.1\"\nport = 2525\nsecurity = \"none\"\nprivate_host = true\n";

/// The demo site built with Bulletin, a logo from the main site, mail, and
/// `options` under `[template.options]`.
#[cfg(test)]
fn built(options: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(MAIL);
    toml.push_str("\n[template]\nname = \"bulletin\"\n[template.options]\n");
    toml.push_str(options);
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
fn bulletin_front_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "index.html"));
}

#[test]
fn bulletin_post() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn bulletin_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}

#[test]
fn the_front_page_puts_the_newsletter_form_in_the_sign_up_panel() {
    let (_temp, out) = built("");
    let page = shown(&out, "index.html");
    let panel = page.split_once("<section class=\"split-cta\"").unwrap().1;
    let panel = panel
        .split_once("</section>\n<section class=\"split-latest\"")
        .unwrap()
        .0;
    assert!(
        panel.contains("<form class=\"subscribe\" method=\"post\" action=\"/blog/subscribe\">"),
        "{panel}"
    );
    assert!(page.contains("<article class=\"split-issue\">"), "{page}");
}

#[test]
fn image_width_sets_how_wide_the_lead_picture_runs() {
    for (width, class) in [
        ("small", "<article class=\"post\">"),
        ("wide", "<article class=\"post post-image-wide\">"),
        ("full", "<article class=\"post post-image-full\">"),
    ] {
        let (_temp, out) = built(&format!("image_width = \"{width}\"\n"));
        let page = shown(&out, "fixing-slow-pages/index.html");
        assert!(page.contains(class), "{width}");
    }
}
