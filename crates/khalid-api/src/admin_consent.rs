//! The consent proof through the Admin API (privacy U11, R18, KTD8): the
//! versions of what readers were shown, with the days each was in force,
//! and how many answers allowed each optional category per day. Read only,
//! and each route needs `settings:write`, because the counts are visitor
//! data.

use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::{Extension, Json, middleware};
use chrono::NaiveDate;
use khalid_analytics::Period;
use khalid_core::{ConsentCategory, UserMessage};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::analytics_desk::AnalyticsDesk;
use crate::api_version::negotiate_version;
use crate::consent_proof::{DayChoices, ShownVersion};
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};

/// How many days the counts cover when none are asked for.
const DEFAULT_DAYS: u32 = 28;

/// The consent proof routes, behind an integration key or staff token.
pub(crate) fn routes(desk: AnalyticsDesk, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(consent_versions))
        .routes(routes!(consent_counts))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(desk)
}

/// One version of what readers were shown.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminConsentVersion {
    /// The settings version, a hash of everything shown.
    #[schema(example = "0123456789abcdef")]
    version: String,
    /// The first day a reader was shown it, in UTC.
    first_day: NaiveDate,
    /// The last day a reader was shown it so far, in UTC.
    last_day: NaiveDate,
    /// What readers were shown: the banner and category text in each
    /// language, the categories, the vendors and the lifetime.
    #[schema(value_type = Object)]
    shown: Value,
}

/// Every version of what readers were shown.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminConsentVersions {
    /// The versions, oldest first.
    versions: Vec<AdminConsentVersion>,
}

/// Lists every version of the banner and category text, categories,
/// vendors and lifetime readers were shown, with the days each was in
/// force. Needs `settings:write`.
#[utoipa::path(
    get, path = "/consent/versions/", tag = "admin consent",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Every version readers were shown", body = AdminConsentVersions),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn consent_versions(
    State(desk): State<AnalyticsDesk>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<AdminConsentVersions>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let versions = desk
        .consent_versions()
        .await
        .map_err(|e| Problem::internal(&e))?;
    Ok(Json(AdminConsentVersions {
        versions: versions.into_iter().map(version).collect(),
    }))
}

fn version(row: ShownVersion) -> AdminConsentVersion {
    // The store only keeps valid JSON; anything else is shown as text.
    let shown = serde_json::from_str(&row.shown).unwrap_or(Value::String(row.shown));
    AdminConsentVersion {
        version: row.version,
        first_day: row.first_day,
        last_day: row.last_day,
        shown,
    }
}

/// How many days to count, ending today.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub(crate) struct ConsentCountsQuery {
    /// From 1 to 366; 28 when left out.
    days: Option<u32>,
}

/// How many answers there were, and how many allowed each optional
/// category. Necessary is always on, so it has no count.
#[derive(Debug, Default, Serialize, ToSchema)]
pub(crate) struct AdminConsentTally {
    /// Every answer: Accept all, Reject all and each saved choice.
    choices: u64,
    /// Answers that allowed Preferences.
    preferences: u64,
    /// Answers that allowed Analytics.
    analytics: u64,
    /// Answers that allowed Marketing.
    marketing: u64,
    /// Answers that allowed External media.
    external_media: u64,
}

impl AdminConsentTally {
    /// The tally of one day's answers.
    fn of(row: &DayChoices) -> Self {
        Self {
            choices: row.choices,
            preferences: row.allowed(ConsentCategory::Preferences),
            analytics: row.allowed(ConsentCategory::Analytics),
            marketing: row.allowed(ConsentCategory::Marketing),
            external_media: row.allowed(ConsentCategory::ExternalMedia),
        }
    }

    /// This tally with `other` added.
    fn plus(self, other: &Self) -> Self {
        Self {
            choices: self.choices.saturating_add(other.choices),
            preferences: self.preferences.saturating_add(other.preferences),
            analytics: self.analytics.saturating_add(other.analytics),
            marketing: self.marketing.saturating_add(other.marketing),
            external_media: self.external_media.saturating_add(other.external_media),
        }
    }
}

/// The answers on one day under one version.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminConsentDay {
    /// The day, in UTC.
    day: NaiveDate,
    /// The version readers were shown.
    #[schema(example = "0123456789abcdef")]
    version: String,
    /// The answers.
    counts: AdminConsentTally,
}

/// How readers answered the consent banner. No address or reader
/// identifier is kept: only these daily totals.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminConsentCounts {
    /// The first day counted, in UTC.
    first: NaiveDate,
    /// The last day counted: today, in UTC.
    last: NaiveDate,
    /// How many days were counted.
    days: u32,
    /// Every answer in the period.
    totals: AdminConsentTally,
    /// The answers per day and version, first day first. A day with no
    /// answers is left out.
    per_day: Vec<AdminConsentDay>,
}

/// Counts the readers' answers to the consent banner in the last `days`
/// days, per day and version, with how many allowed each optional
/// category. Needs `settings:write`.
#[utoipa::path(
    get, path = "/consent/counts/", tag = "admin consent",
    security(("admin_key" = [])),
    params(ConsentCountsQuery),
    responses(
        (status = 200, description = "The answers per day", body = AdminConsentCounts),
        (status = 400, description = "The days are not from 1 to 366", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn consent_counts(
    State(desk): State<AnalyticsDesk>,
    Extension(principal): Extension<Principal>,
    query: Result<Query<ConsentCountsQuery>, QueryRejection>,
) -> Result<Json<AdminConsentCounts>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let days = query.ok().map(|Query(q)| q.days.unwrap_or(DEFAULT_DAYS));
    let period = days
        .and_then(|days| Period::ending(desk.today(), days))
        .ok_or_else(bad_days)?;
    let rows = desk
        .consent_counts(period)
        .await
        .map_err(|e| Problem::internal(&e))?;
    let per_day: Vec<_> = rows
        .iter()
        .map(|row| AdminConsentDay {
            day: row.day,
            version: row.version.clone(),
            counts: AdminConsentTally::of(row),
        })
        .collect();
    let totals = per_day
        .iter()
        .fold(AdminConsentTally::default(), |sum, row| {
            sum.plus(&row.counts)
        });
    Ok(Json(AdminConsentCounts {
        first: period.first(),
        last: period.last(),
        days: period.days(),
        totals,
        per_day,
    }))
}

/// The 400 for days that are not from 1 to 366.
fn bad_days() -> Problem {
    let message = UserMessage::new(
        "The days are not a number from 1 to 366.",
        "The counts go back from today, at most a year at a time.",
        "Ask for days=28, or leave it out.",
    );
    Problem::new(ProblemType::InvalidQuery, &message).with_rule("days")
}
