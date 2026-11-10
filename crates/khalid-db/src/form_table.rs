//! Form answers on SQLite, kept for `khalid-forms` and read back, or
//! deleted, for the Admin API.

use chrono::{DateTime, SecondsFormat, Utc};
use khalid_api::content_store::BoxFuture;
use khalid_api::form_answers::{
    AnswerField, AnswerPage, FormAnswer, FormAnswers, FormAnswersError, FormSummary,
};
use khalid_forms::{FormId, FormStore, FormStoreError, Submission};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::{Database, DbError, SiteId};

/// One site's form answers in `form_submissions`, a visitor table a
/// rebuild never touches. Each row's `fields` is a JSON array of
/// `{"name", "value"}` objects in the order the form sent them. Every
/// query is bound to the site. Cloning shares the connections.
#[derive(Debug, Clone)]
pub struct FormTable {
    db: Database,
    site: SiteId,
}

impl FormTable {
    /// The form answers of `site` in `db`.
    pub fn new(db: Database, site: SiteId) -> Self {
        Self { db, site }
    }

    /// Every answer kept for `form`, oldest first, each with when it was
    /// sent.
    ///
    /// # Errors
    ///
    /// [`DbError::Query`] when the database cannot be read, and
    /// [`DbError::Unreadable`] when a row holds what this table never
    /// writes.
    pub async fn of_form(
        &self,
        form: &FormId,
    ) -> Result<Vec<(Submission, DateTime<Utc>)>, DbError> {
        let rows = sqlx::query(
            "SELECT s.id, s.page_path, s.created_at, \
             json_extract(f.value, '$.name') AS name, json_extract(f.value, '$.value') AS value \
             FROM form_submissions AS s, json_each(s.fields) AS f \
             WHERE s.site_id = ?1 AND s.form = ?2 AND s.status = 'received' \
             ORDER BY s.created_at, s.id, f.key",
        )
        .bind(self.site.as_row())
        .bind(form.as_str())
        .fetch_all(&self.db.readers)
        .await?;
        let mut kept: Vec<(i64, Submission, DateTime<Utc>)> = Vec::new();
        for row in &rows {
            let id: i64 = row.try_get("id")?;
            let field = (row.try_get("name")?, row.try_get("value")?);
            if let Some((last, submission, _)) = kept.last_mut()
                && *last == id
            {
                submission.fields.push(field);
                continue;
            }
            let page: Option<String> = row.try_get("page_path")?;
            let at = sent_at(row, "created_at")?;
            let submission = Submission {
                form: form.clone(),
                page: page.unwrap_or_default(),
                fields: vec![field],
            };
            kept.push((id, submission, at));
        }
        Ok(kept.into_iter().map(|(_, s, at)| (s, at)).collect())
    }

