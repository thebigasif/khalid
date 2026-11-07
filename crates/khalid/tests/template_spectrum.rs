//! The Spectrum template end to end (docs/templates/spectrum.md): `khalid
//! build` of the demo site with `[template] name = "spectrum"` and a main
//! menu that names two of its three topics, then the header and main part
//! of its front page, a post, a topic page and the writers page, and the
//! colour each topic takes from its place in the menu.

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

/// Spectrum with a tagline, three latest posts, and a menu that puts
/// Technical SEO first and On-page SEO second. Content strategy is not in
/// the menu, so it takes its colour from its name.
const SETTINGS: &str = "\n[template]\nname = \"spectrum\"\n[template.options]\ntagline = [\"Ideas, one topic at a time.\"]\nlatest_count = 3\n\n[[nav.primary]]\nlabel = \"Technical SEO\"\nhref = \"/blog/category/technical-seo/\"\nlanguage = \"en\"\n\n[[nav.primary]]\nlabel = \"On-page SEO\"\nhref = \"/blog/category/on-page-seo/\"\nlanguage = \"en\"\n";

/// The demo site built with Spectrum.
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

/// The topic colour, `data-hue`, of every element `css` selects in `html`.
#[cfg(test)]
fn hues(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .filter_map(|element| element.value().attr("data-hue"))
        .map(str::to_owned)
        .collect()
}

#[test]
fn spectrum_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest post leads, the latest posts follow it, and each topic gets
/// a segment of the spectrum bar and a row, in the order of its newest
/// post. A row leaves out the lead story unless it has no other post.
#[test]
fn spectrum_front_page_leads_and_gives_each_topic_a_row() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".sp-lead-story h3"),
        ["Planning a topic cluster in an afternoon"]
    );
    assert_eq!(
        texts(&page, ".sp-latest .feed-item h3"),
        [
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not",
            "How to write a title tag"
        ]
    );
    let topics = ["Content strategy", "Technical SEO", "On-page SEO"];
    assert_eq!(texts(&page, ".sp-bar a"), topics);
    assert_eq!(texts(&page, ".sp-row h2"), topics);
    assert_eq!(
        texts(&page, "#sp-row-1 .sp-story h3"),
        ["Planning a topic cluster in an afternoon"]
    );
    assert_eq!(
        texts(&page, "#sp-row-2 .sp-story h3"),
        [
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not"
        ]
    );
    assert_eq!(
        texts(&page, "#sp-row-2 .sp-all a"),
        ["All in Technical SEO"]
    );
    assert!(page.contains("Ideas, one topic at a time."));
}

/// A topic in the menu takes the colour of its place there: Technical SEO
/// is first and On-page SEO second. Content strategy, not in the menu,
/// takes one from the length of its name, 16 letters, so the fourth.
#[test]
fn each_topic_takes_its_colour_from_its_place_in_the_menu() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(hues(&page, ".sp-bar li"), ["4", "1", "2"]);
    assert_eq!(hues(&page, ".sp-row"), ["4", "1", "2"]);
    assert_eq!(hues(&page, ".sp-lead-story"), ["4"]);
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(hues(&post, ".post-header"), ["1"]);
    assert_eq!(texts(&post, ".sp-band .sp-topic a"), ["Technical SEO"]);
    let topic = shown(&out, "category/on-page-seo/index.html");
    assert_eq!(hues(&topic, ".list-header"), ["2"]);
}

#[test]
fn spectrum_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn spectrum_topic_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn spectrum_writers_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
