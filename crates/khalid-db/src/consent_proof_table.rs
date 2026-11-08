//! Consent proof on SQLite (R15, KTD8): every version of what readers were
//! shown in `consent_versions`, and daily counts of their choices in
//! `consent_counts`. No address or reader identifier is stored.

use chrono::{Months, NaiveDate};
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, Transaction};

use crate::{Database, DbError, SiteId};

/// How long daily counts are kept. Versions are kept as long as the site.
const MONTHS_KEPT: Months = Months::new(25);

/// The widest days a read asks for: wider ones would not sort as text.
const FIRST_DAY: NaiveDate = match NaiveDate::from_ymd_opt(1, 1, 1) {
    Some(day) => day,
    None => NaiveDate::MIN,
};
const LAST_DAY: NaiveDate = match NaiveDate::from_ymd_opt(9999, 12, 31) {
    Some(day) => day,
    None => NaiveDate::MAX,
};

/// An optional consent category, one counter each. Necessary is always on,
/// so it has no counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ConsentCategory {
    /// Remembers the reader's settings, such as a language.
    Preferences,
    /// Measures how the site is used.
    Analytics,
    /// Ads and tracking across sites.
    Marketing,
    /// Players and other content loaded from another site.
    ExternalMedia,
}

impl ConsentCategory {
    /// Every optional category, in the order the banner lists them.
    pub const ALL: [Self; 4] = [
        Self::Preferences,
        Self::Analytics,
        Self::Marketing,
        Self::ExternalMedia,
    ];

    fn bit(self) -> u8 {
        match self {
            Self::Preferences => 1,
            Self::Analytics => 2,
            Self::Marketing => 4,
            Self::ExternalMedia => 8,
        }
    }
}

/// The categories one reader allowed in one answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConsentChoice {
    bits: u8,
}

impl ConsentChoice {
    /// An answer that allowed nothing, such as Reject all.
    pub const NONE: Self = Self { bits: 0 };

    /// An answer that allowed `categories` and nothing else.
    pub fn allowing(categories: impl IntoIterator<Item = ConsentCategory>) -> Self {
        let bits = categories
            .into_iter()
            .fold(0, |bits, category| bits | category.bit());
        Self { bits }
    }

    /// Whether this answer allowed `category`.
    pub fn allows(self, category: ConsentCategory) -> bool {
        self.bits & category.bit() != 0
    }

    /// 1 when this answer allowed `category`, else 0: what it adds to the
    /// category's counter.
    fn adds(self, category: ConsentCategory) -> i64 {
        i64::from(self.allows(category))
    }
}

/// One version of what readers were shown, with the days it was in force.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ConsentVersion {
    /// The hash of everything shown that names this version.
    pub version: String,
    /// The shown text and settings, as JSON.
    pub shown: String,
    /// The first day a reader was shown it.
    pub first_day: NaiveDate,
    /// The last day a reader was shown it so far.
    pub last_day: NaiveDate,
}

/// How readers answered on one day under one version.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ConsentCounts {
    /// The day of the answers.
    pub day: NaiveDate,
    /// The version the readers were shown.
    pub version: String,
    /// How many answers there were.
    pub choices: u64,
    allowed: [u64; 4],
}

impl ConsentCounts {
    /// How many of the answers allowed `category`; the rest refused it.
    pub fn allowed(&self, category: ConsentCategory) -> u64 {
        ConsentCategory::ALL
            .iter()
            .zip(self.allowed)
            .find_map(|(each, count)| (*each == category).then_some(count))
            .unwrap_or(0)
    }
}

/// One site's consent proof. Visitor data: a rebuild keeps it. Cloning
/// shares the connections.
#[derive(Debug, Clone)]
pub struct ConsentProofTable {
    db: Database,
    site: SiteId,
}

impl ConsentProofTable {
    /// The consent proof of `site` in `db`.
    pub fn new(db: Database, site: SiteId) -> Self {
        Self { db, site }
    }

