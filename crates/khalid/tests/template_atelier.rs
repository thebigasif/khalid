//! The Atelier template end to end (docs/templates/atelier.md): `khalid
//! build` of the demo site with `[template] name = "atelier"` and two
//! posts given the pieces of an outfit as facts, then the header and main
//! part of its front page, a look, a list and the people page.

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

const SETTINGS: &str = "\n[template]\nname = \"atelier\"\n[template.options]\ntagline = [\"Coats and knitwear, one look at a time.\"]\njournal_count = 2\n[template.cta]\nlabel = \"Book a fitting\"\nhref = \"https://www.example.com/studio/\"\n";

/// Two posts that become looks: the pieces of each outfit, in order. Only
/// the first has a lead picture.
const LOOKS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Coat\n    value: \"Waxed cotton field coat, slate\"\n  - label: Knit\n    value: \"Lambswool crew, cream\"\n  - label: Hat\n    value: \"Wool felt fedora, sand\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Jacket\n    value: \"Soft-shoulder wool jacket, black\"\n",
    ),
];

/// The demo site built with Atelier, with two posts made looks.
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
    for (rel, facts) in LOOKS {
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
fn atelier_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The look with a picture opens the lookbook as a spread, numbered Look
/// 01, with its pieces as a ledger. Every other post, here the look
/// without a picture too, goes to the journal, as many as
/// `journal_count` asks for.
#[test]
fn the_front_page_opens_with_a_look_and_its_pieces() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".at-lookbook .spread-title"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(texts(&page, ".at-lookbook .spread-kicker"), ["Look 01"]);
    assert_eq!(
        texts(&page, ".at-lookbook .spread-ledger dt"),
        ["Coat", "Knit", "Hat"]
    );
    assert_eq!(texts(&page, ".at-journal .feed-item").len(), 2);
    assert!(texts(&page, ".at-collections a").contains(&"Technical SEO".to_owned()));
    assert!(page.contains("Coats and knitwear, one look at a time."));
}

/// No caption or number is written over a picture: in the spread and in
/// the grid, every caption is a sibling of the picture, not inside it.
#[test]
fn captions_sit_beside_or_under_pictures() {
    let (_temp, out) = built();
    for rel in ["index.html", "fixing-slow-pages/index.html"] {
        let page = shown(&out, rel);
        for caption in [
            ".spread-text",
            ".spread-kicker",
            ".picture-grid-number",
            "h1",
            "h3",
        ] {
            let inside = texts(&page, &format!("figure {caption}"));
            assert!(inside.is_empty(), "{rel}: {caption} on a picture");
        }
    }
}

#[test]
fn atelier_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A look's post opens as a spread with its collection, its title as the
/// one h1 and its pieces. A note, a post without facts, has no ledger.
#[test]
fn a_look_opens_as_a_spread_with_its_pieces() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&page, ".post-header h1.spread-title"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(
        texts(&page, ".post-header .spread-kicker"),
        ["Technical SEO"]
    );
    assert_eq!(
        texts(&page, ".post-header .spread-ledger dd"),
        [
            "Waxed cotton field coat, slate",
            "Lambswool crew, cream",
            "Wool felt fedora, sand",
        ]
    );
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(texts(&note, ".spread-ledger").is_empty());
}

/// A list numbers its looks and lists its other posts as a dated feed.
#[test]
fn a_list_numbers_its_looks_and_lists_its_notes() {
    let (_temp, out) = built();
    let page = shown(&out, "category/on-page-seo/index.html");
    assert_eq!(texts(&page, ".picture-grid-number"), ["Look 01"]);
    assert_eq!(
        texts(&page, ".picture-grid-item h2"),
        ["How to write a title tag"]
    );
}

#[test]
fn atelier_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn atelier_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
