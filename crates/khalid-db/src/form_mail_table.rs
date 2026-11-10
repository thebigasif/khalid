//! Form answer emails that could not be sent, kept for another try (G40).

use chrono::{DateTime, SecondsFormat, Utc};
use khalid_core::Slug;
use khalid_forms::FormId;
use khalid_newsletter::{EmailAddress, Mail};
use sqlx::Row as _;
use sqlx::sqlite::SqliteRow;

use crate::{Database, DbError, SiteId};

/// One site's unsent form answer emails in `form_mail`, a visitor table a
/// rebuild never touches. A row is added when the first send fails and
/// removed once the email is sent. Every query is bound to the site.
/// Cloning shares the connections.
#[derive(Debug, Clone)]
pub struct FormMailTable {
    db: Database,
    site: SiteId,
}

/// A form answer email waiting for its next try.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitingMail {
    /// The row, which [`FormMailTable::sent`] and
    /// [`FormMailTable::failed_again`] name.
    pub id: i64,
    /// The form whose answer it carries.
    pub form: FormId,
    /// The email as first written.
    pub mail: Mail,
    /// How many tries failed so far, at least 1.
    pub failures: usize,
}

/// A form answer email Khalid stopped trying to send.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GivenUpMail {
    /// The form whose answer it carries.
    pub form: FormId,
    /// When the answer came, which is when the first send failed.
    pub first_at: DateTime<Utc>,
    /// Why the last try failed, with no email text and no secret.
    pub last_error: String,
}

/// What one failed try records: when, why, and when to try next, or `None`
/// to give up.
#[derive(Debug, Clone, Copy)]
pub struct Failure<'a> {
    /// When the try failed.
    pub at: DateTime<Utc>,
    /// Why, with no email text and no secret.
    pub error: &'a str,
    /// When to try again, or `None` to give up.
    pub next_at: Option<DateTime<Utc>>,
}

impl FormMailTable {
    /// The unsent form emails of `site` in `db`.
    pub fn new(db: Database, site: SiteId) -> Self {
        Self { db, site }
    }

    /// Keeps `mail` with the answer to `form`, whose first send failed as
    /// `failure` says.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the row cannot be written.
    pub async fn keep(
        &self,
        form: &FormId,
        mail: &Mail,
        failure: Failure<'_>,
    ) -> Result<(), DbError> {
        let at = stamp(failure.at);
        sqlx::query(
            "INSERT INTO form_mail (site_id, form, to_address, subject, body, first_at, \
             failures, last_at, last_error, next_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?6, ?7, ?8)",
        )
        .bind(self.site.as_row())
        .bind(form.as_str())
        .bind(mail.to.as_str())
        .bind(&mail.subject)
        .bind(&mail.text)
        .bind(&at)
        .bind(failure.error)
        .bind(failure.next_at.map(stamp))
        .execute(&self.db.writer)
        .await?;
        Ok(())
    }

    /// Every email whose next try is due at `now`, the longest waiting
    /// first.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the table cannot be read, and
    /// [`DbError::Unreadable`] when a row holds what this table never
    /// writes.
    pub async fn due(&self, now: DateTime<Utc>) -> Result<Vec<WaitingMail>, DbError> {
        let rows = sqlx::query(
            "SELECT id, form, to_address, subject, body, failures FROM form_mail \
             WHERE site_id = ?1 AND next_at IS NOT NULL AND next_at <= ?2 \
             ORDER BY next_at, id",
        )
        .bind(self.site.as_row())
        .bind(stamp(now))
        .fetch_all(&self.db.readers)
        .await?;
        rows.iter().map(waiting).collect()
    }

    /// Forgets the email in row `id`, which was just sent.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the row cannot be removed.
    pub async fn sent(&self, id: i64) -> Result<(), DbError> {
        sqlx::query("DELETE FROM form_mail WHERE id = ?1 AND site_id = ?2")
            .bind(id)
            .bind(self.site.as_row())
            .execute(&self.db.writer)
            .await?;
        Ok(())
    }

    /// Records one more failed try of the email in row `id`.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the row cannot be written.
    pub async fn failed_again(&self, id: i64, failure: Failure<'_>) -> Result<(), DbError> {
        sqlx::query(
            "UPDATE form_mail SET failures = failures + 1, last_at = ?1, last_error = ?2, \
             next_at = ?3 WHERE id = ?4 AND site_id = ?5",
        )
        .bind(stamp(failure.at))
        .bind(failure.error)
        .bind(failure.next_at.map(stamp))
        .bind(id)
        .bind(self.site.as_row())
        .execute(&self.db.writer)
        .await?;
        Ok(())
    }

