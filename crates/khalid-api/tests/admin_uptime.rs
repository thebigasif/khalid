//! The speed and uptime watch through the Admin API (C6, KH-API-5): the
//! report `khalid watch` prints, behind `settings:write`.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::BoxFuture;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::uptime::{
    UptimeError, UptimeReader, UptimeReport, WatchLook, WatchSpan, WatchedPage,
};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const SETTINGS: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const POSTS_ONLY: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

const ROUTE: &str = "/api/admin/v1/watch/";

/// A reader that answers one page, or fails.
struct Reader(bool);

impl UptimeReader for Reader {
    fn report(&self) -> BoxFuture<'_, Result<UptimeReport, UptimeError>> {
        let answer = if self.0 {
            Ok(report())
        } else {
            Err(UptimeError::Internal("the disk is gone".into()))
        };
        Box::pin(async move { answer })
    }
}

#[cfg(test)]
fn report() -> UptimeReport {
    let span = |looks, answered| WatchSpan {
        looks,
        answered,
        middle_ms: Some(120),
        slowest_ms: Some(900),
        longest_gap_minutes: 5,
    };
    UptimeReport {
        pages: vec![WatchedPage {
            page: "https://example.com/blog/".to_owned(),
            last_day: span(288, 287),
            last_30_days: span(8640, 8600),
            last_look: Some(WatchLook {
                at: "2026-11-10T09:30:00Z".to_owned(),
                status: None,
                millis: 10_000,
            }),
        }],
    }
}

/// GETs the watch with `key` from a router whose watch `reader` is kept,
/// or none; answers the status and the body as JSON.
#[cfg(test)]
async fn get(reader: Option<Reader>, key: &str) -> (StatusCode, Value) {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(SETTINGS, Scope::SettingsWrite),
        stored(POSTS_ONLY, Scope::PostsWrite),
    ]);
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let uptime: Option<Arc<dyn UptimeReader>> = match reader {
        Some(reader) => Some(Arc::new(reader)),
        None => None,
    };
    let desks = AdminDesks {
        uptime,
        ..AdminDesks::none()
    };
    let content = Arc::new(common::store());
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    let request = Request::get(ROUTE)
        .header("authorization", format!("Khalid {key}"))
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn the_watch_answers_the_shape_khalid_watch_prints() {
    let (status, json) = get(Some(Reader(true)), SETTINGS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let page = &json["pages"][0];
    assert_eq!(page["page"], "https://example.com/blog/");
    assert_eq!(
        page["last_day"],
        json!({ "looks": 288, "answered": 287, "middle_ms": 120,
            "slowest_ms": 900, "longest_gap_minutes": 5 })
    );
    assert_eq!(page["last_30_days"]["looks"], 8640);
    assert_eq!(page["last_look"]["status"], Value::Null);
    assert_eq!(page["last_look"]["millis"], 10_000);
}

#[tokio::test]
async fn a_key_without_settings_write_is_refused() {
    let (status, json) = get(Some(Reader(true)), POSTS_ONLY).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
}

#[tokio::test]
async fn a_server_without_looks_says_so() {
    let (status, json) = get(None, SETTINGS).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
    let detail = json["detail"].as_str().unwrap_or_default();
    assert!(detail.contains("keeps no looks"), "{json}");
}

#[tokio::test]
async fn a_failed_read_is_an_internal_problem() {
    let (status, _) = get(Some(Reader(false)), SETTINGS).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
}
