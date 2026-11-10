//! The publishing planner through the Admin API: a suggested day for each
//! ready draft, for the admin's calendar (GEO task M4, KH-EDT-4).

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{DateTime, TimeZone, Utc};
use khalid_api::api;
use khalid_api::content_store::BoxFuture;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::plan_posts::{PlanPosts, PlannedPost};
use khalid_api::post_store::{MemoryPostStore, PostStoreError};
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::resources::{AdminPost, Gate};
use khalid_api::schedule_store::{MemoryScheduleStore, Schedule, ScheduleStore};
use khalid_core::{FixedClock, Slug};
use khalid_seo::ClusterRole;
use serde_json::{Value, json};
use tower::ServiceExt;

/// A key made up for these tests. It opens nothing real.
const WRITER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

/// Posts made up front, as the planner reads them.
struct Fixed(Vec<PlannedPost>);

impl PlanPosts for Fixed {
    fn planned(&self) -> BoxFuture<'_, Result<Vec<PlannedPost>, PostStoreError>> {
        Box::pin(std::future::ready(Ok(self.0.clone())))
    }
}

#[cfg(test)]
fn at(month: u32, day: u32, hour: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, month, day, hour, 0, 0).unwrap()
}

/// One post went live on 20 October; a pillar and its member are ready;
/// one draft is blocked; one is scheduled for 3 November.
#[cfg(test)]
fn site() -> Fixed {
    let post = |slug: &str| {
        let (id, made) = (format!("en.{slug}"), Slug::try_new(slug).unwrap());
        AdminPost::new(&id, made, slug, "Text.", at(10, 1, 9))
    };
    let topic = Slug::try_new("links").unwrap();
    Fixed(vec![
        PlannedPost::new(post("old").published(at(10, 20, 0)), Gate::Passes),
        PlannedPost::new(post("member"), Gate::Passes)
            .in_cluster(topic.clone(), ClusterRole::Member),
        PlannedPost::new(post("pillar"), Gate::Passes).in_cluster(topic, ClusterRole::Pillar),
        PlannedPost::new(post("blocked"), Gate::Refuses),
        PlannedPost::new(post("timed"), Gate::Passes),
    ])
}

/// The door on 31 October 2026, with `en.timed` scheduled.
#[cfg(test)]
async fn door() -> PublishDoor {
    let schedules = Arc::new(MemoryScheduleStore::default());
    let timed = Schedule::default().publishing_at(at(11, 3, 12));
    schedules.set("en.timed", timed).await.unwrap();
    PublishDoor::new(Arc::new(MemoryPostStore::new(Vec::new())), Arc::new(Silent))
        .with_schedules(schedules, FixedClock::new(at(10, 31, 9)))
}

/// The plan of [`site`] as `query` asks for it.
#[cfg(test)]
async fn plan(query: &str) -> (StatusCode, Value) {
    ask(door().await.with_planner(Arc::new(site())), query).await
}

#[cfg(test)]
async fn ask(door: PublishDoor, query: &str) -> (StatusCode, Value) {
    let token: KeyToken = WRITER.parse().unwrap();
    let hashing = KeyHashing::fast_for_tests();
    let key = StoredKey::for_integration(&token, vec![Scope::PostsWrite], &hashing).unwrap();
    let keys = Arc::new(MemoryKeyStore::new(vec![key]));
    let (router, _) = api::router_with_door(Arc::new(common::store()), Arc::new(door), keys);
    let request = Request::get(format!("/api/admin/v1/calendar/plan/{query}"))
        .header("authorization", format!("Khalid {WRITER}"))
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn each_ready_draft_gets_a_day_and_the_pillar_goes_first() {
    let (status, json) = plan("?per_week=7").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let want = json!({
        "per_week": 7,
        "pace_from": "asked",
        "calendar": [
            {"day": "2026-11-01", "id": "en.pillar", "title": "pillar", "kind": "suggested"},
            {"day": "2026-11-02", "id": "en.member", "title": "member", "kind": "suggested"},
            {"day": "2026-11-03", "id": "en.timed", "title": "timed", "kind": "scheduled"},
        ],
        "waiting": [{"id": "en.blocked", "title": "blocked", "why": "blocked", "pillar": null}],
        "warnings": [],
    });
    assert_eq!(json["plan"], want, "{json}");
}

#[tokio::test]
async fn with_no_pace_asked_the_site_keeps_its_own() {
    let (status, json) = plan("").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["plan"]["per_week"], 1, "{json}");
    assert_eq!(json["plan"]["pace_from"], "history", "{json}");
    let days: Vec<&str> = json["plan"]["calendar"]
        .as_array()
        .unwrap()
        .iter()
        .map(|slot| slot["day"].as_str().unwrap())
        .collect();
    assert_eq!(days, ["2026-11-01", "2026-11-03", "2026-11-08"], "{json}");
}

#[tokio::test]
async fn a_pace_outside_one_to_fourteen_is_refused() {
    for query in ["?per_week=0", "?per_week=15", "?per_week=two"] {
        let (status, json) = plan(query).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{query}: {json}");
        assert!(json["fix"].as_str().unwrap().contains("per_week"), "{json}");
    }
}

#[tokio::test]
async fn a_server_with_no_planner_says_so() {
    let (status, json) = ask(door().await, "").await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
    assert!(json["detail"].as_str().unwrap().contains("plan"), "{json}");
}
