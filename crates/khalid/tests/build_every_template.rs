//! Every template the engine has builds the demo site's home page, a post
//! and an author page, each with one `h1`, a label on every `nav` and no
//! id twice. The outline of each page's main content is a snapshot per
//! template, so a template another builder adds gets its own snapshot and
//! the Look screen can offer it (templates spec 4.5 and 4.6, item 8).

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use scraper::{ElementRef, Html, Selector};

/// The pages each template must build, by their file in the output.
#[cfg(test)]
const PAGES: [&str; 3] = [
    "index.html",
    "why-internal-links-matter/index.html",
    "authors/asif/index.html",
];

/// The elements an outline names: the page's landmarks and headings.
#[cfg(test)]
const OUTLINED: [&str; 9] = [
    "header", "nav", "section", "aside", "article", "footer", "h1", "h2", "h3",
];

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

/// Builds a copy of the demo site in the template `name`.
#[cfg(test)]
fn built(name: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let toml = at(temp.path(), "site.toml");
    let mut text = fs::read_to_string(&toml).unwrap();
    write!(text, "\n[template]\nname = \"{name}\"\n").unwrap();
    fs::write(&toml, text).unwrap();
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{name}: {stderr}");
    (temp, out)
}

/// Checks `html` the way the demo check reads it, and gives the outline of
/// its main content: one line per landmark and heading, indented by depth,
/// leaving out the post's own text.
#[cfg(test)]
fn outline(rel: &str, html: &str) -> String {
    let page = Html::parse_document(html);
    let all = |css: &str| page.select(&Selector::parse(css).unwrap()).count();
    assert_eq!(all("h1"), 1, "{rel}: one h1");
    assert_eq!(all("nav:not([aria-label])"), 0, "{rel}: every nav labelled");
    let ids: Vec<&str> = (page.select(&Selector::parse("[id]").unwrap()))
        .filter_map(|element| element.value().id())
        .collect();
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), ids.len(), "{rel}: no id twice");
    let main = page.select(&Selector::parse("main").unwrap()).next();
    let mut lines = String::new();
    walk(main.unwrap(), 0, &mut lines);
    lines
}

/// Adds the outline of `element`'s children at `depth` to `lines`.
#[cfg(test)]
fn walk(element: ElementRef<'_>, depth: usize, lines: &mut String) {
    for child in element.children().filter_map(ElementRef::wrap) {
        let tag = child.value().name();
        let classes: Vec<&str> = child.value().classes().collect();
        if classes
            .iter()
            .any(|class| ["post-body", "page-body"].contains(class))
        {
            writeln!(
                lines,
                "{}{tag}.{}: the text",
                "  ".repeat(depth),
                classes.join(".")
            )
            .unwrap();
            continue;
        }
        let next = if OUTLINED.contains(&tag) {
            let mut line = format!("{}{tag}", "  ".repeat(depth));
            for class in &classes {
                write!(line, ".{class}").unwrap();
            }
            if let Some(label) = child.value().attr("aria-label") {
                write!(line, " \"{label}\"").unwrap();
            }
            if tag.starts_with('h') && tag.len() == 2 {
                let text: String = child.text().collect();
                write!(
                    line,
                    ": {}",
                    text.split_whitespace().collect::<Vec<_>>().join(" ")
                )
                .unwrap();
            }
            writeln!(lines, "{line}").unwrap();
            depth + 1
        } else {
            depth
        };
        walk(child, next, lines);
    }
}

#[test]
fn every_template_builds_its_post_index_and_author_pages() {
    for pack in khalid_theme::packs() {
        let (_site, out) = built(pack.name);
        let mut pages = String::new();
        for rel in PAGES {
            let html = fs::read_to_string(at(&out, rel)).unwrap();
            writeln!(pages, "# {rel}\n{}", outline(rel, &html)).unwrap();
        }
        insta::assert_snapshot!(format!("outline_{}", pack.name), pages);
    }
}
