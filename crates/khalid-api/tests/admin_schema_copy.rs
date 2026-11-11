//! "Copy schema from a URL" in the Admin API (KH-SEO-4, G54): the route
//! hands the address to the server's copier and answers with the JSON-LD,
//! or with a plain reason when there is none to copy.

mod common;

use std::sync::{Arc, Mutex};

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::BoxFuture;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::schema_copy::SchemaCopier;
use khalid_core::AbsoluteUrl;
use khalid_outbound::OutboundError;
use khalid_seo::{CopiedSchema, CopySchemaError};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const MEDIA: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

/// A copier that answers by the host asked for, and notes each address.
#[derive(Default)]
struct Pages {
    asked: Mutex<Vec<String>>,
}

#[cfg(test)]
impl SchemaCopier for Pages {
    fn copy<'a>(
        &'a self,
        url: &'a AbsoluteUrl,
    ) -> BoxFuture<'a, Result<CopiedSchema, CopySchemaError>> {
        self.asked.lock().unwrap().push(url.as_str().to_owned());
        let answer = match url.host() {
            "recipes.example" => {
                let html = r#"<script type="application/ld+json">
                    {"@type": "Recipe", "name": "Lentil soup"}</script>"#;
                Ok(CopiedSchema::from_html(html).unwrap())
            }
            "plain.example" => Err(CopySchemaError::NoSchema),
            "gone.example" => Err(CopySchemaError::Status { status: 404 }),
            "slow.example" => Err(CopySchemaError::Fetch {
                source: OutboundError::TimedOut,
            }),
            _ => Err(CopySchemaError::Fetch {
                source: OutboundError::Blocked {
                    host: url.host().to_owned(),
                    address: [10, 0, 0, 1].into(),
                },
            }),
        };
        Box::pin(async move { answer })
    }
}

#[cfg(test)]
fn router(copier: Option<Arc<Pages>>) -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(WRITER, Scope::PostsWrite),
        stored(MEDIA, Scope::MediaWrite),
    ]);
    let desks = AdminDesks {
        schema_copier: copier.map(|copier| -> Arc<dyn SchemaCopier> { copier }),
        ..AdminDesks::none()
    };
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(common::store());
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

#[cfg(test)]
async fn copy(router: &axum::Router, key: &str, url: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/v1/schema/copies/")
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json")
        .body(Body::from(json!({ "url": url }).to_string()))
        .unwrap();
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn a_pages_json_ld_comes_back_ready_for_the_custom_field() {
    let pages = Arc::new(Pages::default());
    let router = router(Some(Arc::clone(&pages)));
    let (status, body) = copy(&router, WRITER, "https://recipes.example/soup/").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["types"], json!(["Recipe"]));
    assert_eq!(body["problem"], Value::Null);
    let json_ld: Value = serde_json::from_str(body["json_ld"].as_str().unwrap()).unwrap();
    assert_eq!(
        json_ld,
        json!({"@context": "https://schema.org", "@type": "Recipe", "name": "Lentil soup"})
    );
    assert_eq!(
        *pages.asked.lock().unwrap(),
        ["https://recipes.example/soup/"]
    );
}

#[tokio::test]
async fn a_key_that_cannot_write_posts_is_refused() {
    let (status, body) = copy(
        &router(Some(Arc::default())),
        MEDIA,
        "https://recipes.example/",
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
}

#[tokio::test]
async fn text_that_is_not_a_web_address_is_never_fetched() {
    let pages = Arc::new(Pages::default());
    let router = router(Some(Arc::clone(&pages)));
    for url in ["recipes.example/soup", "ftp://recipes.example/", ""] {
        let (status, body) = copy(&router, WRITER, url).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{url}: {body}");
        assert!(
            body["detail"].as_str().unwrap().contains("https://"),
            "{body}"
        );
    }
    assert!(pages.asked.lock().unwrap().is_empty());
}

#[tokio::test]
async fn each_failure_is_told_in_plain_words() {
    let router = router(Some(Arc::default()));
    let cases = [
        (
            "https://plain.example/",
            StatusCode::UNPROCESSABLE_ENTITY,
            "no schema",
        ),
        ("https://gone.example/", StatusCode::BAD_GATEWAY, "404"),
        (
            "https://slow.example/",
            StatusCode::BAD_GATEWAY,
            "fetched: the request took longer",
        ),
        (
            "https://inside.example/",
            StatusCode::UNPROCESSABLE_ENTITY,
            "inside",
        ),
    ];
    for (url, expected, words) in cases {
        let (status, body) = copy(&router, WRITER, url).await;
        assert_eq!(status, expected, "{url}: {body}");
        let detail = body["detail"].as_str().unwrap();
        assert!(detail.contains(words), "{url}: {detail}");
    }
}

#[tokio::test]
async fn a_server_that_copies_nothing_says_so() {
    let (status, body) = copy(&router(None), WRITER, "https://recipes.example/").await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
}
