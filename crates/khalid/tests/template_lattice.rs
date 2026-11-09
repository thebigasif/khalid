//! The Lattice template end to end (docs/templates/lattice.md): `khalid
//! build` of the demo site with `[template] name = "lattice"` and one post
//! made the lead explainer, with key numbers (facts), a figure, a footnote
//! and a glossary link. Then the header and main part of its front page,
//! that post, a list and the glossary.

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

const SETTINGS: &str = "\n[template]\nname = \"lattice\"\n[template.options]\ntagline = [\"Science explained plainly.\"]\n";

/// The post that becomes the lead explainer.
const LEAD: &str = "content/en/posts/2026-10-05-fixing-slow-pages/index.md";

/// Its front matter additions: featured, so it leads, and two key numbers.
const FRONT: &str = "featured: true\nfacts:\n  - label: Median page weight\n    value: \"2.3 MB\"\n  - label: Share from images\n    value: \"about half\"\n";

/// Its body additions: a figure, a footnote and a glossary link.
const BODY: &str = "\n:::figure{src=page-weight.png alt=\"A bar chart of page weight by part\"}\nImages weigh the most.\n:::\n\nSee [[glossary:topical-authority]] for the bigger picture.[^1]\n\n[^1]: A note with the source.\n";

/// The demo site built with Lattice.
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
    let path = at(temp.path(), LEAD);
    let mut text = fs::read_to_string(&path)
        .unwrap()
        .replacen('\n', &format!("\n{FRONT}"), 1);
    text.push_str(BODY);
    fs::write(path, text).unwrap();
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
fn lattice_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The featured post leads with its key numbers, and the grid of plates
/// below holds every other post, so the lead never shows twice.
#[test]
fn the_featured_post_leads_with_its_key_numbers() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let lead = texts(&page, ".lt-lead h3");
    assert_eq!(lead, ["Fixing slow pages, starting with the biggest win"]);
    assert_eq!(
        texts(&page, ".lt-lead .figure dd"),
        ["2.3 MB", "about half"]
    );
    let plates = texts(&page, ".lt-section .card h3");
    assert!(!plates.is_empty(), "expected something here");
    assert!(!plates.contains(&lead[0]), "{plates:?}");
    assert!(page.contains("Science explained plainly."));
}

#[test]
fn lattice_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A post shows its facts as key numbers, keeps its figure, footnote and
/// glossary link for the stylesheet to number and mark, and a post
/// without facts has no key numbers.
#[test]
fn a_post_shows_key_numbers_figures_notes_and_terms() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&page, ".lt-key-numbers dt"),
        ["Median page weight", "Share from images"]
    );
    let captions = texts(&page, ".post-body > figure figcaption");
    assert!(
        captions
            .iter()
            .any(|caption| caption.contains("Images weigh the most.")),
        "{captions:?}"
    );
    assert_eq!(texts(&page, ".post-body .footnote-ref").len(), 1);
    assert_eq!(texts(&page, ".post-body .footnotes li").len(), 1);
    assert!(
        page.contains("/glossary/topical-authority/"),
        "glossary link"
    );
    let other = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&other, ".lt-key-numbers").is_empty(),
        "expected nothing here"
    );
}

#[test]
fn lattice_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn lattice_glossary_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "glossary/index.html"));
}
