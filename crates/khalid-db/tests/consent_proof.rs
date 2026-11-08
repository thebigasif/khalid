//! Consent proof on SQLite (privacy plan U7, R15): every version of what
//! readers were shown, with the days it was in force, and daily counts of
//! what they chose, with no address or reader identifier stored.

use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use khalid_api::consent_proof::ConsentProof;
use khalid_core::{ConsentCategory, PrivacySettings, SettingsVersion, SiteCode, Slug};
use khalid_db::{ConsentChoice, ConsentProofTable, Database};

/// The version of the default settings with `built_in` as the release's
/// own banner words.
#[cfg(test)]
fn version(built_in: &str) -> SettingsVersion {
    PrivacySettings::default().version(&SiteCode::default(), &["youtube"], built_in)
}

#[cfg(test)]
fn first() -> SettingsVersion {
    version("We ask before anything tracks you.")
}

#[cfg(test)]
fn reworded() -> SettingsVersion {
    version("Nothing tracks you until you agree.")
}

#[cfg(test)]
fn day(month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, month, date).unwrap()
}

/// The consent proof of site `handle` in the database at `path`.
#[cfg(test)]
async fn table(path: &Path, handle: &str) -> (Database, ConsentProofTable) {
    let db = Database::open(path).await.unwrap();
    db.migrate().await.unwrap();
    let site = db.site(&Slug::try_new(handle).unwrap()).await.unwrap();
    (db.clone(), ConsentProofTable::new(db, site))
}

#[cfg(test)]
fn file(dir: &tempfile::TempDir) -> PathBuf {
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    path
}

const SHOWN: &str = r#"{"banner":{"en":"We ask before anything tracks you."},"lifetime_months":6}"#;
const REWORDED: &str =
    r#"{"banner":{"en":"Nothing tracks you until you agree."},"lifetime_months":6}"#;

#[tokio::test]
async fn a_choice_adds_one_to_the_right_counters_for_its_version() {
    let dir = tempfile::tempdir().unwrap();
    let (db, proof) = table(&file(&dir), "main").await;
    let analytics = ConsentChoice::allowing([ConsentCategory::Analytics]);
    let everything = ConsentChoice::allowing(ConsentCategory::ALL);
    proof
        .record(first(), SHOWN, day(11, 8), analytics)
        .await
        .unwrap();
    proof
        .record(first(), SHOWN, day(11, 8), everything)
        .await
        .unwrap();
    proof
        .record(first(), SHOWN, day(11, 8), ConsentChoice::NONE)
        .await
        .unwrap();
    db.close().await;

    let (_db, proof) = table(&file(&dir), "main").await;
    let counts = proof.counts(day(11, 1), day(11, 30)).await.unwrap();
    assert_eq!(counts.len(), 1);
    let count = &counts[0];
    assert_eq!(
        (count.day, &count.version),
        (day(11, 8), &first().to_string())
    );
    assert_eq!(count.choices, 3);
    assert_eq!(count.allowed(ConsentCategory::Necessary), 3);
    assert_eq!(count.allowed(ConsentCategory::Preferences), 1);
    assert_eq!(count.allowed(ConsentCategory::Analytics), 2);
    assert_eq!(count.allowed(ConsentCategory::Marketing), 1);
    assert_eq!(count.allowed(ConsentCategory::ExternalMedia), 1);
}

#[tokio::test]
async fn each_day_and_each_version_count_apart() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, proof) = table(&file(&dir), "main").await;
    let media = ConsentChoice::allowing([ConsentCategory::ExternalMedia]);
    proof
        .record(first(), SHOWN, day(11, 8), media)
        .await
        .unwrap();
    proof
        .record(first(), SHOWN, day(11, 9), media)
        .await
        .unwrap();
    proof
        .record(reworded(), REWORDED, day(11, 9), media)
        .await
        .unwrap();
    let counts = proof.counts(day(11, 1), day(11, 30)).await.unwrap();
    let seen: Vec<(NaiveDate, String, u64)> = counts
        .iter()
        .map(|count| (count.day, count.version.clone(), count.choices))
        .collect();
    assert_eq!(
        seen,
        [
            (day(11, 8), first().to_string(), 1),
            (day(11, 9), first().to_string(), 1),
            (day(11, 9), reworded().to_string(), 1),
        ]
    );
    let only_ninth = proof.counts(day(11, 9), day(11, 9)).await.unwrap();
    assert_eq!(only_ninth.len(), 2);
}

