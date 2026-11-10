//! Deleting a media item through the Admin API.

mod common;

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::api;
use khalid_api::comment_moderation::NoComments;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::media_api::MediaLibrary;
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_core::{FixedClock, Locale};
use khalid_media::{Library, Storage};
use serde_json::Value;
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const UPLOADER: &str =
    "ki_2222333344445555_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

const PDF: &[u8] = b"%PDF-1.7\nA checklist for choosing a CRM\n%%EOF\n";

/// The API with the media library of `site`.
#[cfg(test)]
async fn app(site: &Path) -> Router {
    let hashing = KeyHashing::fast_for_tests();
    let key = |text: &str, scope| {
        let token = text.parse::<KeyToken>().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        key(UPLOADER, Scope::MediaWrite),
        key(WRITER, Scope::PostsWrite),
    ]);
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let (content, door, keys) = (Arc::new(common::store()), Arc::new(door), Arc::new(keys));
    let clock = FixedClock::new(Utc.with_ymd_and_hms(2026, 11, 10, 8, 0, 0).unwrap());
    let english = Locale::try_new("en").unwrap();
    let library = Library::open(site, Storage::local(site), &english, Arc::new(clock));
    let library = Arc::new(library.await.unwrap());
    let media = MediaLibrary::new(library, "https://www.example.com/blog");
    api::router_with_media(content, door, keys, Arc::new(NoComments), media).0
}

/// Sends `body` to `uri` with `method` and `key`, and reads the answer.
#[cfg(test)]
async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    key: &str,
    body: &[u8],
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Khalid {key}"))
        .body(Body::from(body.to_vec()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// Uploads `bytes` as `name` into `folder`, and returns its identity and
/// `updated_at`.
#[cfg(test)]
async fn upload(app: &Router, name: &str, folder: &str, bytes: &[u8]) -> (String, String) {
    let uri = format!("/api/admin/v1/media/?name={name}&folder={folder}");
    let (status, body) = send(app, "POST", &uri, UPLOADER, bytes).await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    let item = &body["media"][0];
    let text = |field: &str| item[field].as_str().unwrap().to_owned();
    (text("id"), text("updated_at"))
}

#[tokio::test]
async fn an_unused_item_is_deleted_with_the_copy_last_read() {
    let site = tempfile::tempdir().unwrap();
    let app = app(site.path()).await;
    let (id, updated_at) = upload(&app, "checklist.pdf", "guides", PDF).await;
    let uri = format!("/api/admin/v1/media/{id}/?updated_at={updated_at}");
    let (status, _) = send(&app, "DELETE", &uri, UPLOADER, b"").await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let read = format!("/api/admin/v1/media/{id}/");
    let (status, _) = send(&app, "GET", &read, UPLOADER, b"").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, body) = send(&app, "DELETE", &uri, UPLOADER, b"").await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
}

#[tokio::test]
async fn a_delete_needs_updated_at_the_current_copy_and_media_write() {
    let site = tempfile::tempdir().unwrap();
    let app = app(site.path()).await;
    let (id, _) = upload(&app, "checklist.pdf", "", PDF).await;
    let bare = format!("/api/admin/v1/media/{id}/");
    let (status, body) = send(&app, "DELETE", &bare, UPLOADER, b"").await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(body["rule"], "updated_at");
    let old = format!("{bare}?updated_at=2026-11-01T08:00:00Z");
    let (status, body) = send(&app, "DELETE", &old, UPLOADER, b"").await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert!(
        body["detail"]
            .as_str()
            .unwrap()
            .contains("changed this item at 2026-11-10 08:00 UTC"),
        "{body}"
    );
    let (status, _) = send(&app, "DELETE", &old, WRITER, b"").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = send(&app, "GET", &bare, UPLOADER, b"").await;
    assert_eq!(status, StatusCode::OK, "nothing was deleted");
}
