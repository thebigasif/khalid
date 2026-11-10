//! Click counting against SQLite, the fake transport and the routes (G39):
//! with counting on, each link in an email leads through the blog, which
//! counts the click against the email and sends the reader on. Every copy
//! carries the same link, so no click is tied to a person.

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use chrono::{TimeZone, Utc};
use khalid_core::{AbsoluteUrl, FixedClock, Locale, Slug};
use khalid_db::{Database, SubscriberTable};
use khalid_newsletter::{
    EmailAddress, Issue, MemoryMailer, Newsletter, NewsletterSite, Pages, PostalAddress, Segment,
    SiteLinks, SubscriberStore, router,
};
use tower::ServiceExt;

const POST_TEXT: &str = "Choosing anchor text\n\n\
    Read it on the site: https://www.example.com/blog/anchor-text/\n\n\
    See the guide at https://developers.google.com/search/docs. \
    Or read it again: https://www.example.com/blog/anchor-text/\n";

#[cfg(test)]
fn site() -> NewsletterSite {
    let address = AbsoluteUrl::parse("https://www.example.com").unwrap();
    NewsletterSite {
        name: "Field Notes".to_owned(),
        locale: Locale::try_new("en").unwrap(),
        links: SiteLinks::new(&address, "/blog/"),
    }
}

/// A newsletter over a fresh database with two confirmed readers of
/// `main`, sending through `mailer`, and the database's subscribers.
#[cfg(test)]
async fn newsletter(mailer: &MemoryMailer) -> (tempfile::TempDir, SubscriberTable, Newsletter) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let id = db.site(&Slug::try_new("demo").unwrap()).await.unwrap();
    let table = SubscriberTable::new(db, id);
    let clock = Arc::new(FixedClock::new(
        Utc.with_ymd_and_hms(2026, 11, 3, 9, 0, 0).unwrap(),
    ));
    let (store, mail) = (Arc::new(table.clone()), Arc::new(mailer.clone()));
    let flow = Newsletter::new(store, mail, clock, site()).unwrap();
    for address in ["grace@example.com", "ada@example.com"] {
        let email = EmailAddress::parse(address).unwrap();
        flow.sign_up(email, Slug::try_new("main").unwrap())
            .await
            .unwrap();
        let text = mailer.sent().last().unwrap().text.clone();
        let marker = "subscribe/confirm?token=";
        let start = text.find(marker).unwrap() + marker.len();
        flow.confirm(text.get(start..start + 64).unwrap())
            .await
            .unwrap();
    }
    (dir, table, flow)
}

/// Sends the post to `main` through `flow`, and gives each copy's text.
#[cfg(test)]
async fn sent(flow: &Newsletter, mailer: &MemoryMailer) -> Vec<String> {
    let before = mailer.sent().len();
    let issue = Issue::new(
        "post:en/anchor-text".to_owned(),
        "Choosing anchor text".to_owned(),
        POST_TEXT.to_owned(),
    );
    let segment = Segment::new(Slug::try_new("main").unwrap(), []);
    let postal = PostalAddress::new("1 Park Row, Leeds LS1 5AB, UK").unwrap();
    flow.send(&issue, &segment, &postal).await.unwrap();
    let all = mailer.sent();
    all.get(before..)
        .unwrap()
        .iter()
        .map(|mail| mail.text.clone())
        .collect()
}

/// The status and `Location` `router` answers a GET of `uri` with.
#[cfg(test)]
async fn open(router: &Router, uri: &str) -> (StatusCode, Option<String>) {
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = router.clone().oneshot(request).await.unwrap();
    let location = response.headers().get(header::LOCATION);
    let location = location.map(|value| value.to_str().unwrap().to_owned());
    (response.status(), location)
}

#[tokio::test]
async fn with_counting_off_the_links_go_out_as_written() {
    let mailer = MemoryMailer::default();
    let (_dir, _table, flow) = newsletter(&mailer).await;
    for text in sent(&flow, &mailer).await {
        assert!(text.starts_with(POST_TEXT), "{text}");
        assert!(!text.contains("subscribe/click"), "{text}");
    }
}

#[tokio::test]
async fn a_counted_link_leads_on_and_counts_against_the_email() {
    let mailer = MemoryMailer::default();
    let (_dir, table, flow) = newsletter(&mailer).await;
    let flow = flow.with_click_counting();
    let copies = sent(&flow, &mailer).await;
    assert_eq!(copies.len(), 2);
    let first = copies.first().unwrap();
    let body = first.split("\n--\n").next().unwrap();
    let links: Vec<&str> = body
        .split_whitespace()
        .filter(|word| word.starts_with("https://"))
        .collect();
    assert_eq!(
        links,
        [
            "https://www.example.com/blog/subscribe/click?send=1&link=1",
            "https://www.example.com/blog/subscribe/click?send=1&link=2.",
            "https://www.example.com/blog/subscribe/click?send=1&link=1",
        ],
        "{first}"
    );
    let theirs = copies.last().unwrap().split("\n--\n").next().unwrap();
    assert_eq!(theirs, body, "every reader's copy has the same links");
    let app = router(Pages::new(site()).unwrap(), Some(flow));
    let (status, location) = open(&app, "/blog/subscribe/click?send=1&link=2").await;
    assert_eq!(status, StatusCode::FOUND);
    let guide = "https://developers.google.com/search/docs";
    assert_eq!(location.as_deref(), Some(guide));
    open(&app, "/blog/subscribe/click?send=1&link=1").await;
    let numbers = table.sends().await.unwrap();
    assert_eq!(numbers.first().map(|send| send.clicks), Some(2));
    for unknown in [
        "/blog/subscribe/click?send=1&link=9",
        "/blog/subscribe/click?send=7&link=1",
        "/blog/subscribe/click?send=one&link=1",
        "/blog/subscribe/click",
    ] {
        assert_eq!(open(&app, unknown).await, (StatusCode::NOT_FOUND, None));
    }
}