#[tokio::test]
async fn the_same_settings_keep_one_version_and_a_wording_change_adds_one() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, proof) = table(&file(&dir), "main").await;
    proof.shown(first(), SHOWN, day(11, 8)).await.unwrap();
    proof
        .record(first(), SHOWN, day(11, 10), ConsentChoice::NONE)
        .await
        .unwrap();
    proof.shown(first(), SHOWN, day(11, 12)).await.unwrap();
    // A late write for an earlier day does not move the days back.
    proof.shown(first(), SHOWN, day(11, 9)).await.unwrap();
    proof
        .shown(reworded(), REWORDED, day(11, 13))
        .await
        .unwrap();

    let versions = proof.versions().await.unwrap();
    let seen: Vec<(String, &str, NaiveDate, NaiveDate)> = versions
        .iter()
        .map(|kept| {
            (
                kept.version.clone(),
                kept.shown.as_str(),
                kept.first_day,
                kept.last_day,
            )
        })
        .collect();
    assert_eq!(
        seen,
        [
            (first().to_string(), SHOWN, day(11, 8), day(11, 12)),
            (reworded().to_string(), REWORDED, day(11, 13), day(11, 13)),
        ]
    );
}

#[tokio::test]
async fn counts_older_than_25_months_are_pruned() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, proof) = table(&file(&dir), "main").await;
    let old = NaiveDate::from_ymd_opt(2024, 10, 7).unwrap();
    let kept = NaiveDate::from_ymd_opt(2024, 10, 8).unwrap();
    for at in [old, kept] {
        proof
            .record(first(), SHOWN, at, ConsentChoice::NONE)
            .await
            .unwrap();
    }
    proof
        .record(first(), SHOWN, day(11, 8), ConsentChoice::NONE)
        .await
        .unwrap();

    let counts = proof.counts(NaiveDate::MIN, NaiveDate::MAX).await.unwrap();
    let days: Vec<NaiveDate> = counts.iter().map(|count| count.day).collect();
    assert_eq!(days, [kept, day(11, 8)]);
    // Versions are kept for as long as the site exists.
    let versions = proof.versions().await.unwrap();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].first_day, old);
}

#[tokio::test]
async fn each_site_sees_only_its_own_proof() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, main) = table(&file(&dir), "main").await;
    let (_db, other) = table(&file(&dir), "other").await;
    main.record(first(), SHOWN, day(11, 8), ConsentChoice::NONE)
        .await
        .unwrap();
    assert_eq!(other.versions().await.unwrap().len(), 0);
    let none = other.counts(day(11, 1), day(11, 30)).await.unwrap();
    assert_eq!(none.len(), 0);
    other
        .record(first(), SHOWN, day(11, 8), ConsentChoice::NONE)
        .await
        .unwrap();
    let counts = main.counts(day(11, 1), day(11, 30)).await.unwrap();
    assert_eq!(counts[0].choices, 1);
}

#[tokio::test]
async fn settings_that_are_not_json_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, proof) = table(&file(&dir), "main").await;
    let refused = proof.shown(first(), "not json", day(11, 8)).await;
    assert!(refused.is_err());
    assert_eq!(proof.versions().await.unwrap().len(), 0);
}

#[tokio::test]
async fn no_proof_table_has_an_address_or_identifier_column() {
    let dir = tempfile::tempdir().unwrap();
    let path = file(&dir);
    let (db, _proof) = table(&path, "main").await;
    db.close().await;
    let mut connection = <sqlx::sqlite::SqliteConnectOptions as sqlx::ConnectOptions>::connect(
        &sqlx::sqlite::SqliteConnectOptions::new().filename(&path),
    )
    .await
    .unwrap();
    let columns: Vec<String> = sqlx::query_scalar(
        "SELECT name FROM pragma_table_info('consent_versions') \
         UNION ALL SELECT name FROM pragma_table_info('consent_counts')",
    )
    .fetch_all(&mut connection)
    .await
    .unwrap();
    let allowed = [
        "id",
        "site_id",
        "version",
        "shown",
        "first_day",
        "last_day",
        "day",
        "version_id",
        "choices",
        "preferences",
        "analytics",
        "marketing",
        "external_media",
    ];
    for column in &columns {
        assert!(
            allowed.contains(&column.as_str()),
            "unexpected column {column}"
        );
    }
    assert_eq!(columns.len(), 14);
}

#[tokio::test]
async fn the_admin_api_reads_the_same_proof() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, proof) = table(&file(&dir), "main").await;
    let media = ConsentChoice::allowing([ConsentCategory::ExternalMedia]);
    proof
        .record(first(), SHOWN, day(11, 8), media)
        .await
        .unwrap();
    proof
        .record(first(), SHOWN, day(11, 8), ConsentChoice::NONE)
        .await
        .unwrap();
    let read: &dyn ConsentProof = &proof;
    let counts = read.counts(day(11, 1), day(11, 8)).await.unwrap();
    assert_eq!(counts.len(), 1);
    assert_eq!((counts[0].day, counts[0].choices), (day(11, 8), 2));
    assert_eq!(counts[0].version, first().to_string());
    assert_eq!(counts[0].allowed(ConsentCategory::ExternalMedia), 1);
    assert_eq!(counts[0].allowed(ConsentCategory::Marketing), 0);
    let versions = read.versions().await.unwrap();
    assert_eq!(versions.len(), 1);
    assert_eq!(
        (versions[0].first_day, versions[0].shown.as_str()),
        (day(11, 8), SHOWN)
    );
}
