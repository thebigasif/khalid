//! Welcome emails against SQLite and the fake transport: a reader gets the
//! first welcome of each newsletter they join the moment they confirm, or
//! the moment a confirmed reader joins one more, and each later step of the
//! sequence on its day, each once, with an unsubscribe link and the postal
//! address (task 5.9, G39).

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use khalid_core::{AbsoluteUrl, FixedClock, Locale, Slug};
use khalid_db::{Database, SubscriberTable};
use khalid_newsletter::{
    EmailAddress, Lists, MemoryMailer, Newsletter, NewsletterSite, PostalAddress, SiteLinks,
    SubscriberStore, Welcome, Welcomes,
};

const WELCOME: &str =
    "# Welcome to the weekly links\n\nEvery Friday, five links worth your time.\n";

const DAY_THREE: &str = "# Three tips for the weekly links\n\nReply to any issue.\n";

#[cfg(test)]
fn slug(text: &str) -> Slug {
    Slug::try_new(text).unwrap()
}

#[cfg(test)]
fn nine() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 31, 9, 0, 0).unwrap()
}

/// A fresh database's subscribers.
#[cfg(test)]
async fn opened() -> (tempfile::TempDir, Arc<SubscriberTable>) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let id = db.site(&slug("demo")).await.unwrap();
    (dir, Arc::new(SubscriberTable::new(db, id)))
}

/// The newsletter over `store` at `at`, whose `weekly` list has a welcome
/// and a step on day three.
#[cfg(test)]
fn flow(store: &Arc<SubscriberTable>, mailer: &MemoryMailer, at: DateTime<Utc>) -> Newsletter {
    let address = AbsoluteUrl::parse("https://www.example.com").unwrap();
    let site = NewsletterSite {
        name: "Field Notes".to_owned(),
        locale: Locale::try_new("en").unwrap(),
        links: SiteLinks::new(&address, "/blog/"),
    };
    let clock = Arc::new(FixedClock::new(at));
    let shared: Arc<dyn SubscriberStore> = Arc::<SubscriberTable>::clone(store);
    let flow = Newsletter::new(shared, Arc::new(mailer.clone()), clock, site).unwrap();
    let steps = vec![
        Welcome::read(DAY_THREE).unwrap().on_day(3),
        Welcome::read(WELCOME).unwrap(),
    ];
    let welcomes = BTreeMap::from([(slug("weekly"), steps)]);
    let postal = PostalAddress::new("Field Notes Ltd, 1 Park Row, Leeds LS1 5AB, UK").unwrap();
    flow.with_lists(Lists::new([slug("weekly")]))
        .with_welcomes(Welcomes::new(welcomes, postal))
}

/// Signs `address` up for `list` and opens the confirmation link twice.
#[cfg(test)]
async fn join(flow: &Newsletter, mailer: &MemoryMailer, address: &str, list: &str) {
    let email = EmailAddress::parse(address).unwrap();
    flow.sign_up(email, slug(list)).await.unwrap();
    let text = mailer.sent().last().unwrap().text.clone();
    let marker = "subscribe/confirm?token=";
    let start = text.find(marker).unwrap() + marker.len();
    let token = text.get(start..start + 64).unwrap().to_owned();
    flow.confirm(&token).await.unwrap();
    flow.confirm(&token).await.unwrap();
}

/// The subjects of every email sent, in order.
#[cfg(test)]
fn subjects(mailer: &MemoryMailer) -> Vec<String> {
    mailer
        .sent()
        .iter()
        .map(|mail| mail.subject.clone())
        .collect()
}

#[test]
fn a_welcome_is_a_heading_for_the_subject_and_text_below() {
    let welcome = Welcome::read(WELCOME).unwrap();
    assert_eq!(welcome.subject(), "Welcome to the weekly links");
    assert_eq!(
        welcome.text(),
        "Every Friday, five links worth your time.\n"
    );
    assert_eq!(welcome.day(), 0);
    assert_eq!(Welcome::read("Welcome\n\nText.\n"), None);
    assert_eq!(Welcome::read("# Welcome\n"), None);
}

