//! The `ai` route of the Admin API (Yoast package WP4): off until the
//! owner names a provider, ready only with the key that asks it, five
//! options checked against the length rules, words read from a saved post,
//! and `SerpDad` only while it is connected. Every request goes to a fake
//! writer; nothing leaves the test.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::ai::{AiDesk, AiTask, AiWriter, FakeWriter, Lengths};
use khalid_api::api::{self, AdminDesks};
use khalid_api::keys::{KeyHashing, KeyId, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::serpdad_store::{MemorySerpdadStore, SerpdadStore};
use khalid_core::AiProvider;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const WRITER: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";
const READER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

#[cfg(test)]
fn router(ai: AiDesk, serpdad: Option<Arc<dyn SerpdadStore>>) -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(WRITER, Scope::PostsWrite),
        stored(READER, Scope::ContentRead),
    ]);
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let desks = AdminDesks {
        ai,
        serpdad,
        ..AdminDesks::none()
    };
    let content = Arc::new(common::store());
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

#[cfg(test)]
fn desk(provider: AiProvider, writer: &Arc<FakeWriter>) -> AiDesk {
    let writer: Arc<FakeWriter> = Arc::clone(writer);
    let writer: Arc<dyn AiWriter> = writer;
    AiDesk::new(provider, Some(writer), Lengths::default())
}

