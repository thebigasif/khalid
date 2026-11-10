//! One find-and-replace across the library's alt text through the Admin
//! API, previewed first (plan 5.1).

mod common;

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode};
use khalid_api::api;
use khalid_api::comment_moderation::NoComments;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::media_api::MediaLibrary;
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_core::{Locale, SystemClock};
use khalid_media::{Library, Storage};
use serde_json::{Value, json};
use tower::ServiceExt;

const UPLOADER: &str =
    "ki_2222333344445555_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const ALT: &str = "/api/admin/v1/media/alt-text/";

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
    let english = Locale::try_new("en").unwrap();
    let library = Library::open(site, Storage::local(site), &english, Arc::new(SystemClock));
    let library = Arc::new(library.await.unwrap());
    let media = MediaLibrary::new(library, "https://www.example.com/blog");
    let (content, door, keys) = (Arc::new(common::store()), Arc::new(door), Arc::new(keys));
    api::router_with_media(content, door, keys, Arc::new(NoComments), media).0
}

#[cfg(test)]
async fn send(
    app: &Router,
    method: Method,
    uri: &str,
    key: &str,
    body: Vec<u8>,
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// Uploads a small text file named `name` with `alt` as its alt text and
/// answers its identity.
#[cfg(test)]
async fn with_alt(app: &Router, name: &str, alt: &str) -> String {
    let uri = format!("/api/admin/v1/media/?name={name}");
    let bytes = format!("Notes for {name}\n").into_bytes();
    let (status, body) = send(app, Method::POST, &uri, UPLOADER, bytes).await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    let item = &body["media"][0];
    let id = item["id"].as_str().unwrap().to_owned();
    let edit = json!({ "media": [{ "updated_at": item["updated_at"], "alt": alt }] });
    let uri = format!("/api/admin/v1/media/{id}/");
    let (status, body) = send(
        app,
        Method::PUT,
        &uri,
        UPLOADER,
        edit.to_string().into_bytes(),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    id
}

#[cfg(test)]
async fn replace(app: &Router, key: &str, body: &Value) -> (StatusCode, Value) {
    send(app, Method::POST, ALT, key, body.to_string().into_bytes()).await
}

#[cfg(test)]
async fn alt_of(app: &Router, id: &str) -> Value {
    let uri = format!("/api/admin/v1/media/{id}/");
    let (status, body) = send(app, Method::GET, &uri, UPLOADER, Vec::new()).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["media"][0]["alt"].clone()
}

#[tokio::test]
async fn a_preview_lists_the_changes_and_apply_makes_them() {
    let site = tempfile::tempdir().unwrap();
    let app = app(site.path()).await;
    let logo = with_alt(&app, "logo.txt", "The Acme Ltd logo").await;
    let other = with_alt(&app, "other.txt", "A blank page").await;
    let preview = json!({ "find": "Acme Ltd", "replace": "Acme" });
    let (status, body) = replace(&app, UPLOADER, &preview).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body["changes"],
        json!([{ "id": logo, "name": "logo.txt", "before": "The Acme Ltd logo", "after": "The Acme logo" }])
    );
    assert_eq!(body["applied"], false);
    assert_eq!(
        alt_of(&app, &logo).await,
        "The Acme Ltd logo",
        "a preview changes nothing"
    );
    let apply = json!({ "find": "Acme Ltd", "replace": "Acme", "apply": true });
    let (status, body) = replace(&app, UPLOADER, &apply).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["applied"], true);
    assert_eq!(body["changes"][0]["after"], "The Acme logo");
    assert_eq!(alt_of(&app, &logo).await, "The Acme logo");
    assert_eq!(alt_of(&app, &other).await, "A blank page");
}

#[tokio::test]
async fn refuses_a_replace_that_cannot_be_made() {
    let site = tempfile::tempdir().unwrap();
    let app = app(site.path()).await;
    let long = with_alt(&app, "long.txt", &"x".repeat(11)).await;
    for body in [
        json!({ "find": "  ", "replace": "Acme" }),
        json!({ "find": "x", "replace": "y".repeat(100), "apply": true }),
        json!({ "find": "x" }),
    ] {
        let (status, answer) = replace(&app, UPLOADER, &body).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}: {answer}");
    }
    assert_eq!(alt_of(&app, &long).await, "x".repeat(11), "nothing changed");
    let body = json!({ "find": "x", "replace": "y" });
    let (status, _) = replace(&app, WRITER, &body).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "needs media:write");
}
