//! Rate limits on both APIs (RULES.md BL-SEC-5): a busy key, and an
//! address that keeps sending wrong keys, get a 429 problem with
//! `Retry-After`.

mod common;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode, header};
use khalid_api::api::{self, AdminDesks};
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_core::LimitSettings;
use serde_json::Value;
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const CONTENT: &str =
    "kc_0123456789abcdef_00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
/// The content key's id with another secret of the same length.
const WRONG_CONTENT: &str =
    "kc_0123456789abcdef_ffeeddccbbaa99887766554433221100ffeeddccbbaa99887766554433221100";
/// The integration key's id with another secret of the same length.
const WRONG_WRITER: &str =
    "ki_1111222233334444_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

const POSTS: &str = "/api/content/v1/posts/?limit=1";
const DRAFTS: &str = "/api/admin/v1/posts/";

#[cfg(test)]
fn usual() -> LimitSettings {
    LimitSettings::default()
}

#[cfg(test)]
fn router() -> Router {
    router_with(usual()).0
}

/// The router of a site whose `[limits]` are `limits`, and its document.
#[cfg(test)]
fn router_with(limits: LimitSettings) -> (Router, Value) {
    let hashing = KeyHashing::fast_for_tests();
    let token = |text: &str| text.parse::<KeyToken>().unwrap();
    let keys = MemoryKeyStore::new(vec![
        StoredKey::for_integration(&token(WRITER), vec![Scope::PostsWrite], &hashing).unwrap(),
        StoredKey::for_token(&token(CONTENT)),
    ]);
    let posts = MemoryPostStore::new(Vec::new());
    let door = PublishDoor::new(Arc::new(posts), Arc::new(Silent));
    let desks = AdminDesks {
        limits,
        ..AdminDesks::none()
    };
    let content = Arc::new(common::store());
    let (router, document) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    (router, serde_json::to_value(document).unwrap())
}

/// Sends `GET uri` from `peer` with `Authorization: authorization`, and
/// returns the status, the `Retry-After` header and the body.
#[cfg(test)]
async fn send(
    router: &Router,
    uri: &str,
    peer: &str,
    authorization: Option<&str>,
) -> (StatusCode, Option<String>, Value) {
    let mut request = Request::get(uri);
    if let Some(value) = authorization {
        request = request.header(header::AUTHORIZATION, value);
    }
    let mut request = request.body(Body::empty()).unwrap();
    let peer: SocketAddr = peer.parse().unwrap();
    request.extensions_mut().insert(ConnectInfo(peer));
    let response = router.clone().oneshot(request).await.unwrap();
    let (parts, body) = response.into_parts();
    let retry_after = parts
        .headers
        .get(header::RETRY_AFTER)
        .map(|value| value.to_str().unwrap().to_owned());
    let body = to_bytes(body, 1024 * 1024).await.unwrap();
    let json = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (parts.status, retry_after, json)
}

#[cfg(test)]
fn content(key: &str) -> String {
    format!("Khalid-Content {key}")
}

#[cfg(test)]
fn admin(key: &str) -> String {
    format!("Khalid {key}")
}

#[tokio::test]
async fn an_address_that_sends_wrong_content_keys_is_paused_with_a_429() {
    let router = router();
    let peer = "203.0.113.7:5000";
    for _ in 0..usual().wrong_keys_per_minute {
        let (status, ..) = send(&router, POSTS, peer, Some(&content(WRONG_CONTENT))).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    let (status, retry_after, json) = send(&router, POSTS, peer, Some(&content(CONTENT))).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS, "{json}");
    let seconds: u64 = retry_after.unwrap().parse().unwrap();
    assert!((1..=60).contains(&seconds), "{seconds}");
    assert_eq!(
        json["type"],
        "https://khalid.dev/problems/too-many-requests"
    );
    assert_eq!(json["status"], 429);
    assert!(json["fix"].as_str().unwrap().contains("seconds"), "{json}");

    let (status, ..) = send(&router, POSTS, "198.51.100.1:5000", Some(&content(CONTENT))).await;
    assert_eq!(status, StatusCode::OK, "another address is not paused");
}

