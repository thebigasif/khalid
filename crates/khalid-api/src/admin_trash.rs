//! The trash through the Admin API (KH-EDT-6): list the posts and pages
//! that were deleted, restore one, or delete it for good.

use std::sync::Arc;

use axum::extract::{OriginalUri, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, middleware};
use chrono::{DateTime, Utc};
use khalid_core::UserMessage;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_api::AdminPosts;
use crate::admin_key::{AdminDoor, require, require_admin_key, require_publish};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::post_trash::{PostTrash, Restored, TrashError, Trashed, TrashedKind};
use crate::problem::{Problem, ProblemType};
use crate::resources::PostStatus;

/// The trash the routes read and write, or `None` when the site keeps
/// none: the list is empty and every key is unknown.
type Trash = Option<Arc<dyn PostTrash>>;

/// The trash routes, behind an integration key or staff token.
pub(crate) fn routes(trash: Trash, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_trash))
        .routes(routes!(purge_trashed))
        .routes(routes!(restore_trashed))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(trash)
}

/// A deleted post or page, as the Admin API shows it.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct TrashedPost {
    /// Names this copy in the trash, for restoring or deleting it.
    #[schema(example = "20261031T093000Z-en.internal-links")]
    key: String,
    /// The post's identity when it was deleted.
    #[schema(example = "en.internal-links")]
    id: String,
    /// Its title.
    title: String,
    /// Its status when it was deleted.
    status: PostStatus,
    /// When it was deleted.
    trashed_at: DateTime<Utc>,
    /// Whether it was a post or a page.
    kind: TrashedKind,
}

/// The trash as the Admin API wraps it.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct TrashList {
    /// The deleted posts and pages, the most recently deleted first.
    trash: Vec<TrashedPost>,
}

/// Lists the deleted posts and pages, the most recently deleted first.
/// Needs `posts:write`.
#[utoipa::path(
    get, path = "/trash/", tag = "admin trash",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The deleted posts and pages", body = TrashList),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_trash(
    State(trash): State<Trash>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<TrashList>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let listed = trashed(trash.as_deref()).await?;
    let trash = listed.into_iter().map(|trashed| TrashedPost {
        key: trashed.key,
        id: trashed.id,
        title: trashed.title,
        status: trashed.status,
        trashed_at: trashed.trashed_at,
        kind: trashed.kind,
    });
    Ok(Json(TrashList {
        trash: trash.collect(),
    }))
}

/// Puts a deleted post or page back where it was, in the status it had,
/// and answers a post as it is now. Needs `posts:write`, and
/// `posts:publish` for a page or a post that was live or scheduled.
#[utoipa::path(
    post, path = "/trash/{key}/restore/", tag = "admin trash",
    security(("admin_key" = [])),
    params(("key" = String, Path, description = "The deleted post's or page's key in the trash")),
    responses(
        (status = 200, description = "The restored post", body = AdminPosts),
        (status = 204, description = "The page is back, and the page routes read it"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write, or posts:publish for a page or a post that was live or scheduled", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "Nothing in the trash has this key", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "Another post or page has the address the restored one would take", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn restore_trashed(
    State(trash): State<Trash>,
    Extension(principal): Extension<Principal>,
    Path(key): Path<String>,
    OriginalUri(uri): OriginalUri,
) -> Result<Response, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let trash = trash.as_deref().ok_or_else(|| not_in_trash(uri.path()))?;
    let listed = trashed(Some(trash)).await?;
    let found = listed.iter().find(|trashed| trashed.key == key);
    let found = found.ok_or_else(|| not_in_trash(uri.path()))?;
    let kind = found.kind;
    if found.status != PostStatus::Draft || kind == TrashedKind::Page {
        require_publish(&principal)?;
    }
    match trash.restore(&key).await {
        Ok(Restored::Post(post)) => Ok(Json(AdminPosts { posts: vec![*post] }).into_response()),
        Ok(_) => Ok(StatusCode::NO_CONTENT.into_response()),
        Err(error) => Err(refused(&error, kind, uri.path())),
    }
}

/// Deletes a post or page in the trash for good. Needs `posts:publish`.
#[utoipa::path(
    delete, path = "/trash/{key}/", tag = "admin trash",
    security(("admin_key" = [])),
    params(("key" = String, Path, description = "The deleted post's or page's key in the trash")),
    responses(
        (status = 204, description = "The post or page is gone for good"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:publish", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "Nothing in the trash has this key", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn purge_trashed(
    State(trash): State<Trash>,
    Extension(principal): Extension<Principal>,
    Path(key): Path<String>,
    OriginalUri(uri): OriginalUri,
) -> Result<StatusCode, Problem> {
    require_publish(&principal)?;
    let trash = trash.as_deref().ok_or_else(|| not_in_trash(uri.path()))?;
    match trash.purge(&key).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(error) => Err(refused(&error, TrashedKind::Post, uri.path())),
    }
}

/// Every post and page in `trash`, or none when the site keeps no trash.
async fn trashed(trash: Option<&dyn PostTrash>) -> Result<Vec<Trashed>, Problem> {
    match trash {
        Some(trash) => trash.trashed().await.map_err(|e| Problem::internal(&e)),
        None => Ok(Vec::new()),
    }
}

/// The problem for a restore or delete the trash refused, of a copy that
/// was a `kind`.
fn refused(error: &TrashError, kind: TrashedKind, path: &str) -> Problem {
    let noun = match kind {
        TrashedKind::Page => "page",
        _ => "post",
    };
    match error {
        TrashError::NotFound => not_in_trash(path),
        TrashError::Taken { id } => {
            let message = UserMessage::new(
                format!("The {noun} {id} has the address this {noun} had."),
                format!(
                    "Two {noun}s cannot share one address, so restoring it would hide one of them."
                ),
                format!(
                    "Change the slug of the {noun} that has the address now, then restore this one."
                ),
            );
            Problem::new(ProblemType::Conflict, &message).with_instance(path)
        }
        TrashError::Store(error) => Problem::internal(error),
    }
}

/// The 404 for a key nothing in the trash has.
fn not_in_trash(path: &str) -> Problem {
    let message = UserMessage::new(
        "Nothing in the trash has this key.",
        "It may have been restored or deleted for good already.",
        "List the trash to find the post or page you want.",
    );
    Problem::new(ProblemType::NotFound, &message).with_instance(path)
}
