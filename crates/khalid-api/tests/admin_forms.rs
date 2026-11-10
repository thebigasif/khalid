//! Form answers through the Admin API (KH-GRO-1, plan section 4.6): the
//! forms visitors answered, and a page of one form's answers, newest
//! first. Both need `subscribers:write`.

mod common;

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode};
use chrono::{DateTime, TimeZone, Utc};
use khalid_api::api;
use khalid_api::content_store::BoxFuture;
use khalid_api::form_answers::{
    AnswerField, AnswerPage, FormAnswer, FormAnswers, FormAnswersError, FormSummary,
};
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const VISITORS: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const POSTS_ONLY: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

#[cfg(test)]
fn at(minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 3, 9, minute, 0).unwrap()
}

/// Seven answers to `contact`, numbered 1 to 7, the newest last.
struct Answers;

impl FormAnswers for Answers {
    fn forms(&self) -> BoxFuture<'_, Result<Vec<FormSummary>, FormAnswersError>> {
        let summary = |id: &str, answers, minute| FormSummary {
            id: id.to_owned(),
            answers,
            last_at: at(minute),
        };
        let forms = vec![summary("contact", 7, 7), summary("survey", 1, 2)];
        Box::pin(async move { Ok(forms) })
    }

    fn answers<'a>(
        &'a self,
        form: &'a str,
        offset: u64,
        limit: u32,
    ) -> BoxFuture<'a, Result<AnswerPage, FormAnswersError>> {
        let page = if form == "contact" {
            let ids = (1..=7_i64).rev().skip(usize::try_from(offset).unwrap_or(7));
            let answers = ids
                .take(usize::try_from(limit).unwrap_or(7))
                .map(answer)
                .collect();
            AnswerPage { answers, total: 7 }
        } else {
            AnswerPage::default()
        };
        Box::pin(async move { Ok(page) })
    }

    fn delete<'a>(
        &'a self,
        form: &'a str,
        answer: i64,
    ) -> BoxFuture<'a, Result<bool, FormAnswersError>> {
        let deleted = form == "contact" && (1..=7).contains(&answer);
        Box::pin(async move { Ok(deleted) })
    }
}

#[cfg(test)]
fn answer(id: i64) -> FormAnswer {
    let field = |name: &str, value: &str| AnswerField {
        name: name.to_owned(),
        value: value.to_owned(),
    };
    FormAnswer {
        id,
        page: "/blog/write-to-us/".to_owned(),
        sent_at: at(id.try_into().unwrap()),
        fields: vec![
            field("email", "lena@example.com"),
            field("message", "Hello"),
        ],
    }
}

#[cfg(test)]
fn router(forms: Option<Arc<dyn FormAnswers>>) -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = Arc::new(MemoryKeyStore::new(vec![
        stored(VISITORS, Scope::SubscribersWrite),
        stored(POSTS_ONLY, Scope::PostsWrite),
    ]));
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(common::store());
    let (router, _) = match forms {
        Some(forms) => api::router_with_forms(content, Arc::new(door), keys, forms),
        None => api::router_with_door(content, Arc::new(door), keys),
    };
    router
}

/// GETs `route` with `key` from a server keeping [`Answers`].
#[cfg(test)]
async fn get(route: &str, key: &str) -> (StatusCode, Value) {
    get_from(router(Some(Arc::new(Answers))), route, key).await
}

#[cfg(test)]
async fn get_from(router: axum::Router, route: &str, key: &str) -> (StatusCode, Value) {
    send(router, Method::GET, route, key).await
}

/// Sends a DELETE for `route` with `key` from a server keeping [`Answers`].
#[cfg(test)]
async fn delete(route: &str, key: &str) -> (StatusCode, Value) {
    let router = router(Some(Arc::new(Answers)));
    send(router, Method::DELETE, route, key).await
}

#[cfg(test)]
async fn send(router: axum::Router, method: Method, route: &str, key: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(route)
        .header("authorization", format!("Khalid {key}"))
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn the_forms_list_names_each_form_with_its_count() {
    let (status, json) = get("/api/admin/v1/forms/", VISITORS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(
        json,
        json!({ "forms": [
            { "id": "contact", "answers": 7, "last_at": "2026-11-03T09:07:00Z" },
            { "id": "survey", "answers": 1, "last_at": "2026-11-03T09:02:00Z" },
        ]})
    );
}

#[tokio::test]
async fn answers_come_a_page_at_a_time_newest_first() {
    let route = "/api/admin/v1/forms/contact/answers/?page=2&limit=3";
    let (status, json) = get(route, VISITORS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let ids: Vec<i64> = json["answers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|answer| answer["id"].as_i64().unwrap())
        .collect();
    assert_eq!(ids, [4, 3, 2]);
    assert_eq!(
        json["answers"][0]["fields"],
        json!([
            { "name": "email", "value": "lena@example.com" },
            { "name": "message", "value": "Hello" },
        ])
    );
    let pagination = &json["meta"]["pagination"];
    assert_eq!(pagination["total"], 7);
    assert_eq!(pagination["pages"], 3);
    assert_eq!(
        (pagination["next"].clone(), pagination["prev"].clone()),
        (json!(3), json!(1))
    );
}

#[tokio::test]
async fn a_form_with_no_answers_lists_none() {
    let route = "/api/admin/v1/forms/newsletter/answers/";
    let (status, json) = get(route, VISITORS).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["answers"], json!([]));
    assert_eq!(json["meta"]["pagination"]["limit"], 25);
}

#[tokio::test]
async fn a_page_or_limit_out_of_range_answers_400_with_the_fix() {
    for query in ["page=0", "limit=0", "limit=101", "limit=many", "order=id"] {
        let route = format!("/api/admin/v1/forms/contact/answers/?{query}");
        let (status, json) = get(&route, VISITORS).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{query}: {json}");
        assert_eq!(
            json["fix"],
            "Fix page or limit, or leave them out for the newest 25 answers."
        );
    }
}

#[tokio::test]
async fn answers_hold_visitors_words_so_posts_write_is_not_enough() {
    for route in [
        "/api/admin/v1/forms/",
        "/api/admin/v1/forms/contact/answers/",
    ] {
        let (status, _) = get(route, POSTS_ONLY).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{route}");
    }
    let (status, _) = delete("/api/admin/v1/forms/contact/answers/3/", POSTS_ONLY).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn an_owner_deletes_one_answer_on_a_visitor_s_request() {
    let (status, json) = delete("/api/admin/v1/forms/contact/answers/3/", VISITORS).await;
    assert_eq!((status, json), (StatusCode::NO_CONTENT, Value::Null));
}

#[tokio::test]
async fn deleting_an_answer_that_is_not_there_answers_404_with_the_fix() {
    for route in [
        "/api/admin/v1/forms/contact/answers/99/",
        "/api/admin/v1/forms/survey/answers/3/",
        "/api/admin/v1/forms/contact/answers/three/",
    ] {
        let (status, json) = delete(route, VISITORS).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{route}: {json}");
        assert_eq!(json["instance"], route);
        assert_eq!(json["fix"], "List the form's answers to find it.");
    }
}

#[tokio::test]
async fn a_server_without_answers_says_so() {
    let (status, json) = get_from(router(None), "/api/admin/v1/forms/", VISITORS).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
    assert_eq!(json["fix"], "Send the request to the running khalid serve.");
}
