//! Readers added by staff, and lists of readers in and out, as `khalid
//! serve` keeps them (backlog row 9). Each reader keeps the status given
//! or the status their old system gave them, and no email goes out: the
//! CTO's answer to the founder question in the backlog.

use chrono::{DateTime, Utc};
use khalid_api::resources::{
    AdminSubscriber, NewSubscriber, ReaderFile, SkippedReader, SubscriberImport, SubscriberStatus,
};
use khalid_api::subscriber_desk::{ImportMode, SubscriberError};
use khalid_core::Slug;
use khalid_db::SubscriberTable;
use khalid_import::subscribers::{Reader, ReaderSource, Standing, read_readers, write_readers};
use khalid_newsletter::{EmailAddress, Lists};

/// Adds `new` to `table` `at` that time, on the newsletters it names or
/// the site's `main`, after checking each exists in `lists`.
pub(super) async fn add(
    table: &SubscriberTable,
    lists: &Lists,
    new: &NewSubscriber,
    at: DateTime<Utc>,
) -> Result<AdminSubscriber, SubscriberError> {
    let email = EmailAddress::parse(&new.email).map_err(|_| SubscriberError::BadAddress)?;
    let reader = NewSubscriber {
        email: email.as_str().to_owned(),
        lists: joined(lists, &new.lists, None).ok_or(SubscriberError::NoList)?,
        ..new.clone()
    };
    let added = table.bring_in(&[reader], at).await;
    let added = added.map_err(SubscriberError::store)?;
    let id = added
        .first()
        .copied()
        .flatten()
        .ok_or(SubscriberError::Taken)?;
    let one = table.one(id).await.map_err(SubscriberError::store)?;
    one.ok_or(SubscriberError::NotFound)
}

/// Brings the readers of `csv` into `table` `at` that time. A reader the
/// file names no newsletter for joins `list`, or `main`. A row whose
/// address cannot take mail, or whose newsletter the site lacks, is
/// skipped with why; the address is never repeated. A check counts the
/// same way and keeps no one.
pub(super) async fn import(
    table: &SubscriberTable,
    lists: &Lists,
    (csv, list, mode): (&str, Option<&Slug>, ImportMode),
    at: DateTime<Utc>,
) -> Result<SubscriberImport, SubscriberError> {
    if lists.find(list.map(Slug::as_str)).is_none() {
        return Err(SubscriberError::NoList);
    }
    let read = read_readers(csv).map_err(|error| SubscriberError::NotReaders(error.to_string()))?;
    let mut skipped: Vec<(usize, String)> = read
        .unread
        .iter()
        .map(|(line, why)| (*line, why.to_string()))
        .collect();
    let mut readers = Vec::with_capacity(read.readers.len());
    for (line, reader) in read.readers {
        let Ok(email) = EmailAddress::parse(&reader.email) else {
            skipped.push((line, "its address cannot take mail".to_owned()));
            continue;
        };
        let Some(joins) = joined(lists, &reader.lists, list) else {
            skipped.push((
                line,
                "it names a newsletter the site does not have".to_owned(),
            ));
            continue;
        };
        readers.push(NewSubscriber {
            email: email.as_str().to_owned(),
            status: status(reader.standing),
            created_at: reader.since,
            lists: joins,
            labels: reader.labels,
        });
    }
    let added = match mode {
        ImportMode::Save => table.bring_in(&readers, at).await,
        ImportMode::Check => table.count_in(&readers, at).await,
    };
    let added = added.map_err(SubscriberError::store)?;
    let new = added.iter().filter(|id| id.is_some()).count();
    let count = |n: usize| u64::try_from(n).unwrap_or(u64::MAX);
    skipped.sort_by_key(|(line, _)| *line);
    let skipped = skipped
        .into_iter()
        .map(|(line, why)| SkippedReader::new(count(line), why))
        .collect();
    let source = match read.source {
        ReaderSource::Substack => ReaderFile::Substack,
        ReaderSource::Ghost => ReaderFile::Ghost,
        ReaderSource::Khalid => ReaderFile::Khalid,
    };
    let totals = (count(new), count(added.len().saturating_sub(new)));
    let done = SubscriberImport::new(source, totals, skipped);
    Ok(match mode {
        ImportMode::Save => done,
        ImportMode::Check => done.only_counted(),
    })
}

/// Every reader in `table` as Khalid's CSV export, oldest first.
pub(super) async fn export(table: &SubscriberTable) -> Result<String, SubscriberError> {
    let everyone = table.everyone().await.map_err(SubscriberError::store)?;
    let readers: Vec<Reader> = everyone
        .into_iter()
        .map(|each| Reader {
            email: each.email,
            standing: match each.status {
                SubscriberStatus::Pending => Standing::Pending,
                SubscriberStatus::Confirmed => Standing::Confirmed,
                SubscriberStatus::Unsubscribed => Standing::Unsubscribed,
            },
            since: Some(each.created_at),
            lists: each.lists,
            labels: each.labels,
        })
        .collect();
    Ok(write_readers(&readers))
}

/// The newsletters a reader joins: `named` when each is one of `lists`,
/// else `fallback` or `main` when `named` is empty. `None` when one of
/// them does not exist.
fn joined(lists: &Lists, named: &[Slug], fallback: Option<&Slug>) -> Option<Vec<Slug>> {
    if named.is_empty() {
        return lists
            .find(fallback.map(Slug::as_str))
            .map(|list| vec![list]);
    }
    named
        .iter()
        .map(|name| lists.find(Some(name.as_str())))
        .collect()
}

/// The status a reader keeps from where `standing` says they stood.
fn status(standing: Standing) -> SubscriberStatus {
    match standing {
        Standing::Pending => SubscriberStatus::Pending,
        Standing::Confirmed => SubscriberStatus::Confirmed,
        Standing::Unsubscribed => SubscriberStatus::Unsubscribed,
    }
}
