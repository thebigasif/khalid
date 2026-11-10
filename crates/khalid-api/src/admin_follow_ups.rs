//! Search Console through the Admin API: the follow-ups (GEO M3), the
//! long searches that showed each post and the heading that answers each,
//! the report of each post's numbers (G43), those numbers alone for the
//! posts table, and the whole site's totals for a period, which the
//! Analytics screen shows. The routes need
//! `settings:write`, because the numbers come from the owner's Google
//! account.

use std::sync::Arc;

use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::{Extension, Json, middleware};
use khalid_core::UserMessage;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::follow_ups::{FollowUpError, FollowUpFinder, FollowUps};
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::search_report::{SearchPosts, SearchReport, SearchTotals};

/// The finder the route calls.
type Finder = Arc<dyn FollowUpFinder>;

/// How many days the totals cover when none are asked for.
const DEFAULT_DAYS: u32 = 28;

/// The most days the totals cover: the Analytics screen's longest period.
const MAX_DAYS: u32 = 90;

/// The follow-up route, behind an integration key or staff token.
pub(crate) fn routes(finder: Finder, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(follow_ups))
        .routes(routes!(search_report))
        .routes(routes!(search_posts))
        .routes(routes!(search_totals))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(finder)
}

/// Lists the long searches, of 8 words or more, that showed each post in
/// the last 28 days whose numbers are final, with the heading of the post
/// that answers each. Google is asked on each request, so the answer can
/// take some seconds. Needs `settings:write`.
#[utoipa::path(
    get, path = "/search-console/follow-ups/", tag = "admin search console",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The long searches per post", body = FollowUps),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The site is not connected to Search Console", body = Problem, content_type = "application/problem+json"),
        (status = 502, description = "Google refused or could not be reached", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn follow_ups(
    State(finder): State<Finder>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<FollowUps>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    finder.follow_ups().await.map(Json).map_err(problem)
}

/// Lists each published post's clicks, impressions, click-through rate
/// and average position for the last 28 days whose numbers are final,
/// the pages whose clicks fell three months in a row, pages that answer
/// 404 or `noindex` but still get search traffic, and this week's winners
/// and losers against last week. Google is asked on each request, so the
/// answer can take some seconds. Needs `settings:write`.
#[utoipa::path(
    get, path = "/search-console/report/", tag = "admin search console",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Each post's numbers and what to look at", body = SearchReport),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The site is not connected to Search Console", body = Problem, content_type = "application/problem+json"),
        (status = 502, description = "Google refused or could not be reached", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn search_report(
    State(finder): State<Finder>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<SearchReport>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    finder.report().await.map(Json).map_err(problem)
}

/// Lists each published post's clicks, impressions, click-through rate
/// and average position for the last 28 days whose numbers are final,
/// and nothing else, for the posts table. One request to Google finds
/// them, where the whole report needs six. Needs `settings:write`.
#[utoipa::path(
    get, path = "/search-console/posts/", tag = "admin search console",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Each post's numbers for the month", body = SearchPosts),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The site is not connected to Search Console", body = Problem, content_type = "application/problem+json"),
        (status = 502, description = "Google refused or could not be reached", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn search_posts(
    State(finder): State<Finder>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<SearchPosts>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    finder.posts().await.map(Json).map_err(problem)
}

/// How many days to add up, ending on the last day whose numbers are final.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub(crate) struct TotalsQuery {
    /// From 1 to 90, the longest period the Analytics screen shows; 28
    /// when left out.
    days: Option<u32>,
}

/// Adds up every page's clicks, impressions, click-through rate and
/// average position for the last `days` days whose numbers are final, and
/// for the `days` days before, for the Analytics screen. Google is asked
/// on each request. Needs `settings:write`.
#[utoipa::path(
    get, path = "/search-console/totals/", tag = "admin search console",
    security(("admin_key" = [])),
    params(TotalsQuery),
    responses(
        (status = 200, description = "The site's search numbers, now and before", body = SearchTotals),
        (status = 400, description = "The days are not from 1 to 90", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The site is not connected to Search Console", body = Problem, content_type = "application/problem+json"),
        (status = 502, description = "Google refused or could not be reached", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn search_totals(
    State(finder): State<Finder>,
    Extension(principal): Extension<Principal>,
    query: Result<Query<TotalsQuery>, QueryRejection>,
) -> Result<Json<SearchTotals>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let days = query
        .ok()
        .map(|Query(q)| q.days.unwrap_or(DEFAULT_DAYS))
        .filter(|days| (1..=MAX_DAYS).contains(days))
        .ok_or_else(bad_days)?;
    finder.totals(days).await.map(Json).map_err(problem)
}

/// The 400 for days outside the periods the totals cover.
fn bad_days() -> Problem {
    let message = UserMessage::new(
        "The days are not a number from 1 to 90.",
        "The Analytics screen shows at most 90 days, against the 90 days before.",
        "Ask for days=28, or leave it out.",
    );
    Problem::new(ProblemType::InvalidQuery, &message).with_rule("days")
}

/// The problem `error` answers.
fn problem(error: FollowUpError) -> Problem {
    match error {
        FollowUpError::NotConnected(message) => {
            Problem::new(ProblemType::NotSetUp, &message).with_rule("search_console")
        }
        FollowUpError::Google(message) => Problem::new(ProblemType::Upstream, &message),
        error => Problem::internal(&error),
    }
}