    /// Notes that readers were shown `version`, the settings in `shown`,
    /// on `day`: the first time adds the version, later days extend it.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the write fails or `shown` is not JSON.
    pub async fn shown(&self, version: &str, shown: &str, day: NaiveDate) -> Result<(), DbError> {
        let mut tx = self.db.writer.begin().await?;
        self.keep_version(&mut tx, version, shown, day).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Adds one reader's `choice` to the counts of `day` under `version`,
    /// noting the version as [`ConsentProofTable::shown`] does, and drops
    /// counts older than 25 months. The caller rate-limits and runs this
    /// off the request path, so a failure never holds up the reader.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the write fails or `shown` is not JSON.
    pub async fn record(
        &self,
        version: &str,
        shown: &str,
        day: NaiveDate,
        choice: ConsentChoice,
    ) -> Result<(), DbError> {
        let mut tx = self.db.writer.begin().await?;
        let version_id = self.keep_version(&mut tx, version, shown, day).await?;
        sqlx::query(
            "INSERT INTO consent_counts (site_id, day, version_id, choices, \
             preferences, analytics, marketing, external_media) \
             VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6, ?7) \
             ON CONFLICT (site_id, day, version_id) DO UPDATE SET \
             choices = choices + 1, \
             preferences = preferences + excluded.preferences, \
             analytics = analytics + excluded.analytics, \
             marketing = marketing + excluded.marketing, \
             external_media = external_media + excluded.external_media",
        )
        .bind(self.site.as_row())
        .bind(day_text(day))
        .bind(version_id)
        .bind(choice.adds(ConsentCategory::Preferences))
        .bind(choice.adds(ConsentCategory::Analytics))
        .bind(choice.adds(ConsentCategory::Marketing))
        .bind(choice.adds(ConsentCategory::ExternalMedia))
        .execute(&mut *tx)
        .await?;
        if let Some(oldest) = day.checked_sub_months(MONTHS_KEPT) {
            sqlx::query("DELETE FROM consent_counts WHERE site_id = ?1 AND day < ?2")
                .bind(self.site.as_row())
                .bind(day_text(oldest))
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Every version readers were shown, oldest first.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the read fails, or [`DbError::Unreadable`]
    /// when a stored day is not a date.
    pub async fn versions(&self) -> Result<Vec<ConsentVersion>, DbError> {
        let rows = sqlx::query(
            "SELECT version, shown, first_day, last_day FROM consent_versions \
             WHERE site_id = ?1 ORDER BY first_day, id",
        )
        .bind(self.site.as_row())
        .fetch_all(&self.db.readers)
        .await?;
        rows.iter().map(version_row).collect()
    }

    /// The counts of each day from `first` to `last`, both included, by
    /// day and then by version as they were first shown.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the read fails, or [`DbError::Unreadable`]
    /// when a stored day or count cannot be read.
    pub async fn counts(
        &self,
        first: NaiveDate,
        last: NaiveDate,
    ) -> Result<Vec<ConsentCounts>, DbError> {
        let rows = sqlx::query(
            "SELECT c.day, v.version, c.choices, c.preferences, c.analytics, \
             c.marketing, c.external_media FROM consent_counts c \
             JOIN consent_versions v ON v.id = c.version_id \
             WHERE c.site_id = ?1 AND c.day BETWEEN ?2 AND ?3 \
             ORDER BY c.day, v.first_day, v.id",
        )
        .bind(self.site.as_row())
        .bind(day_text(first.max(FIRST_DAY)))
        .bind(day_text(last.min(LAST_DAY)))
        .fetch_all(&self.db.readers)
        .await?;
        rows.iter().map(counts_row).collect()
    }

    /// Adds `version` or widens its days to take in `day`, answering its id.
    async fn keep_version(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        version: &str,
        shown: &str,
        day: NaiveDate,
    ) -> Result<i64, DbError> {
        let id = sqlx::query_scalar(
            "INSERT INTO consent_versions (site_id, version, shown, first_day, last_day) \
             VALUES (?1, ?2, ?3, ?4, ?4) \
             ON CONFLICT (site_id, version) DO UPDATE SET \
             first_day = min(first_day, excluded.first_day), \
             last_day = max(last_day, excluded.last_day) \
             RETURNING id",
        )
        .bind(self.site.as_row())
        .bind(version)
        .bind(shown)
        .bind(day_text(day))
        .fetch_one(&mut **tx)
        .await?;
        Ok(id)
    }
}

/// One stored version, read back.
fn version_row(row: &SqliteRow) -> Result<ConsentVersion, DbError> {
    Ok(ConsentVersion {
        version: row.try_get("version")?,
        shown: row.try_get("shown")?,
        first_day: day(row.try_get("first_day")?)?,
        last_day: day(row.try_get("last_day")?)?,
    })
}

/// One stored day of counts, read back.
fn counts_row(row: &SqliteRow) -> Result<ConsentCounts, DbError> {
    Ok(ConsentCounts {
        day: day(row.try_get("day")?)?,
        version: row.try_get("version")?,
        choices: number(row.try_get("choices")?)?,
        allowed: [
            number(row.try_get("preferences")?)?,
            number(row.try_get("analytics")?)?,
            number(row.try_get("marketing")?)?,
            number(row.try_get("external_media")?)?,
        ],
    })
}

/// A stored count, read back.
fn number(stored: i64) -> Result<u64, DbError> {
    u64::try_from(stored).map_err(|_| DbError::Unreadable {
        column: "consent count",
        value: stored.to_string(),
    })
}

fn day_text(day: NaiveDate) -> String {
    day.format("%Y-%m-%d").to_string()
}

fn day(stored: String) -> Result<NaiveDate, DbError> {
    NaiveDate::parse_from_str(&stored, "%Y-%m-%d").map_err(|_| DbError::Unreadable {
        column: "consent day",
        value: stored,
    })
}
