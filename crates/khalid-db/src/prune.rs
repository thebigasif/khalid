//! Deletes the visitor rows nobody needs any more, so the database does not
//! grow without end on a long-running site: 404s not asked for in a while,
//! finished webhook deliveries, and sessions and sign-in links that can no
//! longer be used.

use chrono::{DateTime, SecondsFormat, TimeDelta, Utc};
use khalid_auth::IDLE;

use crate::{Database, DbError, SiteId};

/// How long an address stays in the 404 log after its last request: long
/// enough for a monthly look, short enough that scanners age out.
pub const MISS_DAYS: i64 = 90;

/// How long a webhook delivery that was delivered or given up stays in the
/// log, so the Webhooks screen shows the last month.
pub const DELIVERY_DAYS: i64 = 30;

/// How many rows one prune deleted, table by table.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Pruned {
    /// Addresses in the 404 log not asked for in [`MISS_DAYS`].
    pub misses: u64,
    /// Finished webhook deliveries older than [`DELIVERY_DAYS`], each with
    /// its attempts.
    pub deliveries: u64,
    /// Sessions past their lifetime or idle too long to be used.
    pub sessions: u64,
    /// Sign-in links past their expiry, used or not.
    pub links: u64,
}

impl Pruned {
    /// The rows deleted in all.
    pub fn total(&self) -> u64 {
        self.misses
            .saturating_add(self.deliveries)
            .saturating_add(self.sessions)
            .saturating_add(self.links)
    }
}

/// The prune of one site's visitor rows. Cloning shares the connections.
#[derive(Debug, Clone)]
pub struct Prune {
    db: Database,
    site: SiteId,
}

impl Prune {
    /// The prune of `site` in `db`.
    pub fn new(db: Database, site: SiteId) -> Self {
        Self { db, site }
    }

    /// Deletes the rows of the site that `now` makes old, in one
    /// transaction, and counts them.
    ///
    /// # Errors
    ///
    /// [`DbError`] when the database cannot be written; nothing is deleted
    /// then.
    pub async fn run(&self, now: DateTime<Utc>) -> Result<Pruned, DbError> {
        let site = self.site.as_row();
        let (now_text, idle) = (text(now), text(now - IDLE));
        let mut tx = self.db.writer.begin().await?;
        let misses = sqlx::query(
            "DELETE FROM not_found_log WHERE site_id = ?1 \
             AND julianday(last_seen) < julianday(?2)",
        )
        .bind(site)
        .bind(text(now - TimeDelta::days(MISS_DAYS)))
        .execute(&mut *tx)
        .await?;
        let deliveries = sqlx::query(
            "DELETE FROM webhook_deliveries WHERE site_id = ?1 \
             AND state IN ('delivered', 'gave_up') \
             AND julianday(state_at) < julianday(?2)",
        )
        .bind(site)
        .bind(text(now - TimeDelta::days(DELIVERY_DAYS)))
        .execute(&mut *tx)
        .await?;
        let sessions = sqlx::query(
            "DELETE FROM sessions WHERE site_id = ?1 \
             AND (julianday(expires_at) <= julianday(?2) \
                  OR julianday(coalesce(last_seen_at, created_at)) <= julianday(?3))",
        )
        .bind(site)
        .bind(&now_text)
        .bind(&idle)
        .execute(&mut *tx)
        .await?;
        let links = sqlx::query(
            "DELETE FROM magic_links WHERE site_id = ?1 \
             AND julianday(expires_at) <= julianday(?2)",
        )
        .bind(site)
        .bind(&now_text)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(Pruned {
            misses: misses.rows_affected(),
            deliveries: deliveries.rows_affected(),
            sessions: sessions.rows_affected(),
            links: links.rows_affected(),
        })
    }
}

fn text(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Secs, true)
}
