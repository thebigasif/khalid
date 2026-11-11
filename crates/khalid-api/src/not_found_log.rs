//! The log of addresses that answered 404, so a person can see what readers
//! and search engines asked for and redirect it in one step (KH-SEO-8).

use std::error::Error;

use chrono::{DateTime, Utc};

use crate::content_store::BoxFuture;

/// The most addresses the log holds, so a scanner trying random addresses
/// cannot grow the database without end. Once full, a new address takes
/// the place of the one hit least, and of those the one seen longest ago,
/// so real 404s are still recorded after a scanner has filled the log.
pub const MAX_MISSES: usize = 10_000;

/// An address that answered 404, with how often and when.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Miss {
    /// The address as requested, percent-encoding undone.
    pub path: String,
    /// How many requests it answered.
    pub hits: u64,
    /// The first request.
    pub first_seen: DateTime<Utc>,
    /// The latest request.
    pub last_seen: DateTime<Utc>,
}

impl Miss {
    /// A logged address, as a log reads it back.
    pub fn new(path: &str, hits: u64, first_seen: DateTime<Utc>, last_seen: DateTime<Utc>) -> Self {
        Self {
            path: path.to_owned(),
            hits,
            first_seen,
            last_seen,
        }
    }
}

/// The site's 404 log. `khalid-db` keeps it in SQLite as visitor data;
/// `MemoryNotFoundLog`, from the `test-util` feature, keeps it in memory
/// for tests.
pub trait NotFoundLog: Send + Sync {
    /// Counts one 404 for `path` at `at`, keeping at most [`MAX_MISSES`]
    /// addresses.
    ///
    /// # Errors
    ///
    /// [`NotFoundLogError`] when the log cannot be written.
    fn record<'a>(
        &'a self,
        path: &'a str,
        at: DateTime<Utc>,
    ) -> BoxFuture<'a, Result<(), NotFoundLogError>>;

    /// Up to `limit` logged addresses, most hit first, then by address.
    ///
    /// # Errors
    ///
    /// [`NotFoundLogError`] when the log cannot be read.
    fn misses(&self, limit: u32) -> BoxFuture<'_, Result<Vec<Miss>, NotFoundLogError>>;

    /// Drops `path` from the log, answering whether it was there.
    ///
    /// # Errors
    ///
    /// [`NotFoundLogError`] when the log cannot be written.
    fn forget<'a>(&'a self, path: &'a str) -> BoxFuture<'a, Result<bool, NotFoundLogError>>;
}

/// The 404 log could not be read or written.
#[derive(Debug, thiserror::Error)]
#[error("the log of addresses not found could not be read or written")]
pub struct NotFoundLogError {
    #[source]
    source: Box<dyn Error + Send + Sync>,
}

impl NotFoundLogError {
    /// Wraps what the storage reported.
    pub fn new(source: impl Error + Send + Sync + 'static) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}

#[cfg(any(test, feature = "test-util"))]
pub use memory::MemoryNotFoundLog;

#[cfg(any(test, feature = "test-util"))]
mod memory {
    use std::collections::BTreeMap;
    use std::sync::{Mutex, PoisonError};

    use chrono::{DateTime, Utc};

    use super::{BoxFuture, MAX_MISSES, Miss, NotFoundLog, NotFoundLogError};

    /// A 404 log held in memory, for tests.
    #[derive(Debug, Default)]
    pub struct MemoryNotFoundLog(Mutex<BTreeMap<String, Miss>>);

    impl MemoryNotFoundLog {
        fn record_now(&self, path: &str, at: DateTime<Utc>) {
            let mut misses = self.0.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some(miss) = misses.get_mut(path) {
                miss.hits += 1;
                miss.last_seen = at;
                return;
            }
            if misses.len() >= MAX_MISSES {
                let least = misses
                    .values()
                    .min_by(|a, b| {
                        (a.hits, a.last_seen, &a.path).cmp(&(b.hits, b.last_seen, &b.path))
                    })
                    .map(|miss| miss.path.clone());
                if let Some(least) = least {
                    misses.remove(&least);
                }
            }
            misses.insert(path.to_owned(), Miss::new(path, 1, at, at));
        }

        fn misses_now(&self, limit: u32) -> Vec<Miss> {
            let misses = self.0.lock().unwrap_or_else(PoisonError::into_inner);
            let mut misses: Vec<Miss> = misses.values().cloned().collect();
            misses.sort_by(|a, b| b.hits.cmp(&a.hits).then_with(|| a.path.cmp(&b.path)));
            misses.truncate(usize::try_from(limit).unwrap_or(usize::MAX));
            misses
        }
    }

    /// Memory answers at once, so each future is ready when returned.
    impl NotFoundLog for MemoryNotFoundLog {
        fn record<'a>(
            &'a self,
            path: &'a str,
            at: DateTime<Utc>,
        ) -> BoxFuture<'a, Result<(), NotFoundLogError>> {
            self.record_now(path, at);
            Box::pin(std::future::ready(Ok(())))
        }

        fn misses(&self, limit: u32) -> BoxFuture<'_, Result<Vec<Miss>, NotFoundLogError>> {
            Box::pin(std::future::ready(Ok(self.misses_now(limit))))
        }

        fn forget<'a>(&'a self, path: &'a str) -> BoxFuture<'a, Result<bool, NotFoundLogError>> {
            let mut misses = self.0.lock().unwrap_or_else(PoisonError::into_inner);
            let forgotten = misses.remove(path).is_some();
            Box::pin(std::future::ready(Ok(forgotten)))
        }
    }

    #[cfg(test)]
    mod tests {
        use chrono::TimeZone as _;

        use super::*;

        #[tokio::test]
        async fn a_full_log_drops_the_least_hit_oldest_address_for_a_new_one() {
            let at = |hour| Utc.with_ymd_and_hms(2026, 11, 3, hour, 0, 0).unwrap();
            let log = MemoryNotFoundLog::default();
            log.record("/blog/old-post/", at(8)).await.unwrap();
            log.record("/blog/old-post/", at(8)).await.unwrap();
            log.record("/blog/stale-probe/", at(7)).await.unwrap();
            for n in 2..MAX_MISSES {
                log.record(&format!("/blog/probe-{n}/"), at(9))
                    .await
                    .unwrap();
            }
            log.record("/blog/one-more/", at(11)).await.unwrap();
            let misses = log.misses(u32::MAX).await.unwrap();
            assert_eq!(misses.len(), MAX_MISSES);
            let paths: Vec<&str> = misses.iter().map(|m| m.path.as_str()).collect();
            assert!(paths.contains(&"/blog/one-more/"));
            assert!(paths.contains(&"/blog/old-post/"));
            assert!(!paths.contains(&"/blog/stale-probe/"));
        }
    }
}
