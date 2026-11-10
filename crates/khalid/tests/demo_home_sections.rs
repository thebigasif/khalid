//! The demo site with a `[reader] home_sections` list shows the featured,
//! category, newsletter, series and latest sections on its home page, the
//! same from `khalid build` and from `khalid serve` (gap G19). Each
//! section is plain HTML in the page, so a reader with scripts off sees
//! all of it.

#[expect(
    dead_code,
    reason = "these tests only read pages, so the Admin API helpers stay unused"
)]
mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use common::{demo_copy, start};
use tempfile::TempDir;

const SECTIONS: &str = r#"
[reader]
home_sections = [
  "featured",
  { category = "technical-seo" },
  "newsletter",
  { series = "links-and-titles" },
  "latest",
]

[mail]
from = "news@example.com"
name = "Field Notes"

[mail.smtp]
host = "127.0.0.1"
port = 2525
security = "none"
private_host = true
"#;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Adds `extra` to the front matter of the demo post `file`, after its
/// author line.
#[cfg(test)]
fn mark(site: &Path, file: &str, extra: &str) {
    let path = at(site, &format!("content/en/posts/{file}"));
    let text = fs::read_to_string(&path).unwrap();
    let marked = text.replacen("author: asif\n", &format!("author: asif\n{extra}"), 1);
    assert_ne!(marked, text, "{file} has an author line");
    fs::write(path, marked).unwrap();
}

/// A copy of the demo site with the sections list, one featured post and
/// a series of two parts.
#[cfg(test)]
fn demo_with_sections() -> TempDir {
    let site = demo_copy();
    let toml = at(site.path(), "site.toml");
    let text = fs::read_to_string(&toml).unwrap();
    fs::write(&toml, format!("{text}{SECTIONS}")).unwrap();
    mark(
        site.path(),
        "2026-10-12-planning-a-topic-cluster.md",
        "featured: true\n",
    );
    let series = "series:\n  name: Links and titles\n  order: ";
    mark(
        site.path(),
        "2026-09-14-why-internal-links-matter.md",
        &format!("{series}1\n"),
    );
    mark(
        site.path(),
        "2026-09-21-how-to-write-a-title-tag.md",
        &format!("{series}2\n"),
    );
    site
}

/// The `<main>` of a home page.
#[cfg(test)]
fn main_of(page: &str) -> String {
    let (_, main) = page.split_once("<main").unwrap();
    let (main, _) = main.split_once("</main>").unwrap();
    main.to_owned()
}

/// The text between `start` and the next `end` in `rest`, if any.
#[cfg(test)]
fn between<'a>(rest: &'a str, start: &str, end: &str) -> Option<(usize, &'a str)> {
    let from = rest.find(start)? + start.len();
    let len = rest.get(from..)?.find(end)?;
    Some((from, rest.get(from..from + len)?))
}

/// The section headings, links and forms of `main` in page order, one a
/// line: what a reader with scripts off can read and follow.
#[cfg(test)]
fn outline(main: &str) -> String {
    let mut found: Vec<(usize, String)> = Vec::new();
    for (start, end, label) in [
        ("<h2", "</h2>", "##"),
        ("href=\"", "\"", "  link"),
        ("<form", ">", "  form"),
    ] {
        let mut offset = 0;
        while let Some((from, text)) = main
            .get(offset..)
            .and_then(|rest| between(rest, start, end))
        {
            let text = match label {
                "##" => text.split_once('>').map_or(text, |(_, words)| words),
                _ => text,
            };
            found.push((offset + from, format!("{label} {}", text.trim())));
            offset += from + text.len();
        }
    }
    found.sort();
    let lines: Vec<String> = found.into_iter().map(|(_, line)| line).collect();
    lines.join("\n")
}

#[test]
fn the_demo_home_page_shows_every_section_from_build() {
    let site = demo_with_sections();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site.path())
        .arg("build")
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{said}");
    let page = fs::read_to_string(at(site.path(), "public/index.html")).unwrap();
    let main = main_of(&page);
    assert!(
        !main.contains("<template"),
        "nothing waits for a script: {main}"
    );
    assert!(
        !main.contains("<noscript"),
        "nothing waits for a script: {main}"
    );
    insta::assert_snapshot!("demo_home_sections", outline(&main));
}

#[test]
fn the_demo_home_page_shows_every_section_from_serve() {
    let site = demo_with_sections();
    let server = start(site.path(), "unused");
    assert!(server.answers("/blog/", 200), "the home page answers");
    let (status, page) = server.raw("GET", "/blog/", &[], None);
    assert_eq!(status, 200, "{page}");
    insta::assert_snapshot!("demo_home_sections", outline(&main_of(&page)));
}
