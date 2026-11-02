//! Schema aggregation: the whole site's JSON-LD in one graph per language,
//! at `schema.json`, linked from each home page (Yoast comparison, 2 November
//! 2026).

#![cfg(test)]

mod common;

use common::{demo_site, internal_links_post, internal_links_post_de, locale, path};
use khalid_seo::{
    PageKind, PublishState, RobotsTxt, SchemaMap, SeoPage, SeoSite, UrlPath, head_tags,
    schema_map_url, site_schema,
};

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

fn home_head(site: &SeoSite, language: &str) -> String {
    let home = SeoPage::new(PageKind::Home, locale(language), UrlPath::root(), "");
    head_tags(site, &home).unwrap().to_html()
}

#[test]
fn each_language_home_page_links_its_file_and_robots_txt_stays_valid() {
    let site = demo_site();
    assert_eq!(
        schema_map_url(&site, &locale("en")).unwrap().as_str(),
        "https://www.example.com/blog/schema.json"
    );
    assert_eq!(
        schema_map_url(&site, &locale("de")).unwrap().as_str(),
        "https://www.example.com/blog/de/schema.json"
    );
    assert!(
        home_head(&site, "en").contains(
            "<link rel=\"alternate\" type=\"application/ld+json\" \
             href=\"https://www.example.com/blog/schema.json\">\n"
        ),
        "{}",
        home_head(&site, "en")
    );
    assert!(
        home_head(&site, "de").contains("href=\"https://www.example.com/blog/de/schema.json\""),
        "{}",
        home_head(&site, "de")
    );
    let post = head_tags(&site, &internal_links_post()).unwrap().to_html();
    assert!(!post.contains("schema.json"), "{post}");
    // Lighthouse refuses a directive it does not know, so the Yoast line
    // waits for the owner to ask for it.
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    assert!(!robots.contains("Schemamap"), "{robots}");
}

#[test]
fn the_robots_txt_switch_adds_the_yoast_line_and_off_drops_everything() {
    let mut yoast = demo_site();
    yoast.schema_map = SchemaMap::FilesAndRobotsTxt;
    let robots = RobotsTxt::for_site(&yoast).unwrap().to_string();
    assert!(
        robots.ends_with(
            "Schemamap: https://www.example.com/blog/schema.json\n\
             Schemamap: https://www.example.com/blog/de/schema.json\n"
        ),
        "{robots}"
    );
    let mut off = demo_site();
    off.schema_map = SchemaMap::Off;
    let robots = RobotsTxt::for_site(&off).unwrap().to_string();
    assert!(!robots.contains("Schemamap"), "{robots}");
    assert!(!home_head(&off, "en").contains("schema.json"));
}
