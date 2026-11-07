//! The Mosaic template end to end (docs/templates/mosaic.md): `khalid
//! build` of the demo site with `[template] name = "mosaic"` and two posts
//! given notes as facts, then the header and main part of its front page,
//! a post, a list and the people page, every post on the front page as a
//! tile, and the facts beside a post's title.

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

const SETTINGS: &str = "\n[template]\nname = \"mosaic\"\n[template.options]\ntagline = [\"Pictures of the web, a few at a time.\"]\n[template.cta]\nlabel = \"See every picture\"\nhref = \"https://www.example.com/archive/\"\n";

/// Two posts given notes about their pictures, with the writer's own labels.
const NOTES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Park\n    value: \"Grand Canyon\"\n  - label: Where\n    value: \"South Rim\"\n  - label: Light\n    value: \"Cloud, clearing\"\n  - label: Taken\n    value: \"December\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Film\n    value: \"Kodak Portra 400\"\n  - label: Lens\n    value: \"50mm\"\n",
    ),
];

/// The demo site built with Mosaic, with two posts given notes as facts.
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
    for (rel, facts) in NOTES {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', &format!("\n{facts}"), 1)).unwrap();
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
fn mosaic_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Every post on the front page is one tile of the mosaic, newest first,
/// each with its title under its picture, and the tagline sits beside the
/// site's name.
#[test]
fn every_post_on_the_front_page_is_a_tile() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".mosaic .tile-title"),
        [
            "Planning a topic cluster in an afternoon",
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not",
            "How to write a title tag",
            "Why internal links matter",
        ]
    );
    assert!(page.contains("Pictures of the web, a few at a time."));
}

/// A post shows its facts beside its title, in their order and with the
/// writer's own labels. A post without facts shows none.
#[test]
fn a_post_shows_its_facts_beside_the_title() {
    let (_temp, out) = built();
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&post, ".post-header .photo-facts dt"),
        ["Park", "Where", "Light", "Taken"]
    );
    assert_eq!(
        texts(&post, ".photo-facts dd"),
        ["Grand Canyon", "South Rim", "Cloud, clearing", "December"]
    );
    let film = shown(&out, "how-to-write-a-title-tag/index.html");
    assert_eq!(texts(&film, ".photo-facts dt"), ["Film", "Lens"]);
    let plain = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&plain, ".photo-facts").is_empty(),
        "a post without facts shows none"
    );
}

#[test]
fn mosaic_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn mosaic_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn mosaic_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
