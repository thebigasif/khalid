//! Search Console follow-ups through the Admin API (GEO M3): the long
//! searches that showed each post, and the heading that answers each,
//! the report, each post's month for the posts table, and the site's
//! totals. Needs `settings:write`.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::NaiveDate;
use khalid_api::api;
use khalid_api::content_store::BoxFuture;
use khalid_api::follow_ups::{FollowUp, FollowUpError, FollowUpFinder, FollowUpPost, FollowUps};
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::search_report::{
    FallingPage, LostPage, LostReason, Mover, PostNumbers, SearchNumbers, SearchPosts,
    SearchReport, SearchTotals,
};
use khalid_core::UserMessage;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const SETTINGS: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const POSTS_ONLY: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

const FOLLOW_UPS: &str = "/api/admin/v1/search-console/follow-ups/";
const REPORT: &str = "/api/admin/v1/search-console/report/";
const TOTALS: &str = "/api/admin/v1/search-console/totals/";
const POSTS: &str = "/api/admin/v1/search-console/posts/";

/// What the finder in a test answers.
#[derive(Clone, Copy)]
enum Answer {
    Found,
    NotConnected,
    GoogleRefused,
}

struct Finder(Answer);

impl FollowUpFinder for Finder {
    fn follow_ups(&self) -> BoxFuture<'_, Result<FollowUps, FollowUpError>> {
        let answer = self.answered(found);
        Box::pin(async move { answer })
    }

    fn report(&self) -> BoxFuture<'_, Result<SearchReport, FollowUpError>> {
        let answer = match self.0 {
            Answer::Found => Ok(report()),
            _ => Err(FollowUpError::Google(UserMessage::new(
                "Google turned the request down (503).",
                "Google does this when it is busy.",
                "Wait a few minutes, then try again.",
            ))),
        };
        Box::pin(async move { answer })
    }

    fn totals(&self, days: u32) -> BoxFuture<'_, Result<SearchTotals, FollowUpError>> {
        let answer = self.answered(|| totals(days));
        Box::pin(async move { answer })
    }

    fn posts(&self) -> BoxFuture<'_, Result<SearchPosts, FollowUpError>> {
        let answer = self.answered(|| {
            let SearchReport {
                property,
                first,
                last,
                posts,
                ..
            } = report();
            SearchPosts {
                property,
                first,
                last,
                posts,
            }
        });
        Box::pin(async move { answer })
    }
}

impl Finder {
    /// What `found` makes, or the refusal the test asked for.
    fn answered<T>(&self, found: impl FnOnce() -> T) -> Result<T, FollowUpError> {
        match self.0 {
            Answer::Found => Ok(found()),
            Answer::NotConnected => Err(FollowUpError::NotConnected(UserMessage::new(
                "This site is not connected to Google Search Console.",
                "Khalid asks Google nothing until the owner opts in.",
                "Follow the steps in docs/deploy.md.",
            ))),
            Answer::GoogleRefused => Err(FollowUpError::Google(UserMessage::new(
                "Google's quota for these requests is spent for now.",
                "Google allows a set number of requests a day.",
                "Wait until tomorrow.",
            ))),
        }
    }
}

/// The site's totals for `days` days, the days before shown nothing.
#[cfg(test)]
fn totals(days: u32) -> SearchTotals {
    let none = SearchNumbers {
        clicks: 0.0,
        impressions: 0.0,
        ctr: 0.0,
        position: None,
    };
    SearchTotals {
        property: "sc-domain:example.com".to_owned(),
        days,
        first: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
        last: NaiveDate::from_ymd_opt(2026, 10, 28).unwrap(),
        now: SearchNumbers {
            clicks: 40.0,
            impressions: 400.0,
            ctr: 0.1,
            position: Some(4.0),
        },
        before: none,
    }
}

/// Two posts' numbers, one falling and one missing page, and a loser.
#[cfg(test)]
fn report() -> SearchReport {
    let sitemap = "https://example.com/blog/what-a-sitemap-does/";
    let post = |id: &str, url: &str, clicks: f64, position: Option<f64>| PostNumbers {
        id: id.to_owned(),
        title: id.to_owned(),
        url: url.to_owned(),
        clicks,
        impressions: clicks * 20.0,
        ctr: 0.05,
        position,
    };
    SearchReport {
        property: "sc-domain:example.com".to_owned(),
        first: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
        last: NaiveDate::from_ymd_opt(2026, 10, 28).unwrap(),
        posts: vec![
            post("en.what-a-sitemap-does", sitemap, 40.0, Some(3.1)),
            post("en.unseen", "https://example.com/blog/unseen/", 0.0, None),
        ],
        decaying: vec![FallingPage {
            id: Some("en.what-a-sitemap-does".to_owned()),
            title: "What a sitemap does".to_owned(),
            url: sitemap.to_owned(),
            clicks: vec![90.0, 70.0, 50.0, 40.0],
            action: "Refresh this post".to_owned(),
            go_deeper: "https://app.serpdad.com/go-deeper".to_owned(),
        }],
        lost: vec![LostPage {
            url: "https://example.com/blog/gone/".to_owned(),
            clicks: 3.0,
            impressions: 60.0,
            why: LostReason::Missing,
            not_found_hits: Some(12),
        }],
        week_clicks: 8.0,
        last_week_clicks: 12.0,
        winners: Vec::new(),
        losers: vec![Mover {
            id: Some("en.what-a-sitemap-does".to_owned()),
            title: "What a sitemap does".to_owned(),
            url: sitemap.to_owned(),
            before: 12.0,
            now: 8.0,
            go_deeper: None,
        }],
    }
}

