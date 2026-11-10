//! Subscribers as the Admin API lists and changes them, on SQLite (G39,
//! plan 4.6): a filtered page newest first, labels by identity,
//! forgetting a reader, each newsletter's counts, and each site seeing
//! only its own.

#![cfg(test)]

use std::path::PathBuf;

use chrono::{DateTime, TimeZone, Utc};
use khalid_api::resources::{SubscriberFilter, SubscriberStatus};
use khalid_api::subscriber_desk::LabelChange;
use khalid_core::Slug;
use khalid_db::{Database, SubscriberTable};
use khalid_newsletter::{EmailAddress, SignUp, SignedUp, SubscriberStore, SubscriberToken};

async fn tables() -> (tempfile::TempDir, SubscriberTable, SubscriberTable) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let demo = db.site(&slug("demo")).await.unwrap();
    let other = db.site(&slug("other")).await.unwrap();
    let tables = (
        SubscriberTable::new(db.clone(), demo),
        SubscriberTable::new(db, other),
    );
    (dir, tables.0, tables.1)
}

fn nine() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 1, 9, 0, 0).unwrap()
}

fn slug(text: &str) -> Slug {
    Slug::try_new(text).unwrap()
}

/// What a new reader does after the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Then {
    Confirm,
    Wait,
}

/// Signs `address` up for `list`; answers their identity when they are
/// new, and opens the link when `then` says so.
async fn subscribe(table: &SubscriberTable, address: &str, list: &str, then: Then) -> Option<i64> {
    let token = SubscriberToken::generate().unwrap();
    let email = EmailAddress::parse(address).unwrap();
    let asked = SignUp::new(email, slug(list), token.hash(), nine());
    let SignedUp::ConfirmationDue(id) = table.sign_up(&asked).await.unwrap() else {
        return None;
    };
    if then == Then::Confirm {
        table.confirm(&token.hash(), nine()).await.unwrap();
    }
    Some(id.as_row())
}

/// Lena confirmed on main and weekly, Omar waiting on main, and Ines
/// confirmed then gone; answers their identities.
async fn three(table: &SubscriberTable) -> [i64; 3] {
    let lena = subscribe(table, "lena@example.com", "main", Then::Confirm).await;
    subscribe(table, "lena@example.com", "weekly", Then::Confirm).await;
    let omar = subscribe(table, "omar@example.com", "main", Then::Wait).await;
    let ines = subscribe(table, "ines@example.org", "main", Then::Confirm).await;
    let [lena, omar, ines] = [lena, omar, ines].map(Option::unwrap);
    let token = SubscriberToken::generate().unwrap();
    let id = khalid_newsletter::SubscriberId::from_row(ines);
    table
        .add_unsubscribe(id, &token.hash(), nine())
        .await
        .unwrap();
    table.unsubscribe(&token.hash(), nine()).await.unwrap();
    [lena, omar, ines]
}

async fn emails(table: &SubscriberTable, filter: &SubscriberFilter) -> Vec<String> {
    let page = table.page(filter, (1, 50)).await.unwrap();
    page.subscribers
        .into_iter()
        .map(|each| each.email)
        .collect()
}

#[tokio::test]
async fn the_page_is_newest_first_with_lists_and_status() {
    let (_dir, demo, _) = tables().await;
    let [lena, ..] = three(&demo).await;
    let page = demo
        .page(&SubscriberFilter::default(), (1, 50))
        .await
        .unwrap();
    assert_eq!(page.total, 3);
    let order: Vec<_> = page
        .subscribers
        .iter()
        .map(|each| each.email.as_str())
        .collect();
    assert_eq!(
        order,
        ["ines@example.org", "omar@example.com", "lena@example.com"]
    );
    let first = demo.one(lena).await.unwrap().unwrap();
    assert_eq!(first.status, SubscriberStatus::Confirmed);
    assert_eq!(first.lists, [slug("main"), slug("weekly")]);
    assert_eq!(first.confirmed_at, Some(nine()));
    let gone = &page.subscribers[0];
    assert_eq!(gone.status, SubscriberStatus::Unsubscribed);
    assert!(gone.lists.is_empty(), "{gone:?}");
}

