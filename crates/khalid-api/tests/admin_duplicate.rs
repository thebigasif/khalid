//! Copying a post as a new draft through the Admin API, as Yoast's
//! Duplicate Post does (Yoast package WP3).

mod common;

use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::api;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_core::FixedClock;
use serde_json::{Value, json};
use tower::ServiceExt;

/// A key made up for these tests. It opens nothing real.
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

const POSTS: &str = "/api/admin/v1/posts/";

/// The API over an empty post store, with the time fixed.
#[cfg(test)]
fn router() -> Router {
    let hashing = KeyHashing::fast_for_tests();
    let token = WRITER.parse::<KeyToken>().unwrap();
    let key = StoredKey::for_integration(&token, vec![Scope::PostsWrite], &hashing).unwrap();
    let now = Utc.with_ymd_and_hms(2026, 11, 2, 9, 0, 0).unwrap();
    let posts = MemoryPostStore::default().with_clock(FixedClock::new(now));
    let keys = Arc::new(MemoryKeyStore::new(vec![key]));
    let (router, _) = api::router(Arc::new(common::store()), Arc::new(posts), keys);
    router
}

#[cfg(test)]
async fn send(
    router: &Router,
    method: &str,
    path: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(path)
        .header("authorization", format!("Khalid {WRITER}"))
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
async fn a_copy_is_a_new_draft_with_the_text_and_settings_but_not_the_old_addresses() {
    let router = router();
    let new =
        json!({"posts": [{"title": "Choosing anchor text", "markdown": "Say where it goes."}]});
    let (status, made) = send(&router, "POST", POSTS, Some(new)).await;
    assert_eq!(status, StatusCode::CREATED, "{made}");
    let id = made["posts"][0]["id"].as_str().unwrap().to_owned();
    let settings = json!({"focus_keyword": "anchor text", "aliases": ["/2019/05/anchor/"],
        "secondary_keywords": ["link text"]});
    let edit =
        json!({"posts": [{"settings": settings, "updated_at": made["posts"][0]["updated_at"]}]});
    let (status, saved) = send(&router, "PUT", &format!("{POSTS}{id}/"), Some(edit)).await;
    assert_eq!(status, StatusCode::OK, "{saved}");

    let route = format!("{POSTS}{id}/duplicate/");
    let (status, first) = send(&router, "POST", &route, None).await;
    assert_eq!(status, StatusCode::CREATED, "{first}");
    let copy = &first["posts"][0];
    assert_eq!(copy["title"], "Choosing anchor text (copy)", "{first}");
    assert_eq!(copy["slug"], "choosing-anchor-text-copy", "{first}");
    assert_eq!(copy["status"], "draft", "{first}");
    assert_eq!(copy["markdown"], "Say where it goes.", "{first}");
    assert_eq!(copy["settings"]["focus_keyword"], "anchor text", "{first}");
    assert_eq!(copy["settings"]["secondary_keywords"], json!(["link text"]));
    assert_eq!(
        copy["settings"]["aliases"],
        json!([]),
        "old addresses stay with the post"
    );
    let (_, second) = send(&router, "POST", &route, None).await;
    assert_eq!(
        second["posts"][0]["slug"], "choosing-anchor-text-copy-2",
        "{second}"
    );

    let missing = format!("{POSTS}en.no-such-post/duplicate/");
    let (status, _) = send(&router, "POST", &missing, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
