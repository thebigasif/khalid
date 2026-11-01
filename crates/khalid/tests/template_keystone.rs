//! The Keystone template (templates spec 3.3), end to end: `khalid build`
//! of the demo site with `[template] name = "keystone"` gives a featured
//! post above a picture grid and no author box or related posts, and a
//! site's own options and `[reader]` keys win.

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

/// Builds a copy of the demo site in Keystone with `table` added to its
/// `site.toml`, one post marked featured, and expects success.
#[cfg(test)]
fn built(table: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str("\n[template]\nname = \"keystone\"\n");
    toml.push_str(table);
    fs::write(&settings, toml).unwrap();
    let featured = at(
        temp.path(),
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
    );
    let post = fs::read_to_string(&featured).unwrap();
    fs::write(&featured, post.replacen('\n', "\nfeatured: true\n", 1)).unwrap();
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

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

/// The page's `main` element, the part a template changes.
#[cfg(test)]
fn main_of(page: &str) -> &str {
    let (_, main) = page.split_once("<main").unwrap();
    let (main, _) = main.split_once("</main>").unwrap();
    main
}

#[test]
fn the_keystone_index_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(main_of(&page(&out, "index.html")));
}

#[test]
fn the_keystone_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(main_of(&page(&out, "authors/lena-vogt/index.html")));
}

#[test]
fn keystone_turns_on_its_parts_and_ships_its_own_rules() {
    let (_temp, out) = built("");
    let home = page(&out, "index.html");
    assert!(home.contains("<section class=\"featured\""), "{home}");
    assert!(
        home.contains("<ul class=\"cards\" role=\"list\">"),
        "{home}"
    );
    let post = page(&out, "what-a-sitemap-does/index.html");
    assert!(!post.contains("class=\"author-box\""), "{post}");
    assert!(!post.contains("class=\"related"), "{post}");
    assert!(!post.contains("class=\"reading-time\""), "{post}");
    assert!(!post.contains("class=\"toc-"), "{post}");
    let (_, sheet) = post
        .split_once("<link rel=\"stylesheet\" href=\"/blog/")
        .unwrap();
    let (sheet, _) = sheet.split_once('"').unwrap();
    let css = page(&out, sheet);
    assert!(css.contains(".topic-row"), "{sheet}");
}

#[test]
fn a_site_switches_keystone_options_off_and_its_reader_keys_win() {
    let (_temp, out) = built(
        "[template.options]\nfeatured = false\nshare = \"none\"\n\n[reader]\nauthor_box = true\n",
    );
    let home = page(&out, "index.html");
    assert!(!home.contains("class=\"featured\""), "{home}");
    let post = page(&out, "what-a-sitemap-does/index.html");
    assert!(!post.contains("class=\"share"), "{post}");
    assert!(post.contains("class=\"author-box\""), "{post}");
}
