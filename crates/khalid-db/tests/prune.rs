//! The hourly prune: 404s not seen for 90 days, finished webhook
//! deliveries older than 30 days, dead sessions and sign-in links that can
//! no longer be used go, and everything still in use stays, on one site
//! only.

use std::path::PathBuf;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use khalid_api::keys::Role;
use khalid_api::not_found_log::NotFoundLog;
use khalid_auth::{AccountStore, LinkStore, SessionStore, SessionToken};
use khalid_core::{AbsoluteUrl, Slug};
use khalid_db::{
    AccountTable, Database, DeliveryLog, LinkTable, NotFoundTable, Prune, Pruned, SessionTable,
};
use khalid_webhooks::{Attempt, DeliveryStore, NewDelivery, Outcome, RetrySchedule, WebhookId};
use sqlx::ConnectOptions;
use sqlx::sqlite::SqliteConnectOptions;

/// The moment the prune runs.
#[cfg(test)]
fn now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 12, 9, 0, 0).unwrap()
}

#[cfg(test)]
fn days_ago(days: i64) -> DateTime<Utc> {
    now() - TimeDelta::days(days)
}

/// A migrated database with the sites `demo` and `other`, and webhook 7 on
/// `demo`.
#[cfg(test)]
async fn opened() -> (tempfile::TempDir, Database) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    db.site(&Slug::try_new("demo").unwrap()).await.unwrap();
    db.site(&Slug::try_new("other").unwrap()).await.unwrap();
    let mut connection = SqliteConnectOptions::new()
        .filename(&path)
        .connect()
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO webhooks (id, site_id, event, url, secret, created_at) VALUES \
         (7, 1, 'post.published', 'https://hooks.example.com/khalid', zeroblob(32), \
         '2026-01-01T09:00:00Z')",
    )
    .execute(&mut connection)
    .await
    .unwrap();
    (dir, db)
}

#[cfg(test)]
async fn site(db: &Database, handle: &str) -> khalid_db::SiteId {
    db.site(&Slug::try_new(handle).unwrap()).await.unwrap()
}

/// Sends one delivery first made `days` ago, answered with `status`.
#[cfg(test)]
async fn delivered(log: &DeliveryLog, days: i64, status: u16) {
    let mut delivery = log
        .insert(NewDelivery::new(
            WebhookId::new(7),
            "post.published".parse().unwrap(),
            AbsoluteUrl::parse("https://hooks.example.com/khalid").unwrap(),
            br#"{"post":{"title":"What a sitemap does"}}"#.to_vec(),
            days_ago(days),
        ))
        .await
        .unwrap();
    delivery.record(
        Attempt::new(days_ago(days), Outcome::Answered { status }),
        &RetrySchedule::default(),
    );
    log.update(&delivery).await.unwrap();
}

#[tokio::test]
async fn old_rows_go_and_rows_in_use_stay() {
    let (_dir, db) = opened().await;
    let demo = site(&db, "demo").await;

    let misses = NotFoundTable::new(db.clone(), demo);
    misses
        .record("/blog/wp-login.php", days_ago(91))
        .await
        .unwrap();
    misses
        .record("/blog/old-post/", days_ago(120))
        .await
        .unwrap();
    misses.record("/blog/old-post/", days_ago(2)).await.unwrap();

    let deliveries = DeliveryLog::new(db.clone(), demo);
    delivered(&deliveries, 31, 200).await;
    delivered(&deliveries, 3, 200).await;
    // Still being tried: a 503 waits for its next try, however old.
    delivered(&deliveries, 40, 503).await;

    let accounts = AccountTable::new(db.clone(), demo);
    let email = "asif@example.com".parse().unwrap();
    let asif = accounts
        .create(&email, "Asif", Role::Editor, days_ago(60))
        .await
        .unwrap();
    let sessions = SessionTable::new(db.clone(), demo);
    let (expired, idle, live) = (
        SessionToken::generate().unwrap().hash(),
        SessionToken::generate().unwrap().hash(),
        SessionToken::generate().unwrap().hash(),
    );
    sessions
        .begin(expired, asif.id(), days_ago(15))
        .await
        .unwrap();
    sessions.begin(idle, asif.id(), days_ago(1)).await.unwrap();
    sessions
        .begin(live, asif.id(), now() - TimeDelta::hours(1))
        .await
        .unwrap();
    let links = LinkTable::new(db.clone(), demo);
    let (old_link, new_link) = (
        SessionToken::generate().unwrap().hash(),
        SessionToken::generate().unwrap().hash(),
    );
    links
        .create(old_link, asif.id(), days_ago(1))
        .await
        .unwrap();
    links
        .create(new_link, asif.id(), now() - TimeDelta::minutes(5))
        .await
        .unwrap();

    let pruned = Prune::new(db.clone(), demo).run(now()).await.unwrap();
    assert_eq!(
        pruned,
        Pruned {
            misses: 1,
            deliveries: 1,
            sessions: 2,
            links: 1,
        }
    );

    let kept = misses.misses(10).await.unwrap();
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].path, "/blog/old-post/");
    assert_eq!(deliveries.log(WebhookId::new(7)).await.unwrap().len(), 2);
    assert!(sessions.find(live).await.unwrap().is_some());
    assert!(sessions.find(expired).await.unwrap().is_none());
    assert!(sessions.find(idle).await.unwrap().is_none());
    assert!(links.redeem(new_link, now()).await.unwrap().is_some());

    let again = Prune::new(db.clone(), demo).run(now()).await.unwrap();
    assert_eq!(again, Pruned::default());
    db.close().await;
}

#[tokio::test]
async fn a_prune_leaves_other_sites_alone() {
    let (_dir, db) = opened().await;
    let (demo, other) = (site(&db, "demo").await, site(&db, "other").await);
    NotFoundTable::new(db.clone(), other)
        .record("/blog/wp-login.php", days_ago(200))
        .await
        .unwrap();
    let pruned = Prune::new(db.clone(), demo).run(now()).await.unwrap();
    assert_eq!(pruned, Pruned::default());
    let theirs = NotFoundTable::new(db.clone(), other).misses(10).await;
    assert_eq!(theirs.unwrap().len(), 1);
    db.close().await;
}
