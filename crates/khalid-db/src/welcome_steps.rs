//! Welcome sequences on SQLite (G39): which welcome step each subscriber
//! was sent.

use chrono::{DateTime, Utc};
use khalid_core::Slug;
use khalid_newsletter::{EmailAddress, Recipient, SubscriberId};

use crate::subscriber_table::stamp;
use crate::{DbError, SubscriberTable};

impl SubscriberTable {
    pub(crate) async fn claim_welcome_now(
        &self,
        subscriber: SubscriberId,
        list: &Slug,
        day: u16,
        at: DateTime<Utc>,
    ) -> Result<bool, DbError> {
        let claimed = sqlx::query(
            "INSERT INTO welcome_steps (site_id, subscriber_id, list, day, sent_at) \
             SELECT site_id, id, ?3, ?4, ?5 FROM subscribers WHERE site_id = ?1 AND id = ?2 \
             ON CONFLICT DO NOTHING",
        )
        .bind(self.site.as_row())
        .bind(subscriber.as_row())
        .bind(list.as_str())
        .bind(day)
        .bind(stamp(at))
        .execute(&self.db.writer)
        .await?;
        Ok(claimed.rows_affected() == 1)
    }

    /// A step is due `day` days after the later of the subscriber's
    /// confirmation and their joining `list`.
    pub(crate) async fn welcomes_due_now(
        &self,
        list: &Slug,
        day: u16,
        (after, until): (DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<Recipient>, DbError> {
        let rows: Vec<(i64, String)> = sqlx::query_as(
            "SELECT s.id, s.email FROM subscribers s \
             JOIN subscriber_lists l ON l.subscriber_id = s.id AND l.list = ?2 \
             WHERE s.site_id = ?1 AND s.status = 'confirmed' AND s.confirmed_at IS NOT NULL \
             AND max(julianday(l.joined_at), julianday(s.confirmed_at)) + ?3 > julianday(?4) \
             AND max(julianday(l.joined_at), julianday(s.confirmed_at)) + ?3 <= julianday(?5) \
             AND NOT EXISTS (SELECT 1 FROM welcome_steps w \
                 WHERE w.subscriber_id = s.id AND w.list = ?2 AND w.day = ?3) \
             ORDER BY s.id",
        )
        .bind(self.site.as_row())
        .bind(list.as_str())
        .bind(day)
        .bind(stamp(after))
        .bind(stamp(until))
        .fetch_all(&self.db.readers)
        .await?;
        rows.into_iter()
            .map(|(id, email)| match EmailAddress::parse(&email) {
                Ok(email) => Ok(Recipient::new(SubscriberId::from_row(id), email)),
                Err(_) => Err(DbError::Unreadable {
                    column: "email",
                    value: email,
                }),
            })
            .collect()
    }
}
