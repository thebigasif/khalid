//! The speed and uptime watch's report (C6, KH-API-5): how fast and how
//! reliably the blog answered at its public address over the last day and
//! the last 30 days, from the looks `khalid serve` keeps. `khalid watch
//! --json` prints the same shape, and the Admin API answers it under
//! `/api/admin/v1/watch/`.

use std::error::Error;

use serde::Serialize;
use utoipa::ToSchema;

use crate::content_store::BoxFuture;

/// Reads the watch's report. The `khalid` binary implements it over the
/// site's database with the same code `khalid watch` runs.
pub trait UptimeReader: Send + Sync {
    /// Every watched page's report, at the time the reader's clock gives.
    ///
    /// # Errors
    ///
    /// [`UptimeError::Internal`] when the looks cannot be read.
    fn report(&self) -> BoxFuture<'_, Result<UptimeReport, UptimeError>>;
}

/// Why the report could not be read.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum UptimeError {
    /// The site's database could not be read.
    #[error("the watch's looks could not be read")]
    Internal(#[source] Box<dyn Error + Send + Sync>),
}

/// Every page the watch looked at. The list is empty until `khalid serve`
/// has looked once, which it does only when it answers more than its own
/// computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct UptimeReport {
    /// Each watched page, in the order of its address.
    pub pages: Vec<WatchedPage>,
}

/// One page's looks, over two spans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct WatchedPage {
    /// The page's full address: the blog's home page.
    #[schema(example = "https://example.com/blog/")]
    pub page: String,
    /// The looks of the last 24 hours.
    pub last_day: WatchSpan,
    /// The looks of the last 30 days, all the watch keeps.
    pub last_30_days: WatchSpan,
    /// The newest look, or `null` when there is none.
    pub last_look: Option<WatchLook>,
}

/// The looks of one span of time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
pub struct WatchSpan {
    /// How many looks the watch took.
    #[schema(example = 288)]
    pub looks: usize,
    /// How many of them got the page: a status from 200 to 399.
    #[schema(example = 287)]
    pub answered: usize,
    /// The middle answer time of the looks that got the page, in
    /// milliseconds, or `null` when none did.
    #[schema(example = 120)]
    pub middle_ms: Option<u32>,
    /// The slowest of them, in milliseconds, or `null` when none did.
    #[schema(example = 900)]
    pub slowest_ms: Option<u32>,
    /// The longest time between two looks, or from the last look to now,
    /// in minutes. A long gap is time nobody watched, as when the server
    /// itself was down.
    #[schema(example = 5)]
    pub longest_gap_minutes: i64,
}

/// One look at the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct WatchLook {
    /// When the look started, in UTC.
    #[schema(example = "2026-11-10T09:30:00Z")]
    pub at: String,
    /// The HTTP status it got, or `null` when nothing answered in time.
    #[schema(example = 200)]
    pub status: Option<u16>,
    /// How long it took, in milliseconds.
    #[schema(example = 120)]
    pub millis: u32,
}
