//! The speed and uptime watch through the Admin API (C6, KH-API-5): the
//! report `khalid watch` prints, for the admin and for agents through
//! `khalid mcp`. It needs `settings:write`, the scope the Analytics
//! routes need, because it tells how the site runs.

use std::sync::Arc;

use axum::extract::State;
use axum::{Extension, Json, middleware};
use khalid_core::UserMessage;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::uptime::{UptimeReader, UptimeReport};

/// The reader the route calls, or `None` on a server that keeps no looks.
type Kept = Option<Arc<dyn UptimeReader>>;

/// The watch route, behind an integration key or staff token.
pub(crate) fn routes(reader: Kept, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(uptime_report))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(reader)
}

/// Reports how fast and how reliably the blog answered at its public
/// address over the last 24 hours and the last 30 days, from the looks
/// `khalid serve` takes every few minutes, as `khalid watch` does. The
/// list is empty until the server has looked once. Needs `settings:write`.
#[utoipa::path(
    get, path = "/watch/", tag = "admin watch",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Each watched page's looks", body = UptimeReport),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server keeps no looks", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn uptime_report(
    State(reader): State<Kept>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<UptimeReport>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let reader = reader.ok_or_else(not_kept)?;
    let report = reader.report().await;
    report.map(Json).map_err(|error| Problem::internal(&error))
}

/// The 404 of a server that keeps no looks.
fn not_kept() -> Problem {
    let message = UserMessage::new(
        "This server keeps no looks at the blog.",
        "The watch keeps its looks in the site's database, which only khalid serve and khalid mcp open.",
        "Send the request to the running khalid serve.",
    );
    Problem::new(ProblemType::NotFound, &message)
}
