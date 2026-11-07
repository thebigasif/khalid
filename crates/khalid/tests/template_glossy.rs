//! The Glossy template end to end (docs/templates/glossy.md): `khalid
//! build` of the demo site with `[template] name = "glossy"`, then the
//! header and main part of its front page, a post, a list and the people
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

const SETTINGS: &str = "\n[template]\nname = \"glossy\"\n[template.options]\ntagline = [\"Home, food and travel for slow weekends.\"]\ncover_lines = 2\n[template.cta]\nlabel = \"Get the Friday letter\"\nhref = \"https://www.example.com/letter/\"\n";

/// The demo site built with Glossy.
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(SETTINGS);
    fs::write(&settings, toml).unwrap();
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

/// The texts of every element `css` selects in `html`.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>())
        .collect()
}

#[test]
fn glossy_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The only post with a picture is the cover story. The next stories, as
/// many as `cover_lines` asks for, are its cover lines, and the grid starts
/// after them, so no story shows twice; a story without a picture is a
/// plain card.
#[test]
fn the_cover_leads_with_its_cover_lines_and_the_cards_follow() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".gl-cover-title"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(
        texts(&page, ".gl-cover-line"),
        [
            "Planning a topic cluster in an afternoon",
            "What a sitemap does, and what it does not",
        ]
    );
    assert_eq!(
        texts(&page, ".gl-grid > .gl-card h3"),
        ["How to write a title tag", "Why internal links matter"]
    );
    assert_eq!(texts(&page, ".gl-grid > .gl-card-plain").len(), 2);
    assert!(texts(&page, ".gl-browse-list a").contains(&"Technical SEO".to_owned()));
    assert!(page.contains("Home, food and travel for slow weekends."));
}

/// No text is written over a picture: on the front page and on a post,
/// every title, label and caption is a sibling of the picture, not
/// inside it.
#[test]
fn text_sits_beside_or_under_pictures() {
    let (_temp, out) = built();
    for rel in ["index.html", "fixing-slow-pages/index.html"] {
        let page = shown(&out, rel);
        for text in [
            ".gl-cover-title",
            ".gl-section",
            ".gl-pill",
            "h1",
            "h2",
            "h3",
        ] {
            let inside = texts(&page, &format!("figure {text}"));
            assert!(inside.is_empty(), "{rel}: {text} on a picture");
        }
    }
}

#[test]
fn glossy_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A post opens with its section as a label, its title as the one h1 and
/// its lead picture after the byline.
#[test]
fn a_post_opens_with_its_section_title_and_picture() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".gl-post-section a"), ["Technical SEO"]);
    assert_eq!(
        texts(&page, ".post-header h1"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(texts(&page, ".post-header .gl-lead").len(), 1);
}

/// A list shows its stories in the card grid, titles at the second level.
#[test]
fn a_list_shows_its_stories_as_cards() {
    let (_temp, out) = built();
    let page = shown(&out, "category/on-page-seo/index.html");
    assert_eq!(
        texts(&page, ".gl-card h2"),
        ["How to write a title tag", "Why internal links matter"]
    );
}

#[test]
fn glossy_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn glossy_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
