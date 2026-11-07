//! The Alto template end to end, based on Ghost's Alto theme: `khalid
//! build` of the demo site with `[template] name = "alto"`, a logo, three
//! featured posts and a call to action, then the header and main part of
//! its front page, a post and an author page, and the options that change
//! the featured strip and the share links.

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

const SETTINGS: &str = "\n[template]\nname = \"alto\"\n[template.cta]\nlabel = \"Get the free checklist\"\nhref = \"https://www.example.com/checklist/\"\nnote = \"Twenty checks for every new post.\"\n";

const FEATURED: [&str; 3] = [
    "content/en/posts/2026-09-14-why-internal-links-matter.md",
    "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
    "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
];

/// The demo site built with Alto, a logo from the main site, three posts
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

/// How many elements `css` selects in the whole page at `rel`.
#[cfg(test)]
fn count(out: &Path, rel: &str, css: &str) -> usize {
    let page = fs::read_to_string(at(out, rel)).unwrap();
    let html = scraper::Html::parse_document(&page);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector).count()
}

#[test]
fn alto_front_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// By default the front page opens with a strip of the three featured
/// posts, each with its topic as a pill and its date, then lists every
/// post one under another.
#[test]
fn alto_front_page_opens_with_the_featured_strip_then_the_feed() {
    let (_temp, out) = built("");
    let rel = "index.html";
    assert_eq!(count(&out, rel, ".al-featured .al-strip > li"), 3);
    assert_eq!(count(&out, rel, ".al-pick h3 a"), 3);
    assert_eq!(count(&out, rel, ".al-pick .al-date time"), 3);
    assert_eq!(count(&out, rel, "main > h1.visually-hidden"), 1);
    assert_eq!(count(&out, rel, "main > ul.cards > li"), 5);
}

/// With no picks, the front page has no featured strip.
#[test]
fn alto_picks_zero_leaves_the_featured_strip_out() {
    let (_temp, out) = built("picks = 0\n");
    let rel = "index.html";
    assert_eq!(count(&out, rel, ".al-featured"), 0);
    assert_eq!(count(&out, rel, "main > ul.cards > li"), 5);
}

/// A post shows its meta line over the title, the Share pill after the
/// words and the author box; with share at the top, the pill moves into
/// the header.
#[test]
fn alto_post_shares_at_the_end_or_at_the_top() {
    let rel = "fixing-slow-pages/index.html";
    let (_temp, out) = built("");
    assert_eq!(count(&out, rel, ".post-header > .al-meta + h1"), 1);
    assert_eq!(count(&out, rel, ".post > .post-body + .al-share"), 1);
    assert_eq!(count(&out, rel, ".post > .author-box"), 1);
    let (_temp, out) = built("share = \"top\"\n");
    assert_eq!(count(&out, rel, ".post-header > .al-share"), 1);
    assert_eq!(count(&out, rel, ".post > .al-share"), 0);
}

#[test]
fn alto_post() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

#[test]
fn alto_author_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "authors/asif/index.html"));
}
