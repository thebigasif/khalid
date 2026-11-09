//! The Manual template end to end (docs/templates/manual.md): `khalid
//! build` of the demo site with `[template] name = "manual"`, two posts
//! made the steps of a guide (a series, with facts) and one featured,
//! then the header and main part of its front page, an article, a topic
//! and a guide, and the next steps panel an article shows with no script.

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

const SETTINGS: &str = "\n[template]\nname = \"manual\"\n[template.options]\ntagline = [\"Answers on search, from the people who do it.\"]\n[template.cta]\nlabel = \"Contact support\"\nhref = \"https://www.example.com/support/\"\n";

/// Two posts that become the steps of a guide, each with facts for
/// "Before you start", and a third marked featured.
const ARTICLES: [(&str, &str); 3] = [
    (
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
        "series:\n  name: Search basics\n  order: 1\nfacts:\n  - label: Time\n    value: \"12 minutes\"\n  - label: Level\n    value: \"Easy\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "series:\n  name: Search basics\n  order: 2\nfacts:\n  - label: Time\n    value: \"9 minutes\"\n",
    ),
    (
        "content/en/posts/2026-09-28-what-a-sitemap-does.md",
        "featured: true\n",
    ),
];

/// The demo site built with Manual, with two posts made a guide and one featured.
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
    for (rel, extra) in ARTICLES {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', &format!("\n{extra}"), 1)).unwrap();
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
fn manual_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The front page leads with the tagline and a search field, numbers the
/// topics in the order of categories.toml with their newest articles,
/// and shows the featured post as a popular article.
#[test]
fn the_front_page_numbers_the_topics_and_offers_search() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".hero-intro").concat().trim(),
        "Answers on search, from the people who do it."
    );
    assert!(page.contains("<form class=\"help-search\" role=\"search\" action=\"/blog/search/\""));
    assert_eq!(texts(&page, ".topic-number"), ["01", "02", "03"]);
    assert_eq!(
        texts(&page, ".topic-tile h3"),
        ["On-page SEO", "Technical SEO", "Content strategy"]
    );
    assert_eq!(
        texts(&page, ".topic-tile:first-child .topic-links a"),
        ["How to write a title tag", "Why internal links matter"]
    );
    assert_eq!(
        texts(&page, ".help-popular h3"),
        ["What a sitemap does, and what it does not"]
    );
    assert_eq!(
        texts(&page, ".help-quick a"),
        ["What a sitemap does, and what it does not"]
    );
    assert_eq!(texts(&page, ".help-updates .feed-item").len(), 5);
}

#[test]
fn manual_article() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "why-internal-links-matter/index.html"));
}

/// An article in a guide says which step it is, lists the guide's steps
/// with this one marked, numbers its sections, and ends with the next
/// steps panel: the next step for yes, and search, the topic and the
/// call to action for not quite. All of it is links, with no script.
#[test]
fn an_article_shows_its_step_and_the_next_steps() {
    let (_temp, out) = built();
    let page = shown(&out, "why-internal-links-matter/index.html");
    assert_eq!(texts(&page, ".help-step"), ["Step 1 of 2"]);
    assert_eq!(texts(&page, ".help-before dd"), ["12 minutes", "Easy"]);
    assert_eq!(texts(&page, ".help-guide li").len(), 2);
    assert_eq!(
        texts(&page, ".help-guide [aria-current]"),
        ["Why internal links matter"]
    );
    assert!(page.contains("<div class=\"post-body help-numbered help-nested\">"));
    assert!(page.contains("<nav class=\"toc-side\" aria-label=\"On this page\">"));
    assert_eq!(
        texts(&page, ".next-steps h2"),
        ["Did this answer your question?"]
    );
    assert!(
        texts(&page, ".next-yes a")
            .concat()
            .contains("How to write a title tag")
    );
    assert_eq!(
        texts(&page, ".next-no a"),
        [
            "Search the help centre",
            "View all posts in On-page SEO",
            "Contact support",
        ]
    );
    assert!(!page.contains("<form"), "no form in the next steps");
    let alone = shown(&out, "fixing-slow-pages/index.html");
    assert!(
        texts(&alone, ".help-step").is_empty(),
        "expected nothing here"
    );
    assert_eq!(texts(&alone, ".next-steps").len(), 1);
}

#[test]
fn manual_topic_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/on-page-seo/index.html"));
}

/// A topic's page says it is a topic, counts its articles and lists them
/// with their summaries; a guide's page numbers its articles as steps.
#[test]
fn a_topic_lists_its_articles_and_a_guide_numbers_its_steps() {
    let (_temp, out) = built();
    let page = shown(&out, "category/on-page-seo/index.html");
    assert_eq!(texts(&page, ".list-kind"), ["Topic"]);
    assert_eq!(texts(&page, ".list-count"), ["2 articles"]);
    assert_eq!(
        texts(&page, ".article-row h2"),
        ["How to write a title tag", "Why internal links matter"]
    );
    let guide = shown(&out, "series/search-basics/index.html");
    assert_eq!(texts(&guide, ".list-kind"), ["Guide"]);
    assert_eq!(texts(&guide, ".article-step"), ["Step 1", "Step 2"]);
}
