//! The alerts raised for the site's owner (KH-GRO-6, backlog row 15):
//! what `khalid serve` told the owner by email and the `alert.raised`
//! webhook, for an owner with neither. The Admin API answers it under
//! `/api/admin/v1/alerts/`.

use std::error::Error;

use serde::Serialize;
use utoipa::ToSchema;

use crate::content_store::BoxFuture;

/// Reads the alerts still raised. The `khalid` binary implements it over
/// the site's database, with the same store that keeps each alert from
/// going out twice.
pub trait AlertReader: Send + Sync {
    /// Every alert still raised, one per kind.
    ///
    /// # Errors
    ///
    /// [`AlertError::Internal`] when the alerts cannot be read.
    fn alerts(&self) -> BoxFuture<'_, Result<AlertList, AlertError>>;
}

/// Why the alerts could not be read.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AlertError {
    /// The site's database could not be read.
    #[error("the raised alerts could not be read")]
    Internal(#[source] Box<dyn Error + Send + Sync>),
}

/// The alerts still raised. A problem that clears, such as a renewed
/// certificate, leaves the list at the next check; one that happens once,
/// such as a burst of 404s, stays 30 days.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct AlertList {
    /// One alert per kind, in the order alerts go out.
    pub alerts: Vec<RaisedAlert>,
}

/// The findings of one kind still raised.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct RaisedAlert {
    /// What the alert is about: the `kind` of the `alert.raised` webhook.
    #[schema(example = "stale-post")]
    pub kind: String,
    /// What happened, in one sentence.
    #[schema(example = "Some live posts have not changed for a long time.")]
    pub what: String,
    /// Why it matters, in one sentence.
    #[schema(
        example = "Old facts and dead links make readers and search engines trust a post less."
    )]
    pub why: String,
    /// What to do, in one sentence.
    #[schema(example = "Read each post below, update what has changed, and save it.")]
    pub fix: String,
    /// When its newest finding was raised, in UTC.
    #[schema(example = "2026-11-03T09:00:00Z")]
    pub raised_at: String,
    /// Each problem, newest first.
    pub findings: Vec<RaisedFinding>,
}

/// One problem an alert raised.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct RaisedFinding {
    /// Stays the same while the problem lasts.
    #[schema(example = "stale-post:en.old-post")]
    pub key: String,
    /// The problem in one line.
    #[schema(example = "Old post (en.old-post), last changed 2024-03-01")]
    pub line: String,
}
