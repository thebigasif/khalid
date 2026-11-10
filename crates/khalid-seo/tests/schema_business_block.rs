//! Business cards and location blocks write their own `LocalBusiness`, and
//! a thing a block describes is never written twice (brief G15, KH-SEO-3).

#![cfg(test)]

mod common;

use common::{demo_site, internal_links_post, internal_links_post_de, url};
use khalid_core::PageSchemaType;
use khalid_seo::{
    Course, CustomSchema, FaqItem, IssueLevel, LocalBusiness, MetaTemplate, PostalAddress, Recipe,
    SchemaBlock, SchemaGraph, SchemaNode, SchemaTemplate, SchemaType, TemplateScope, check_graph,
    page_graph,
};

fn studio() -> LocalBusiness {
    let mut business = LocalBusiness::new(
        "ProfessionalService",
        "Larkspur Studio",
        PostalAddress::new("4 Tannery Yard", "Bristol", "BS1 6QA", "GB"),
    );
    business.telephone = Some(String::from("+44 117 496 0000"));
    business.email = Some(String::from("hello@example.com"));
    business.url = Some(url("https://www.example.com/"));
    business
}

fn kinds(graph: &SchemaGraph) -> Vec<&str> {
    graph.nodes().iter().map(SchemaNode::kind).collect()
}

fn blocking(graph: &SchemaGraph) -> Vec<String> {
    check_graph(graph)
        .into_iter()
        .filter(|issue| issue.level() == IssueLevel::Blocks)
        .map(|issue| format!("{} {}", issue.kind(), issue.property()))
        .collect()
}

#[test]
fn a_business_card_writes_its_local_business() {
    let mut post = internal_links_post();
    post.blocks = vec![SchemaBlock::Business(studio())];
    let graph = page_graph(&demo_site(), &post).unwrap();
    assert_eq!(blocking(&graph), Vec::<String>::new());
    let node = graph.nodes().last().unwrap().clone();
    insta::assert_snapshot!(SchemaGraph::new(vec![node]).to_json());
}

#[test]
fn a_business_card_without_a_website_points_at_the_page() {
    let mut business = studio();
    business.url = None;
    for mut post in [internal_links_post(), internal_links_post_de()] {
        post.blocks = vec![SchemaBlock::Business(business.clone())];
        let graph = page_graph(&demo_site(), &post).unwrap();
        let page = graph.nodes().last().unwrap().id().unwrap().as_str();
        let page = page.trim_end_matches("#business-1");
        let json = graph.to_json();
        assert!(json.contains(&format!("\"url\": \"{page}\"")), "{json}");
        assert!(page.ends_with('/'), "{page}");
    }
}

#[test]
fn a_location_block_writes_the_sites_location_under_its_own_id() {
    let mut site = demo_site();
    site.locations = vec![studio()];
    let mut post = internal_links_post();
    post.blocks = vec![
        SchemaBlock::Location(String::from("Larkspur Studio")),
        SchemaBlock::Location(String::from("Nowhere")),
    ];
    let graph = page_graph(&site, &post).unwrap();
    let node = graph.nodes().last().unwrap();
    assert_eq!(node.kind(), "ProfessionalService");
    assert_eq!(
        node.id().unwrap().as_str(),
        "https://www.example.com/blog/#location-1",
        "the same id as on the home page, so engines see one place"
    );
    let places = kinds(&graph);
    let places = places.iter().filter(|k| **k == "ProfessionalService");
    assert_eq!(
        places.count(),
        1,
        "a name the site does not list writes nothing"
    );
    assert_eq!(blocking(&graph), Vec::<String>::new());
}

#[test]
fn a_block_wins_over_the_schema_field_and_a_template_of_its_type() {
    let mut site = demo_site();
    let mut template = SchemaTemplate::new(
        TemplateScope::Category(String::from("On-page SEO")),
        SchemaType::try_new("Course").unwrap(),
    );
    template
        .set("name", MetaTemplate::parse("{title}").unwrap())
        .unwrap();
    site.schema_templates = vec![template];
    let mut post = internal_links_post();
    post.custom_schema = CustomSchema::parse(
        r#"{"@context": "https://schema.org", "@graph": [
            {"@type": "Recipe", "name": "Oat bars by hand"},
            {"@type": "Book", "name": "Links", "author": "Asif"}
        ]}"#,
    )
    .unwrap();
    let recipe = Recipe::new("Oat bars", vec![String::from("oats")], vec![]);
    post.blocks = vec![
        SchemaBlock::Recipe(recipe),
        SchemaBlock::Course(Course::new("Links", "Internal links, in six lessons.")),
    ];
    let graph = page_graph(&site, &post).unwrap();
    let found = kinds(&graph);
    for kind in ["Recipe", "Course", "Book"] {
        let count = found.iter().filter(|k| **k == kind).count();
        assert_eq!(count, 1, "{kind} in {found:?}");
    }
    let json = graph.to_json();
    assert!(
        !json.contains("Oat bars by hand"),
        "the block's recipe: {json}"
    );
    assert!(
        !json.contains("#template-course"),
        "the block's course: {json}"
    );
}

#[test]
fn an_faq_page_with_an_faq_block_writes_one_faq_page() {
    let mut post = internal_links_post();
    post.page_type = Some(PageSchemaType::FaqPage);
    post.blocks = vec![SchemaBlock::Faq(vec![FaqItem::new(
        "What is an internal link?",
        "A link to another page on the same site.",
    )])];
    let graph = page_graph(&demo_site(), &post).unwrap();
    let found = kinds(&graph);
    assert_eq!(
        found.iter().filter(|k| **k == "FAQPage").count(),
        1,
        "{found:?}"
    );
    assert!(found.contains(&"WebPage"), "{found:?}");

    post.blocks = Vec::new();
    let graph = page_graph(&demo_site(), &post).unwrap();
    assert!(
        kinds(&graph).contains(&"FAQPage"),
        "the field alone still counts"
    );
}
