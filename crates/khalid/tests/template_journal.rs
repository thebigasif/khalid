//! The Journal template end to end: `khalid build` of the demo site with
//! `[template] name = "journal"`, a logo and a `[mail]` table, so the
//! front page's side column and sign-up band hold the newsletter form,
//! then the header and main part of its front page, a post and an author
//! page.

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

/// The demo site built with Journal, a logo from the main site, mail, and
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
    toml.push_str("\n[template]\nname = \"journal\"\n[template.options]\n");
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
fn journal_front_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "index.html"));
}

#[test]
fn journal_post() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn journal_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}

#[test]
fn the_front_page_opens_with_the_latest_issue_and_asks_for_the_email_twice() {
    let (_temp, out) = built("");
    let page = shown(&out, "index.html");
    assert!(
        page.contains("<article class=\"journal-latest\">"),
        "{page}"
    );
    let side = page.split_once("<aside class=\"journal-side\"").unwrap().1;
    let side = side.split_once("</aside>").unwrap().0;
    assert!(side.contains("id=\"journal-signup-email\""), "{side}");
    let band = page.split_once("<div class=\"journal-band\">").unwrap().1;
    assert!(band.contains("id=\"journal-band-email\""), "{band}");
}

#[test]
fn sidebar_off_leaves_the_past_issues_alone() {
    let (_temp, out) = built("sidebar = false\n");
    let page = shown(&out, "index.html");
    assert!(!page.contains("journal-side"), "{page}");
    assert!(
        page.contains("<div class=\"journal-columns journal-single\">"),
        "{page}"
    );
}

#[test]
fn a_post_ends_with_a_ruled_row_back_and_on() {
    let (_temp, out) = built("");
    let page = shown(&out, "fixing-slow-pages/index.html");
    let row = page.split_once("<nav class=\"journal-turn\"").unwrap().1;
    let row = row.split_once("</nav>").unwrap().0;
    assert!(row.contains("<p class=\"journal-back\">"), "{row}");
    assert!(row.contains("<div class=\"share\">"), "{row}");
}
