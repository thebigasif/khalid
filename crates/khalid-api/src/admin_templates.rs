//! The Admin API's template list, `GET /api/admin/v1/templates/`: every
//! built-in template the running Khalid has, what each turns on and off,
//! and the options it takes (templates spec 4.1). The site's choice is one
//! of its settings, under `/api/admin/v1/settings/`.

use std::sync::Arc;

use axum::extract::State;
use axum::{Extension, Json, middleware};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_entries::Noun;
use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::problem::Problem;
use crate::resources::AdminTemplate;

/// How messages name the templates.
const TEMPLATES: Noun = Noun {
    one: "template",
    many: "templates",
    example: "classic",
};

/// The templates the server has, when it has the engine's list.
type Templates = Option<Arc<[AdminTemplate]>>;

/// The template route, under the Admin API's base. It needs
/// `settings:write`, as the settings it feeds do.
pub(crate) fn routes(templates: Templates, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_templates))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(templates)
}

/// Every built-in template, as the Admin API wraps the list.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct TemplateList {
    /// The templates, classic first.
    templates: Vec<AdminTemplate>,
}

/// Lists every built-in template the running Khalid has, classic first,
/// each with its summary, the reader features it turns on or off and the
/// options `[template.options]` may set for it. Choose one through the
/// settings route. Needs `settings:write`.
#[utoipa::path(
    get, path = "/templates/", tag = "admin settings",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The templates", body = TemplateList),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server lists no templates", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_templates(
    State(templates): State<Templates>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<TemplateList>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let templates = TEMPLATES.store(templates.as_deref())?.to_vec();
    Ok(Json(TemplateList { templates }))
}
