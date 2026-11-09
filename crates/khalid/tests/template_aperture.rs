//! The Aperture template end to end (docs/templates/aperture.md): `khalid
//! build` of the demo site with `[template] name = "aperture"`, two
//! stories on the front page and two posts given camera details as facts
//! (camera, lens, exposure, date taken), then the header and main part of
//! its front page, a post, a list and the people page, and the exposure
//! plate read from the facts by position.

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

const SETTINGS: &str = "\n[template]\nname = \"aperture\"\n[template.options]\nstories = 2\ntagline = [\"Pictures of the web, one at a time.\"]\n[template.cta]\nlabel = \"See every picture\"\nhref = \"https://www.example.com/archive/\"\n";

/// Two posts given camera details, in the order Aperture reads them.
const CAMERAS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Camera\n    value: \"Canon EOS 5D Mark IV\"\n  - label: Lens\n    value: \"EF 24-70mm f/4L at 24mm\"\n  - label: Exposure\n    value: \"f/8 · 1/320 s · ISO 100\"\n  - label: Taken\n    value: \"4 August 2026\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Film\n    value: \"Kodak Portra 400\"\n  - label: Lens\n    value: \"50mm\"\n",
    ),
];

/// The demo site built with Aperture, with two posts given camera details.
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
    for (rel, facts) in CAMERAS {
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
fn aperture_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest posts, as many as `stories` says, are full-width stories;
/// the other posts on the page go into the contact sheet, so no post
/// shows twice.
#[test]
fn the_front_page_splits_stories_from_the_contact_sheet() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".story-title"),
        [
            "Planning a topic cluster in an afternoon",
            "Fixing slow pages, starting with the biggest win",
        ]
    );
    assert_eq!(
        texts(&page, ".contact-sheet .sheet-title"),
        [
            "What a sitemap does, and what it does not",
            "How to write a title tag",
            "Why internal links matter",
        ]
    );
    assert!(page.contains("Pictures of the web, one at a time."));
}

/// The exposure plate shows a post's facts in their order, with the
/// writer's own labels, under its picture on the front page and on the
/// post. A post without facts has no plate.
#[test]
fn the_exposure_plate_reads_the_facts_by_position() {
    let (_temp, out) = built();
    let home = shown(&out, "index.html");
    assert_eq!(
        texts(&home, ".story .plate dt"),
        ["Camera", "Lens", "Exposure", "Taken"]
    );
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&post, ".print .plate dd"),
        [
            "Canon EOS 5D Mark IV",
            "EF 24-70mm f/4L at 24mm",
            "f/8 · 1/320 s · ISO 100",
            "4 August 2026",
        ]
    );
    let film = shown(&out, "how-to-write-a-title-tag/index.html");
    assert_eq!(texts(&film, ".plate dt"), ["Film", "Lens"]);
    let plain = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&plain, ".plate").is_empty(),
        "a post without facts has no plate"
    );
}

#[test]
fn aperture_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn aperture_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn aperture_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
