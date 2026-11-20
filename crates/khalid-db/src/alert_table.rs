//! The alerts already raised on SQLite, for `khalid-alerts` (KH-GRO-6).

use std::collections::BTreeSet;

use chrono::{DateTime, SecondsFormat, Utc};
use khalid_alerts::{BoxFuture, Finding, Held, Kind, Raised, RaisedError};

use crate::{Database, DbError, SiteId};

/// One site's raised alert keys in `alerts_raised`. It is visitor data: a
/// rebuild keeps it. Cloning shares the connections.
#[derive(Debug, Clone)]
pub struct AlertTable {
    db: Database,
    site: SiteId,
}

impl AlertTable {
    /// The raised alerts of `site` in `db`.
    pub fn new(db: Database, site: SiteId) -> Self {
        Self { db, site }
    }

    async fn raised_now(&self, kind: Kind) -> Result<BTreeSet<String>, DbError> {
        let prefix = kind.key("");
        let keys: Vec<String> = sqlx::query_scalar(
            "SELECT key FROM alerts_raised \
             WHERE site_id = ?1 AND substr(key, 1, length(?2)) = ?2",
        )
        .bind(self.site.as_row())
        .bind(&prefix)
        .fetch_all(&self.db.readers)
        .await?;
        Ok(keys.into_iter().collect())
    }

    async fn remember_now(&self, findings: &[Finding], at: DateTime<Utc>) -> Result<(), DbError> {
        let at = text(at);
        let mut transaction = self.db.writer.begin().await?;
        for finding in findings {
            sqlx::query(
                "INSERT INTO alerts_raised (site_id, key, raised_at, line) VALUES (?1, ?2, ?3, ?4) \
                 ON CONFLICT (site_id, key) DO UPDATE \
                 SET raised_at = excluded.raised_at, line = excluded.line",
            )
            .bind(self.site.as_row())
            .bind(&finding.key)
            .bind(&at)
            .bind(&finding.line)
            .execute(&mut *transaction)
            .await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    async fn held_now(&self) -> Result<Vec<Held>, DbError> {
        let rows: Vec<(String, String, String)> = sqlx::query_as(
            "SELECT key, line, raised_at FROM alerts_raised WHERE site_id = ?1 ORDER BY key",
        )
        .bind(self.site.as_row())
        .fetch_all(&self.db.readers)
        .await?;
        let mut held = Vec::with_capacity(rows.len());
        for (key, line, at) in rows {
            let raised_at = DateTime::parse_from_rfc3339(&at)
                .map(|at| at.with_timezone(&Utc))
                .map_err(|_| DbError::Unreadable {
                    column: "alerts_raised.raised_at",
                    value: at.clone(),
                })?;
            held.push(Held {
                key,
                line,
                raised_at,
            });
        }
        Ok(held)
    }

    async fn forget_now(&self, keys: &[String]) -> Result<(), DbError> {
        let mut transaction = self.db.writer.begin().await?;
        for key in keys {
            sqlx::query("DELETE FROM alerts_raised WHERE site_id = ?1 AND key = ?2")
                .bind(self.site.as_row())
                .bind(key)
                .execute(&mut *transaction)
                .await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    async fn forget_before_now(&self, before: DateTime<Utc>) -> Result<(), DbError> {
        sqlx::query("DELETE FROM alerts_raised WHERE site_id = ?1 AND raised_at < ?2")
            .bind(self.site.as_row())
            .bind(text(before))
            .execute(&self.db.writer)
            .await?;
        Ok(())
    }
}

impl Raised for AlertTable {
    fn raised(&self, kind: Kind) -> BoxFuture<'_, Result<BTreeSet<String>, RaisedError>> {
        Box::pin(async move { self.raised_now(kind).await.map_err(RaisedError::new) })
    }

    fn remember<'a>(
        &'a self,
        findings: &'a [Finding],
        at: DateTime<Utc>,
    ) -> BoxFuture<'a, Result<(), RaisedError>> {
        Box::pin(async move {
            self.remember_now(findings, at)
                .await
                .map_err(RaisedError::new)
        })
    }

    fn held(&self) -> BoxFuture<'_, Result<Vec<Held>, RaisedError>> {
        Box::pin(async move { self.held_now().await.map_err(RaisedError::new) })
    }

    fn forget<'a>(&'a self, keys: &'a [String]) -> BoxFuture<'a, Result<(), RaisedError>> {
        Box::pin(async move { self.forget_now(keys).await.map_err(RaisedError::new) })
    }

    fn forget_before(&self, before: DateTime<Utc>) -> BoxFuture<'_, Result<(), RaisedError>> {
        Box::pin(async move {
            self.forget_before_now(before)
                .await
                .map_err(RaisedError::new)
        })
    }
}

/// `at` as the table writes it: RFC 3339 in UTC, to the second, so it
/// sorts as text.
fn text(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Secs, true)
}