/// One post with a long search a heading answers and one none does.
#[cfg(test)]
fn found() -> FollowUps {
    let search = |query: &str, answered_by: Option<&str>| FollowUp {
        query: query.to_owned(),
        clicks: 2.0,
        impressions: 40.0,
        position: 7.5,
        answered_by: answered_by.map(str::to_owned),
    };
    FollowUps {
        property: "sc-domain:example.com".to_owned(),
        first: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
        last: NaiveDate::from_ymd_opt(2026, 10, 28).unwrap(),
        words: 8,
        posts: vec![FollowUpPost {
            id: "en.what-a-sitemap-does".to_owned(),
            title: "What a sitemap does".to_owned(),
            url: "https://example.com/blog/what-a-sitemap-does/".to_owned(),
            searches: vec![
                search(
                    "how often does google read the sitemap of a small blog",
                    Some("How often Google reads a sitemap"),
                ),
                search("do i need a sitemap if my blog has only ten posts", None),
            ],
        }],
    }
}

#[cfg(test)]
fn router(answer: Answer) -> axum::Router {
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
    let content = Arc::new(common::store());
    let finder = Arc::new(Finder(answer));
    let (router, _) = api::router_with_follow_ups(content, Arc::new(door), Arc::new(keys), finder);
    router
}

/// GETs the follow-ups with `key`; answers the status and the body as JSON.
#[cfg(test)]
async fn get(answer: Answer, key: &str) -> (StatusCode, Value) {
    get_at(FOLLOW_UPS, answer, key).await
}

