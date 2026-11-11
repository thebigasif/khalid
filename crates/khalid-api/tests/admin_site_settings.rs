//! The site's core settings through the Admin API, with the 409 a stale
//! copy gets and the 422 a value the site could not start with gets
//! (KH-API-2).

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
use khalid_api::settings_store::{MemorySettings, SettingsStore};
use khalid_core::FixedClock;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const OWNER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const EDITOR: &str =
    "ki_2222333344445555_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

const SETTINGS: &str = "/api/admin/v1/settings/";
const LAST_CHANGE: &str = "2026-10-20T09:00:00Z";

/// A site keeping `settings`, or none at all.
#[cfg(test)]
fn router(settings: Option<Arc<dyn SettingsStore>>) -> Router {
    let hashing = KeyHashing::fast_for_tests();
    let token = |text: &str| text.parse::<KeyToken>().unwrap();
    let key = |text, scopes| StoredKey::for_integration(&token(text), scopes, &hashing).unwrap();
    let keys = MemoryKeyStore::new(vec![
        key(OWNER, vec![Scope::SettingsWrite]),
        key(EDITOR, vec![Scope::PostsWrite, Scope::PostsPublish]),
    ]);
    let desks = AdminDesks {
        settings,
        ..AdminDesks::none()
    };
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(MemoryContentStore::new(Vec::new()));
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

/// The demo's settings, last changed on 20 October.
#[cfg(test)]
fn demo() -> Router {
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
        code: Vec::new(),
        home_sections: Vec::new(),
        nav: NavSetting::default(),
        updated_at: Utc.with_ymd_and_hms(2026, 10, 20, 9, 0, 0).unwrap(),
        restart_needed: false,
    };
    let now = Utc.with_ymd_and_hms(2026, 10, 27, 9, 0, 0).unwrap();
    let store = MemorySettings::new(settings).with_clock(FixedClock::new(now));
    router(Some(Arc::new(store)))
}

#[cfg(test)]
async fn send(
    router: &Router,
    method: &str,
    key: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(SETTINGS)
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json");
    let body = body.map_or_else(Body::empty, |body| Body::from(body.to_string()));
    let request = request.body(body).unwrap();
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn the_settings_are_read_and_changed_with_settings_write() {
    let router = demo();
    let (status, json) = send(&router, "GET", OWNER, None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["settings"]["mount"], "/blog/", "{json}");
    let change = json!({"settings": {"name": "Field Notes", "updated_at": LAST_CHANGE}});
    let (status, json) = send(&router, "PUT", OWNER, Some(change.clone())).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["settings"]["name"], "Field Notes", "{json}");
    assert_eq!(
        json["settings"]["updated_at"], "2026-10-27T09:00:00Z",
        "{json}"
    );
    assert_eq!(json["settings"]["template"]["name"], "classic", "{json}");
    let (status, json) = send(&router, "PUT", OWNER, Some(change)).await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "a replay of the old copy: {json}"
    );
    assert_eq!(json["rule"], "updated_at", "{json}");
}

#[tokio::test]
async fn a_change_the_site_could_not_start_with_is_refused() {
    let router = demo();
    let orphan = json!({"settings": {"default_language": "fr", "updated_at": LAST_CHANGE}});
    let (status, json) = send(&router, "PUT", OWNER, Some(orphan)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["rule"], "default_language", "{json}");
    let mode = json!({"settings": {"mode": "static", "updated_at": LAST_CHANGE}});
    let (status, _) = send(&router, "PUT", OWNER, Some(mode)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (_, json) = send(&router, "GET", OWNER, None).await;
    assert_eq!(
        json["settings"]["default_language"], "en",
        "nothing saved: {json}"
    );
}

#[tokio::test]
async fn settings_need_settings_write_and_a_server_that_keeps_them() {
    let site = demo();
    let (status, json) = send(&site, "GET", EDITOR, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["rule"], "settings:write", "{json}");
    let change = json!({"settings": {"name": "Mine", "updated_at": LAST_CHANGE}});
    let (status, _) = send(&site, "PUT", EDITOR, Some(change)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, json) = send(&router(None), "GET", OWNER, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
}

#[tokio::test]
async fn the_home_sections_are_saved_and_read_back_in_order() {
    let router = demo();
    let (_, json) = send(&router, "GET", OWNER, None).await;
    assert_eq!(json["settings"]["home_sections"], json!([]), "{json}");
    let sections = json!([
        {"kind": "featured"},
        {"kind": "category", "slug": "technical-seo"},
        {"kind": "newsletter"},
        {"kind": "series", "slug": "link-building-101"},
        {"kind": "latest"},
    ]);
    let change = json!({"settings": {"home_sections": sections, "updated_at": LAST_CHANGE}});
    let (status, json) = send(&router, "PUT", OWNER, Some(change)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["settings"]["home_sections"], sections, "{json}");
    let unknown = json!({"settings": {
        "home_sections": [{"kind": "author", "slug": "asif"}],
        "updated_at": "2026-10-27T09:00:00Z",
    }});
    let (status, json) = send(&router, "PUT", OWNER, Some(unknown)).await;
    assert!(status.is_client_error(), "{status}: {json}");
}

/// The admin types its settings screens from the document, so the answer
/// it documents must be the one the routes send: one `AdminSettings`, not
/// the Content API's list of the same name.
#[tokio::test]
async fn the_document_names_the_settings_the_routes_send() {
    let request = Request::get("/api/openapi.json").body(Body::empty());
    let response = demo().oneshot(request.unwrap()).await.unwrap();
    let bytes = to_bytes(response.into_body(), 4 * 1024 * 1024)
        .await
        .unwrap();
    let document: Value = serde_json::from_slice(&bytes).unwrap();
    let path = &document["paths"][SETTINGS];
    for method in ["get", "put"] {
        let answer = &path[method]["responses"]["200"]["content"]["application/json"]["schema"];
        let name = answer["$ref"].as_str().unwrap_or_default();
        let wrapper = name.trim_start_matches("#/components/schemas/");
        let settings = &document["components"]["schemas"][wrapper]["properties"]["settings"];
        assert_eq!(
            settings["$ref"], "#/components/schemas/AdminSettings",
            "{method} answers {wrapper}: {settings}"
        );
    }
}

#[tokio::test]
async fn a_menu_sent_replaces_that_menu_and_leaves_the_other() {
    let router = demo();
    let (_, json) = send(&router, "GET", OWNER, None).await;
    let empty = json!({"primary": [], "footer": []});
    assert_eq!(json["settings"]["nav"], empty, "{json}");
    let primary = json!([
        {"label": "Guides", "href": "/blog/guides/"},
        {"label": "Impressum", "href": "/blog/de/impressum/", "language": "de"},
    ]);
    let change = json!({"settings": {"nav": {"primary": primary}, "updated_at": LAST_CHANGE}});
    let (status, json) = send(&router, "PUT", OWNER, Some(change)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let saved = json!({"primary": primary, "footer": []});
    assert_eq!(json["settings"]["nav"], saved, "{json}");
    let unknown = json!({"settings": {
        "nav": {"sidebar": []},
        "updated_at": "2026-10-27T09:00:00Z",
    }});
    let (status, json) = send(&router, "PUT", OWNER, Some(unknown)).await;
    assert!(status.is_client_error(), "{status}: {json}");
}