#[tokio::test]
async fn filters_narrow_the_page_and_pages_split_it() {
    let (_dir, demo, _) = tables().await;
    three(&demo).await;
    let confirmed = SubscriberFilter {
        status: Some(SubscriberStatus::Confirmed),
        ..SubscriberFilter::default()
    };
    assert_eq!(emails(&demo, &confirmed).await, ["lena@example.com"]);
    let weekly = SubscriberFilter {
        list: Some(slug("weekly")),
        ..SubscriberFilter::default()
    };
    assert_eq!(emails(&demo, &weekly).await, ["lena@example.com"]);
    let typed = SubscriberFilter {
        search: Some("OMAR".to_owned()),
        ..SubscriberFilter::default()
    };
    assert_eq!(emails(&demo, &typed).await, ["omar@example.com"]);
    let second = demo
        .page(&SubscriberFilter::default(), (2, 2))
        .await
        .unwrap();
    assert_eq!(second.total, 3);
    assert_eq!(second.subscribers.len(), 1);
    assert_eq!(second.subscribers[0].email, "lena@example.com");
}

#[tokio::test]
async fn a_label_is_given_by_identity_and_filters_the_page() {
    let (_dir, demo, _) = tables().await;
    let [lena, omar, ines] = three(&demo).await;
    let paid = slug("paid");
    let given = demo
        .change_label(lena, &paid, (LabelChange::Add, nine()))
        .await;
    assert_eq!(given.unwrap().unwrap().labels, std::slice::from_ref(&paid));
    demo.change_label(omar, &paid, (LabelChange::Add, nine()))
        .await
        .unwrap();
    let left = demo
        .change_label(ines, &paid, (LabelChange::Add, nine()))
        .await;
    assert!(
        left.unwrap().unwrap().labels.is_empty(),
        "a reader who left got a label"
    );
    let labelled = SubscriberFilter {
        label: Some(paid.clone()),
        ..SubscriberFilter::default()
    };
    assert_eq!(
        emails(&demo, &labelled).await,
        ["omar@example.com", "lena@example.com"]
    );
    let taken = demo
        .change_label(omar, &paid, (LabelChange::Remove, nine()))
        .await;
    assert_eq!(taken.unwrap().unwrap().labels, Vec::<Slug>::new());
    assert_eq!(emails(&demo, &labelled).await, ["lena@example.com"]);
    let nobody = demo
        .change_label(999, &paid, (LabelChange::Add, nine()))
        .await;
    assert_eq!(nobody.unwrap(), None);
}

#[tokio::test]
async fn a_forgotten_reader_leaves_no_row() {
    let (_dir, demo, other) = tables().await;
    let [lena, ..] = three(&demo).await;
    assert!(
        !other.erase(lena).await.unwrap(),
        "another site removed a reader"
    );
    assert!(demo.erase(lena).await.unwrap());
    assert_eq!(demo.one(lena).await.unwrap(), None);
    assert!(!demo.erase(lena).await.unwrap());
    let counts = demo.list_counts().await.unwrap();
    assert_eq!(counts.get("weekly"), None);
}

#[tokio::test]
async fn each_newsletter_counts_its_confirmed_and_waiting_readers() {
    let (_dir, demo, other) = tables().await;
    three(&demo).await;
    let counts = demo.list_counts().await.unwrap();
    let counted: Vec<_> = counts.iter().map(|(list, n)| (list.as_str(), *n)).collect();
    assert_eq!(counted, [("main", (1, 1)), ("weekly", (1, 0))]);
    assert!(other.list_counts().await.unwrap().is_empty());
    let page = other
        .page(&SubscriberFilter::default(), (1, 50))
        .await
        .unwrap();
    assert_eq!(page.total, 0);
}
