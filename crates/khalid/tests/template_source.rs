//! The Source template end to end, based on Ghost's Source theme: `khalid
//! build` of the demo site with `[template] name = "source"`, a logo,
//! three featured posts and a call to action, then the header and main
//! part of its front page in each header style, a post and an author page.

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

const SETTINGS: &str = "\n[template]\nname = \"source\"\n[template.cta]\nlabel = \"Get the free checklist\"\nhref = \"https://www.example.com/checklist/\"\nnote = \"Twenty checks for every new post.\"\n";

const FEATURED: [&str; 3] = [
    "content/en/posts/2026-09-14-why-internal-links-matter.md",
    "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
    "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
];

/// The demo site built with Source, a logo from the main site, three posts
/// marked featured, a call to action and `options` under
/// `[template.options]`.
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
    toml.push_str(SETTINGS);
    if !options.is_empty() {
        toml.push_str("[template.options]\n");
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
    for rel in FEATURED {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', "\nfeatured: true\n", 1)).unwrap();
    }
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

/// The text of every element `css` selects in `page`.
#[cfg(test)]
fn texts(page: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(page);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .collect()
}

/// Every post title in `page` once at most.
#[cfg(test)]
fn assert_each_post_once(page: &str) {
    let mut titles = texts(
        page,
        "main .sc-lead-card h2 a, main .card h2 a, main .card h3 a",
    );
    let count = titles.len();
    titles.sort_unstable();
    titles.dedup();
    assert_eq!(titles.len(), count, "a post shows twice: {titles:?}");
}

#[test]
fn source_front_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The highlight header: the newest post large, the next three beside it,
/// then the fifth in the last column, as no editor's picks are asked for.
#[test]
fn source_highlight_front_page_lists_the_next_posts_beside_the_lead() {
    let (_temp, out) = built("header_style = \"highlight\"\n");
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".sc-lead-card h2").len(), 1);
    assert_eq!(texts(&page, ".sc-middle .card h2").len(), 3);
    assert_eq!(texts(&page, ".sc-side .card h2").len(), 1);
    assert_eq!(texts(&page, ".sc-hero"), Vec::<String>::new());
    assert_each_post_once(&page);
}

/// The magazine header shows every post of the small demo at the top, so
/// the featured posts are already there and no row of picks repeats them.
#[test]
fn source_magazine_front_page_shows_each_post_once() {
    let (_temp, out) = built("header_style = \"magazine\"\npicks = 3\n");
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".sc-lead-card h2").len(), 1);
    assert_eq!(texts(&page, ".sc-start .card h2").len(), 3);
    assert!(texts(&page, ".sc-picks").is_empty(), "{page}");
    assert_each_post_once(&page);
}

#[test]
fn source_post() {
    let (_temp, out) = built("drop_cap = true\n");
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn source_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}
