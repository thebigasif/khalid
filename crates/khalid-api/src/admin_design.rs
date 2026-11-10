//! The Admin API's Design route: the site's colours, fonts, shape and
//! logo in `theme/tokens.toml`, read with the contrast of each colour pair
//! and changed in place (phase 4, task 4.7).

use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::{Extension, Json, middleware};
use khalid_core::UserMessage;
use khalid_theme::TokensError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_entries::Noun;
use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::design_desk::{DesignDesk, DesignError};
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::resources::{AdminDesign, DesignChange};

/// How messages name the design.
const DESIGN: Noun = Noun {
    one: "design",
    many: "designs",
    example: "colors.link",
};

/// The site's design, when the server keeps its files.
type Desk = Option<DesignDesk>;

/// The Design routes, under the Admin API's base. Both need
/// `settings:write`, which only the owner and admins hold among staff.
pub(crate) fn routes(desk: Desk, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(read_design, change_design))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(desk)
}

/// The design as the Admin API wraps it.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct SiteDesign {
    /// The site's design.
    design: AdminDesign,
}

/// A change, wrapped the way the Admin API wraps the design.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct DesignChanges {
    /// The change.
    design: DesignChange,
}

/// Reads the colours, fonts, shape and logo the site's pages use, each
/// from `theme/tokens.toml` or else the chosen template, with the WCAG
/// contrast of every colour pair. Needs `settings:write`.
#[utoipa::path(
    get, path = "/design/", tag = "admin settings",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The design", body = SiteDesign),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server keeps no design", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "theme/tokens.toml holds a mistake the pages cannot load", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn read_design(
    State(desk): State<Desk>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<SiteDesign>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let design = DESIGN.store(desk.as_ref())?.read().await.map_err(problem)?;
    Ok(Json(SiteDesign { design }))
}

/// Changes the tokens sent in `theme/tokens.toml`, in place: comments and
/// every other table stay as written. Each value must be one the pages
/// load. The change carries the `updated_at` of the copy it was made on;
/// when the file changed since, the answer is a 409 (RULES.md BL-API-7).
/// The running server shows the change on the next page it renders.
/// Needs `settings:write`.
#[utoipa::path(
    put, path = "/design/", tag = "admin settings",
    security(("admin_key" = [])),
    request_body = DesignChanges,
    responses(
        (status = 200, description = "The design as saved", body = SiteDesign),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks settings:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server keeps no design", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "theme/tokens.toml changed after the copy the change was made on, or holds a mistake", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The body could not be read, or a value is not one the pages load", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn change_design(
    State(desk): State<Desk>,
    Extension(principal): Extension<Principal>,
    body: Result<Json<DesignChanges>, JsonRejection>,
) -> Result<Json<SiteDesign>, Problem> {
    require(&principal, Scope::SettingsWrite)?;
    let change = DESIGN.body(body)?.design;
    let changed = DESIGN.store(desk.as_ref())?.change(change).await;
    let design = changed.map_err(problem)?;
    Ok(Json(SiteDesign { design }))
}

/// The problem a refused read or change answers with.
fn problem(error: DesignError) -> Problem {
    match error {
        DesignError::Stale { updated_at } => {
            let when = updated_at.map_or_else(
                || "was taken away".to_owned(),
                |at| format!("changed at {}", at.format("%Y-%m-%d %H:%M UTC")),
            );
            let message = UserMessage::new(
                format!("theme/tokens.toml {when}, after the copy you have."),
                "Saving your copy would undo that change.",
                "Read the design again, make your change to it, and send the new updated_at.",
            );
            Problem::new(ProblemType::Conflict, &message).with_rule("updated_at")
        }
        DesignError::UnknownToken { name } => refused(
            &name,
            &format!(
                "the Design screen changes only the tokens of theme/tokens.toml, such as colors.link, and {name} is not one"
            ),
        ),
        DesignError::Tokens(TokensError::Value {
            token,
            value,
            problem,
        }) => refused(
            token.as_str(),
            &format!("{token} cannot be {value:?}, because {problem}"),
        ),
        DesignError::Tokens(error @ TokensError::Parse { .. }) => {
            let line = error
                .line()
                .map_or_else(String::new, |line| format!(" on line {line}"));
            let message = UserMessage::new(
                format!("theme/tokens.toml has a mistake{line}."),
                "The blog's pages cannot load their colours and fonts until it is fixed.",
                "Fix that line by hand, or run khalid theme check to see it, then open Design again.",
            );
            Problem::new(ProblemType::Conflict, &message).with_rule("theme/tokens.toml")
        }
        error => Problem::internal(&error),
    }
}

/// A 422 for a value or name the file cannot take, naming its token.
fn refused(token: &str, why: &str) -> Problem {
    let message = UserMessage::new(
        format!("This change was refused: {why}."),
        "The blog's pages would not load with it.",
        format!("Change {token} and save again; nothing was saved."),
    );
    Problem::new(ProblemType::InvalidBody, &message).with_rule(token)
}
