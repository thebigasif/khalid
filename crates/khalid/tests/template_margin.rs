//! The Margin template end to end (docs/templates/margin.md): `khalid
//! build` of the demo site with `[template] name = "margin"`, a call to
//! action, notes for the sign-up band, Lena as the writer and a footnote
//! in one post, then the header and main part of its front page and a
//! post, the essays as rows with their margin notes, and the footnotes
//! kept in the post's body, where the margin shows them.

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

/// Margin with Lena as the writer, a tagline, two notes for the sign-up
/// band and a call to action, which the band shows since the demo site
/// sends no mail.
const SETTINGS: &str = "\n[template]\nname = \"margin\"\n[template.options]\nwriter = \"lena-vogt\"\ntagline = [\"Essays on search, written slowly.\"]\nletter_notes = [\"Once a month\", \"Free, with no adverts\"]\n[template.cta]\nlabel = \"Get the letter\"\nhref = \"https://www.example.com/letter/\"\nnote = \"One letter a month with the new essay.\"\n";

/// The same site with no options: the newest essay's author shows in the
/// margin, and the band has no notes.
const PLAIN: &str = "\n[template]\nname = \"margin\"\n[template.cta]\nlabel = \"Get the letter\"\nhref = \"https://www.example.com/letter/\"\n";

/// A footnote added to the post with the lead picture.
const FOOTNOTE: &str = "\nThe biggest win is nearly always the largest picture.[^1]\n\n[^1]: Measured on a phone over a slow network.\n";

/// The demo site built with Margin and `settings`, with a footnote in
/// the post `fixing-slow-pages`.
#[cfg(test)]
fn built(settings: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let site = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&site).unwrap();
    toml.push_str(settings);
    fs::write(&site, toml).unwrap();
    let post = at(
        temp.path(),
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
    );
    let mut text = fs::read_to_string(&post).unwrap();
    text.push_str(FOOTNOTE);
    fs::write(&post, text).unwrap();
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

/// The texts of every element `css` selects in `html`, trimmed.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .collect()
}

#[test]
fn margin_front_page() {
    let (_temp, out) = built(SETTINGS);
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The introduction has the tagline and the writer the option names in
/// its margin; the sign-up band has the call to action and the notes.
#[test]
fn the_front_page_sets_the_writer_and_the_letter_notes_in_the_margin() {
    let (_temp, out) = built(SETTINGS);
    let page = shown(&out, "index.html");
    assert!(page.contains("Essays on search, written slowly."));
    assert_eq!(texts(&page, ".mg-writer-name a"), ["Lena Vogt"]);
    assert_eq!(
        texts(&page, ".mg-letter .mg-notes li"),
        ["Once a month", "Free, with no adverts"]
    );
    assert_eq!(texts(&page, ".mg-letter .button"), ["Get the letter"]);
    assert_eq!(texts(&page, ".mg-letter h2"), ["New essays by email"]);
}

/// Every essay is a row with its title in the text and its date in the
/// margin, newest first.
#[test]
fn every_essay_is_a_row_with_its_date_in_the_margin() {
    let (_temp, out) = built(SETTINGS);
    let page = shown(&out, "index.html");
    let titles = texts(&page, ".mg-rows .mg-text h3");
    assert_eq!(
        titles.first().map(String::as_str),
        Some("Planning a topic cluster in an afternoon")
    );
    assert_eq!(texts(&page, ".mg-rows .mg-note time").len(), titles.len());
    assert!(texts(&page, ".mg-rows .card").is_empty(), "no cards");
}

/// Without the option, the newest essay's author is the writer, and the
/// band shows no notes.
#[test]
fn without_options_the_newest_author_is_the_writer() {
    let (_temp, out) = built(PLAIN);
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".mg-writer-name a").len(), 1);
    assert!(texts(&page, ".mg-notes").is_empty(), "expected no notes");
}

#[test]
fn margin_post() {
    let (_temp, out) = built(SETTINGS);
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A post's header holds the title before the byline, and its footnotes
/// stay the last part of its body, where the margin takes them.
#[test]
fn a_post_keeps_its_footnotes_in_the_body_for_the_margin() {
    let (_temp, out) = built(SETTINGS);
    let post = shown(&out, "fixing-slow-pages/index.html");
    let heading = post.find("<div class=\"post-heading\">").unwrap();
    let meta = post.find("<div class=\"post-meta\">").unwrap();
    assert!(heading < meta, "the title comes before the byline");
    assert_eq!(
        texts(&post, ".post-body > .footnotes li p"),
        ["Measured on a phone over a slow network. ↩"]
    );
}
