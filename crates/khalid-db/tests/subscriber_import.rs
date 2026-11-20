//! Readers added by staff or brought in from a file (backlog row 9): each
//! keeps the status given, an address the site has is left alone, no
//! confirmation link is made, and the export lists everyone.

#![cfg(test)]

use std::path::PathBuf;

use chrono::{DateTime, TimeZone, Utc};
use khalid_api::resources::{NewSubscriber, SubscriberStatus};
use khalid_core::Slug;
use khalid_db::{Database, SubscriberTable};
use khalid_newsletter::{EmailAddress, SignUp, SubscriberStore, SubscriberToken};

async fn table() -> (tempfile::TempDir, SubscriberTable) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let demo = db.site(&slug("demo")).await.unwrap();
    (dir, SubscriberTable::new(db, demo))
}

fn at(day: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, day, 9, 0, 0).unwrap()
}

fn slug(text: &str) -> Slug {
    Slug::try_new(text).unwrap()
}

fn new(email: &str, status: SubscriberStatus, labels: &[&str]) -> NewSubscriber {
    NewSubscriber {
        email: email.to_owned(),
        status,
        created_at: Some(at(1)),
        lists: vec![slug("main")],
        labels: labels.iter().map(|label| slug(label)).collect(),
    }
}

#[tokio::test]
async fn each_reader_keeps_the_status_given() {
    let (_dir, table) = table().await;
    let readers = [
        new("lena@example.com", SubscriberStatus::Confirmed, &["paid"]),
        new("omar@example.com", SubscriberStatus::Pending, &[]),
        new("ines@example.org", SubscriberStatus::Unsubscribed, &[]),
    ];
    let added = table.bring_in(&readers, at(5)).await.unwrap();
    assert!(added.iter().all(Option::is_some), "{added:?}");
    let everyone = table.everyone().await.unwrap();
    let standing: Vec<_> = everyone
        .iter()
        .map(|each| (each.email.as_str(), each.status, each.lists.len()))
        .collect();
    assert_eq!(
        standing,
        [
            ("lena@example.com", SubscriberStatus::Confirmed, 1),
            ("omar@example.com", SubscriberStatus::Pending, 1),
            ("ines@example.org", SubscriberStatus::Unsubscribed, 0),
        ]
    );
    let [lena, _, ines] = everyone.as_slice() else {
        panic!("three readers");
    };
    assert_eq!((lena.created_at, lena.confirmed_at), (at(1), Some(at(1))));
    assert_eq!(lena.labels, [slug("paid")]);
    assert_eq!(ines.unsubscribed_at, Some(at(5)));
    let main = table.list_counts().await.unwrap();
    assert_eq!(main.get("main"), Some(&(1, 1)));
}

#[tokio::test]
async fn an_address_the_site_has_is_left_alone() {
    let (_dir, table) = table().await;
    let gone = [new("ines@example.org", SubscriberStatus::Unsubscribed, &[])];
    table.bring_in(&gone, at(2)).await.unwrap();
    let again = [
        new("ines@example.org", SubscriberStatus::Confirmed, &["paid"]),
        new("kim@example.com", SubscriberStatus::Confirmed, &[]),
    ];
    let added = table.bring_in(&again, at(3)).await.unwrap();
    assert_eq!((added[0], added[1].is_some()), (None, true));
    let ines = &table.everyone().await.unwrap()[0];
    assert_eq!(
        (ines.status, ines.labels.len()),
        (SubscriberStatus::Unsubscribed, 0)
    );
}

#[tokio::test]
async fn an_imported_reader_gets_no_confirmation_link() {
    let (_dir, table) = table().await;
    let waiting = [new("omar@example.com", SubscriberStatus::Pending, &[])];
    table.bring_in(&waiting, at(1)).await.unwrap();
    // Signing up on the form later asks for a confirmation as usual: the
    // import left no link that could have gone out.
    let token = SubscriberToken::generate().unwrap();
    let email = EmailAddress::parse("omar@example.com").unwrap();
    let asked = SignUp::new(email, slug("main"), token.hash(), at(9));
    let signed_up = table.sign_up(&asked).await.unwrap();
    assert!(
        matches!(signed_up, khalid_newsletter::SignedUp::ConfirmationDue(_)),
        "{signed_up:?}"
    );
}
