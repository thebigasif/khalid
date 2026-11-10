//! "Copy schema from a URL" (KH-SEO-4, G54): another page's JSON-LD, read
//! through `Outbound` into a starting point for a custom JSON-LD field.

#![cfg(test)]

use khalid_core::AbsoluteUrl;
use khalid_outbound::{Limits, Outbound, OutboundError, Reach, Resolver};
use khalid_seo::{CopiedSchema, CopySchemaError, CustomSchema};
use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A page with `blocks` as its JSON-LD scripts.
fn page(blocks: &[&str]) -> String {
    let scripts = blocks
        .iter()
        .map(|block| {
            [
                "<script type=\"application/ld+json\">",
                block,
                "</script>\n",
            ]
            .concat()
        })
        .collect::<String>();
    format!(
        "<!doctype html><html><head><title>A page</title>\n{scripts}</head><body></body></html>"
    )
}

fn parsed(copied: &CopiedSchema) -> Value {
    serde_json::from_str(copied.text()).unwrap()
}

#[test]
fn one_node_is_copied_with_the_schema_org_context() {
    let html = page(&[r#"{"@context": "https://schema.org", "@type": "Recipe",
        "name": "Lentil soup", "recipeYield": "4 bowls"}"#]);
    let copied = CopiedSchema::from_html(&html).unwrap();
    assert_eq!(copied.types(), ["Recipe"]);
    assert_eq!(
        parsed(&copied),
        json!({"@context": "https://schema.org", "@type": "Recipe",
            "name": "Lentil soup", "recipeYield": "4 bowls"})
    );
    assert_eq!(copied.problem(), None);
    CustomSchema::parse(copied.text()).unwrap();
}

#[test]
fn the_other_sites_ids_and_references_are_left_out() {
    let html = page(&[r#"{"@context": "https://schema.org", "@graph": [
        {"@type": "WebSite", "@id": "https://other.example/#site", "name": "Other"},
        {"@type": ["Event", "Thing"], "@id": "https://other.example/gig/#event",
         "name": "Gig", "isPartOf": {"@id": "https://other.example/#site"},
         "location": {"@type": "Place", "@id": "https://other.example/#hall", "name": "Hall"},
         "sameAs": [{"@id": "https://other.example/a"}], "eventStatus": null}
    ]}"#]);
    let copied = CopiedSchema::from_html(&html).unwrap();
    assert_eq!(copied.types(), ["WebSite", "Event"]);
    assert_eq!(
        parsed(&copied),
        json!({"@context": "https://schema.org", "@graph": [
            {"@type": "WebSite", "name": "Other"},
            {"@type": "Event", "name": "Gig", "location": {"@type": "Place", "name": "Hall"}},
        ]})
    );
    assert_eq!(copied.problem(), None);
}

#[test]
fn every_block_on_the_page_joins_one_graph() {
    let html = page(&[
        r#"{"@context": "https://schema.org", "@type": "Organization", "name": "Shop"}"#,
        "this is not JSON",
        r#"[{"@context": "https://schema.org", "@type": "Product", "name": "Lamp"}]"#,
    ]);
    let copied = CopiedSchema::from_html(&html).unwrap();
    assert_eq!(copied.types(), ["Organization", "Product"]);
    let graph = parsed(&copied)["@graph"].as_array().unwrap().len();
    assert_eq!(graph, 2);
}

#[test]
fn a_script_tag_written_another_way_is_still_found() {
    let html =
        "<SCRIPT data-x='1' TYPE='application/ld+json'>\n{\"@type\": \"FAQPage\"}\n</script >";
    let copied = CopiedSchema::from_html(html).unwrap();
    assert_eq!(copied.types(), ["FAQPage"]);
}

#[test]
fn a_page_without_json_ld_gives_nothing() {
    assert!(CopiedSchema::from_html(&page(&[])).is_none());
    assert!(CopiedSchema::from_html(&page(&[r#"{"name": "no type"}"#])).is_none());
    let other = "<script type=\"application/json\">{\"@type\": \"Event\"}</script>";
    assert!(CopiedSchema::from_html(other).is_none());
}

#[test]
fn json_ld_this_site_would_refuse_comes_with_the_reason() {
    let html = page(&[r#"{"@type": "Event", "name": "Gig", "organizer": {"name": "No type"}}"#]);
    let copied = CopiedSchema::from_html(&html).unwrap();
    let problem = copied.problem().unwrap();
    assert!(problem.contains("organizer"), "{problem}");
}

/// A client that may reach the stand-in server on loopback.
fn loopback() -> Outbound {
    let resolver = Resolver::for_tests(Vec::new(), Reach::PublicAndLoopback);
    Outbound::with_resolver(Limits::default(), resolver).unwrap()
}

async fn serving(status: u16, body: String) -> (MockServer, AbsoluteUrl) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/recipes/soup/"))
        .respond_with(ResponseTemplate::new(status).set_body_string(body))
        .expect(1)
        .mount(&server)
        .await;
    let url = AbsoluteUrl::parse(&format!("{}/recipes/soup/", server.uri())).unwrap();
    (server, url)
}

#[tokio::test]
async fn a_page_is_fetched_through_outbound_and_its_schema_copied() {
    let html = page(&[r#"{"@type": "Recipe", "name": "Lentil soup"}"#]);
    let (_server, url) = serving(200, html).await;
    let copied = CopiedSchema::fetch(&loopback(), &url).await.unwrap();
    assert_eq!(copied.types(), ["Recipe"]);
}

#[tokio::test]
async fn a_page_that_answers_with_an_error_is_named_by_its_status() {
    let (_server, url) = serving(404, String::new()).await;
    let error = CopiedSchema::fetch(&loopback(), &url).await.unwrap_err();
    assert!(
        matches!(error, CopySchemaError::Status { status: 404 }),
        "{error:?}"
    );
}

#[tokio::test]
async fn a_page_with_no_schema_says_so() {
    let (_server, url) = serving(200, page(&[])).await;
    let error = CopiedSchema::fetch(&loopback(), &url).await.unwrap_err();
    assert!(matches!(error, CopySchemaError::NoSchema), "{error:?}");
}

#[tokio::test]
async fn an_address_inside_the_network_is_never_asked() {
    let outbound = Outbound::new(Limits::default()).unwrap();
    let url = AbsoluteUrl::parse("http://127.0.0.1:9/admin/").unwrap();
    let error = CopiedSchema::fetch(&outbound, &url).await.unwrap_err();
    assert!(
        matches!(
            error,
            CopySchemaError::Fetch {
                source: OutboundError::Blocked { .. }
            }
        ),
        "{error:?}"
    );
}
