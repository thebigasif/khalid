//! The site's health score for the dashboard (plan 5.10): the last audit,
//! by `khalid audit` or by `khalid serve`'s nightly run, and the trend of
//! the runs before it, as `data/audit/runs.jsonl` keeps them.

use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

/// The most runs the trend carries: a month of nightly audits.
pub const MOST_RUNS: usize = 30;

/// One audit as the dashboard shows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
pub struct HealthRun {
    /// When the audit ran, in UTC.
    pub at: DateTime<Utc>,
    /// The health score out of 100: the share of pages with no error.
    #[schema(example = 96)]
    pub score: u8,
    /// How many pages it read.
    #[schema(example = 48)]
    pub pages: usize,
    /// How many errors it found.
    #[schema(example = 2)]
    pub errors: usize,
    /// How many warnings it found.
    #[schema(example = 5)]
    pub warnings: usize,
    /// How many notices it found.
    #[schema(example = 9)]
    pub notices: usize,
}

/// The site's health: its last audit and the trend.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, ToSchema)]
pub struct SiteHealth {
    /// The last audit, or `null` when none has run yet.
    pub last: Option<HealthRun>,
    /// Up to the last 30 audits, oldest first, the last among them.
    pub trend: Vec<HealthRun>,
}

impl SiteHealth {
    /// The health of every run in `runs`, oldest first: the last of them,
    /// and up to [`MOST_RUNS`] of the newest as the trend.
    pub fn of(runs: &[HealthRun]) -> Self {
        let trend = runs
            .iter()
            .skip(runs.len().saturating_sub(MOST_RUNS))
            .copied()
            .collect();
        Self {
            last: runs.last().copied(),
            trend,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeDelta};

    use super::{HealthRun, MOST_RUNS, SiteHealth};

    fn run(day: i64, score: u8) -> HealthRun {
        HealthRun {
            at: DateTime::UNIX_EPOCH + TimeDelta::days(day),
            score,
            pages: 10,
            errors: 0,
            warnings: 0,
            notices: 0,
        }
    }

    #[test]
    fn no_audit_yet_has_no_score_and_no_trend() {
        assert_eq!(SiteHealth::of(&[]), SiteHealth::default());
    }

    #[test]
    fn the_trend_keeps_the_newest_runs_oldest_first() {
        let runs: Vec<HealthRun> = (0..40).map(|day| run(day, 60)).collect();
        let health = SiteHealth::of(&runs);
        assert_eq!(health.last, runs.last().copied());
        assert_eq!(health.trend.len(), MOST_RUNS);
        assert_eq!(health.trend.first(), runs.get(10));
        assert_eq!(health.trend.last(), runs.last());
    }
}
