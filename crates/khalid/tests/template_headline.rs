//! The Headline template end to end, based on Ghost's Headline theme:
//! `khalid build` of the demo site with `[template] name = "headline"`, a
//! logo and a call to action, then the header and main part of its front
//! page with its latest posts, topic sections and topic lists, a post and
//! an author page.

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

const SETTINGS: &str = "\n[template]\nname = \"headline\"\n[template.cta]\nlabel = \"Get the free checklist\"\nhref = \"https://www.example.com/checklist/\"\nnote = \"Twenty checks for every new post.\"\n";

/// The demo site built with Headline, a logo from the main site, a call to
/// action and `options` under `[template.options]`.
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

#[test]
fn headline_front_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Latest shows every post of the small demo, the first one large, then
/// one section per category, each led by its newest post.
#[test]
fn headline_front_page_leads_each_section_with_one_large_story() {
    let (_temp, out) = built("");
    let page = shown(&out, "index.html");
    let latest = texts(&page, "section[aria-labelledby=hl-latest] .hl-card h3");
    assert_eq!(latest.len(), 5, "{latest:?}");
    let large = texts(&page, ".hl-topic .hl-large h3");
    assert_eq!(large.len(), 4, "{large:?}");
    assert_eq!(texts(&page, ".hl-topic").len(), 4);
    assert_eq!(texts(&page, ".hl-lists"), Vec::<String>::new());
}

/// With one topic section, the other categories show as short lists of
/// headlines, and no category shows in both.
#[test]
fn headline_lists_the_topics_past_its_sections() {
    let (_temp, out) = built("topic_sections = 1\nlatest_count = 3\n");
    let page = shown(&out, "index.html");
    let latest = texts(&page, "section[aria-labelledby=hl-latest] .hl-card h3");
    assert_eq!(latest.len(), 3, "{latest:?}");
    let sections = texts(&page, ".hl-topic h2");
    assert_eq!(sections.len(), 2, "{sections:?}");
    let lists = texts(&page, ".hl-list h2");
    assert_eq!(lists.len(), 2, "{lists:?}");
    assert!(
        lists.iter().all(|name| !sections.contains(name)),
        "{lists:?}"
    );
}

#[test]
fn headline_post() {
    let (_temp, out) = built("post_image = \"wide\"\n");
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn headline_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}