#[test]
fn a_file_name_gives_the_day_its_step_is_sent() {
    assert_eq!(Welcome::day_of_file("welcome.md"), Some(0));
    assert_eq!(Welcome::day_of_file("welcome-day-3.md"), Some(3));
    assert_eq!(Welcome::day_of_file("welcome-day-365.md"), Some(365));
    for name in [
        "welcome-day-0.md",
        "welcome-day-03.md",
        "welcome-day-366.md",
        "welcome-day-.md",
        "welcome-day-+3.md",
        "welcome-day-3.txt",
        "page.md",
    ] {
        assert_eq!(Welcome::day_of_file(name), None, "{name}");
    }
}

#[tokio::test]
async fn confirming_sends_the_welcome_of_the_newsletter_joined_once() {
    let mailer = MemoryMailer::default();
    let (_dir, store) = opened().await;
    join(
        &flow(&store, &mailer, nine()),
        &mailer,
        "grace@example.com",
        "weekly",
    )
    .await;
    let sent = mailer.sent();
    assert_eq!(sent.len(), 2, "{sent:?}");
    let welcome = sent.last().unwrap();
    assert_eq!(welcome.to.as_str(), "grace@example.com");
    assert_eq!(welcome.subject, "Welcome to the weekly links");
    assert!(welcome.text.starts_with("Every Friday"), "{}", welcome.text);
    assert!(
        welcome
            .text
            .contains("https://www.example.com/blog/unsubscribe?token=")
    );
    assert!(
        welcome.text.ends_with("Leeds LS1 5AB, UK\n"),
        "{}",
        welcome.text
    );
}

#[tokio::test]
async fn a_newsletter_without_a_welcome_sends_none() {
    let mailer = MemoryMailer::default();
    let (_dir, store) = opened().await;
    join(
        &flow(&store, &mailer, nine()),
        &mailer,
        "grace@example.com",
        "main",
    )
    .await;
    assert_eq!(mailer.sent().len(), 1, "only the confirmation");
}

#[tokio::test]
async fn a_confirmed_reader_who_joins_one_more_newsletter_gets_its_welcome_once() {
    let mailer = MemoryMailer::default();
    let (_dir, store) = opened().await;
    let first = flow(&store, &mailer, nine());
    join(&first, &mailer, "grace@example.com", "main").await;
    let later = flow(&store, &mailer, nine() + TimeDelta::days(1));
    for _ in 0..2 {
        let grace = EmailAddress::parse("grace@example.com").unwrap();
        later.sign_up(grace, slug("weekly")).await.unwrap();
    }
    assert_eq!(
        subjects(&mailer),
        [
            "Confirm your email for Field Notes",
            "Welcome to the weekly links"
        ]
    );
}

#[tokio::test]
async fn a_later_step_goes_out_on_its_day_once_and_not_a_week_late() {
    let mailer = MemoryMailer::default();
    let (_dir, store) = opened().await;
    join(
        &flow(&store, &mailer, nine()),
        &mailer,
        "grace@example.com",
        "weekly",
    )
    .await;
    let early = flow(&store, &mailer, nine() + TimeDelta::days(2));
    assert_eq!(early.send_due_welcomes().await.unwrap(), 0);
    let on_day = flow(&store, &mailer, nine() + TimeDelta::days(3));
    assert_eq!(on_day.send_due_welcomes().await.unwrap(), 1);
    assert_eq!(on_day.send_due_welcomes().await.unwrap(), 0);
    let last = mailer.sent().last().unwrap().clone();
    assert_eq!(last.subject, "Three tips for the weekly links");
    assert!(last.text.ends_with("Leeds LS1 5AB, UK\n"), "{}", last.text);
    let (_late_dir, late_store) = opened().await;
    let late_mailer = MemoryMailer::default();
    let joined = flow(&late_store, &late_mailer, nine());
    join(&joined, &late_mailer, "ben@example.com", "weekly").await;
    let too_late = flow(&late_store, &late_mailer, nine() + TimeDelta::days(11));
    assert_eq!(too_late.send_due_welcomes().await.unwrap(), 0);
}