#[cfg(test)]
async fn send(
    router: &axum::Router,
    key: &str,
    (method, path): (&str, &str),
    body: Option<&Value>,
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(format!("/api/admin/v1{path}"))
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json")
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .unwrap();
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

const STATE: (&str, &str) = ("GET", "/ai/");

#[tokio::test]
async fn off_by_default() {
    let router = router(AiDesk::off(), None);
    let (status, json) = send(&router, WRITER, STATE, None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let off = json!({"on": false, "provider": "off", "label": "Off", "ready": false});
    assert_eq!(json, off);

    let ask = json!({"task": "seo_title", "title": "Anchor text"});
    let (status, json) = send(&router, WRITER, SUGGEST, Some(&ask)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    let detail = json["detail"].as_str().unwrap();
    assert!(detail.starts_with("Writing suggestions are off."), "{json}");
}

#[tokio::test]
async fn on_and_ready_with_a_writer() {
    let writer = Arc::new(FakeWriter::answering(&[]));
    let router = router(desk(AiProvider::Anthropic, &writer), None);
    let (_, json) = send(&router, WRITER, STATE, None).await;
    let on = json!({"on": true, "provider": "anthropic", "label": "Anthropic", "ready": true});
    assert_eq!(json, on);
}

#[tokio::test]
async fn on_but_not_ready_without_a_key() {
    let desk = AiDesk::new(AiProvider::Openai, None, Lengths::default());
    let router = router(desk, None);
    let (_, json) = send(&router, WRITER, STATE, None).await;
    let label = "OpenAI-compatible service";
    let waiting = json!({"on": true, "provider": "openai", "label": label, "ready": false});
    assert_eq!(json, waiting);
}

#[tokio::test]
async fn a_key_that_cannot_write_posts_is_refused() {
    let router = router(AiDesk::off(), None);
    let (status, _) = send(&router, READER, STATE, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

/// Five titles, the second too long and the fourth too short.
const TITLES: [&str; 5] = [
    "Choosing anchor text that helps readers",
    "Choosing anchor text: a long guide to every kind of link on a blog",
    "Anchor text that search engines understand",
    "Anchors",
    "How to choose anchor text for internal links",
];
const SUGGEST: (&str, &str) = ("POST", "/ai/suggestions/");

#[tokio::test]
async fn five_titles_each_checked_against_the_length_rules() {
    let writer = Arc::new(FakeWriter::answering(&TITLES));
    let router = router(desk(AiProvider::Anthropic, &writer), None);
    let (_, state) = send(&router, WRITER, ("GET", "/ai/"), None).await;
    assert_eq!(
        state,
        json!({"on": true, "provider": "anthropic", "label": "Anthropic", "ready": true})
    );

    let ask = json!({"task": "seo_title", "title": "Anchor text", "text": "How to write anchor text.", "focus_keyword": "anchor text"});
    let (status, json) = send(&router, WRITER, SUGGEST, Some(&ask)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["label"], "AI suggestion from Anthropic");
    let options = json["suggestions"].as_array().unwrap();
    assert_eq!(options.len(), 5);
    let fits: Vec<bool> = options
        .iter()
        .map(|o| o["fits"].as_bool().unwrap())
        .collect();
    assert_eq!(
        fits,
        [true, true, true, false, false],
        "fitting options come first"
    );
    assert_eq!(options[0]["text"], TITLES[0]);
    assert_eq!(options[0]["characters"], 39);
    assert_eq!(
        options[3]["note"],
        "Too long: 66 characters, and 60 is the most that shows in full."
    );
    assert_eq!(
        options[4]["note"],
        "Too short: 7 characters, and it needs at least 20 to say enough."
    );

    let asked = writer.asked();
    assert_eq!(asked.len(), 1);
    assert_eq!(asked[0].task, AiTask::SeoTitle);
    assert_eq!(asked[0].count, 5);
    assert!(
        asked[0].prompt.contains("Focus keyword: anchor text"),
        "{}",
        asked[0].prompt
    );
}

#[tokio::test]
async fn words_come_from_the_saved_post_and_given_words_win() {
    let writer = Arc::new(FakeWriter::answering(&TITLES));
    let router = router(desk(AiProvider::Anthropic, &writer), None);
    let post = json!({"posts": [{"title": "Choosing anchor text", "markdown": "Saved words about links."}]});
    let (status, made) = send(&router, WRITER, ("POST", "/posts/"), Some(&post)).await;
    assert_eq!(status, StatusCode::CREATED, "{made}");
    let id = made["posts"][0]["id"].as_str().unwrap();

    let ask = json!({"task": "meta_description", "post": id, "title": "Unsaved title"});
    let (status, json) = send(&router, WRITER, SUGGEST, Some(&ask)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let prompt = &writer.asked()[0].prompt;
    assert!(prompt.contains("Title: Unsaved title"), "{prompt}");
    assert!(prompt.contains("Saved words about links."), "{prompt}");

    let ask = json!({"task": "meta_description", "post": "no-such-post"});
    let (status, _) = send(&router, WRITER, SUGGEST, Some(&ask)).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn without_a_key_the_route_names_the_variable() {
    let router = router(
        AiDesk::new(AiProvider::Anthropic, None, Lengths::default()),
        None,
    );
    let (_, state) = send(&router, WRITER, ("GET", "/ai/"), None).await;
    assert_eq!(state["ready"], false);
    let ask = json!({"task": "seo_title", "title": "Anchor text"});
    let (status, json) = send(&router, WRITER, SUGGEST, Some(&ask)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert!(
        json["detail"].as_str().unwrap().contains("KHALID_AI_KEY"),
        "{json}"
    );
}

#[tokio::test]
async fn serpdad_writes_only_while_it_is_connected() {
    let at = Utc.with_ymd_and_hms(2026, 11, 2, 9, 0, 0).unwrap();
    let serpdad: Arc<dyn SerpdadStore> = Arc::new(MemorySerpdadStore::new(at));
    let writer = Arc::new(FakeWriter::answering(&TITLES));
    let router = router(
        desk(AiProvider::Serpdad, &writer),
        Some(Arc::clone(&serpdad)),
    );
    let ask = json!({"task": "seo_title", "title": "Anchor text"});
    let (status, json) = send(&router, WRITER, SUGGEST, Some(&ask)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert!(
        json["detail"]
            .as_str()
            .unwrap()
            .starts_with("SerpDad is not connected."),
        "{json}"
    );

    serpdad
        .connect(&KeyId::try_new("9999aaaabbbbcccc").unwrap())
        .await
        .unwrap();
    let (status, json) = send(&router, WRITER, SUGGEST, Some(&ask)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
}
