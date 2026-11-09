//! The consent proof through the Admin API (privacy U11, R18): every
//! version of what readers were shown with its days, and how many answers
//! allowed each optional category per day. Needs `settings:write`.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{NaiveDate, TimeZone, Utc};
use khalid_api::analytics_desk::{AnalyticsDesk, ViewsInMemory};
use khalid_api::api;
use khalid_api::consent_proof::{DayChoices, MemoryConsentProof, ShownVersion};
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_core::{ConsentCategory, FixedClock};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const SETTINGS: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const POSTS_ONLY: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

/// Two versions: the first wording, then a new one from 26 October.
const FIRST: &str = "0123456789abcdef";
const SECOND: &str = "fedcba9876543210";

#[cfg(test)]
fn date(day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 10, day).unwrap()
}

/// `choices` answers on `day` under `version`: `analytics` allowed
/// Analytics, one allowed External media, and none allowed the rest.
#[cfg(test)]
fn answers(day: u32, version: &str, choices: u64, analytics: u64) -> DayChoices {
    DayChoices::new(date(day), version, choices, |category| match category {
        ConsentCategory::Analytics => analytics,
        ConsentCategory::ExternalMedia => 1,
        _ => 0,
    })
}

/// The API on 28 October, with `proof` read through its analytics desk.
#[cfg(test)]
fn router(proof: Option<MemoryConsentProof>) -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(SETTINGS, Scope::SettingsWrite),
        stored(POSTS_ONLY, Scope::PostsWrite),
    ]);
    let now = Utc.with_ymd_and_hms(2026, 10, 28, 15, 0, 0).unwrap();
    let views = Arc::new(ViewsInMemory::default());
    let mut analytics = AnalyticsDesk::new(views, Arc::new(FixedClock::new(now)));
    if let Some(proof) = proof {
        analytics = analytics.with_consent(Arc::new(proof));
    }
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(common::store());
    let (router, _) =
        api::router_with_analytics(content, Arc::new(door), Arc::new(keys), analytics);
    router
}

/// The proof of a site that changed its wording on 26 October.
#[cfg(test)]
fn proof() -> MemoryConsentProof {
    let versions = vec![
        ShownVersion::new(FIRST, r#"{"lifetime":6}"#, date(1), date(25)),
        ShownVersion::new(SECOND, r#"{"lifetime":6,"text":"new"}"#, date(26), date(28)),
    ];
    let counts = vec![
        answers(20, FIRST, 9, 9),
        answers(25, FIRST, 2, 1),
        answers(26, SECOND, 3, 2),
        answers(28, SECOND, 4, 0),
    ];
    MemoryConsentProof::new(versions, counts)
}

/// GETs `path` under the Admin API with `key`; answers the status and the
/// body as JSON.
#[cfg(test)]
async fn get(proof: Option<MemoryConsentProof>, key: &str, path: &str) -> (StatusCode, Value) {
    let request = Request::get(format!("/api/admin/v1{path}"))
        .header("authorization", format!("Khalid {key}"))
        .body(Body::empty())
        .unwrap();
    let response = router(proof).oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// The counts of one day or period, as the API writes them.
#[cfg(test)]
fn tally(choices: u64, analytics: u64, media: u64) -> Value {
    json!({"choices": choices, "preferences": 0, "analytics": analytics,
        "marketing": 0, "external_media": media})
}

#[tokio::test]
async fn the_counts_route_totals_each_category_for_a_range() {
    let (status, json) = get(Some(proof()), SETTINGS, "/consent/counts/?days=4").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(
        json,
        json!({
            "first": "2026-10-25", "last": "2026-10-28", "days": 4,
            "totals": tally(9, 3, 3),
            "per_day": [
                {"day": "2026-10-25", "version": FIRST, "counts": tally(2, 1, 1)},
                {"day": "2026-10-26", "version": SECOND, "counts": tally(3, 2, 1)},
                {"day": "2026-10-28", "version": SECOND, "counts": tally(4, 0, 1)},
            ],
        })
    );
}

#[tokio::test]
async fn the_versions_route_lists_every_version_with_its_days() {
    let (status, json) = get(Some(proof()), SETTINGS, "/consent/versions/").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(
        json,
        json!({"versions": [
            {"version": FIRST, "first_day": "2026-10-01", "last_day": "2026-10-25",
                "shown": {"lifetime": 6}},
            {"version": SECOND, "first_day": "2026-10-26", "last_day": "2026-10-28",
                "shown": {"lifetime": 6, "text": "new"}},
        ]})
    );
}

#[tokio::test]
async fn a_site_with_no_proof_has_no_versions_and_counts_nothing() {
    let (_, versions) = get(None, SETTINGS, "/consent/versions/").await;
    assert_eq!(versions, json!({"versions": []}));
    let (status, counts) = get(None, SETTINGS, "/consent/counts/").await;
    assert_eq!(status, StatusCode::OK, "{counts}");
    assert_eq!(counts["days"], 28);
    assert_eq!(counts["totals"], tally(0, 0, 0));
    assert_eq!(counts["per_day"], json!([]));
}

#[tokio::test]
async fn a_range_past_a_year_is_refused() {
    let (status, json) = get(Some(proof()), SETTINGS, "/consent/counts/?days=367").await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["rule"], "days");
}

#[tokio::test]
async fn a_key_without_settings_write_reads_no_proof() {
    for path in ["/consent/versions/", "/consent/counts/"] {
        let (status, _) = get(Some(proof()), POSTS_ONLY, path).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{path}");
    }
}
