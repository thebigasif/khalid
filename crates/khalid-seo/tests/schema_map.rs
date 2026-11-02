//! Schema aggregation: the whole site's JSON-LD in one graph per language,
//! at `schema.json`, named in robots.txt (Yoast comparison, 2 November 2026).

#![cfg(test)]

mod common;

use common::{demo_site, internal_links_post, internal_links_post_de, locale, path};
use khalid_seo::{PageKind, PublishState, RobotsTxt, SeoPage, schema_map_url, site_schema};

fn graph_json(language: &str, pages: &[SeoPage]) -> serde_json::Value {
    let graph = site_schema(&demo_site(), pages, &locale(language)).unwrap();
    serde_json::from_str(&graph.to_json()).unwrap()
}

fn ids(json: &serde_json::Value) -> Vec<String> {
    json["@graph"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|node| node["@id"].as_str().map(str::to_owned))
        .collect()
}

#[test]
fn one_graph_holds_every_listed_page_of_a_language_once() {
    let mut draft = SeoPage::new(PageKind::Post, locale("en"), path("/draft/"), "A draft");
    draft.state = PublishState::Draft;
    let about = SeoPage::new(PageKind::Page, locale("en"), path("/about/"), "About");
    let pages = [
        internal_links_post(),
        internal_links_post_de(),
        draft,
        about,
    ];
    let json = graph_json("en", &pages);
    let ids = ids(&json);
    let mut unique = ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(ids.len(), unique.len(), "each node once: {ids:?}");
    let has = |needle: &str| ids.iter().any(|id| id.contains(needle));
    assert!(has("/blog/why-internal-links-matter/"), "{ids:?}");
    assert!(has("/blog/about/"), "{ids:?}");
    assert!(!has("/draft/"), "{ids:?}");
    assert!(!has("/blog/de/"), "{ids:?}");
    let german = self::ids(&graph_json("de", &pages));
    assert!(
        german.iter().any(|id| id.contains("/blog/de/warum")),
        "{german:?}"
    );
}

#[test]
fn each_language_has_its_file_and_robots_txt_names_them() {
    let site = demo_site();
    assert_eq!(
        schema_map_url(&site, &locale("en")).unwrap().as_str(),
        "https://www.example.com/blog/schema.json"
    );
    assert_eq!(
        schema_map_url(&site, &locale("de")).unwrap().as_str(),
        "https://www.example.com/blog/de/schema.json"
    );
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    assert!(
        robots.ends_with(
            "Schemamap: https://www.example.com/blog/schema.json\n\
             Schemamap: https://www.example.com/blog/de/schema.json\n"
        ),
        "{robots}"
    );
    let mut off = demo_site();
    off.schema_map = false;
    let robots = RobotsTxt::for_site(&off).unwrap().to_string();
    assert!(!robots.contains("Schemamap"), "{robots}");
}
