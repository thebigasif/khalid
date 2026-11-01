//! The Folio template (templates spec 3.10), end to end: `khalid build`
//! of the demo site with `[template] name = "folio"`, and its home, post
//! and author pages as snapshots.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

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

/// The settings every test here adds to the demo's `site.toml`.
const FOLIO: &str = "\n[template]\nname = \"folio\"\n\n[template.cta]\nlabel = \"Try it free\"\nhref = \"https://www.example.com/signup/\"\nnote = \"Set up in five minutes.\"\n";

/// The options the snapshots show: a tagline and a quote.
const OPTIONS: &str = "\n[template.options]\ntagline = [\"Notes from the team that builds the blog.\"]\nquote = [\"Write for one reader at a time.\", \"Lena Vogt\"]\n";

/// The post marked featured, which leads the front page.
const FEATURED: &str = "content/en/posts/2026-09-21-how-to-write-a-title-tag.md";

/// Builds a copy of the demo site in Folio, with `extra` added to its
/// `site.toml` and one post marked featured.
#[cfg(test)]
fn built(extra: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let toml = fs::read_to_string(&settings).unwrap();
    fs::write(&settings, format!("{toml}{FOLIO}{extra}")).unwrap();
    let post = at(temp.path(), FEATURED);
    let text = fs::read_to_string(&post).unwrap();
    fs::write(&post, text.replacen('\n', "\nfeatured: true\n", 1)).unwrap();
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

/// The page at `rel`, from the site header to the end of `<main>`.
#[cfg(test)]
fn shown(out: &Path, rel: &str) -> String {
    let page = fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"));
    let start = page.find("<header class=\"site-header\">").unwrap();
    let end = page.find("</main>").unwrap();
    page.get(start..end).unwrap().to_owned()
}

#[test]
fn the_folio_home_page() {
    let (_temp, out) = built(OPTIONS);
    insta::assert_snapshot!(shown(&out, "index.html"));
}

#[test]
fn folio_ships_its_own_rules_after_the_shared_ones() {
    let (_temp, out) = built("");
    let css = fs::read_dir(at(&out, "assets"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("site.") && name.ends_with(".css")
        })
        .unwrap();
    let css = fs::read_to_string(css).unwrap();
    assert!(css.contains("--text-h1: clamp(2.5rem,"), "{css}");
    assert!(css.contains("--measure: 66ch;"), "{css}");
    assert!(css.contains("--radius: 0.75rem;"), "{css}");
    assert!(css.contains(".masthead-quote"), "{css}");
}

#[test]
fn without_its_options_the_masthead_shows_the_blog_s_name_and_no_quote() {
    let (_temp, out) = built("");
    let home = shown(&out, "index.html");
    assert!(
        home.contains("<header class=\"masthead\">\n<h1>Field Notes on SEO</h1>"),
        "{home}"
    );
    assert!(!home.contains("masthead-tagline"), "{home}");
    assert!(!home.contains("masthead-quote"), "{home}");
}

#[test]
fn the_masthead_takes_its_own_name() {
    let (_temp, out) = built("\n[template.options]\nmasthead_name = [\"The Workshop\"]\n");
    let home = shown(&out, "index.html");
    assert!(home.contains("<h1>The Workshop</h1>"), "{home}");
}

#[test]
fn a_quote_needs_no_name_beside_it() {
    let (_temp, out) = built("\n[template.options]\nquote = [\"Ship small.\"]\n");
    let home = shown(&out, "index.html");
    assert!(
        home.contains("<blockquote>\n<p>Ship small.</p>\n</blockquote>\n</figure>"),
        "{home}"
    );
}
