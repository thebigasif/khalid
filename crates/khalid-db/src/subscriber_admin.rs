//! Subscribers as the Admin API lists and changes them, on SQLite (G39,
//! plan 4.6): a filtered page, one subscriber, labels by identity,
//! forgetting a reader, and each newsletter's counts.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use khalid_api::resources::{AdminSubscriber, SubscriberFilter, SubscriberPage, SubscriberStatus};
use khalid_api::subscriber_desk::LabelChange;
use khalid_core::Slug;

use crate::subscriber_table::stamp;
use crate::{DbError, SubscriberTable};

/// The columns of one subscriber: identity, address, status, the three
/// times, and the lists and labels each joined with commas, which no slug
/// holds. A macro, so `concat!` makes each query one fixed string.
macro_rules! columns {
    () => {
        "SELECT s.id, s.email, s.status, s.created_at, s.confirmed_at, \
         s.unsubscribed_at, \
         (SELECT group_concat(list, ',') FROM (SELECT list FROM subscriber_lists \
          WHERE subscriber_id = s.id ORDER BY list)), \
         (SELECT group_concat(label, ',') FROM (SELECT label FROM subscriber_labels \
          WHERE subscriber_id = s.id ORDER BY label)) "
    };
}

/// Which subscribers of site `?1` a filter keeps: status `?2`, newsletter
/// `?3`, label `?4` and address text `?5`, each skipped when `NULL`.
macro_rules! matching {
    () => {
        "FROM subscribers s WHERE s.site_id = ?1 \
         AND (?2 IS NULL OR s.status = ?2) \
         AND (?3 IS NULL OR EXISTS (SELECT 1 FROM subscriber_lists l \
              WHERE l.subscriber_id = s.id AND l.list = ?3)) \
         AND (?4 IS NULL OR EXISTS (SELECT 1 FROM subscriber_labels b \
              WHERE b.subscriber_id = s.id AND b.label = ?4)) \
         AND (?5 IS NULL OR instr(lower(s.email), lower(?5)) > 0) "
    };
}

/// One row as `columns!` reads it.
type Row = (
    i64,
    String,
    String,
    DateTime<Utc>,
    Option<DateTime<Utc>>,
    Option<DateTime<Utc>>,
    Option<String>,
    Option<String>,
);

impl SubscriberTable {
    /// The subscribers `filter` matches, newest first, on page `page` (from
    /// 1) of `limit` each, and how many match in all.
    ///
    /// # Errors
    ///
    /// [`DbError`] when the table cannot be read, or holds a value no
    /// subscriber has.
    pub async fn page(
        &self,
        filter: &SubscriberFilter,
        (page, limit): (u32, u32),
    ) -> Result<SubscriberPage, DbError> {
        let status = filter.status.map(status_name);
        let list = filter.list.as_ref().map(Slug::as_str);
        let label = filter.label.as_ref().map(Slug::as_str);
        let search = filter.search.as_deref();
        let total: i64 = sqlx::query_scalar(concat!("SELECT count(*) ", matching!()))
            .bind(self.site.as_row())
            .bind(status)
            .bind(list)
            .bind(label)
            .bind(search)
            .fetch_one(&self.db.readers)
            .await?;
        let skip = i64::from(page.saturating_sub(1)) * i64::from(limit);
        let sql = concat!(
            columns!(),
            matching!(),
            "ORDER BY s.id DESC LIMIT ?6 OFFSET ?7"
        );
        let rows: Vec<Row> = sqlx::query_as(sql)
            .bind(self.site.as_row())
            .bind(status)
            .bind(list)
            .bind(label)
            .bind(search)
            .bind(i64::from(limit))
            .bind(skip)
            .fetch_all(&self.db.readers)
            .await?;
        let subscribers = rows.into_iter().map(subscriber).collect::<Result<_, _>>()?;
        Ok(SubscriberPage::new(
            subscribers,
            u64::try_from(total).unwrap_or(0),
        ))
    }

    /// Subscriber `id` in any status, or `None` when there is none.
    ///
    /// # Errors
    ///
    /// [`DbError`] when the table cannot be read.
    pub async fn one(&self, id: i64) -> Result<Option<AdminSubscriber>, DbError> {
        let sql = concat!(
            columns!(),
            "FROM subscribers s WHERE s.site_id = ?1 AND s.id = ?2"
        );
        let row: Option<Row> = sqlx::query_as(sql)
            .bind(self.site.as_row())
            .bind(id)
            .fetch_optional(&self.db.writer)
            .await?;
        row.map(subscriber).transpose()
    }

