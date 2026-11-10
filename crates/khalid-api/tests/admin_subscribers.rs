//! The subscriber and newsletter routes of the Admin API (G39, plan 4.6):
//! the list with its filters and pages, labels, forgetting a reader, the
//! newsletters with their counts, and sending a post once. Every route
//! needs `subscribers:write`.

#![cfg(test)]

mod common;

use std::sync::{Arc, Mutex};

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::BoxFuture;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::resources::{
    AdminNewsletter, AdminSend, AdminSubscriber, NewsletterSend, SendReadiness, SubscriberFilter,
    SubscriberPage, SubscriberStatus,
};
use khalid_api::subscriber_desk::{LabelChange, SubscriberDesk, SubscriberError};
use khalid_core::Slug;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const OWNER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const WRITER: &str =
    "ki_5555666677778888_0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";

fn slug(text: &str) -> Slug {
    Slug::try_new(text).unwrap()
}

/// A subscriber `id` at `email`, standing at `status`, on `main`.
fn reader(id: i64, email: &str, status: SubscriberStatus) -> AdminSubscriber {
    AdminSubscriber {
        id,
        email: email.to_owned(),
        status,
        lists: vec![slug("main")],
        labels: Vec::new(),
        created_at: Utc.with_ymd_and_hms(2026, 11, 1, 9, 0, 0).unwrap(),
        confirmed_at: None,
        unsubscribed_at: None,
    }
}

/// Subscribers kept in memory, and the posts sent from them.
#[derive(Default)]
struct Desk {
    readers: Mutex<Vec<AdminSubscriber>>,
    sent: Mutex<Vec<String>>,
    readiness: Option<SendReadiness>,
}

impl Desk {
    fn with(readers: Vec<AdminSubscriber>) -> Arc<Self> {
        Arc::new(Self {
            readers: Mutex::new(readers),
            ..Self::default()
        })
    }
}

impl SubscriberDesk for Desk {
    fn list<'a>(
        &'a self,
        filter: &'a SubscriberFilter,
        (page, limit): (u32, u32),
    ) -> BoxFuture<'a, Result<SubscriberPage, SubscriberError>> {
        let readers = self.readers.lock().unwrap();
        let matching: Vec<_> = readers
            .iter()
            .rev()
            .filter(|each| filter.status.is_none_or(|status| each.status == status))
            .filter(|each| {
                filter
                    .label
                    .as_ref()
                    .is_none_or(|l| each.labels.contains(l))
            })
            .filter(|each| {
                filter
                    .search
                    .as_ref()
                    .is_none_or(|q| each.email.contains(q))
            })
            .cloned()
            .collect();
        let total = u64::try_from(matching.len()).unwrap();
        let skip = usize::try_from((page - 1) * limit).unwrap();
        let shown = matching
            .into_iter()
            .skip(skip)
            .take(usize::try_from(limit).unwrap());
        let page = SubscriberPage::new(shown.collect(), total);
        Box::pin(async move { Ok(page) })
    }

    fn label<'a>(
        &'a self,
        id: i64,
        label: &'a Slug,
        change: LabelChange,
    ) -> BoxFuture<'a, Result<AdminSubscriber, SubscriberError>> {
        let mut readers = self.readers.lock().unwrap();
        let found = readers.iter_mut().find(|each| each.id == id);
        let answer = match found {
            None => Err(SubscriberError::NotFound),
            Some(each) if each.status == SubscriberStatus::Unsubscribed => {
                Err(SubscriberError::Unsubscribed)
            }
            Some(each) => {
                each.labels.retain(|kept| kept != label);
                if change == LabelChange::Add {
                    each.labels.push(label.clone());
                }
                Ok(each.clone())
            }
        };
        Box::pin(async move { answer })
    }

    fn erase(&self, id: i64) -> BoxFuture<'_, Result<(), SubscriberError>> {
        let mut readers = self.readers.lock().unwrap();
        let before = readers.len();
        readers.retain(|each| each.id != id);
        let answer = if readers.len() == before {
            Err(SubscriberError::NotFound)
        } else {
            Ok(())
        };
        Box::pin(async move { answer })
    }

    fn newsletters(&self) -> BoxFuture<'_, Result<Vec<AdminNewsletter>, SubscriberError>> {
        let form = "<form method=\"post\"></form>".to_owned();
        let main = AdminNewsletter::new(slug("main"), (2, 1), form);
        Box::pin(async move { Ok(vec![main]) })
    }

    fn sends(&self) -> BoxFuture<'_, Result<Vec<AdminSend>, SubscriberError>> {
        let at = Utc.with_ymd_and_hms(2026, 11, 2, 8, 0, 0).unwrap();
        let sent = self.sent.lock().unwrap().clone();
        let sends = sent
            .into_iter()
            .map(|key| AdminSend::new(key, slug("main"), at, (2, 0, 1)))
            .collect();
        Box::pin(async move { Ok(sends) })
    }

    fn readiness(&self) -> SendReadiness {
        self.readiness.unwrap_or(SendReadiness::Ready)
    }

    fn send<'a>(
        &'a self,
        list: &'a Slug,
        send: &'a NewsletterSend,
    ) -> BoxFuture<'a, Result<String, SubscriberError>> {
        let key = format!("post:{}", send.post.replace('.', "/"));
        let answer = if let Some(readiness) = self.readiness {
            Err(SubscriberError::NotReady(readiness))
        } else if list.as_str() != "main" {
            Err(SubscriberError::NoList)
        } else if send.post == "en.draft" {
            Err(SubscriberError::NotPublished)
        } else if self.sent.lock().unwrap().contains(&key) {
            Err(SubscriberError::SentBefore)
        } else {
            self.sent.lock().unwrap().push(key.clone());
            Ok(key)
        };
        Box::pin(async move { answer })
    }
}

