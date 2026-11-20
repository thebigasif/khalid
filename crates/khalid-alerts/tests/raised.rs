//! Each problem is raised once while it lasts, and again after it clears
//! and comes back (KH-GRO-6).

use chrono::{DateTime, Duration, TimeZone, Utc};
use khalid_alerts::{Alert, Finding, Held, Kind, MemoryRaised, Raised, fresh, standing};

#[cfg(test)]
fn at() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 3, 9, 0, 0).unwrap()
}

#[cfg(test)]
fn finding(kind: Kind, subject: &str) -> Finding {
    Finding {
        key: kind.key(subject),
        line: subject.to_owned(),
    }
}

/// The subjects an alert raised, or none.
#[cfg(test)]
async fn raised(store: &MemoryRaised, kind: Kind, subjects: &[&str]) -> Vec<String> {
    let findings = subjects.iter().map(|s| finding(kind, s)).collect();
    let alert = fresh(store, kind, findings, at()).await.unwrap();
    alert.map_or_else(Vec::new, |alert| {
        alert.findings().iter().map(|f| f.line.clone()).collect()
    })
}

#[tokio::test]
async fn a_lasting_problem_is_raised_once_and_again_after_it_clears() {
    let store = MemoryRaised::default();
    let stale = Kind::StalePost;
    assert_eq!(
        raised(&store, stale, &["en.a", "en.b"]).await,
        ["en.a", "en.b"]
    );
    assert_eq!(
        raised(&store, stale, &["en.a", "en.b"]).await,
        Vec::<String>::new()
    );
    assert_eq!(
        raised(&store, stale, &["en.a"]).await,
        Vec::<String>::new(),
        "b cleared"
    );
    assert_eq!(
        raised(&store, stale, &["en.a", "en.b"]).await,
        ["en.b"],
        "b is back"
    );
}

#[tokio::test]
async fn a_one_off_problem_is_not_forgotten_when_the_next_round_misses_it() {
    let store = MemoryRaised::default();
    let refused = Kind::FailedSchedule;
    assert_eq!(raised(&store, refused, &["en.a"]).await, ["en.a"]);
    assert_eq!(raised(&store, refused, &["en.b"]).await, ["en.b"]);
    assert_eq!(
        raised(&store, refused, &["en.a"]).await,
        Vec::<String>::new()
    );
}

#[tokio::test]
async fn kinds_keep_their_own_keys_and_old_keys_are_dropped() {
    let store = MemoryRaised::default();
    raised(&store, Kind::StalePost, &["x"]).await;
    assert_eq!(
        raised(&store, Kind::Certificate, &[]).await,
        Vec::<String>::new()
    );
    assert_eq!(
        store.raised(Kind::StalePost).await.unwrap().len(),
        1,
        "kept"
    );
    store
        .forget_before(at() + Duration::days(30))
        .await
        .unwrap();
    assert_eq!(store.raised(Kind::StalePost).await.unwrap().len(), 0);
}

#[tokio::test]
async fn what_is_still_raised_is_listed_by_kind_with_its_lines() {
    let store = MemoryRaised::default();
    raised(&store, Kind::StalePost, &["en.a"]).await;
    let later = vec![
        finding(Kind::StalePost, "en.a"),
        finding(Kind::StalePost, "en.b"),
    ];
    fresh(&store, Kind::StalePost, later, at() + Duration::hours(1))
        .await
        .unwrap();
    raised(&store, Kind::Certificate, &["blog"]).await;
    let alerts = standing(store.held().await.unwrap());
    let kinds: Vec<Kind> = alerts.iter().map(Alert::kind).collect();
    assert_eq!(
        kinds,
        [Kind::Certificate, Kind::StalePost],
        "Kind::ALL order"
    );
    let stale = &alerts[1];
    let lines: Vec<&str> = stale.findings().iter().map(|f| f.line.as_str()).collect();
    assert_eq!(lines, ["en.b", "en.a"], "newest first");
    assert_eq!(stale.raised_at(), at() + Duration::hours(1));
    assert_eq!(
        stale.what(),
        "Some live posts have not changed for a long time."
    );
}

#[test]
fn a_key_kept_before_lines_were_kept_shows_its_subject() {
    let held = vec![Held {
        key: Kind::StalePost.key("en.old"),
        line: String::new(),
        raised_at: at(),
    }];
    let alerts = standing(held);
    assert_eq!(alerts[0].findings()[0].line, "en.old");
    let unknown = vec![Held {
        key: "gone-kind:x".to_owned(),
        line: "x".to_owned(),
        raised_at: at(),
    }];
    assert_eq!(standing(unknown), [], "a kind this version does not know");
}
