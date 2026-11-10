//! The site's glossary through the Admin API: terms a post links with a
//! wikilink, each on its own page (KH-API-2).

use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::MemoryContentStore;
use khalid_api::entry_store::{EntryStore, MemoryEntries};
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::resources::AdminTerm;
use khalid_core::{FixedClock, Locale, Slug};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const EDITOR: &str =
    "ki_2222333344445555_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

const GLOSSARY: &str = "/api/admin/v1/glossary/";
/// The one term.
const AUTHORITY: &str = "/api/admin/v1/glossary/en.topical-authority/";

/// A site keeping `glossary`, or none at all.
#[cfg(test)]
fn router(glossary: Option<Arc<dyn EntryStore<AdminTerm>>>) -> Router {
    let hashing = KeyHashing::fast_for_tests();
    let token = |text: &str| text.parse::<KeyToken>().unwrap();
    let key = |text, scopes| StoredKey::for_integration(&token(text), scopes, &hashing).unwrap();
    let keys = MemoryKeyStore::new(vec![
        key(WRITER, vec![Scope::PostsWrite]),
        key(EDITOR, vec![Scope::PostsWrite, Scope::PostsPublish]),
    ]);
    let desks = AdminDesks {
        glossary,
        ..AdminDesks::none()
    };
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(MemoryContentStore::new(Vec::new()));
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

/// A site with one term, "Topical authority".
#[cfg(test)]
fn with_authority() -> Router {
    let at = Utc.with_ymd_and_hms(2026, 10, 20, 9, 0, 0).unwrap();
    let slug = Slug::try_new("topical-authority").unwrap();
    let english = Locale::try_new("en").unwrap();
    let term = AdminTerm::new(
        (english, slug),
        "Topical authority",
        "How much search engines trust a site on one subject.",
        at,
    )
    .with_translation(Some("authority"));
    let now = Utc.with_ymd_and_hms(2026, 10, 27, 9, 0, 0).unwrap();
    let glossary = MemoryEntries::new(vec![term]).with_clock(FixedClock::new(now));
    router(Some(Arc::new(glossary)))
}

#[cfg(test)]
async fn send(
    router: &Router,
    method: &str,
    uri: &str,
    key: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json");
    let body = body.map_or_else(Body::empty, |body| Body::from(body.to_string()));
    let response = router
        .clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn terms_are_listed_and_read_with_posts_write() {
    let router = with_authority();
    let (status, json) = send(&router, "GET", GLOSSARY, WRITER, None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["glossary"][0]["id"], "en.topical-authority", "{json}");
    assert_eq!(json["glossary"][0]["translation"], "authority", "{json}");
    let (status, json) = send(&router, "GET", AUTHORITY, WRITER, None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["glossary"][0]["term"], "Topical authority", "{json}");
    let missing = "/api/admin/v1/glossary/en.anchor-text/";
    let (status, json) = send(&router, "GET", missing, WRITER, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
    let fix = json["fix"].as_str().unwrap();
    assert!(fix.contains("en.topical-authority"), "{json}");
}

#[tokio::test]
async fn a_new_term_is_made_once_in_the_default_language() {
    let router = with_authority();
    let new = json!({"glossary": [{"slug": "anchor-text", "term": "Anchor text", "markdown": "The words a link is written on."}]});
    let (status, json) = send(&router, "POST", GLOSSARY, EDITOR, Some(new.clone())).await;
    assert_eq!(status, StatusCode::CREATED, "{json}");
    assert_eq!(json["glossary"][0]["id"], "en.anchor-text", "{json}");
    assert_eq!(json["glossary"][0]["translation"], Value::Null, "{json}");
    assert_eq!(json["glossary"][0]["updated_at"], "2026-10-27T09:00:00Z");
    let (status, json) = send(&router, "POST", GLOSSARY, EDITOR, Some(new)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert_eq!(json["rule"], "slug", "{json}");
}

#[tokio::test]
async fn a_term_that_cannot_be_kept_or_is_sent_without_rights_is_refused() {
    let router = with_authority();
    let blank = json!({"glossary": [{"slug": "a", "term": " ", "markdown": "M."}]});
    let (status, json) = send(&router, "POST", GLOSSARY, EDITOR, Some(blank)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["rule"], "term", "{json}");
    let undefined = json!({"glossary": [{"slug": "a", "term": "A", "markdown": ""}]});
    let (status, json) = send(&router, "POST", GLOSSARY, EDITOR, Some(undefined)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["rule"], "markdown", "{json}");
    let new = json!({"glossary": [{"slug": "a", "term": "A", "markdown": "M."}]});
    let (status, json) = send(&router, "POST", GLOSSARY, WRITER, Some(new)).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["rule"], "posts:publish", "{json}");
    let (_, json) = send(&router, "GET", GLOSSARY, WRITER, None).await;
    assert_eq!(json["glossary"].as_array().map(Vec::len), Some(1), "{json}");
}

#[tokio::test]
async fn a_server_without_a_glossary_answers_404_naming_why() {
    let (status, json) = send(&router(None), "GET", GLOSSARY, EDITOR, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
    let detail = json["detail"].as_str().unwrap();
    assert!(detail.contains("no glossary terms"), "{json}");
}
