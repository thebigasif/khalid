//! What was raised already, so each problem is sent once while it lasts.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::pin::Pin;

use chrono::{DateTime, Utc};

use crate::{Alert, Finding, Kind};

/// A boxed future, so [`Raised`] can be used as `dyn Raised`.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Where the keys of raised findings are kept: a table in `khalid-db` in
/// the binary, [`MemoryRaised`] in tests.
pub trait Raised: Send + Sync {
    /// Every key of `kind` raised and not forgotten.
    ///
    /// # Errors
    ///
    /// [`RaisedError`] when the keys cannot be read.
    fn raised(&self, kind: Kind) -> BoxFuture<'_, Result<BTreeSet<String>, RaisedError>>;

    /// Keeps the keys of `findings`, with their lines, as raised at `at`.
    ///
    /// # Errors
    ///
    /// [`RaisedError`] when the keys cannot be written.
    fn remember<'a>(
        &'a self,
        findings: &'a [Finding],
        at: DateTime<Utc>,
    ) -> BoxFuture<'a, Result<(), RaisedError>>;

    /// Every finding raised and not forgotten, of every kind, so an owner
    /// with no email sees them too.
    ///
    /// # Errors
    ///
    /// [`RaisedError`] when the keys cannot be read.
    fn held(&self) -> BoxFuture<'_, Result<Vec<Held>, RaisedError>>;

    /// Drops `keys`, so each is raised again if it comes back.
    ///
    /// # Errors
    ///
    /// [`RaisedError`] when the keys cannot be written.
    fn forget<'a>(&'a self, keys: &'a [String]) -> BoxFuture<'a, Result<(), RaisedError>>;

    /// Drops every key raised before `before`, so the store stays small.
    ///
    /// # Errors
    ///
    /// [`RaisedError`] when the keys cannot be written.
    fn forget_before(&self, before: DateTime<Utc>) -> BoxFuture<'_, Result<(), RaisedError>>;
}

/// The raised keys could not be read or written.
#[derive(Debug, thiserror::Error)]
#[error("the alerts already raised could not be read or written")]
pub struct RaisedError {
    #[source]
    source: Box<dyn Error + Send + Sync>,
}

