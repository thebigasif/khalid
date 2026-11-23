//! The alerts raised for the owner through the Admin API (KH-GRO-6,
//! backlog row 15), so an owner with no email set up sees them on the
//! dashboard. It needs `settings:write`, the scope the watch route needs,
//! because it tells how the site runs.

use std::sync::Arc;

use axum::extract::State;
use axum::{Extension, Json, middleware};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::alerts::{AlertList, AlertReader};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::problem::Problem;

/// The reader the route calls, or `None` on a server with no database.
type Kept = Option<Arc<dyn AlertReader>>;

/// The alerts route, behind an integration key or staff token.
pub(crate) fn routes(reader: Kept, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_alerts))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(reader)
}

/// Lists the alerts still raised for the site's owner: a site that
/// stopped answering or answers slowly, a certificate near its end, a
/// burst of 404s, stale posts, a scheduled post the linter refused, a
/// crawler that stopped coming. Each says what happened, why it matters
/// and what to do. The list is empty on a server with no database. Needs
/// `settings:write`.
#[utoipa::path(
    get, path = "/alerts/", tag = "admin alerts",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The alerts still raised", body = AlertList),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_alerts(
    State(reader): State<Kept>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<AlertList>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let Some(reader) = reader else {
        return Ok(Json(AlertList { alerts: Vec::new() }));
    };
    let list = reader.alerts().await;
    list.map(Json).map_err(|error| Problem::internal(&error))
}