/// GETs `route` with `key`; answers the status and the body as JSON.
#[cfg(test)]
async fn get_at(route: &str, answer: Answer, key: &str) -> (StatusCode, Value) {
    let request = Request::get(route)
        .header("authorization", format!("Khalid {key}"))
        .body(Body::empty())
        .unwrap();
    let response = router(answer).oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn the_long_searches_come_with_the_heading_that_answers_each() {
    let (status, json) = get(Answer::Found, SETTINGS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let search = |query: &str, answered_by: Value| {
        json!({
            "query": query, "clicks": 2.0, "impressions": 40.0, "position": 7.5,
            "answered_by": answered_by,
        })
    };
    assert_eq!(
        json,
        json!({
            "property": "sc-domain:example.com",
            "first": "2026-10-01", "last": "2026-10-28", "words": 8,
            "posts": [{
                "id": "en.what-a-sitemap-does",
                "title": "What a sitemap does",
                "url": "https://example.com/blog/what-a-sitemap-does/",
                "searches": [
                    search(
                        "how often does google read the sitemap of a small blog",
                        json!("How often Google reads a sitemap"),
                    ),
                    search("do i need a sitemap if my blog has only ten posts", Value::Null),
                ],
            }],
        })
    );
}

#[tokio::test]
async fn a_site_not_connected_answers_409_with_what_to_do() {
    let (status, json) = get(Answer::NotConnected, SETTINGS).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert_eq!(json["type"], "https://khalid.dev/problems/not-set-up");
    assert_eq!(
        json["detail"],
        "This site is not connected to Google Search Console. \
         Khalid asks Google nothing until the owner opts in."
    );
    assert_eq!(json["fix"], "Follow the steps in docs/deploy.md.");
}

#[tokio::test]
async fn google_refusing_answers_502_in_plain_words() {
    let (status, json) = get(Answer::GoogleRefused, SETTINGS).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY, "{json}");
    assert_eq!(json["type"], "https://khalid.dev/problems/upstream");
    assert_eq!(json["fix"], "Wait until tomorrow.");
}

#[tokio::test]
async fn a_key_without_settings_write_is_refused() {
    let (status, json) = get(Answer::Found, POSTS_ONLY).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
}

#[tokio::test]
async fn the_document_lists_the_route() {
    let request = Request::get("/api/openapi.json").body(Body::empty());
    let response = router(Answer::Found).oneshot(request.unwrap()).await;
    let bytes = to_bytes(response.unwrap().into_body(), 4 * 1024 * 1024).await;
    let served: Value = serde_json::from_slice(&bytes.unwrap()).unwrap();
    assert!(served["paths"][FOLLOW_UPS]["get"].is_object());
    assert!(served["paths"][REPORT]["get"].is_object());
    assert!(served["paths"][POSTS]["get"].is_object());
    let days = &served["paths"][TOTALS]["get"]["parameters"][0];
    assert_eq!(
        (&days["name"], &days["in"]),
        (&json!("days"), &json!("query"))
    );
}

#[tokio::test]
async fn the_totals_add_up_the_site_for_the_days_asked_and_the_days_before() {
    let (status, json) = get_at(&format!("{TOTALS}?days=7"), Answer::Found, SETTINGS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(
        json,
        json!({
            "property": "sc-domain:example.com", "days": 7,
            "first": "2026-10-01", "last": "2026-10-28",
            "now": { "clicks": 40.0, "impressions": 400.0, "ctr": 0.1, "position": 4.0 },
            "before": { "clicks": 0.0, "impressions": 0.0, "ctr": 0.0, "position": null },
        })
    );
    let (status, json) = get_at(TOTALS, Answer::Found, SETTINGS).await;
    assert_eq!((status, &json["days"]), (StatusCode::OK, &json!(28)));
}

#[tokio::test]
async fn the_totals_refuse_days_outside_1_to_90_in_plain_words() {
    for days in ["0", "91", "a-month", "-3"] {
        let route = format!("{TOTALS}?days={days}");
        let (status, json) = get_at(&route, Answer::Found, SETTINGS).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{days}: {json}");
        assert_eq!(
            json["detail"],
            "The days are not a number from 1 to 90. \
            The Analytics screen shows at most 90 days, against the 90 days before."
        );
        assert_eq!(json["fix"], "Ask for days=28, or leave it out.");
    }
    let (status, _) = get_at(&format!("{TOTALS}?days=90"), Answer::Found, SETTINGS).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn the_totals_say_how_to_connect_or_why_google_failed_and_need_settings_write() {
    let (status, json) = get_at(TOTALS, Answer::NotConnected, SETTINGS).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert_eq!(json["type"], "https://khalid.dev/problems/not-set-up");
    let (status, json) = get_at(TOTALS, Answer::GoogleRefused, SETTINGS).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY, "{json}");
    assert_eq!(json["fix"], "Wait until tomorrow.");
    let (status, _) = get_at(TOTALS, Answer::Found, POSTS_ONLY).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn the_report_names_each_post_by_its_identity_for_the_editor() {
    let (status, json) = get_at(REPORT, Answer::Found, SETTINGS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["last"], "2026-10-28");
    assert_eq!(json["posts"][0]["id"], "en.what-a-sitemap-does");
    assert_eq!(json["posts"][1]["position"], Value::Null);
    assert_eq!(
        json["decaying"][0]["clicks"],
        json!([90.0, 70.0, 50.0, 40.0])
    );
    assert_eq!(
        json["lost"][0],
        json!({ "url": "https://example.com/blog/gone/", "clicks": 3.0,
            "impressions": 60.0, "why": "missing", "not_found_hits": 12 })
    );
    assert_eq!(json["losers"][0]["id"], "en.what-a-sitemap-does");
    assert_eq!(
        (&json["week_clicks"], &json["last_week_clicks"]),
        (&json!(8.0), &json!(12.0))
    );
}

#[tokio::test]
async fn the_report_answers_502_when_google_refuses_and_needs_settings_write() {
    let (status, json) = get_at(REPORT, Answer::GoogleRefused, SETTINGS).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY, "{json}");
    assert_eq!(json["fix"], "Wait a few minutes, then try again.");
    let (status, _) = get_at(REPORT, Answer::Found, POSTS_ONLY).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn each_posts_month_comes_alone_for_the_posts_table() {
    let (status, json) = get_at(POSTS, Answer::Found, SETTINGS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["posts"], json!(report().posts), "{json}");
    assert_eq!(
        (&json["first"], &json["last"]),
        (&json!("2026-10-01"), &json!("2026-10-28"))
    );
    assert_eq!(json["week_clicks"], Value::Null, "only the month: {json}");
}

#[tokio::test]
async fn each_posts_month_says_how_to_connect_or_why_google_failed_and_needs_settings_write() {
    let (status, json) = get_at(POSTS, Answer::NotConnected, SETTINGS).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert_eq!(json["type"], "https://khalid.dev/problems/not-set-up");
    let (status, json) = get_at(POSTS, Answer::GoogleRefused, SETTINGS).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY, "{json}");
    assert_eq!(json["fix"], "Wait until tomorrow.");
    let (status, _) = get_at(POSTS, Answer::Found, POSTS_ONLY).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}
