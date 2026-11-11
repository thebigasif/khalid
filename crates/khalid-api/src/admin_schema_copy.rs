//! The "copy schema from a URL" route of the Admin API,
//! `POST /api/admin/v1/schema/copies/` (G54, KH-SEO-4). It needs
//! `posts:write`, since the copy goes into a post's custom JSON-LD field.

use std::sync::Arc;

use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::{Extension, Json, middleware};
use khalid_core::{AbsoluteUrl, UserMessage};
use khalid_seo::CopySchemaError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::schema_copy::SchemaCopier;

/// The server's copier, or `None` for a server that fetches no pages.
pub(crate) type Copier = Option<Arc<dyn SchemaCopier>>;

/// The page to copy schema from.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SchemaCopyAsk {
    /// The page's full web address.
    #[schema(example = "https://www.example.com/recipes/lentil-soup/")]
    url: String,
}

/// Another page's JSON-LD, as text for a custom JSON-LD field.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct SchemaCopy {
    /// Every JSON-LD node on the page in one graph, pretty-printed, without
    /// the other site's `@id`s.
    #[schema(example = "{\n  \"@context\": \"https://schema.org\",\n  \"@type\": \"Recipe\"\n}")]
    json_ld: String,
    /// The `@type` of each node at the top, in page order.
    #[schema(example = json!(["Recipe"]))]
    types: Vec<String>,
    /// Why the custom field would refuse the copy as it is, or `null` when
    /// it takes it.
    problem: Option<String>,
}

/// The copy route, declared with `utoipa::path` (RULES.md BL-API-4),
/// behind an integration key or staff token.
pub(crate) fn routes(copier: Copier, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(copy_schema))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(copier)
}

/// Fetches the page at `url` through the shared outbound client and
/// answers with its JSON-LD, ready to edit in a post's custom JSON-LD
/// field. Nothing is saved. Addresses inside the server's network are
/// refused. Needs `posts:write`.
#[utoipa::path(
    post, path = "/schema/copies/", tag = "admin posts",
    security(("admin_key" = [])),
    request_body = SchemaCopyAsk,
    responses(
        (status = 200, description = "The page's JSON-LD", body = SchemaCopy),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server fetches no pages", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The address is not a public web address, or the page has no JSON-LD", body = Problem, content_type = "application/problem+json"),
        (status = 502, description = "The page could not be fetched, or answered with an error", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn copy_schema(
    State(copier): State<Copier>,
    Extension(principal): Extension<Principal>,
    body: Result<Json<SchemaCopyAsk>, JsonRejection>,
) -> Result<Json<SchemaCopy>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let copier = copier.ok_or_else(no_copier)?;
    let Json(ask) = body.map_err(|rejection| not_an_address(&rejection.body_text()))?;
    let url =
        AbsoluteUrl::parse(ask.url.trim()).map_err(|error| not_an_address(&error.to_string()))?;
    let page = copier.copy(&url).await.map_err(|error| problem(&error))?;
    Ok(Json(SchemaCopy {
        json_ld: page.text().to_owned(),
        types: page.types().to_vec(),
        problem: page.problem().map(str::to_owned),
    }))
}

fn problem(error: &CopySchemaError) -> Problem {
    let fix = "Check the address in a browser, or write the JSON-LD by hand.";
    if let Some(host) = error.inward_host() {
        let message = UserMessage::new(
            format!("{host} points inside the server's own network."),
            "Khalid fetches public pages only, so a pasted link cannot reach private machines.",
            "Copy from a page anyone on the web can open.",
        );
        return Problem::new(ProblemType::InvalidBody, &message);
    }
    let (kind, what, why) = match error {
        CopySchemaError::NoSchema => (
            ProblemType::InvalidBody,
            "The page has no schema to copy.".to_owned(),
            "It carries no JSON-LD script with a schema.org type in it.",
        ),
        CopySchemaError::Status { status } => (
            ProblemType::Upstream,
            format!("The page answered with error {status}."),
            "Only a page that loads can be copied.",
        ),
        CopySchemaError::Fetch { source } => (
            ProblemType::Upstream,
            format!("The page could not be fetched: {source}."),
            "The site did not answer in time, or its answer could not be read.",
        ),
        other => (
            ProblemType::Upstream,
            format!("The page could not be copied: {other}."),
            "The site did not answer in time, or its answer could not be read.",
        ),
    };
    Problem::new(kind, &UserMessage::new(what, why, fix))
}

fn not_an_address(why: &str) -> Problem {
    let message = UserMessage::new(
        "That is not a full web address, such as https://www.example.com/page/.",
        format!("Only an http or https page can be copied ({why})."),
        "Paste the page's whole address from the browser's address bar.",
    );
    Problem::new(ProblemType::InvalidBody, &message)
}

fn no_copier() -> Problem {
    let message = UserMessage::new(
        "This server fetches no pages to copy schema from.",
        "Copying needs the site's own server, which fetches the page.",
        "Send the request to the running khalid serve.",
    );
    Problem::new(ProblemType::NotFound, &message)
}
