//! The built-in template packs end to end (templates spec 3.7 and 3.8):
//! `khalid build` of the demo site in each pack, with sign-ups, a call to
//! action and a featured post, and a snapshot of the main part of its
//! front page, a post and an author page. Each page also passes the plain
//! HTML checks: one `h1` and no id twice.

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

/// What every pack's build adds to the demo's `site.toml`: sign-ups by
/// email and a call to action.
const SETTINGS: &str = "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\n\
host = \"127.0.0.1\"\nport = 2525\nsecurity = \"none\"\nprivate_host = true\n\n\
[template]\nname = \"{name}\"\n\n[template.cta]\nlabel = \"Start a free trial\"\n\
href = \"https://www.example.com/signup/\"\nnote = \"No card needed.\"\n";

/// The demo post marked featured, so the front page has its featured slot.
const FEATURED: &str = "content/en/posts/2026-09-21-how-to-write-a-title-tag.md";

/// The demo post whose page each pack snapshots.
const POST: &str = "what-a-sitemap-does/index.html";

/// Builds a copy of the demo site in the pack `name` and expects success.
#[cfg(test)]
fn built(name: &str) -> (tempfile::TempDir, PathBuf) {
    built_with(name, "")
}

/// Builds like [`built`], with `options` as the `[template.options]` table.
#[cfg(test)]
fn built_with(name: &str, options: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(&SETTINGS.replace("{name}", name));
    if !options.is_empty() {
        toml.push_str("\n[template.options]\n");
        toml.push_str(options);
        toml.push('\n');
    }
    fs::write(&settings, toml).unwrap();
    let featured = at(temp.path(), FEATURED);
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

/// The page at `rel`, after the plain HTML checks.
#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    let page = fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"));
    assert_eq!(page.matches("<h1").count(), 1, "one h1 in {rel}");
    let mut ids: Vec<&str> = page
        .split(" id=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap())
        .collect();
    let all = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), all, "an id twice in {rel}");
    page
}

/// The `<main>` of `page`, with a post's body cut down to one line, so a
/// snapshot shows the template and not the writing.
#[cfg(test)]
fn main_of(page: &str) -> String {
    let start = page.find("<main").unwrap();
    let end = page.find("</main>").unwrap();
    let main = page.get(start..end).unwrap();
    let open = "<div class=\"post-body\">";
    let Some(body) = main.find(open) else {
        return main.to_owned();
    };
    let inside = body + open.len();
    let (mut depth, mut at) = (1, inside);
    while depth > 0 {
        let rest = main.get(at..).unwrap();
        let close = rest.find("</div>").unwrap();
        match rest.find("<div") {
            Some(next) if next < close => {
                depth += 1;
                at += next + 4;
            }
            _ => {
                depth -= 1;
                at += close + if depth > 0 { 6 } else { 0 };
            }
        }
    }
    format!(
        "{}\n[the post's body]\n{}",
        main.get(..inside).unwrap(),
        main.get(at..).unwrap()
    )
}

#[test]
fn workbench_pages() {
    let (_temp, out) = built("workbench");
    insta::assert_snapshot!("workbench_home", main_of(&page(&out, "index.html")));
    insta::assert_snapshot!("workbench_post", main_of(&page(&out, POST)));
    insta::assert_snapshot!(
        "workbench_author",
        main_of(&page(&out, "authors/asif/index.html"))
    );
}

#[test]
fn workbench_options_change_the_closing_line_and_the_newsletter_note() {
    let options = "closing_line = false\nnewsletter_note = [\"A new review every Tuesday.\"]";
    let (_temp, out) = built_with("workbench", options);
    let note = "<p class=\"newsletter-note\">A new review every Tuesday.</p>";
    let home = page(&out, "index.html");
    assert_eq!(home.matches(note).count(), 2, "{home}");
    let post = page(&out, POST);
    assert!(!post.contains("wb-closing"), "{post}");
    assert!(
        !post.contains("newsletter-note"),
        "the note is for the front page"
    );
}