    /// Gives subscriber `id` the label `label` `at` that time, or takes it
    /// away, as `change` says, and returns them. A reader who unsubscribed
    /// gets no new label. `None` when there is no such subscriber.
    ///
    /// # Errors
    ///
    /// [`DbError`] when the table cannot be read or written.
    pub async fn change_label(
        &self,
        id: i64,
        label: &Slug,
        (change, at): (LabelChange, DateTime<Utc>),
    ) -> Result<Option<AdminSubscriber>, DbError> {
        let query = match change {
            LabelChange::Add => sqlx::query(
                "INSERT INTO subscriber_labels (site_id, subscriber_id, label, added_at) \
                 SELECT site_id, id, ?3, ?4 FROM subscribers \
                 WHERE site_id = ?1 AND id = ?2 AND status != 'unsubscribed' \
                 ON CONFLICT DO NOTHING",
            ),
            LabelChange::Remove => sqlx::query(
                "DELETE FROM subscriber_labels \
                 WHERE site_id = ?1 AND subscriber_id = ?2 AND label = ?3",
            ),
        };
        let query = query.bind(self.site.as_row()).bind(id).bind(label.as_str());
        let query = match change {
            LabelChange::Add => query.bind(stamp(at)),
            LabelChange::Remove => query,
        };
        query.execute(&self.db.writer).await?;
        self.one(id).await
    }

    /// Removes subscriber `id` with their newsletters, labels and links.
    /// The totals of emails sent to them stay, as they name no one.
    /// `false` when there was no such subscriber.
    ///
    /// # Errors
    ///
    /// [`DbError`] when the table cannot be written.
    pub async fn erase(&self, id: i64) -> Result<bool, DbError> {
        let done = sqlx::query("DELETE FROM subscribers WHERE site_id = ?1 AND id = ?2")
            .bind(self.site.as_row())
            .bind(id)
            .execute(&self.db.writer)
            .await?;
        Ok(done.rows_affected() > 0)
    }

    /// Each newsletter anyone joined, by name, with its confirmed and
    /// waiting readers.
    ///
    /// # Errors
    ///
    /// [`DbError`] when the table cannot be read.
    pub async fn list_counts(&self) -> Result<BTreeMap<String, (u64, u64)>, DbError> {
        let rows: Vec<(String, i64, i64)> = sqlx::query_as(
            "SELECT l.list, sum(s.status = 'confirmed'), sum(s.status = 'pending') \
             FROM subscriber_lists l JOIN subscribers s ON s.id = l.subscriber_id \
             WHERE l.site_id = ?1 GROUP BY l.list ORDER BY l.list",
        )
        .bind(self.site.as_row())
        .fetch_all(&self.db.readers)
        .await?;
        let count = |n: i64| u64::try_from(n).unwrap_or(0);
        Ok(rows
            .into_iter()
            .map(|(list, confirmed, pending)| (list, (count(confirmed), count(pending))))
            .collect())
    }
}

/// The name the table keeps `status` under.
fn status_name(status: SubscriberStatus) -> &'static str {
    match status {
        SubscriberStatus::Pending => "pending",
        SubscriberStatus::Confirmed => "confirmed",
        SubscriberStatus::Unsubscribed => "unsubscribed",
    }
}

/// A row as the Admin API sends it.
fn subscriber(row: Row) -> Result<AdminSubscriber, DbError> {
    let (id, email, status, created_at, confirmed_at, unsubscribed_at, lists, labels) = row;
    let status = match status.as_str() {
        "pending" => SubscriberStatus::Pending,
        "confirmed" => SubscriberStatus::Confirmed,
        "unsubscribed" => SubscriberStatus::Unsubscribed,
        _ => {
            return Err(DbError::Unreadable {
                column: "status",
                value: status,
            });
        }
    };
    Ok(AdminSubscriber {
        id,
        email,
        status,
        lists: slugs(lists.as_deref()),
        labels: slugs(labels.as_deref()),
        created_at,
        confirmed_at,
        unsubscribed_at,
    })
}

/// The slugs in `joined`, a list with commas between.
fn slugs(joined: Option<&str>) -> Vec<Slug> {
    let names = joined.unwrap_or_default().split(',');
    names.filter_map(|name| Slug::try_new(name).ok()).collect()
}
