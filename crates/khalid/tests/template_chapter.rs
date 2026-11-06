//! The Chapter template end to end (docs/templates/chapter.md): `khalid
//! build` of the demo site with `[template] name = "chapter"`, its
//! Technical SEO category named as the books, On-page SEO as the events
//! and Content strategy as the excerpts, one book and one event given
//! facts, then the header and main part of its front page, a book, an
//! excerpt, a list and the people page.

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

const SETTINGS: &str = "\n[template]\nname = \"chapter\"\n[template.options]\ntagline = [\"Novels about the web.\"]\nbooks_category = \"technical-seo\"\nevents_category = \"on-page-seo\"\nexcerpts_category = \"content-strategy\"\n";

/// The facts of one book and one event, in the order Chapter reads them.
const FACTS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Publisher\n    value: \"Fernhill Press\"\n  - label: Published\n    value: \"2026\"\n  - label: Length\n    value: \"336 pages\"\n  - label: Order a copy\n    value: \"https://www.example.com/buy/\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: When\n    value: \"Thu 19 November 2026, 7 pm\"\n  - label: Venue\n    value: \"Castle Street Books\"\n  - label: Where\n    value: \"Hereford\"\n  - label: Tickets\n    value: \"https://www.example.com/tickets/\"\n",
    ),
];

/// The demo site built with Chapter, with one book and one event given
/// their facts.
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
    for (rel, facts) in FACTS {
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
fn chapter_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Books form the shelf and lead as the new book, events the dated rows,
/// and every other post the contents page, so no post shows twice. The
/// parts are numbered I, II and III.
#[test]
fn chapter_front_page_splits_books_events_and_writing() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let books = texts(&page, ".cover-shelf h3");
    assert_eq!(
        books,
        [
            "Fixing slow pages, starting with the biggest win",
            "What a sitemap does, and what it does not",
        ]
    );
    assert_eq!(texts(&page, ".ch-new h2"), [books[0].clone()]);
    let events = texts(&page, ".event-what h3");
    assert!(
        events.contains(&"How to write a title tag".to_owned()),
        "{events:?}"
    );
    assert!(page.contains("Thu 19 November 2026, 7 pm"));
    let writing = texts(&page, ".contents-line h3");
    assert!(
        writing.contains(&"Planning a topic cluster in an afternoon".to_owned()),
        "{writing:?}"
    );
    assert!(
        writing
            .iter()
            .all(|title| !books.contains(title) && !events.contains(title)),
        "{writing:?}"
    );
    assert_eq!(texts(&page, ".ch-numeral"), ["I", "II", "III"]);
    assert!(page.contains("Novels about the web."));
}

#[test]
fn chapter_book() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A book's last fact, a link, is its order button and not a row of the
/// facts list; an event's tickets link is a button too.
#[test]
fn a_book_and_an_event_show_their_link_as_a_button() {
    let (_temp, out) = built();
    let book = shown(&out, "fixing-slow-pages/index.html");
    assert!(book.contains("data-kind=\"book\""));
    assert_eq!(texts(&book, ".ch-actions a"), ["Order a copy"]);
    assert_eq!(
        texts(&book, ".ch-facts dt"),
        ["Publisher", "Published", "Length"]
    );
    let event = shown(&out, "how-to-write-a-title-tag/index.html");
    assert!(event.contains("data-kind=\"event\""));
    assert_eq!(texts(&event, ".event-panel dt"), ["When", "Venue", "Where"]);
    assert_eq!(texts(&event, ".ch-actions a"), ["Tickets"]);
}

/// A post in the excerpts category is set as a page of the book: it says
/// so on its article and opens with the ornament.
#[test]
fn an_excerpt_opens_like_a_chapter() {
    let (_temp, out) = built();
    let page = shown(&out, "planning-a-topic-cluster/index.html");
    assert!(page.contains("data-kind=\"excerpt\""));
    assert_eq!(texts(&page, ".ch-ornament").len(), 1);
}

#[test]
fn chapter_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn chapter_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
