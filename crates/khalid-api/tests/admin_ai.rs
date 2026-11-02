//! The `ai` route of the Admin API (Yoast package WP4): off until the
//! owner names a provider, and ready only with the key that asks it.
//! Every request goes to a fake writer; nothing leaves the test.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use khalid_api::ai::{AiDesk, AiWriter, FakeWriter, Lengths};
use khalid_api::api::{self, AdminDesks};
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::serpdad_store::SerpdadStore;
use khalid_core::AiProvider;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const WRITER: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";
const READER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

#[cfg(test)]
fn router(ai: AiDesk, serpdad: Option<Arc<dyn SerpdadStore>>) -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(WRITER, Scope::PostsWrite),
        stored(READER, Scope::ContentRead),
    ]);
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let desks = AdminDesks {
        ai,
        serpdad,
        ..AdminDesks::none()
    };
    let content = Arc::new(common::store());
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

#[cfg(test)]
fn desk(provider: AiProvider, writer: &Arc<FakeWriter>) -> AiDesk {
    let writer: Arc<FakeWriter> = Arc::clone(writer);
    let writer: Arc<dyn AiWriter> = writer;
    AiDesk::new(provider, Some(writer), Lengths::default())
}

#[cfg(test)]
async fn send(
    router: &axum::Router,
    key: &str,
    (method, path): (&str, &str),
    body: Option<&Value>,
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(format!("/api/admin/v1{path}"))
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json")
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .unwrap();
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

const STATE: (&str, &str) = ("GET", "/ai/");

#[tokio::test]
async fn off_by_default() {
    let router = router(AiDesk::off(), None);
    let (status, json) = send(&router, WRITER, STATE, None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let off = json!({"on": false, "provider": "off", "label": "Off", "ready": false});
    assert_eq!(json, off);
}

#[tokio::test]
async fn on_and_ready_with_a_writer() {
    let writer = Arc::new(FakeWriter::answering(&[]));
    let router = router(desk(AiProvider::Anthropic, &writer), None);
    let (_, json) = send(&router, WRITER, STATE, None).await;
    let on = json!({"on": true, "provider": "anthropic", "label": "Anthropic", "ready": true});
    assert_eq!(json, on);
}

#[tokio::test]
async fn on_but_not_ready_without_a_key() {
    let desk = AiDesk::new(AiProvider::Openai, None, Lengths::default());
    let router = router(desk, None);
    let (_, json) = send(&router, WRITER, STATE, None).await;
    let label = "OpenAI-compatible service";
    let waiting = json!({"on": true, "provider": "openai", "label": label, "ready": false});
    assert_eq!(json, waiting);
}

#[tokio::test]
async fn a_key_that_cannot_write_posts_is_refused() {
    let router = router(AiDesk::off(), None);
    let (status, _) = send(&router, READER, STATE, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}
