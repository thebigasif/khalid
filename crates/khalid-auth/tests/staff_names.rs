//! A signed-in staff member shows by their account's name, not their
//! identity, where the admin says who has a post open (backlog row 12,
//! KH-EDT-2).

use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::api;
use khalid_api::content_store::MemoryContentStore;
use khalid_api::keys::{MemoryKeyStore, Role};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::resources::AdminPost;
use khalid_api::session_check::SessionCheck;
use khalid_auth::{
    AccountStore, Accounts, MemoryAccounts, MemorySessions, Password, PasswordHashing, staff_routes,
};
use khalid_core::{FixedClock, Slug};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Accounts where Maria Lopez, an editor, has the password "correct horse
/// battery".
#[cfg(test)]
async fn accounts() -> Accounts {
    let at = Utc.with_ymd_and_hms(2026, 11, 10, 9, 0, 0).unwrap();
    let store = Arc::new(MemoryAccounts::new());
    let hashing = PasswordHashing::fast_for_tests();
    let email = "maria@example.com".parse().unwrap();
    let maria = store.create(&email, "Maria Lopez", Role::Editor, at).await;
    let maria = maria.unwrap();
    let password = Password::new("correct horse battery".to_owned()).unwrap();
    let hash = hashing.hash(password).await.unwrap();
    store.set_password(maria.id(), &hash).await.unwrap();
    let clock = Arc::new(FixedClock::new(at));
    Accounts::new(store, Arc::new(MemorySessions::new()), hashing, clock)
}

#[cfg(test)]
async fn send(router: &Router, request: Request<Body>) -> (StatusCode, String, Value) {
    let response = router.clone().oneshot(request).await.unwrap();
    let (parts, body) = response.into_parts();
    let cookie = parts.headers.get("set-cookie").map(|value| {
        let value = value.to_str().unwrap();
        value.split(';').next().unwrap().to_owned()
    });
    let body = to_bytes(body, 1024 * 1024).await.unwrap();
    let json = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (parts.status, cookie.unwrap_or_default(), json)
}

#[tokio::test]
async fn the_accounts_name_each_staff_member_by_the_identity_they_sign_in_with() {
    let accounts = accounts().await;
    let names = accounts.names().await.unwrap();
    let names: Vec<(&str, &str)> = names
        .iter()
        .map(|(id, name)| (id.as_str(), name.as_str()))
        .collect();
    assert_eq!(names, [("0000000000000001", "Maria Lopez")]);
}

#[tokio::test]
async fn a_signed_in_editor_holds_a_post_by_their_name() {
    let accounts = accounts().await;
    let at = Utc.with_ymd_and_hms(2026, 11, 10, 8, 0, 0).unwrap();
    let slug = Slug::try_new("choosing-anchor-text").unwrap();
    let draft = AdminPost::new("en.anchor", slug, "Choosing anchor text", "Words.", at);
    let (router, _) = api::router_with_staff(
        Arc::new(MemoryContentStore::new(Vec::new())),
        Arc::new(MemoryPostStore::new(vec![draft])),
        Arc::new(MemoryKeyStore::new(Vec::new())),
        staff_routes(accounts.clone()),
        Arc::new(accounts),
    );
    let sign_in = json!({ "email": "maria@example.com", "password": "correct horse battery" });
    let request = Request::post("/api/admin/v1/session/")
        .header("content-type", "application/json")
        .body(Body::from(sign_in.to_string()))
        .unwrap();
    let (status, cookie, json) = send(&router, request).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let csrf = json["csrf"].as_str().unwrap();
    let request = Request::post("/api/admin/v1/posts/en.anchor/presence/")
        .header("cookie", cookie)
        .header("x-khalid-csrf", csrf)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "claim": "join" }).to_string()))
        .unwrap();
    let (status, _, json) = send(&router, request).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["holder"]["who"], "0000000000000001", "{json}");
    assert_eq!(json["holder"]["name"], "Maria Lopez", "{json}");
}
