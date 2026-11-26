//! The schema check in the Admin API: hand-written JSON-LD measured against
//! the schema.org vocabulary and against what Google needs, in two reports,
//! with a one-click change where a finding has one.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::BoxFuture;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::schema_copy::SchemaCopier;
use khalid_core::AbsoluteUrl;
use khalid_seo::{CopiedSchema, CopySchemaError, PageMarkup};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const MEDIA: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

/// Pages by host: a shop page with a slip and a price it does not show,
/// and a page with no schema.
struct Pages;

#[cfg(test)]
impl SchemaCopier for Pages {
    fn copy<'a>(
        &'a self,
        _url: &'a AbsoluteUrl,
    ) -> BoxFuture<'a, Result<CopiedSchema, CopySchemaError>> {
        Box::pin(async { Err(CopySchemaError::NoSchema) })
    }

    fn markup<'a>(
        &'a self,
        url: &'a AbsoluteUrl,
    ) -> BoxFuture<'a, Result<PageMarkup, CopySchemaError>> {
        let html = match url.host() {
            "shop.example" => {
                r#"<script type="application/ld+json">{"@type": "Product", "name": "Kit",
                  "offers": {"@type": "Offer", "price": "29.99", "pricecurrency": "GBP"}}</script>
                  <h1>Kit</h1><p>Only 19.99 pounds.</p>"#
            }
            _ => "<p>No schema here.</p>",
        };
        let page = PageMarkup::from_html(html).ok_or(CopySchemaError::NoSchema);
        Box::pin(async move { page })
    }
}

#[cfg(test)]
fn router() -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(WRITER, Scope::PostsWrite),
        stored(MEDIA, Scope::MediaWrite),
    ]);
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(common::store());
    let desks = AdminDesks {
        schema_copier: Some(Arc::new(Pages)),
        ..AdminDesks::none()
    };
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

#[cfg(test)]
async fn check(key: &str, body: Value) -> (StatusCode, Value) {
    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/v1/schema/checks/")
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let response = router().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn a_slip_comes_back_in_the_schema_org_report_with_its_change() {
    let markup = r#"{"@context": "https://schema.org", "@type": "Product", "name": "Kit",
      "offers": {"@type": "Offer", "price": "5", "pricecurrency": "USD"}}"#;
    let (status, body) = check(WRITER, json!({ "json_ld": markup })).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["types"], json!(["Product"]));
    let found = &body["schema_org"][0];
    assert_eq!(found["level"], "error");
    assert_eq!(found["path"], "offers.pricecurrency");
    assert_eq!(
        found["change"],
        json!({
            "target": "property",
            "path": "offers.pricecurrency",
            "from": "pricecurrency",
            "to": "priceCurrency",
            "label": "Change pricecurrency to priceCurrency"
        })
    );
    assert_eq!(body["schema_org"].as_array().unwrap().len(), 1, "{body}");
}

#[tokio::test]
async fn the_google_report_names_what_a_rich_result_needs() {
    let markup = r#"{"@type": "Restaurant", "name": "Leeds Noodle Bar",
      "image": "https://example.com/bar.jpg"}"#;
    let (status, body) = check(
        WRITER,
        json!({ "json_ld": markup, "visible_text": "Leeds Noodle Bar is open daily." }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["schema_org"], json!([]));
    let google = body["google"].as_array().unwrap();
    // A restaurant is measured as the local business it is a kind of.
    let errors: Vec<&str> = google
        .iter()
        .filter(|f| f["level"] == "error")
        .map(|f| f["what"].as_str().unwrap())
        .collect();
    assert_eq!(errors, ["This LocalBusiness has no address."], "{body}");
    assert!(google.iter().any(|f| f["level"] == "warning"), "{body}");
}

#[tokio::test]
async fn text_that_is_not_json_is_refused_with_where() {
    let (status, body) = check(WRITER, json!({ "json_ld": "{\"@type\": }" })).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["detail"].as_str().unwrap().contains("line 1"),
        "{body}"
    );
}

#[tokio::test]
async fn a_key_that_cannot_write_posts_is_refused() {
    let (status, body) = check(MEDIA, json!({ "json_ld": "{}" })).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
}

#[tokio::test]
async fn a_page_address_is_fetched_and_checked_against_what_it_shows() {
    let (status, body) = check(WRITER, json!({ "url": "https://shop.example/kit/" })).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["types"], json!(["Product"]));
    assert_eq!(body["schema_org"][0]["path"], "offers.pricecurrency");
    let unseen = body["google"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["path"] == "offers.price");
    assert!(unseen, "{body}");
}

#[tokio::test]
async fn a_page_with_no_schema_or_a_bad_address_says_so() {
    let (status, body) = check(WRITER, json!({ "url": "https://plain.example/" })).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["detail"].as_str().unwrap().contains("no schema"),
        "{body}"
    );
    let (status, body) = check(WRITER, json!({ "url": "plain.example" })).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let both = json!({ "url": "https://shop.example/", "json_ld": "{}" });
    let (status, body) = check(WRITER, both).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
}
