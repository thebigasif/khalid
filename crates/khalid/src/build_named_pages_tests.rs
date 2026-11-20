use std::fs;
use std::path::PathBuf;

use khalid_content::{ContentTree, read_content};
use khalid_core::Environment;
use khalid_lint::StructuredDataGap;
use khalid_seo::page_graph;
use serde_json::Value;

use super::*;

/// The demo site's settings in a fresh folder whose only content is the
/// English page `tools.md` written as `text`.
fn site_with_page(text: &str) -> (tempfile::TempDir, Config, SiteContent) {
    let dir = tempfile::tempdir().unwrap();
    let within = |parts: &[&str]| {
        let mut path = dir.path().to_path_buf();
        path.extend(parts);
        path
    };
    let demo: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "../../fixtures/demo-site/site.toml",
    ]
    .iter()
    .collect();
    fs::copy(demo, within(&["site.toml"])).unwrap();
    fs::create_dir_all(within(&["content", "en", "pages"])).unwrap();
    fs::write(within(&["content", "en", "pages", "tools.md"]), text).unwrap();
    let config = Config::load(dir.path(), &Environment::empty()).unwrap();
    let content = read_content(&ContentTree::walk(dir.path()).unwrap()).unwrap();
    (dir, config, content)
}

const TOOLS: &str = "---\ntitle: Tools\nslug: tools\n---\n\n\
    :::review{item=\"Quillmeter\" type=\"SoftwareApplication\" rating=\"4\" best=\"5\" average=\"4.2\" ratings=\"310\"}\n\
    Quick and plain.\n:::\n\n\
    :::product{name=\"Brief Kit\" price=\"29.00\" currency=\"GBP\" rating=\"4.5\" average=\"4.6\" ratings=\"52\"}\n\
    A brief template.\n:::\n";

/// The schema graph of the page `TOOLS`, as JSON, and the properties
/// Google needs that it lacks.
fn tools_graph() -> (Value, Vec<StructuredDataGap>) {
    let (_dir, config, content) = site_with_page(TOOLS);
    let locale = config.default_language().clone();
    let pages = plain_pages(&config, &Wikilinks::new(), &content, &locale).unwrap();
    let site = build_seo::site(&config).unwrap();
    let graph = page_graph(&site, &pages[0].seo).unwrap().to_json();
    let gaps = build_seo::structured_data_gaps(&site, &pages[0].seo);
    (serde_json::from_str(&graph).unwrap(), gaps)
}

/// Every value of `key` anywhere in `json`, in document order.
fn all<'a>(json: &'a Value, key: &str, found: &mut Vec<&'a Value>) {
    match json {
        Value::Object(map) => {
            for (name, value) in map {
                if name == key {
                    found.push(value);
                }
                all(value, key, found);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| all(item, key, found)),
        _ => {}
    }
}

#[test]
fn a_review_and_a_product_on_a_page_write_their_schema_with_the_readers_average() {
    let (graph, gaps) = tools_graph();
    let mut kinds = Vec::new();
    all(&graph, "@type", &mut kinds);
    for kind in ["Review", "Product", "AggregateRating", "Offer"] {
        assert!(kinds.contains(&&Value::from(kind)), "{kind}: {graph:#}");
    }
    let mut counts = Vec::new();
    all(&graph, "ratingCount", &mut counts);
    assert_eq!(counts, [&Value::from(310), &Value::from(52)]);
    assert_eq!(gaps, []);
}

#[test]
fn a_score_on_a_page_with_no_author_is_the_publishers() {
    // The review's and the product score's author both point at the
    // publisher's node, and no made-up Person stands in for them.
    let (graph, _) = tools_graph();
    let mut authors = Vec::new();
    all(&graph, "author", &mut authors);
    assert_eq!(authors.len(), 2, "{graph:#}");
    for author in authors {
        let id = author["@id"].as_str().unwrap_or_default();
        assert!(id.ends_with("#organization"), "{graph:#}");
    }
}

#[test]
fn a_pages_robots_canonical_cards_and_image_reach_its_head() {
    let text = "---\ntitle: Tools\nrobots: noindex\ncanonical: https://serpdad.com/tools/\n\
                image:\n  src: desk.jpg\n  alt: A desk\n\
                share_cards:\n  facebook:\n    title: Our tools\n---\n\nThe tools.\n";
    let (_dir, config, content) = site_with_page(text);
    let locale = config.default_language().clone();
    let pages = plain_pages(&config, &Wikilinks::new(), &content, &locale).unwrap();
    let page = &pages[0];
    let site = build_seo::site(&config).unwrap();
    let html = khalid_seo::head_tags(&site, &page.seo).unwrap().to_html();
    for want in [
        r#"<meta name="robots" content="noindex, follow">"#,
        r#"<link rel="canonical" href="https://serpdad.com/tools/">"#,
        r#"<meta property="og:title" content="Our tools">"#,
    ] {
        assert!(html.contains(want), "{want} missing from {html}");
    }
    let lead = page.lead.as_ref().map(|image| image.src.as_str());
    assert_eq!(lead, Some("desk.jpg"));
}

#[test]
fn a_plain_page_keeps_the_sites_defaults() {
    let (_dir, config, content) = site_with_page("---\ntitle: Tools\n---\n\nThe tools.\n");
    let locale = config.default_language().clone();
    let pages = plain_pages(&config, &Wikilinks::new(), &content, &locale).unwrap();
    let page = &pages[0];
    assert_eq!((&page.seo.robots, &page.seo.canonical), (&None, &None));
    assert_eq!(page.seo.cards, khalid_seo::CardOverrides::default());
}
