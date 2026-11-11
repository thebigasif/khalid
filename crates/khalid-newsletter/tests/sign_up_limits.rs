//! The sign-up form asks the router's limits before it sends a
//! confirmation email, so one script cannot mail thousands of inboxes from
//! the site's account (RULES.md BL-SEC-5). A reader over the limit gets a
//! page saying when to try again, and nothing is sent.

use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::ConnectInfo;
use axum::http::{HeaderMap, Request, StatusCode, header};
use khalid_core::{AbsoluteUrl, Locale, Slug, SystemClock};
use khalid_db::{Database, SubscriberTable};
use khalid_newsletter::{
    MemoryMailer, Newsletter, NewsletterSite, Pages, SignUpCheck, SignUpLimits, SiteLinks, router,
};
use tower::ServiceExt;

/// Lets each address sign up twice, then asks it to wait five minutes.
/// `broken` makes every check fail, as when the day's salt cannot be made.
#[derive(Debug, Default)]
struct TwoEach {
    seen: Mutex<BTreeMap<IpAddr, usize>>,
    broken: bool,
}

impl SignUpLimits for TwoEach {
    fn check(&self, peer: IpAddr, _headers: &HeaderMap) -> SignUpCheck {
        if self.broken {
            return SignUpCheck::Unknown;
        }
        let mut seen = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        let count = seen.entry(peer).or_default();
        if *count >= 2 {
            return SignUpCheck::Wait(Duration::from_secs(270));
        }
        *count += 1;
        SignUpCheck::Allowed
    }
}

#[cfg(test)]
fn site() -> NewsletterSite {
    let address = AbsoluteUrl::parse("https://www.example.com").unwrap();
    NewsletterSite {
        name: "Field Notes".to_owned(),
        locale: Locale::try_new("en").unwrap(),
        links: SiteLinks::new(&address, "/blog/"),
    }
}

/// The routes with `limits` over a fresh database and `mailer`, kept alive
/// by the folder.
#[cfg(test)]
async fn routes(mailer: &MemoryMailer, limits: TwoEach) -> (tempfile::TempDir, Router) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let id = db.site(&Slug::try_new("demo").unwrap()).await.unwrap();
    let store = Arc::new(SubscriberTable::new(db, id));
    let newsletter = Newsletter::new(
        store,
        Arc::new(mailer.clone()),
        Arc::new(SystemClock),
        site(),
    )
    .unwrap();
    let pages = Pages::new(site()).unwrap();
    (dir, router(pages, Some(newsletter), Some(Box::new(limits))))
}

/// Signs up `email` from `peer`, or from no known peer, and returns the
/// status, the `Retry-After` header and the page.
#[cfg(test)]
async fn sign_up(
    router: &Router,
    peer: Option<[u8; 4]>,
    email: &str,
) -> (StatusCode, Option<String>, String) {
    let mut request = Request::post("/blog/subscribe")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(format!("email={email}%40example.com&homepage=")))
        .unwrap();
    if let Some(peer) = peer {
        let peer = SocketAddr::new(IpAddr::V4(Ipv4Addr::from(peer)), 40_000);
        request.extensions_mut().insert(ConnectInfo(peer));
    }
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let retry = response.headers().get(header::RETRY_AFTER);
    let retry = retry.map(|value| value.to_str().unwrap().to_owned());
    let body = to_bytes(response.into_body(), 1 << 16).await.unwrap();
    (status, retry, String::from_utf8(body.to_vec()).unwrap())
}

#[tokio::test]
async fn an_address_over_the_limit_is_told_when_to_try_again_and_nothing_is_sent() {
    let mailer = MemoryMailer::default();
    let (_dir, app) = routes(&mailer, TwoEach::default()).await;
    let reader = Some([203, 0, 113, 7]);
    for email in ["ada", "grace"] {
        let (status, _, page) = sign_up(&app, reader, email).await;
        assert_eq!(status, StatusCode::OK, "{page}");
    }
    let (status, retry, page) = sign_up(&app, reader, "linus").await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS, "{page}");
    assert_eq!(retry.as_deref(), Some("300"));
    assert!(page.contains("<h1>Too many sign-ups</h1>"), "{page}");
    assert!(page.contains("try again in 5 minutes"), "{page}");
    assert_eq!(mailer.sent().len(), 2, "the refused sign-up sent an email");
    let (status, _, page) = sign_up(&app, Some([198, 51, 100, 9]), "linus").await;
    assert_eq!(
        status,
        StatusCode::OK,
        "another address is not held up: {page}"
    );
    assert_eq!(mailer.sent().len(), 3);
}

#[tokio::test]
async fn a_wrong_address_is_not_counted() {
    let mailer = MemoryMailer::default();
    let (_dir, app) = routes(&mailer, TwoEach::default()).await;
    let reader = Some([203, 0, 113, 7]);
    for _ in 0..3 {
        let (status, _, _) = sign_up(&app, reader, "not%20an").await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
    let (status, _, page) = sign_up(&app, reader, "ada").await;
    assert_eq!(status, StatusCode::OK, "{page}");
}

#[tokio::test]
async fn limits_that_cannot_be_checked_send_nothing() {
    let mailer = MemoryMailer::default();
    let broken = TwoEach {
        broken: true,
        ..TwoEach::default()
    };
    let (_dir, app) = routes(&mailer, broken).await;
    let (status, _, page) = sign_up(&app, Some([203, 0, 113, 7]), "ada").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{page}");
    assert!(page.contains("<h1>Something went wrong</h1>"), "{page}");
    let (_dir, app) = routes(&mailer, TwoEach::default()).await;
    let (status, _, page) = sign_up(&app, None, "ada").await;
    assert_eq!(
        status,
        StatusCode::SERVICE_UNAVAILABLE,
        "no peer address: {page}"
    );
    assert!(mailer.sent().is_empty(), "{:?}", mailer.sent());
}
