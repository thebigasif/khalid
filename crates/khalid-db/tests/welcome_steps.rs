//! Welcome sequences and link clicks on SQLite (G39): a step is claimed
//! once, falls due a number of days after the reader confirmed and joined,
//! and a click counts against the email's link, never a person.

use std::path::PathBuf;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use khalid_core::{AbsoluteUrl, Slug};
use khalid_db::{Database, SubscriberTable};
use khalid_newsletter::{
    EmailAddress, SendId, SignUp, SignedUp, SubscriberId, SubscriberStore, SubscriberToken,
};

#[cfg(test)]
async fn table() -> (tempfile::TempDir, SubscriberTable) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let site = db.site(&slug("demo")).await.unwrap();
    (dir, SubscriberTable::new(db, site))
}

/// No subscriber, as [`due`] lists them.
const NOBODY: [i64; 0] = [];

#[cfg(test)]
fn slug(text: &str) -> Slug {
    Slug::try_new(text).unwrap()
}

#[cfg(test)]
fn nine() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 2, 9, 0, 0).unwrap()
}

/// Signs `email` up for `list` at `at` and confirms them at once.
#[cfg(test)]
async fn confirmed(
    table: &SubscriberTable,
    email: &str,
    list: &str,
    at: DateTime<Utc>,
) -> SubscriberId {
    let token = SubscriberToken::generate().unwrap();
    let email = EmailAddress::parse(email).unwrap();
    let asked = SignUp::new(email, slug(list), token.hash(), at);
    let SignedUp::ConfirmationDue(id) = table.sign_up(&asked).await.unwrap() else {
        panic!("no confirmation was due");
    };
    table.confirm(&token.hash(), at).await.unwrap();
    id
}

#[cfg(test)]
async fn due(table: &SubscriberTable, list: &str, day: u16, until: DateTime<Utc>) -> Vec<i64> {
    let after = until - TimeDelta::days(7);
    let found = table.welcomes_due(&slug(list), day, (after, until)).await;
    found.unwrap().iter().map(|one| one.id.as_row()).collect()
}

#[tokio::test]
async fn a_welcome_step_is_claimed_once() {
    let (_dir, table) = table().await;
    let id = confirmed(&table, "grace@example.com", "main", nine()).await;
    let weekly = slug("weekly");
    assert!(table.claim_welcome(id, &weekly, 0, nine()).await.unwrap());
    assert!(!table.claim_welcome(id, &weekly, 0, nine()).await.unwrap());
    assert!(table.claim_welcome(id, &weekly, 3, nine()).await.unwrap());
}

#[tokio::test]
async fn a_step_falls_due_its_days_after_joining_and_only_until_it_is_sent() {
    let (_dir, table) = table().await;
    let id = confirmed(&table, "grace@example.com", "main", nine()).await;
    let early = nine() + TimeDelta::days(3) - TimeDelta::minutes(1);
    assert_eq!(due(&table, "main", 3, early).await, NOBODY);
    let on_time = nine() + TimeDelta::days(3);
    assert_eq!(due(&table, "main", 3, on_time).await, [id.as_row()]);
    assert_eq!(due(&table, "weekly", 3, on_time).await, NOBODY);
    table
        .claim_welcome(id, &slug("main"), 3, on_time)
        .await
        .unwrap();
    assert_eq!(due(&table, "main", 3, on_time).await, NOBODY);
    let long_after = nine() + TimeDelta::days(11);
    assert_eq!(
        due(&table, "main", 4, long_after).await,
        NOBODY,
        "past the window"
    );
}

#[tokio::test]
async fn a_second_newsletter_s_steps_count_from_joining_it() {
    let (_dir, table) = table().await;
    let id = confirmed(&table, "grace@example.com", "main", nine()).await;
    let later = nine() + TimeDelta::days(5);
    let token = SubscriberToken::generate().unwrap();
    let email = EmailAddress::parse("grace@example.com").unwrap();
    let asked = SignUp::new(email, slug("weekly"), token.hash(), later);
    assert_eq!(
        table.sign_up(&asked).await.unwrap(),
        SignedUp::JoinedNow(id)
    );
    let day_two = nine() + TimeDelta::days(2);
    assert_eq!(due(&table, "weekly", 0, day_two).await, NOBODY);
    assert_eq!(
        due(&table, "weekly", 2, later + TimeDelta::days(2)).await,
        [id.as_row()]
    );
}

#[tokio::test]
async fn a_click_counts_against_the_email_and_leads_where_its_link_did() {
    let (_dir, table) = table().await;
    let send = table
        .start_send("post:en/hello", &slug("main"), nine())
        .await;
    let send = send.unwrap().unwrap();
    let url = AbsoluteUrl::parse("https://www.example.com/blog/hello/").unwrap();
    table.add_link(send, 1, &url).await.unwrap();
    for _ in 0..2 {
        assert_eq!(table.click(send, 1).await.unwrap(), Some(url.clone()));
    }
    assert_eq!(table.click(send, 2).await.unwrap(), None);
    assert_eq!(table.click(SendId::from_row(99), 1).await.unwrap(), None);
    let numbers = table.sends().await.unwrap();
    assert_eq!(numbers.first().map(|one| one.clicks), Some(2));
}