impl RaisedError {
    /// Wraps the store's own `source`.
    pub fn new(source: impl Error + Send + Sync + 'static) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}

/// The alert of `kind` for the `findings` not raised before, at `at`, or
/// `None` when every one was. Its keys are kept before it is sent, so a
/// failed send is not tried again; the webhook log retries its own.
///
/// When `kind` [clears](Kind::clears), `findings` is the whole picture, so
/// a key of `kind` no longer found is forgotten and raised again if it
/// comes back.
///
/// # Errors
///
/// [`RaisedError`] when `store` cannot be read or written.
pub async fn fresh(
    store: &dyn Raised,
    kind: Kind,
    findings: Vec<Finding>,
    at: DateTime<Utc>,
) -> Result<Option<Alert>, RaisedError> {
    let before = store.raised(kind).await?;
    if kind.clears() {
        let found: BTreeSet<&String> = findings.iter().map(|finding| &finding.key).collect();
        let gone: Vec<String> = before
            .iter()
            .filter(|key| !found.contains(key))
            .cloned()
            .collect();
        if !gone.is_empty() {
            store.forget(&gone).await?;
        }
    }
    let new: Vec<Finding> = findings
        .into_iter()
        .filter(|finding| !before.contains(&finding.key))
        .collect();
    if !new.is_empty() {
        store.remember(&new, at).await?;
    }
    Ok(Alert::new(kind, new, at))
}

/// One finding a store still holds as raised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Held {
    /// The finding's key, such as `stale-post:en.old-post`.
    pub key: String,
    /// The finding's line, or empty when it was raised before lines were
    /// kept.
    pub line: String,
    /// When it was raised.
    pub raised_at: DateTime<Utc>,
}

/// The `held` findings as one alert per kind, in [`Kind::ALL`] order. Each
/// alert lists its findings newest first and carries the newest one's
/// time. A finding raised before lines were kept shows its key's subject,
/// and a key of a kind this version does not know is left out.
pub fn standing(held: Vec<Held>) -> Vec<Alert> {
    let mut by_kind: BTreeMap<Kind, Vec<Held>> = BTreeMap::new();
    for one in held {
        if let Some(kind) = Kind::ALL.into_iter().find(|kind| kind.owns(&one.key)) {
            by_kind.entry(kind).or_default().push(one);
        }
    }
    let mut alerts = Vec::new();
    for kind in Kind::ALL {
        let Some(mut found) = by_kind.remove(&kind) else {
            continue;
        };
        found.sort_by(|a, b| b.raised_at.cmp(&a.raised_at).then(a.key.cmp(&b.key)));
        let Some(newest) = found.first().map(|one| one.raised_at) else {
            continue;
        };
        let findings = found.into_iter().map(|one| finding_of(kind, one)).collect();
        alerts.extend(Alert::new(kind, findings, newest));
    }
    alerts
}

/// `held` as a finding, its line falling back to the key's subject.
fn finding_of(kind: Kind, held: Held) -> Finding {
    let line = if held.line.is_empty() {
        let prefix = kind.key("");
        held.key
            .strip_prefix(&prefix)
            .unwrap_or(&held.key)
            .to_owned()
    } else {
        held.line
    };
    Finding {
        key: held.key,
        line,
    }
}

#[cfg(any(test, feature = "test-util"))]
pub use memory::MemoryRaised;

#[cfg(any(test, feature = "test-util"))]
mod memory {
    use std::collections::BTreeMap;
    use std::sync::{Mutex, PoisonError};

    use super::{BTreeSet, BoxFuture, DateTime, Finding, Held, Kind, Raised, RaisedError, Utc};

    /// Each raised key with its line and time.
    type Keys = BTreeMap<String, (String, DateTime<Utc>)>;

    /// Raised keys in memory, for tests.
    #[derive(Debug, Default)]
    pub struct MemoryRaised(Mutex<Keys>);

    impl MemoryRaised {
        fn keys(&self) -> std::sync::MutexGuard<'_, Keys> {
            self.0.lock().unwrap_or_else(PoisonError::into_inner)
        }
    }

    impl Raised for MemoryRaised {
        fn raised(&self, kind: Kind) -> BoxFuture<'_, Result<BTreeSet<String>, RaisedError>> {
            let keys = self
                .keys()
                .keys()
                .filter(|key| kind.owns(key))
                .cloned()
                .collect();
            Box::pin(async move { Ok(keys) })
        }

        fn remember<'a>(
            &'a self,
            findings: &'a [Finding],
            at: DateTime<Utc>,
        ) -> BoxFuture<'a, Result<(), RaisedError>> {
            let kept = findings
                .iter()
                .map(|found| (found.key.clone(), (found.line.clone(), at)));
            self.keys().extend(kept);
            Box::pin(async { Ok(()) })
        }

        fn held(&self) -> BoxFuture<'_, Result<Vec<Held>, RaisedError>> {
            let held = self
                .keys()
                .iter()
                .map(|(key, (line, at))| Held {
                    key: key.clone(),
                    line: line.clone(),
                    raised_at: *at,
                })
                .collect();
            Box::pin(async move { Ok(held) })
        }

        fn forget<'a>(&'a self, keys: &'a [String]) -> BoxFuture<'a, Result<(), RaisedError>> {
            let mut held = self.keys();
            for key in keys {
                held.remove(key);
            }
            Box::pin(async { Ok(()) })
        }

        fn forget_before(&self, before: DateTime<Utc>) -> BoxFuture<'_, Result<(), RaisedError>> {
            self.keys().retain(|_, (_, at)| *at >= before);
            Box::pin(async { Ok(()) })
        }
    }
}
