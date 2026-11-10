//! The Admin API's glossary routes: terms such as "topical authority" that
//! a post links with a wikilink and that each get their own page (plan
//! section 5.12, KH-API-2).

use axum::extract::rejection::JsonRejection;
use axum::extract::{OriginalUri, Path, State};
use axum::http::header::LOCATION;
use axum::http::{HeaderName, StatusCode};
use axum::{Extension, Json, middleware};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_api::BASE;
use crate::admin_entries::{Kept, Noun};
use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::entry_store::Entry;
use crate::keys::{Principal, Scope};
use crate::problem::Problem;
use crate::resources::{AdminTerm, NewTerm};

/// How messages name a glossary term.
const TERM: Noun = Noun {
    one: "glossary term",
    many: "glossary terms",
    example: "en.topical-authority",
};

/// The glossary routes, under the Admin API's base. Reading needs
/// `posts:write`; a term is a live page once saved, so writing needs
/// `posts:publish`.
pub(crate) fn routes(glossary: Kept<AdminTerm>, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_terms, create_term))
        .routes(routes!(read_term))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(glossary)
}

/// Glossary terms as the Admin API wraps them.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminGlossary {
    /// The term or terms.
    glossary: Vec<AdminTerm>,
}

impl AdminGlossary {
    fn one(term: AdminTerm) -> Json<Self> {
        Json(Self {
            glossary: vec![term],
        })
    }
}

/// Lists every glossary term in every language. A site has tens, so the
/// list comes whole. Needs `posts:write`.
#[utoipa::path(
    get, path = "/glossary/", tag = "admin glossary",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Every glossary term", body = AdminGlossary),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server keeps no glossary terms", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_terms(
    State(glossary): State<Kept<AdminTerm>>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<AdminGlossary>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let listed = TERM.store(glossary.as_deref())?.list().await;
    let glossary = listed.map_err(|error| Problem::internal(&error))?;
    Ok(Json(AdminGlossary { glossary }))
}

/// A new term, wrapped the way the Admin API wraps the glossary: exactly one.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct NewTerms {
    /// The one new term.
    glossary: Vec<NewTerm>,
}

/// Creates a glossary term in the language it names or the site's
/// default. Needs `posts:publish`.
#[utoipa::path(
    post, path = "/glossary/", tag = "admin glossary",
    security(("admin_key" = [])),
    request_body = NewTerms,
    responses(
        (status = 201, description = "The new glossary term", body = AdminGlossary),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:publish", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server keeps no glossary terms", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "Another term in that language has the slug", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The body could not be read, the term or definition is empty, or the site has no such language", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn create_term(
    State(glossary): State<Kept<AdminTerm>>,
    Extension(principal): Extension<Principal>,
    body: Result<Json<NewTerms>, JsonRejection>,
) -> Result<(StatusCode, [(HeaderName, String); 1], Json<AdminGlossary>), Problem> {
    require(&principal, Scope::PostsPublish)?;
    let new = TERM.one(TERM.body(body)?.glossary)?;
    TERM.filled("term", new.term())?;
    TERM.filled("markdown", new.markdown())?;
    let made = TERM.store(glossary.as_deref())?.create(new).await;
    let term = made.map_err(|error| TERM.problem(error, ""))?;
    let location = format!("{BASE}/glossary/{}/", term.id());
    Ok((
        StatusCode::CREATED,
        [(LOCATION, location)],
        AdminGlossary::one(term),
    ))
}

/// Reads one glossary term. Needs `posts:write`.
#[utoipa::path(
    get, path = "/glossary/{id}/", tag = "admin glossary",
    security(("admin_key" = [])),
    params(("id" = String, Path, description = "The language and slug, such as en.topical-authority")),
    responses(
        (status = 200, description = "The glossary term", body = AdminGlossary),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No term has this identity, or this server keeps no glossary terms", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn read_term(
    State(glossary): State<Kept<AdminTerm>>,
    Extension(principal): Extension<Principal>,
    Path(id): Path<String>,
    OriginalUri(uri): OriginalUri,
) -> Result<Json<AdminGlossary>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let found = TERM.store(glossary.as_deref())?.get(&id).await;
    let term = found.map_err(|error| Problem::internal(&error))?;
    let term = term.ok_or_else(|| TERM.missing(uri.path()))?;
    Ok(AdminGlossary::one(term))
}
