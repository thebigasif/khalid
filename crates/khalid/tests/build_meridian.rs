//! The Meridian template (templates spec 3.9), end to end: `khalid build`
//! of the demo site with `[template] name = "meridian"`, and the main part
//! of its home, post and author pages as snapshots.

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
const MERIDIAN: &str = "\n[template]\nname = \"meridian\"\n\n[template.cta]\nlabel = \"Book a demo\"\nhref = \"https://www.example.com/demo/\"\nnote = \"See your own blog in Meridian.\"\n";

/// Builds a copy of the demo site in Meridian, with `extra` added to its
/// `site.toml` and the post `featured` marked featured.
#[cfg(test)]
fn built(extra: &str, featured: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let toml = fs::read_to_string(&settings).unwrap();
    fs::write(&settings, format!("{toml}{MERIDIAN}{extra}")).unwrap();
    let post = at(temp.path(), featured);
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

const FEATURED: &str = "content/en/posts/2026-09-21-how-to-write-a-title-tag.md";

#[test]
fn the_meridian_home_page() {
    let (_temp, out) = built("", FEATURED);
    insta::assert_snapshot!(shown(&out, "index.html"));
}

#[test]
fn meridian_ships_its_own_rules_after_the_shared_ones() {
    let (_temp, out) = built("", FEATURED);
    let assets = at(&out, "assets");
    let css = fs::read_dir(&assets)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("site.") && name.ends_with(".css")
        })
        .unwrap();
    let css = fs::read_to_string(css).unwrap();
    assert!(css.contains("--text-h1: clamp(2.5rem,"), "{css}");
    assert!(css.contains("--measure: 64ch;"), "{css}");
    assert!(css.contains("minmax(6rem, 34%)"), "{css}");
}

#[test]
fn a_site_option_wins_over_meridian_s_default() {
    let (_temp, out) = built("[template.options]\ncards = \"text\"\n", FEATURED);
    let home = shown(&out, "index.html");
    assert!(home.contains("<ul class=\"cards cards-text\""), "{home}");
}