#[tokio::test]
async fn an_address_that_sends_wrong_admin_keys_is_paused_with_a_429() {
    let router = router();
    let peer = "[2001:db8::1]:5000";
    for _ in 0..usual().wrong_keys_per_minute {
        let (status, ..) = send(&router, DRAFTS, peer, Some(&admin(WRONG_WRITER))).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    // Another address in the same /64 network counts as the same one.
    let (status, retry_after, _) =
        send(&router, DRAFTS, "[2001:db8::2]:5000", Some(&admin(WRITER))).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert!(retry_after.is_some());
}

#[tokio::test]
async fn a_request_with_no_key_is_never_counted() {
    let router = router();
    let peer = "203.0.113.8:5000";
    for _ in 0..=usual().wrong_keys_per_minute {
        let (status, ..) = send(&router, POSTS, peer, None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    let (status, ..) = send(&router, POSTS, peer, Some(&content(CONTENT))).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn a_busy_admin_key_gets_a_429_with_retry_after() {
    let router = router();
    let peer = "203.0.113.9:5000";
    let mut limited = None;
    // Twice the limit: the key earns back at most one request in each
    // tenth of a second, far fewer than this loop sends.
    for _ in 0..2 * usual().admin_per_minute {
        let answer = send(&router, DRAFTS, peer, Some(&admin(WRITER))).await;
        if answer.0 == StatusCode::TOO_MANY_REQUESTS {
            limited = Some(answer);
            break;
        }
        assert_eq!(answer.0, StatusCode::OK, "{}", answer.2);
    }
    let (_, retry_after, json) = limited.expect("the key was never limited");
    assert!(retry_after.is_some());
    assert!(
        json["detail"].as_str().unwrap().contains("this key"),
        "{json}"
    );
}

#[tokio::test]
async fn a_busy_content_key_gets_a_429_with_retry_after() {
    let router = router();
    let peer = "203.0.113.10:5000";
    let mut limited = None;
    for _ in 0..2 * usual().content_per_minute {
        let answer = send(&router, POSTS, peer, Some(&content(CONTENT))).await;
        if answer.0 == StatusCode::TOO_MANY_REQUESTS {
            limited = Some(answer);
            break;
        }
    }
    let (_, retry_after, _) = limited.expect("the key was never limited");
    assert!(retry_after.is_some());
}

#[tokio::test]
async fn a_site_sets_its_own_limits_in_site_toml() {
    let mut own = usual();
    own.admin_per_minute = 3;
    own.wrong_keys_per_minute = 2;
    let (router, _) = router_with(own);
    let peer = "203.0.113.11:5000";
    for _ in 0..3 {
        let (status, ..) = send(&router, DRAFTS, peer, Some(&admin(WRITER))).await;
        assert_eq!(status, StatusCode::OK);
    }
    let (status, retry_after, _) = send(&router, DRAFTS, peer, Some(&admin(WRITER))).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(retry_after.as_deref(), Some("20"));

    let other = "203.0.113.12:5000";
    for _ in 0..2 {
        let (status, ..) = send(&router, POSTS, other, Some(&content(WRONG_CONTENT))).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    let (status, ..) = send(&router, POSTS, other, Some(&content(CONTENT))).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
}

#[test]
fn every_operation_behind_a_key_lists_the_429() {
    let (_, document) = router_with(usual());
    let shared = &document["components"]["responses"]["TooManyRequests"];
    assert!(shared["headers"]["Retry-After"].is_object(), "{shared}");
    let problem = &shared["content"]["application/problem+json"]["schema"]["$ref"];
    assert_eq!(problem, "#/components/schemas/Problem");
    let mut keyed = 0;
    for (path, item) in document["paths"].as_object().unwrap() {
        for (method, operation) in item.as_object().unwrap() {
            let Some(security) = operation.get("security") else {
                continue;
            };
            if security.as_array().is_none_or(Vec::is_empty) {
                continue;
            }
            keyed += 1;
            let answer = &operation["responses"]["429"];
            assert!(answer.is_object(), "{method} {path} lists no 429");
        }
    }
    assert!(keyed > 100, "only {keyed} operations need a key");
    let search = &document["paths"]["/api/admin/v1/media/search/"]["get"]["responses"]["429"];
    assert!(
        search["description"].as_str().is_some(),
        "an operation's own 429 is kept: {search}"
    );
}