    /// The emails Khalid gave up on at or after `since`, oldest first.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the table cannot be read, and
    /// [`DbError::Unreadable`] when a row holds what this table never
    /// writes.
    pub async fn given_up_since(&self, since: DateTime<Utc>) -> Result<Vec<GivenUpMail>, DbError> {
        let rows = sqlx::query(
            "SELECT form, first_at, last_error FROM form_mail \
             WHERE site_id = ?1 AND next_at IS NULL AND last_at >= ?2 ORDER BY first_at, id",
        )
        .bind(self.site.as_row())
        .bind(stamp(since))
        .fetch_all(&self.db.readers)
        .await?;
        rows.iter().map(given_up).collect()
    }

    /// The emails Khalid gave up on at or after `since` for the site
    /// called `handle` in `db`, oldest first. It only reads, so `khalid
    /// doctor` can ask without adding the site or changing the schema.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the table cannot be read, such as in a
    /// database from before it was added, and [`DbError::Unreadable`] when
    /// a row holds what this table never writes.
    pub async fn given_up_of(
        db: &Database,
        handle: &Slug,
        since: DateTime<Utc>,
    ) -> Result<Vec<GivenUpMail>, DbError> {
        let rows = sqlx::query(
            "SELECT m.form, m.first_at, m.last_error FROM form_mail AS m \
             JOIN sites AS s ON s.id = m.site_id \
             WHERE s.handle = ?1 AND m.next_at IS NULL AND m.last_at >= ?2 \
             ORDER BY m.first_at, m.id",
        )
        .bind(handle.as_str())
        .bind(stamp(since))
        .fetch_all(&db.readers)
        .await?;
        rows.iter().map(given_up).collect()
    }

    /// Forgets the emails Khalid gave up on before `before`, so the table
    /// stays small.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the rows cannot be removed.
    pub async fn forget_given_up_before(&self, before: DateTime<Utc>) -> Result<(), DbError> {
        sqlx::query(
            "DELETE FROM form_mail WHERE site_id = ?1 AND next_at IS NULL AND last_at < ?2",
        )
        .bind(self.site.as_row())
        .bind(stamp(before))
        .execute(&self.db.writer)
        .await?;
        Ok(())
    }
}

/// The waiting email in `row`.
fn waiting(row: &SqliteRow) -> Result<WaitingMail, DbError> {
    let to: String = row.try_get("to_address")?;
    let to = EmailAddress::parse(&to).map_err(|_| unreadable("to_address", &to))?;
    let failures: i64 = row.try_get("failures")?;
    Ok(WaitingMail {
        id: row.try_get("id")?,
        form: form(row)?,
        mail: Mail::new(to, row.try_get("subject")?, row.try_get("body")?),
        failures: usize::try_from(failures)
            .map_err(|_| unreadable("failures", &failures.to_string()))?,
    })
}

/// The given-up email in `row`.
fn given_up(row: &SqliteRow) -> Result<GivenUpMail, DbError> {
    Ok(GivenUpMail {
        form: form(row)?,
        first_at: time(row, "first_at")?,
        last_error: row.try_get("last_error")?,
    })
}

/// The form `row` names.
fn form(row: &SqliteRow) -> Result<FormId, DbError> {
    let form: String = row.try_get("form")?;
    FormId::try_new(&form).map_err(|_| unreadable("form", &form))
}

/// The time in `column` of `row`.
fn time(row: &SqliteRow, column: &'static str) -> Result<DateTime<Utc>, DbError> {
    let text: String = row.try_get(column)?;
    DateTime::parse_from_rfc3339(&text)
        .map(|time| time.with_timezone(&Utc))
        .map_err(|_| unreadable(column, &text))
}

/// A time as the table writes it: UTC to the microsecond, so text order is
/// time order.
fn stamp(time: DateTime<Utc>) -> String {
    time.to_rfc3339_opts(SecondsFormat::Micros, true)
}

fn unreadable(column: &'static str, value: &str) -> DbError {
    DbError::Unreadable {
        column,
        value: value.to_owned(),
    }
}