/// The API with `desk`, a key that manages subscribers and one that only
/// writes posts.
fn router(desk: Option<Arc<dyn SubscriberDesk>>) -> axum::Router {
    let hashing = KeyHashing::fast_for_tests();
    let stored = |text: &str, scope| {
        let token: KeyToken = text.parse().unwrap();
        StoredKey::for_integration(&token, vec![scope], &hashing).unwrap()
    };
    let keys = MemoryKeyStore::new(vec![
        stored(OWNER, Scope::SubscribersWrite),
        stored(WRITER, Scope::PostsWrite),
    ]);
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(common::store());
    let desks = AdminDesks {
        subscribers: desk,
        ..AdminDesks::none()
    };
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

/// The three readers most tests start from: Lena confirmed, Omar waiting,
/// and Ines gone.
fn three() -> Arc<Desk> {
    Desk::with(vec![
        reader(1, "lena@example.com", SubscriberStatus::Confirmed),
        reader(2, "omar@example.com", SubscriberStatus::Pending),
        reader(3, "ines@example.org", SubscriberStatus::Unsubscribed),
    ])
}

/// Sends `method` to `path` with `key` and `body`; answers the status and
/// the body as JSON, or `null` when there is none.
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
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

/// The addresses in a subscriber list.
fn emails(json: &Value) -> Vec<&str> {
    let listed = json["subscribers"].as_array().unwrap();
    listed
        .iter()
        .map(|each| each["email"].as_str().unwrap())
        .collect()
}

#[tokio::test]
async fn the_list_is_newest_first_with_its_total() {
    let router = router(Some(three()));
    let (status, json) = send(&router, OWNER, ("GET", "/subscribers/"), None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let all = ["ines@example.org", "omar@example.com", "lena@example.com"];
    assert_eq!(emails(&json), all);
    assert_eq!(json["meta"]["pagination"]["total"], 3);
    assert_eq!(json["subscribers"][2]["lists"], json!(["main"]));
    assert_eq!(json["subscribers"][2]["status"], "confirmed");
}

#[tokio::test]
async fn the_list_filters_and_pages() {
    let router = router(Some(three()));
    let path = "/subscribers/?status=confirmed&q=example.com";
    let (status, json) = send(&router, OWNER, ("GET", path), None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(emails(&json), ["lena@example.com"]);
    let path = "/subscribers/?limit=2&page=2";
    let (_, json) = send(&router, OWNER, ("GET", path), None).await;
    assert_eq!(emails(&json), ["lena@example.com"]);
    assert_eq!(json["meta"]["pagination"]["pages"], 2);
}

#[tokio::test]
async fn an_unknown_filter_is_refused() {
    let router = router(Some(three()));
    for path in ["/subscribers/?status=gone", "/subscribers/?colour=red"] {
        let (status, json) = send(&router, OWNER, ("GET", path), None).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{path}: {json}");
    }
}

#[tokio::test]
async fn a_key_without_the_scope_sees_no_reader() {
    let router = router(Some(three()));
    for (method, path) in [
        ("GET", "/subscribers/"),
        ("GET", "/newsletters/"),
        ("DELETE", "/subscribers/1/"),
    ] {
        let (status, json) = send(&router, WRITER, (method, path), None).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{path}: {json}");
        assert_eq!(json["rule"], "subscribers:write");
    }
}

#[tokio::test]
async fn a_label_is_given_and_taken_away() {
    let router = router(Some(three()));
    let path = "/subscribers/1/labels/paid/";
    let (status, json) = send(&router, OWNER, ("PUT", path), None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["subscribers"][0]["labels"], json!(["paid"]));
    let (_, json) = send(&router, OWNER, ("GET", "/subscribers/?label=paid"), None).await;
    assert_eq!(emails(&json), ["lena@example.com"]);
    let (status, json) = send(&router, OWNER, ("DELETE", path), None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["subscribers"][0]["labels"], json!([]));
}

#[tokio::test]
async fn a_label_needs_a_slug_and_a_reader_who_stayed() {
    let router = router(Some(three()));
    let (status, json) = send(
        &router,
        OWNER,
        ("PUT", "/subscribers/1/labels/Paid!/"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["rule"], "label");
    let (status, json) = send(&router, OWNER, ("PUT", "/subscribers/3/labels/paid/"), None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["rule"], "status");
    let (status, _) = send(&router, OWNER, ("PUT", "/subscribers/9/labels/paid/"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_forgotten_reader_is_gone() {
    let router = router(Some(three()));
    let (status, _) = send(&router, OWNER, ("DELETE", "/subscribers/2/"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (_, json) = send(&router, OWNER, ("GET", "/subscribers/"), None).await;
    assert_eq!(emails(&json), ["ines@example.org", "lena@example.com"]);
    let (status, json) = send(&router, OWNER, ("DELETE", "/subscribers/2/"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
}

#[tokio::test]
async fn the_newsletters_show_their_readers_and_form() {
    let router = router(Some(three()));
    let (status, json) = send(&router, OWNER, ("GET", "/newsletters/"), None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let main = &json["newsletters"][0];
    assert_eq!(
        (main["name"].as_str(), main["confirmed"].as_u64()),
        (Some("main"), Some(2))
    );
    assert_eq!(main["pending"], 1);
    assert!(main["form"].as_str().unwrap().starts_with("<form"));
    assert_eq!(json["sending"], "ready");
    assert_eq!(json["sends"], json!([]));
}

#[tokio::test]
async fn a_post_goes_out_once() {
    let router = router(Some(three()));
    let body = json!({"post": "en.hello", "labels": ["paid"]});
    let route = ("POST", "/newsletters/main/sends/");
    let (status, json) = send(&router, OWNER, route, Some(&body)).await;
    assert_eq!(status, StatusCode::ACCEPTED, "{json}");
    assert_eq!(json, json!({"key": "post:en/hello"}));
    let (_, json) = send(&router, OWNER, ("GET", "/newsletters/"), None).await;
    let sent = json!({"key": "post:en/hello", "list": "main", "sent": 2, "failed": 0, "unsubscribed": 1, "started_at": "2026-11-02T08:00:00Z"});
    assert_eq!(json["sends"], json!([sent]));
    let (status, json) = send(&router, OWNER, route, Some(&body)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert!(json["title"].is_string());
}

#[tokio::test]
async fn a_send_names_what_is_wrong() {
    let router = router(Some(three()));
    let cases = [
        (
            "/newsletters/weekly/sends/",
            json!({"post": "en.hello"}),
            StatusCode::NOT_FOUND,
        ),
        (
            "/newsletters/main/sends/",
            json!({"post": "en.draft"}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "/newsletters/main/sends/",
            json!({"slug": "hello"}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
    ];
    for (path, body, expected) in cases {
        let (status, json) = send(&router, OWNER, ("POST", path), Some(&body)).await;
        assert_eq!(status, expected, "{path} {body}: {json}");
    }
}

#[tokio::test]
async fn a_site_that_cannot_send_says_what_it_lacks() {
    let desk = Arc::new(Desk {
        readiness: Some(SendReadiness::NoPostalAddress),
        ..Desk::default()
    });
    let router = router(Some(desk));
    let (_, json) = send(&router, OWNER, ("GET", "/newsletters/"), None).await;
    assert_eq!(json["sending"], "no_postal_address");
    let body = json!({"post": "en.hello"});
    let route = ("POST", "/newsletters/main/sends/");
    let (status, json) = send(&router, OWNER, route, Some(&body)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert!(
        json["detail"].as_str().unwrap().contains("postal address"),
        "{json}"
    );
}

#[tokio::test]
async fn a_server_with_no_subscribers_lists_none() {
    let router = router(None);
    let (status, json) = send(&router, OWNER, ("GET", "/subscribers/"), None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["subscribers"], json!([]));
    let (_, json) = send(&router, OWNER, ("GET", "/newsletters/"), None).await;
    assert_eq!(json["sending"], "no_mail");
    let (status, _) = send(&router, OWNER, ("DELETE", "/subscribers/1/"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
