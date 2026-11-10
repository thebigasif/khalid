//! Unsent form answer emails on SQLite: kept after a failed send, due at
//! their next try, gone once sent, and listed once Khalid gives up, each
//! site seeing only its own (G40).

use std::path::PathBuf;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use khalid_core::Slug;
use khalid_db::{Database, Failure, FormMailTable};
use khalid_forms::FormId;
use khalid_newsletter::{EmailAddress, Mail};

#[cfg(test)]
fn at(minutes: i64) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 10, 9, 0, 0).unwrap() + TimeDelta::minutes(minutes)
}

/// A migrated database with the unsent emails of the sites `demo` and
/// `other`.
#[cfg(test)]
async fn opened() -> (tempfile::TempDir, FormMailTable, FormMailTable) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let demo = db.site(&Slug::try_new("demo").unwrap()).await.unwrap();
    let other = db.site(&Slug::try_new("other").unwrap()).await.unwrap();
    let tables = (
        FormMailTable::new(db.clone(), demo),
        FormMailTable::new(db, other),
    );
    (dir, tables.0, tables.1)
}

#[cfg(test)]
fn contact() -> FormId {
    FormId::try_new("contact").unwrap()
}

#[cfg(test)]
fn mail(text: &str) -> Mail {
    let to = EmailAddress::parse("hello@example.com").unwrap();
    Mail::new(
        to,
        "New answer to the contact form".to_owned(),
        text.to_owned(),
    )
}

#[cfg(test)]
fn failure(minutes: i64, next: Option<i64>) -> Failure<'static> {
    Failure {
        at: at(minutes),
        error: "the email could not be sent: the SMTP server did not take the email",
        next_at: next.map(at),
    }
}

#[tokio::test]
async fn a_kept_email_is_due_at_its_next_try_and_gone_once_sent() {
    let (_dir, demo, other) = opened().await;
    let text = "name: Lena Vogt\nmessage: Quotes \" and ü.\n";
    demo.keep(&contact(), &mail(text), failure(0, Some(1)))
        .await
        .unwrap();
    assert_eq!(demo.due(at(0)).await.unwrap(), [], "not due yet");
    let due = demo.due(at(1)).await.unwrap();
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].form, contact());
    assert_eq!(due[0].mail, mail(text), "the email comes back as written");
    assert_eq!(due[0].failures, 1);
    assert_eq!(other.due(at(60)).await.unwrap(), [], "another site's");

    demo.failed_again(due[0].id, failure(1, Some(6)))
        .await
        .unwrap();
    assert_eq!(demo.due(at(5)).await.unwrap(), []);
    let again = demo.due(at(6)).await.unwrap();
    assert_eq!(again[0].failures, 2);

    demo.sent(again[0].id).await.unwrap();
    assert_eq!(demo.due(at(600)).await.unwrap(), []);
    assert_eq!(demo.given_up_since(at(0)).await.unwrap(), []);
}

#[tokio::test]
async fn the_longest_waiting_email_comes_first() {
    let (_dir, demo, _) = opened().await;
    demo.keep(&contact(), &mail("second"), failure(1, Some(6)))
        .await
        .unwrap();
    demo.keep(&contact(), &mail("first"), failure(0, Some(5)))
        .await
        .unwrap();
    let due = demo.due(at(10)).await.unwrap();
    let texts: Vec<&str> = due.iter().map(|w| w.mail.text.as_str()).collect();
    assert_eq!(texts, ["first", "second"]);
}

#[tokio::test]
async fn a_given_up_email_is_never_due_and_listed_until_forgotten() {
    let (_dir, demo, other) = opened().await;
    demo.keep(&contact(), &mail("Hello"), failure(0, Some(1)))
        .await
        .unwrap();
    let id = demo.due(at(1)).await.unwrap()[0].id;
    demo.failed_again(id, failure(1, None)).await.unwrap();
    assert_eq!(demo.due(at(100_000)).await.unwrap(), []);

    let given_up = demo.given_up_since(at(0)).await.unwrap();
    assert_eq!(given_up.len(), 1);
    assert_eq!(given_up[0].form, contact());
    assert_eq!(given_up[0].first_at, at(0));
    assert!(given_up[0].last_error.contains("SMTP"), "{given_up:?}");
    assert_eq!(demo.given_up_since(at(2)).await.unwrap(), [], "too old");
    assert_eq!(other.given_up_since(at(0)).await.unwrap(), []);

    other.forget_given_up_before(at(2)).await.unwrap();
    assert_eq!(demo.given_up_since(at(0)).await.unwrap().len(), 1);
    demo.forget_given_up_before(at(2)).await.unwrap();
    assert_eq!(demo.given_up_since(at(0)).await.unwrap(), []);
}
