//! The mail hook's checks of who sent an event, what an event says, and a
//! bounce that stops a subscriber on SQLite.

use std::path::PathBuf;

use axum::body::Body;
use axum::http::Request;
use khalid_core::{AbsoluteUrl, Locale, Slug, SystemClock};
use khalid_db::{Database, SubscriberTable};
use khalid_newsletter::{MemoryMailer, NewsletterSite, SignedUp, SiteLinks};
use tower::ServiceExt;

use super::*;

/// A Resend signing secret made for these tests.
fn resend_secret() -> String {
    format!(
        "whsec_{}",
        STANDARD.encode(b"made-for-this-test-0123456789ab")
    )
}

/// The three headers Resend sends with `body`, signed with `secret`.
fn resend(body: &[u8], secret: &str) -> HeaderMap {
    let key = STANDARD
        .decode(secret.trim_start_matches("whsec_"))
        .unwrap();
    let mut mac = Hmac::<Sha256>::new_from_slice(&key).unwrap();
    for part in [b"msg_1".as_slice(), b".", b"1762900000", b".", body] {
        mac.update(part);
    }
    let sent = STANDARD.encode(mac.finalize().into_bytes());
    let mut headers = HeaderMap::new();
    headers.insert("svix-id", "msg_1".parse().unwrap());
    headers.insert("svix-timestamp", "1762900000".parse().unwrap());
    let signature = format!("v1,{} v1,{sent}", STANDARD.encode(b"an older key"));
    headers.insert("svix-signature", signature.parse().unwrap());
    headers
}

/// The header Postmark sends when the hook's address holds `user:password`.
fn postmark(pair: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let basic = format!("Basic {}", STANDARD.encode(pair));
    headers.insert(header::AUTHORIZATION, basic.parse().unwrap());
    headers
}

#[test]
fn a_resend_signature_counts_only_for_its_body_and_secret() {
    let secret = resend_secret();
    assert!(signed(&resend(b"{}", &secret), b"{}", &secret));
    assert!(!signed(&resend(b"{}", &secret), b"{\"a\":1}", &secret));
    let other = format!("whsec_{}", STANDARD.encode(b"another"));
    assert!(!signed(&resend(b"{}", &other), b"{}", &secret));
    assert!(!signed(&HeaderMap::new(), b"{}", &secret));
}

#[test]
fn a_postmark_event_counts_only_with_the_secret_as_its_password() {
    let secret = "made-for-this-test";
    assert!(allowed(&postmark("khalid:made-for-this-test"), secret));
    assert!(allowed(&postmark("anyone:made-for-this-test"), secret));
    assert!(!allowed(&postmark("khalid:made-for-this-tesT"), secret));
    assert!(!allowed(&postmark("made-for-this-test"), secret));
    assert!(!allowed(&HeaderMap::new(), secret));
}

#[test]
fn only_hard_bounces_and_complaints_stop_an_address() {
    let grace = || EmailAddress::parse("grace@example.com").unwrap();
    let resend_bounce = |kind: &str| {
        let body = format!(
            r#"{{"type":"email.bounced","data":{{"to":["Grace <grace@example.com>"],"bounce":{{"type":"{kind}"}}}}}}"#
        );
        stops(Service::Resend, body.as_bytes())
    };
    assert_eq!(
        resend_bounce("Permanent"),
        Some(vec![(grace(), Stop::Bounced)])
    );
    assert_eq!(resend_bounce("Transient"), Some(Vec::new()));
    let complained = br#"{"type":"email.complained","data":{"to":["grace@example.com"]}}"#;
    let complained = stops(Service::Resend, complained);
    assert_eq!(complained, Some(vec![(grace(), Stop::Complained)]));
    let postmark_event = |record: &str, kind: &str| {
        let body =
            format!(r#"{{"RecordType":"{record}","Type":"{kind}","Email":"grace@example.com"}}"#);
        stops(Service::Postmark, body.as_bytes())
    };
    let hard = postmark_event("Bounce", "HardBounce");
    assert_eq!(hard, Some(vec![(grace(), Stop::Bounced)]));
    assert_eq!(postmark_event("Bounce", "SoftBounce"), Some(Vec::new()));
    let spam = postmark_event("SpamComplaint", "SpamComplaint");
    assert_eq!(spam, Some(vec![(grace(), Stop::Complained)]));
    assert_eq!(postmark_event("Delivery", ""), Some(Vec::new()));
    assert_eq!(stops(Service::Postmark, b"not json"), None);
}

/// The hook for Postmark over a fresh database, and the newsletter it
/// stops mail through, kept alive by the folder.
async fn on_postmark() -> (tempfile::TempDir, Router, Newsletter) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let id = db.site(&Slug::try_new("demo").unwrap()).await.unwrap();
    let address = AbsoluteUrl::parse("https://www.example.com").unwrap();
    let site = NewsletterSite {
        name: "Field Notes".to_owned(),
        locale: Locale::try_new("en").unwrap(),
        links: SiteLinks::new(&address, "/"),
    };
    let store = Arc::new(SubscriberTable::new(db, id));
    let mail = Arc::new(MemoryMailer::default());
    let newsletter = Newsletter::new(store, mail, Arc::new(SystemClock), site).unwrap();
    let secret = Secret::new("made-for-this-test".to_owned());
    let routes = hook(Service::Postmark, newsletter.clone(), secret);
    (dir, routes, newsletter)
}

async fn post_event(routes: &Router, pair: &str, body: &str) -> StatusCode {
    let mut request = Request::post(HOOK_PATH);
    if let Some(headers) = request.headers_mut() {
        headers.extend(postmark(pair));
    }
    let request = request.body(Body::from(body.to_owned())).unwrap();
    routes.clone().oneshot(request).await.unwrap().status()
}

#[tokio::test]
async fn a_hard_bounce_from_postmark_stops_the_subscriber() {
    let (_dir, routes, newsletter) = on_postmark().await;
    let grace = EmailAddress::parse("grace@example.com").unwrap();
    let main = Slug::try_new("main").unwrap();
    newsletter
        .sign_up(grace.clone(), main.clone())
        .await
        .unwrap();
    let bounce = r#"{"RecordType":"Bounce","Type":"HardBounce","Email":"grace@example.com"}"#;
    let refused = post_event(&routes, "khalid:not-the-secret", bounce).await;
    assert_eq!(refused, StatusCode::UNAUTHORIZED);
    let junk = post_event(&routes, "khalid:made-for-this-test", "[]").await;
    assert_eq!(junk, StatusCode::BAD_REQUEST);
    let later = Slug::try_new("later").unwrap();
    let before = newsletter.sign_up(grace.clone(), later).await.unwrap();
    assert_ne!(before, SignedUp::Stopped);
    let noted = post_event(&routes, "khalid:made-for-this-test", bounce).await;
    assert_eq!(noted, StatusCode::OK);
    let again = post_event(&routes, "khalid:made-for-this-test", bounce).await;
    assert_eq!(again, StatusCode::OK);
    let after = newsletter.sign_up(grace, main).await.unwrap();
    assert_eq!(after, SignedUp::Stopped);
}
