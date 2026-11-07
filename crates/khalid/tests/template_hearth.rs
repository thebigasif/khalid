//! The Hearth template end to end (docs/templates/hearth.md): `khalid
//! build` of the demo site with `[template] name = "hearth"` and one post
//! marked featured, then the header and main part of its front page, a
//! post, a list and the people page, the lead story taken out of the card
//! grid, and the author named on cards and posts.

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

const SETTINGS: &str = "\n[template]\nname = \"hearth\"\n[template.options]\ntagline = [\"Notes from a small team, every week.\"]\n[template.cta]\nlabel = \"Come and visit\"\nhref = \"https://www.example.com/visit/\"\n";

/// The post marked featured, so it leads the front page.
const FEATURED: &str = "content/en/posts/2026-09-28-what-a-sitemap-does.md";

/// The demo site built with Hearth, with one post marked featured.
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
    let featured = at(temp.path(), FEATURED);
    let text = fs::read_to_string(&featured).unwrap();
    fs::write(featured, text.replacen('\n', "\nfeatured: true\n", 1)).unwrap();
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
fn hearth_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest featured post leads the front page, and the card grid
/// holds every other post, newest first, so no post shows twice.
#[test]
fn the_featured_post_leads_and_stays_out_of_the_grid() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".hh-lead h2"),
        ["What a sitemap does, and what it does not"]
    );
    assert_eq!(
        texts(&page, ".hh-card h3"),
        [
            "Planning a topic cluster in an afternoon",
            "Fixing slow pages, starting with the biggest win",
            "How to write a title tag",
            "Why internal links matter",
        ]
    );
    assert!(page.contains("Notes from a small team, every week."));
}

/// The lead story and each card name their author with the date, and a
/// post names its author under its title.
#[test]
fn cards_and_posts_name_their_authors() {
    let (_temp, out) = built();
    let home = shown(&out, "index.html");
    assert_eq!(texts(&home, ".hh-lead .hh-byline-name"), ["Lena Vogt"]);
    assert_eq!(
        texts(&home, ".hh-card .hh-byline-name"),
        ["Asif", "Lena Vogt", "Asif", "Asif"]
    );
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&post, ".hh-authors a"), ["Lena Vogt"]);
}

#[test]
fn hearth_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn hearth_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn hearth_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
