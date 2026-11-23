//! The alerts raised for the owner through the Admin API (backlog row 15,
//! KH-GRO-6): what email and webhooks were told, for an owner with
//! neither, behind `settings:write`.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use khalid_api::alerts::{AlertError, AlertList, AlertReader, RaisedAlert, RaisedFinding};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::BoxFuture;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const SETTINGS: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const POSTS_ONLY: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

const ROUTE: &str = "/api/admin/v1/alerts/";

/// A reader that answers one alert, or fails.
struct Reader(bool);

impl AlertReader for Reader {
    fn alerts(&self) -> BoxFuture<'_, Result<AlertList, AlertError>> {
        let answer = if self.0 {
            Ok(list())
        } else {
            Err(AlertError::Internal("the disk is gone".into()))
        };
        Box::pin(async move { answer })
    }
}

#[cfg(test)]
fn list() -> AlertList {
    AlertList {
        alerts: vec![RaisedAlert {
            kind: "stale-post".to_owned(),
            what: "Some live posts have not changed for a long time.".to_owned(),
            why: "Old facts make readers trust a post less.".to_owned(),
            fix: "Read each post below and update it.".to_owned(),
            raised_at: "2026-11-03T09:00:00Z".to_owned(),
            findings: vec![RaisedFinding {
                key: "stale-post:en.a".to_owned(),
                line: "Old post (en.a), last changed 2024-03-01".to_owned(),
            }],
        }],
    }
}

/// GETs the alerts with `key` from a router whose `reader` is kept, or
/// none; answers the status and the body as JSON.
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
    let alerts: Option<Arc<dyn AlertReader>> = match reader {
        Some(reader) => Some(Arc::new(reader)),
        None => None,
    };
    let desks = AdminDesks {
        alerts,
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
async fn the_raised_alerts_are_listed_with_what_why_and_fix() {
    let (status, json) = get(Some(Reader(true)), SETTINGS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let alert = &json["alerts"][0];
    assert_eq!(alert["kind"], "stale-post");
    assert_eq!(alert["raised_at"], "2026-11-03T09:00:00Z");
    assert_eq!(
        alert["findings"],
        json!([{ "key": "stale-post:en.a",
            "line": "Old post (en.a), last changed 2024-03-01" }])
    );
    assert!(alert["what"].as_str().is_some_and(|what| !what.is_empty()));
}

#[tokio::test]
async fn a_key_without_settings_write_is_refused() {
    let (status, json) = get(Some(Reader(true)), POSTS_ONLY).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
}

#[tokio::test]
async fn a_server_without_a_database_lists_none() {
    let (status, json) = get(None, SETTINGS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json, json!({ "alerts": [] }));
}

#[tokio::test]
async fn a_failed_read_is_an_internal_problem() {
    let (status, _) = get(Some(Reader(false)), SETTINGS).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
}
