//! Bots per AI tool through the Admin API: how often each assistant's bots
//! read the site, beside the readers it sent. Needs `settings:write`.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use khalid_analytics::{AiTool, CrawlerKind, Crawls, ReferrerGroup, Views};
use khalid_api::analytics_desk::{AnalyticsDesk, ViewsInMemory};
use khalid_api::api;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_core::FixedClock;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const SETTINGS: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const POSTS_ONLY: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

const POST: &str = "/blog/internal-links/";

#[cfg(test)]
fn at(day: u32, hour: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, day, hour, 0, 0).unwrap()
}

#[cfg(test)]
fn read(day: u32, path: &str, crawler: &str, kind: CrawlerKind) -> Crawls {
    Crawls {
        day: at(day, 0).date_naive(),
        path: path.to_owned(),
        crawler: crawler.to_owned(),
        kind,
        hits: 2,
        last_seen: at(day, 9),
        last_status: 200,
    }
}

/// The API on 28 October. The bots of two assistants read this week, and
/// so did `CCBot`, which serves none; one assistant sent three readers.
/// Googlebot is not an AI bot.
#[cfg(test)]
fn router() -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(SETTINGS, Scope::SettingsWrite),
        stored(POSTS_ONLY, Scope::PostsWrite),
    ]);
    let day = NaiveDate::from_ymd_opt(2026, 10, 27).unwrap();
    let views = vec![Views::new(day, POST, ReferrerGroup::Ai(AiTool::ChatGpt), 3)];
    let crawls = vec![
        read(26, POST, "GPTBot", CrawlerKind::Ai),
        read(27, POST, "GPTBot", CrawlerKind::Ai),
        read(27, "/blog/sitemaps/", "OAI-SearchBot", CrawlerKind::Ai),
        read(27, POST, "ClaudeBot", CrawlerKind::Ai),
        read(27, POST, "CCBot", CrawlerKind::Ai),
        read(27, POST, "Googlebot", CrawlerKind::Search),
    ];
    let store = ViewsInMemory::new(views).with_crawls(crawls);
    let now = at(28, 15);
    let analytics = AnalyticsDesk::new(Arc::new(store), Arc::new(FixedClock::new(now)));
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(common::store());
    let (router, _) =
        api::router_with_analytics(content, Arc::new(door), Arc::new(keys), analytics);
    router
}

/// GETs `path` under the Admin API with `key`; answers the status and the
/// body as JSON.
#[cfg(test)]
async fn get(key: &str, path: &str) -> (StatusCode, Value) {
    let request = Request::get(format!("/api/admin/v1{path}"))
        .header("authorization", format!("Khalid {key}"))
        .body(Body::empty())
        .unwrap();
    let response = router().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn each_assistants_bots_are_listed_beside_the_readers_it_sent() {
    let (status, json) = get(SETTINGS, "/analytics/ai-bots/?days=7").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(
        json,
        json!({
            "first": "2026-10-22", "last": "2026-10-28", "days": 7,
            "tools": [
                {
                    "tool": "chatgpt", "name": "ChatGPT", "reads": 6, "pages": 2, "sent": 3,
                    "bots": [{"bot": "GPTBot", "reads": 4}, {"bot": "OAI-SearchBot", "reads": 2}],
                },
                {
                    "tool": "claude", "name": "Claude", "reads": 2, "pages": 1, "sent": 0,
                    "bots": [{"bot": "ClaudeBot", "reads": 2}],
                },
                {
                    "tool": "other", "name": "AI bots of no assistant named here",
                    "reads": 2, "pages": 1, "sent": 0,
                    "bots": [{"bot": "CCBot", "reads": 2}],
                },
            ],
        })
    );
}

#[tokio::test]
async fn reads_before_the_period_are_left_out_and_thirty_days_is_the_default() {
    let (_, today) = get(SETTINGS, "/analytics/ai-bots/?days=1").await;
    assert_eq!(today["tools"], json!([]));
    let (status, json) = get(SETTINGS, "/analytics/ai-bots/").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(
        (json["days"].as_u64(), json["first"].as_str()),
        (Some(30), Some("2026-09-29"))
    );
}

#[tokio::test]
async fn days_past_what_the_crawl_log_keeps_are_refused_in_plain_words() {
    for days in ["0", "91", "a-month"] {
        let (status, json) = get(SETTINGS, &format!("/analytics/ai-bots/?days={days}")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{days}: {json}");
        let detail = json["detail"].as_str().unwrap_or_default();
        assert!(
            detail.starts_with("The days are not a number from 1 to 90."),
            "{json}"
        );
    }
    let (status, _) = get(POSTS_ONLY, "/analytics/ai-bots/").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn the_document_lists_the_route() {
    let request = Request::get("/api/openapi.json").body(Body::empty());
    let response = router().oneshot(request.unwrap()).await;
    let bytes = to_bytes(response.unwrap().into_body(), 4 * 1024 * 1024).await;
    let served: Value = serde_json::from_slice(&bytes.unwrap()).unwrap();
    assert!(served["paths"]["/api/admin/v1/analytics/ai-bots/"]["get"].is_object());
}