    async fn insert_now(&self, submission: &Submission, at: DateTime<Utc>) -> Result<(), DbError> {
        let mut tx = self.db.writer.begin().await?;
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO form_submissions (site_id, form, page_path, fields, status, created_at) \
             VALUES (?1, ?2, ?3, '[]', 'received', ?4) RETURNING id",
        )
        .bind(self.site.as_row())
        .bind(submission.form.as_str())
        .bind(&submission.page)
        .bind(at.to_rfc3339_opts(SecondsFormat::Micros, true))
        .fetch_one(&mut *tx)
        .await?;
        for (name, value) in &submission.fields {
            sqlx::query(
                "UPDATE form_submissions \
                 SET fields = json_insert(fields, '$[#]', json_object('name', ?1, 'value', ?2)) \
                 WHERE id = ?3",
            )
            .bind(name)
            .bind(value)
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

impl FormTable {
    async fn forms_now(&self) -> Result<Vec<FormSummary>, DbError> {
        let rows = sqlx::query(
            "SELECT form, COUNT(*) AS answers, MAX(created_at) AS last_at \
             FROM form_submissions WHERE site_id = ?1 AND status = 'received' \
             GROUP BY form ORDER BY last_at DESC, form",
        )
        .bind(self.site.as_row())
        .fetch_all(&self.db.readers)
        .await?;
        let summary = |row: &SqliteRow| {
            let answers: i64 = row.try_get("answers")?;
            Ok(FormSummary {
                id: row.try_get("form")?,
                answers: u64::try_from(answers).unwrap_or_default(),
                last_at: sent_at(row, "last_at")?,
            })
        };
        rows.iter().map(summary).collect()
    }

    async fn answers_now(
        &self,
        form: &str,
        offset: u64,
        limit: u32,
    ) -> Result<AnswerPage, DbError> {
        // A name no form block can have was never kept.
        let Ok(form) = FormId::try_new(form) else {
            return Ok(AnswerPage::default());
        };
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM form_submissions \
             WHERE site_id = ?1 AND form = ?2 AND status = 'received'",
        )
        .bind(self.site.as_row())
        .bind(form.as_str())
        .fetch_one(&self.db.readers)
        .await?;
        let rows = sqlx::query(
            "SELECT s.id, s.page_path, s.created_at, \
             json_extract(f.value, '$.name') AS name, json_extract(f.value, '$.value') AS value \
             FROM (SELECT id, page_path, created_at, fields FROM form_submissions \
                   WHERE site_id = ?1 AND form = ?2 AND status = 'received' \
                   ORDER BY created_at DESC, id DESC LIMIT ?3 OFFSET ?4) AS s \
             LEFT JOIN json_each(s.fields) AS f \
             ORDER BY s.created_at DESC, s.id DESC, f.key",
        )
        .bind(self.site.as_row())
        .bind(form.as_str())
        .bind(i64::from(limit))
        .bind(i64::try_from(offset).unwrap_or(i64::MAX))
        .fetch_all(&self.db.readers)
        .await?;
        let mut answers: Vec<FormAnswer> = Vec::new();
        for row in &rows {
            let id: i64 = row.try_get("id")?;
            if answers.last().is_none_or(|last| last.id != id) {
                let page: Option<String> = row.try_get("page_path")?;
                answers.push(FormAnswer {
                    id,
                    page: page.unwrap_or_default(),
                    sent_at: sent_at(row, "created_at")?,
                    fields: Vec::new(),
                });
            }
            let name: Option<String> = row.try_get("name")?;
            if let (Some(name), Some(answer)) = (name, answers.last_mut()) {
                let value: Option<String> = row.try_get("value")?;
                let value = value.unwrap_or_default();
                answer.fields.push(AnswerField { name, value });
            }
        }
        let total = u64::try_from(total).unwrap_or_default();
        Ok(AnswerPage { answers, total })
    }

    async fn delete_now(&self, form: &str, answer: i64) -> Result<bool, DbError> {
        // A name no form block can have was never kept.
        let Ok(form) = FormId::try_new(form) else {
            return Ok(false);
        };
        let deleted = sqlx::query(
            "DELETE FROM form_submissions WHERE site_id = ?1 AND form = ?2 AND id = ?3",
        )
        .bind(self.site.as_row())
        .bind(form.as_str())
        .bind(answer)
        .execute(&self.db.writer)
        .await?;
        Ok(deleted.rows_affected() > 0)
    }
}

impl FormAnswers for FormTable {
    fn forms(&self) -> BoxFuture<'_, Result<Vec<FormSummary>, FormAnswersError>> {
        Box::pin(async move { self.forms_now().await.map_err(read_error) })
    }

    fn answers<'a>(
        &'a self,
        form: &'a str,
        offset: u64,
        limit: u32,
    ) -> BoxFuture<'a, Result<AnswerPage, FormAnswersError>> {
        Box::pin(async move {
            self.answers_now(form, offset, limit)
                .await
                .map_err(read_error)
        })
    }

    fn delete<'a>(
        &'a self,
        form: &'a str,
        answer: i64,
    ) -> BoxFuture<'a, Result<bool, FormAnswersError>> {
        Box::pin(async move { self.delete_now(form, answer).await.map_err(read_error) })
    }
}

/// `error` as the Admin API's form answers error.
fn read_error(error: DbError) -> FormAnswersError {
    FormAnswersError(Box::new(error))
}

/// The time in `column` of `row`, which this table writes as RFC 3339.
fn sent_at(row: &SqliteRow, column: &'static str) -> Result<DateTime<Utc>, DbError> {
    let text: String = row.try_get(column)?;
    DateTime::parse_from_rfc3339(&text)
        .map(|time| time.with_timezone(&Utc))
        .map_err(|_| DbError::Unreadable {
            column,
            value: text,
        })
}

impl FormStore for FormTable {
    async fn insert(
        &self,
        submission: &Submission,
        at: DateTime<Utc>,
    ) -> Result<(), FormStoreError> {
        self.insert_now(submission, at)
            .await
            .map_err(|error| FormStoreError::Write {
                source: Box::new(error),
            })
    }
}
