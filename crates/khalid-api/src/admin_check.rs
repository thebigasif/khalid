//! `GET /api/admin/v1/posts/{id}/check/`: how a post reads to search
//! engines before it goes live, for the editor's SEO panel (G54), and
//! `GET .../posts/{id}/link-suggestions/`, the posts it should link to.

use std::sync::Arc;

use axum::extract::{OriginalUri, Path, State};
use axum::{Extension, Json, middleware};
use khalid_core::UserMessage;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_api::missing;
use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::link_suggestions::LinkSuggestion;
use crate::post_check::{PostCheck, PostChecks};
use crate::problem::{Problem, ProblemType};
use crate::site_health::SiteHealth;
use crate::site_issues::SiteIssue;

/// The site's post checks, or `None` for a server that makes none.
pub(crate) type Checks = Option<Arc<dyn PostChecks>>;

/// The check route, declared with `utoipa::path` (RULES.md BL-API-4),
/// behind an integration key or staff token.
pub(crate) fn routes(checks: Checks, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(check_post))
        .routes(routes!(suggest_links))
        .routes(routes!(list_issues))
        .routes(routes!(site_health))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(checks)
}

/// Checks as the Admin API wraps them.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminChecks {
    /// The one post's check.
    checks: Vec<PostCheck>,
}

/// Checks one post as it would go live now, saving nothing: every lint
/// finding with the score out of 100, the search result its page would
/// make, and the JSON-LD the page would carry. A draft is checked as if
/// published today. Needs `posts:write`.
#[utoipa::path(
    get, path = "/posts/{id}/check/", tag = "admin posts",
    security(("admin_key" = [])),
    params(("id" = String, Path, description = "The post's identity")),
    responses(
        (status = 200, description = "The post's check", body = AdminChecks),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No post has this identity, or this server checks no posts", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn check_post(
    State(checks): State<Checks>,
    Extension(principal): Extension<Principal>,
    OriginalUri(uri): OriginalUri,
    Path(id): Path<String>,
) -> Result<Json<AdminChecks>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let checks = checks.ok_or_else(no_checks)?;
    let check = checks
        .check(&id)
        .await
        .map_err(|e| Problem::internal(&e))?
        .ok_or_else(|| missing(uri.path()))?;
    Ok(Json(AdminChecks {
        checks: vec![check],
    }))
}

/// Suggestions as the Admin API wraps them.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminLinkSuggestions {
    /// Up to ten posts, best first.
    suggestions: Vec<LinkSuggestion>,
}

/// The blog's other posts in the same language that this post does not
/// link to yet, ranked by what they share with it: its category, its tags,
/// its topic cluster, with the cluster's pillar first, and their focus
/// keyword in this post's text, given back as the words to link. A draft
/// is weighed as if published today. Needs `posts:write`.
#[utoipa::path(
    get, path = "/posts/{id}/link-suggestions/", tag = "admin posts",
    security(("admin_key" = [])),
    params(("id" = String, Path, description = "The post's identity")),
    responses(
        (status = 200, description = "Up to ten posts to link to, best first", body = AdminLinkSuggestions),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No post has this identity, or this server checks no posts", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn suggest_links(
    State(checks): State<Checks>,
    Extension(principal): Extension<Principal>,
    OriginalUri(uri): OriginalUri,
    Path(id): Path<String>,
) -> Result<Json<AdminLinkSuggestions>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let checks = checks.ok_or_else(no_checks)?;
    let suggestions = checks
        .link_suggestions(&id)
        .await
        .map_err(|e| Problem::internal(&e))?
        .ok_or_else(|| missing(uri.path()))?;
    Ok(Json(AdminLinkSuggestions { suggestions }))
}

/// Issues as the Admin API wraps them.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminIssues {
    /// Up to ten problems, those that stop posts going live first.
    issues: Vec<SiteIssue>,
}

/// The live site's most common lint problems, for the dashboard's task
/// list: each rule that fires, on how many posts, with what, why and fix
/// and one post to open first. Those that stop posts going live come
/// first, then those on the most posts. Every rule runs on every post, as
/// `khalid lint` does, so the dashboard asks for it after the rest. Needs
/// `posts:write`.
#[utoipa::path(
    get, path = "/lint/issues/", tag = "admin posts",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Up to ten problems, most pressing first", body = AdminIssues),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server checks no posts", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_issues(
    State(checks): State<Checks>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<AdminIssues>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let checks = checks.ok_or_else(no_checks)?;
    let issues = checks.site_issues().await;
    let issues = issues.map_err(|e| Problem::internal(&e))?;
    Ok(Json(AdminIssues { issues }))
}

/// The site's health score out of 100 from its last audit, for the
/// dashboard, and up to the last 30 audits as the trend, oldest first.
/// `khalid serve` audits the site once a day, and `khalid audit` adds a
/// run too. Before the first audit, `last` is `null` and the trend is
/// empty. Needs `posts:write`.
#[utoipa::path(
    get, path = "/site/health/", tag = "admin posts",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The last audit and the trend", body = SiteHealth),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server checks no posts", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn site_health(
    State(checks): State<Checks>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<SiteHealth>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let checks = checks.ok_or_else(no_checks)?;
    let health = checks.site_health().await;
    Ok(Json(health.map_err(|e| Problem::internal(&e))?))
}

fn no_checks() -> Problem {
    let message = UserMessage::new(
        "This server does not check posts.",
        "Posts are checked by the site's own server, which reads its settings and its other posts.",
        "Send the request to the running khalid serve.",
    );
    Problem::new(ProblemType::NotFound, &message)
}
