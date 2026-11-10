//! The `[privacy]` table and the tagged `[[code]]` entries through the
//! Admin API's settings routes, as the admin's Privacy screen saves them
//! (privacy plan U8, R17, R19, R24).

use std::collections::BTreeMap;
use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::MemoryContentStore;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::resources::{AdminSettings, NavSetting, TemplateSetting};
use khalid_api::settings_store::MemorySettings;
use khalid_core::FixedClock;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const OWNER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const EDITOR: &str =
    "ki_2222333344445555_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
/// An integration key with `settings:write` and nothing more.
const SETTINGS_ONLY: &str =
    "ki_3333444455556666_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

const SETTINGS: &str = "/api/admin/v1/settings/";
const LAST_CHANGE: &str = "2026-11-02T09:00:00Z";

/// The demo site with one untagged script and no `[privacy]` table.
#[cfg(test)]
fn demo() -> Router {
    let hashing = KeyHashing::fast_for_tests();
    let token = |text: &str| text.parse::<KeyToken>().unwrap();
    let key = |text, scopes| StoredKey::for_integration(&token(text), scopes, &hashing).unwrap();
    let keys = MemoryKeyStore::new(vec![
        key(OWNER, vec![Scope::AdminAll]),
        key(EDITOR, vec![Scope::PostsWrite]),
        key(SETTINGS_ONLY, vec![Scope::SettingsWrite]),
    ]);
    let script = json!({"head": "<script src='https://stats.example.net/a.js'></script>"});
    let settings = AdminSettings {
        name: "Field Notes on SEO".to_owned(),
        address: "https://www.example.com".to_owned(),
        mount: "/blog/".to_owned(),
        default_language: "en".to_owned(),
        languages: vec!["en".to_owned(), "de".to_owned()],
        mode: "server".to_owned(),
        template: TemplateSetting {
            name: "classic".to_owned(),
            options: BTreeMap::new(),
        },
        seo: serde_json::Map::new(),
        privacy: serde_json::Map::new(),
        code: vec![serde_json::from_value(script).unwrap()],
        home_sections: Vec::new(),
        nav: NavSetting::default(),
        updated_at: Utc.with_ymd_and_hms(2026, 11, 2, 9, 0, 0).unwrap(),
        restart_needed: false,
    };
    let now = Utc.with_ymd_and_hms(2026, 11, 8, 9, 0, 0).unwrap();
    let store = MemorySettings::new(settings).with_clock(FixedClock::new(now));
    let desks = AdminDesks {
        settings: Some(Arc::new(store)),
        ..AdminDesks::none()
    };
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(MemoryContentStore::new(Vec::new()));
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

#[cfg(test)]
async fn send(router: &Router, key: &str, body: Option<Value>) -> (StatusCode, Value) {
    let method = if body.is_some() { "PUT" } else { "GET" };
    let request = Request::builder()
        .method(method)
        .uri(SETTINGS)
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json");
    let body = body.map_or_else(Body::empty, |body| Body::from(body.to_string()));
    let response = router.clone().oneshot(request.body(body).unwrap()).await;
    let response = response.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn a_privacy_part_saves_and_reads_back_with_the_mode() {
    let router = demo();
    let (status, json) = send(&router, OWNER, None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["settings"]["mode"], "server", "{json}");
    assert_eq!(json["settings"]["privacy"], json!({}), "{json}");
    let change = json!({"settings": {"updated_at": LAST_CHANGE, "privacy": {
        "lifetime_months": 12, "gpc": "reject-all",
        "contact": {"name": "Field Notes Ltd", "email": "privacy@example.com"},
    }}});
    let (status, json) = send(&router, OWNER, Some(change)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let (_, json) = send(&router, OWNER, None).await;
    let privacy = &json["settings"]["privacy"];
    assert_eq!(privacy["lifetime_months"], 12, "{json}");
    assert_eq!(privacy["gpc"], "reject-all", "{json}");
    assert_eq!(privacy["contact"]["email"], "privacy@example.com", "{json}");
}

#[tokio::test]
async fn a_script_tagged_as_tracking_no_one_saves_and_shows_after_a_reload() {
    let router = demo();
    let (_, json) = send(&router, OWNER, None).await;
    let mut entry = json["settings"]["code"][0].clone();
    assert_eq!(entry.get("category"), None, "untagged at first: {json}");
    entry["category"] = json!("no-tracking");
    entry["vendor"] = json!("Example Stats");
    let change = json!({"settings": {"updated_at": LAST_CHANGE, "code": [entry]}});
    let (status, json) = send(&router, OWNER, Some(change)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let (_, json) = send(&router, OWNER, None).await;
    let saved = &json["settings"]["code"][0];
    assert_eq!(saved["category"], "no-tracking", "{json}");
    assert_eq!(saved["vendor"], "Example Stats", "{json}");
    assert!(
        saved["head"]
            .as_str()
            .is_some_and(|head| head.contains("stats.example.net"))
    );
}

#[tokio::test]
async fn a_lifetime_of_fourteen_months_is_refused_naming_the_field() {
    let router = demo();
    let change = json!({"settings": {"updated_at": LAST_CHANGE,
        "privacy": {"lifetime_months": 14}}});
    let (status, json) = send(&router, OWNER, Some(change)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["rule"], "lifetime_months", "{json}");
    let (_, json) = send(&router, OWNER, None).await;
    assert_eq!(
        json["settings"]["privacy"],
        json!({}),
        "nothing saved: {json}"
    );
}

#[tokio::test]
async fn a_category_outside_the_four_is_refused() {
    let router = demo();
    let entry = json!({"head": "<script></script>", "category": "necessary"});
    let change = json!({"settings": {"updated_at": LAST_CHANGE, "code": [entry]}});
    let (status, json) = send(&router, OWNER, Some(change)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["rule"], "category", "{json}");
}

#[tokio::test]
async fn a_stale_copy_gets_a_conflict() {
    let router = demo();
    let first = json!({"settings": {"updated_at": LAST_CHANGE, "privacy": {"counts": false}}});
    let (status, _) = send(&router, OWNER, Some(first.clone())).await;
    assert_eq!(status, StatusCode::OK);
    let (status, json) = send(&router, OWNER, Some(first)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert_eq!(json["rule"], "updated_at", "{json}");
}

#[tokio::test]
async fn a_key_without_settings_write_cannot_change_privacy() {
    let router = demo();
    let change = json!({"settings": {"updated_at": LAST_CHANGE, "privacy": {"counts": false}}});
    let (status, json) = send(&router, EDITOR, Some(change)).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["rule"], "settings:write", "{json}");
}

#[tokio::test]
async fn a_settings_only_key_cannot_change_site_code_and_nothing_is_saved() {
    let router = demo();
    let entry = json!({"head": "<script src='https://other.example.org/x.js'></script>"});
    let change = json!({"settings": {"updated_at": LAST_CHANGE, "code": [entry]}});
    let (status, json) = send(&router, SETTINGS_ONLY, Some(change)).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["rule"], "code", "{json}");
    let (_, json) = send(&router, OWNER, None).await;
    let head = json["settings"]["code"][0]["head"].as_str().unwrap_or("");
    assert!(head.contains("stats.example.net"), "nothing saved: {json}");
}

#[tokio::test]
async fn a_settings_only_key_cannot_remove_site_code() {
    let router = demo();
    let change = json!({"settings": {"updated_at": LAST_CHANGE, "code": []}});
    let (status, json) = send(&router, SETTINGS_ONLY, Some(change)).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["rule"], "code", "{json}");
}

#[tokio::test]
async fn a_settings_only_key_may_send_the_code_back_unchanged_with_other_changes() {
    let router = demo();
    let (status, json) = send(&router, SETTINGS_ONLY, None).await;
    assert_eq!(status, StatusCode::OK, "it may still read the code: {json}");
    let code = json["settings"]["code"].clone();
    let change = json!({"settings": {"updated_at": LAST_CHANGE,
        "privacy": {"counts": false}, "code": code}});
    let (status, json) = send(&router, SETTINGS_ONLY, Some(change)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["settings"]["privacy"]["counts"], false, "{json}");
}
