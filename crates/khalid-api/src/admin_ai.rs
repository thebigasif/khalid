//! The `ai` route of the Admin API under `/api/admin/v1/ai/`: writing
//! suggestions for the editor, the media library and the bulk SEO editor
//! (Yoast package WP4,
//! `docs/adr/0000-ai-suggestions-ask-a-service-the-owner-names.md`).
//! Every method needs `posts:write`. Nothing is sent anywhere until the
//! owner names a provider in `[ai]`.

use axum::extract::State;
use axum::{Extension, Json, middleware};
use khalid_core::AiProvider;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::ai::AiDesk;
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::problem::Problem;

/// The `ai` route, declared with `utoipa::path` (RULES.md BL-API-4),
/// behind an integration key or staff token.
pub(crate) fn routes(desk: AiDesk, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(read_ai))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(desk)
}

/// Whether suggestions are on, and who writes them.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AiState {
    /// Whether the owner named a provider in `[ai]`.
    on: bool,
    /// `off`, `serpdad`, `anthropic` or `openai`.
    #[schema(example = "anthropic")]
    provider: &'static str,
    /// The provider's name as people know it.
    #[schema(example = "Anthropic")]
    label: &'static str,
    /// Whether the server holds the key it asks with, so a button can ask.
    ready: bool,
}

/// Reads whether suggestions are on, so the admin shows its buttons only
/// then. Needs `posts:write`.
#[utoipa::path(
    get, path = "/ai/", tag = "admin ai",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Whether suggestions are on", body = AiState),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn read_ai(
    State(desk): State<AiDesk>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<AiState>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let provider = desk.provider;
    Ok(Json(AiState {
        on: provider != AiProvider::Off,
        provider: name(provider),
        label: provider.label(),
        ready: provider != AiProvider::Off && desk.writer.is_some(),
    }))
}

/// `provider` as `site.toml` spells it.
fn name(provider: AiProvider) -> &'static str {
    match provider {
        AiProvider::Serpdad => "serpdad",
        AiProvider::Anthropic => "anthropic",
        AiProvider::Openai => "openai",
        _ => "off",
    }
}
