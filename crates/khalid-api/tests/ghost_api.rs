//! The Ghost-compatible Content API's door (R15, KTD13): Ghost's key, its
//! versions, its error shape, the cache headers, and `/settings/`.

mod common;
mod ghost;

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ghost::{KEY, error, get, send};
use khalid_api::content_api;
use khalid_api::keys::{KeyToken, MemoryKeyStore, StoredKey};
use tower::ServiceExt as _;

/// The settings with the key, as Ghost's client asks for them.
const SETTINGS: &str = "/ghost/api/content/settings/";

#[tokio::test]
async fn the_settings_come_from_site_toml_as_one_object() {
    let (status, json) = get("settings/").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let settings = &json["settings"];
    assert_eq!(settings["title"], "Field Notes");
    assert_eq!(settings["description"], "Plain notes on search.");
    assert_eq!(settings["logo"], "https://example.com/logo.png");
    assert_eq!(settings["lang"], "en");
    assert_eq!(settings["timezone"], "Etc/UTC");
    assert_eq!(settings["url"], "https://example.com/blog/");
    assert_eq!(settings["navigation"][0]["label"], "About");
    assert_eq!(settings["navigation"][0]["url"], "/blog/about/");
    assert_eq!(
        settings["secondary_navigation"][0]["url"],
        "https://serpdad.com/"
    );
    assert!(settings["codeinjection_head"].is_null());
    assert_eq!(json["meta"], serde_json::json!({}));
}

#[tokio::test]
async fn a_missing_or_wrong_key_is_ghosts_401_and_never_echoes_the_key() {
    let wrong = KEY.replace('f', "e");
    let uris = [
        SETTINGS.to_owned(),
        format!("{SETTINGS}?key={wrong}"),
        format!("{SETTINGS}?key=0123456789abcdef0123456789"),
    ];
    for uri in uris {
        let (status, headers, json) = send(&uri, &[]).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{uri}");
        assert_eq!(headers["content-type"], "application/json", "{uri}");
        assert_eq!(
            error(&json),
            ("UnauthorizedError", "Authorization failed"),
            "{uri}"
        );
        let context = json["errors"][0]["context"].as_str().unwrap();
        assert!(context.contains("content key"), "{context}");
        assert!(!json.to_string().contains(KEY.get(20..).unwrap()), "{json}");
    }
}

#[tokio::test]
async fn khalids_own_key_header_does_not_open_the_ghost_path() {
    let header = format!("Khalid-Content {KEY}");
    let (status, _, _) = send(SETTINGS, &[("authorization", &header)]).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn version_five_is_accepted_here_and_named_in_the_answer() {
    let uri = format!("{SETTINGS}?key={KEY}");
    for version in ["v5.0", "v5.82", "v5"] {
        let (status, headers, _) = send(&uri, &[("accept-version", version)]).await;
        assert_eq!(status, StatusCode::OK, "{version}");
        assert_eq!(headers["content-version"], "v5.0", "{version}");
    }
    let (status, headers, _) = send(&uri, &[]).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["content-version"], "v5.0");
    for version in ["v6.0", "v4.0", "v1.0", "five"] {
        let (status, _, json) = send(&uri, &[("accept-version", version)]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{version}");
        let (kind, message) = error(&json);
        assert_eq!(kind, "BadRequestError", "{version}");
        assert!(message.contains("v5"), "{message}");
    }
}

#[tokio::test]
async fn khalids_own_content_api_still_refuses_version_five() {
    let token: KeyToken = KEY.parse().unwrap();
    let keys = MemoryKeyStore::new(vec![StoredKey::for_token(&token)]);
    let (router, _) = content_api::router(Arc::new(common::store()), Arc::new(keys));
    let request = Request::get(format!("/api/content/v1/posts/?key={KEY}"))
        .header("accept-version", "v5.0")
        .body(Body::empty())
        .unwrap();
    let answer = router.oneshot(request).await.unwrap();
    assert_eq!(answer.status(), StatusCode::NOT_ACCEPTABLE);
}

#[tokio::test]
async fn an_answer_is_cacheable_with_an_etag() {
    let uri = format!("{SETTINGS}?key={KEY}");
    let (status, headers, _) = send(&uri, &[]).await;
    assert_eq!(status, StatusCode::OK);
    let tag = headers["etag"].to_str().unwrap().to_owned();
    assert_eq!(
        headers["cache-control"],
        "public, max-age=0, must-revalidate"
    );
    let (status, headers, _) = send(&uri, &[("if-none-match", &tag)]).await;
    assert_eq!(status, StatusCode::NOT_MODIFIED);
    assert_eq!(headers["etag"], tag.as_str());
}

#[tokio::test]
async fn an_address_the_api_lacks_is_a_plain_404() {
    let (status, _, _) = send(&format!("/ghost/api/content/tiers/?key={KEY}"), &[]).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
