//! The Admin API's post copy, `POST /api/admin/v1/posts/{id}/duplicate/`:
//! a new draft with a post's text and settings, as Yoast's Duplicate Post
//! makes one, to start a similar post or rework a live one safely.

use std::sync::Arc;

use axum::extract::{OriginalUri, Path, State};
use axum::http::header::LOCATION;
use axum::http::{HeaderName, StatusCode};
use axum::{Extension, Json};
use khalid_core::{Slug, UserMessage};

use crate::admin_api::{BASE, missing};
use crate::admin_key::require;
use crate::draft_lint::CreatedPosts;
use crate::keys::{Principal, Scope};
use crate::post_store::{CreatePostError, PostStore};
use crate::problem::{Problem, ProblemType};
use crate::publish_door::PublishDoor;
use crate::resources::{AdminPost, PostDraft, PostEdit, SettingsEdit};

/// How many numbered copies of one post are tried for a free slug.
const MOST_COPIES: u32 = 20;

/// Copies a post, in any status, as a new draft: its title with
/// "(copy)" after it, its text, its author, and every setting but its
/// date, its review, its old addresses and its head code. The copy's slug
/// is the post's with `-copy`, or `-copy-2` and on when that is taken.
/// Answers like a create, with the copy's lint findings and where it
/// opens in the admin. Needs `posts:write`.
#[utoipa::path(
    post, path = "/posts/{id}/duplicate/", tag = "admin posts",
    security(("admin_key" = [])),
    params(("id" = String, Path, description = "The identity of the post to copy")),
    responses(
        (status = 201, description = "The new draft, its lint findings and its admin path", body = CreatedPosts),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No post has this identity", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "Every copy slug is taken", body = Problem, content_type = "application/problem+json"),
    )
)]
pub(crate) async fn duplicate_admin_post(
    State(posts): State<Arc<PublishDoor>>,
    Extension(principal): Extension<Principal>,
    OriginalUri(uri): OriginalUri,
    Path(id): Path<String>,
) -> Result<(StatusCode, [(HeaderName, String); 1], Json<CreatedPosts>), Problem> {
    require(&principal, Scope::PostsWrite)?;
    let store = posts.posts();
    let by = principal.id().as_str();
    let original = store.get(&id).await.map_err(|e| Problem::internal(&e))?;
    let original = original.ok_or_else(|| missing(uri.path()))?;
    let draft = made(store, &original, by).await?;
    let settings = SettingsEdit::copying(original.settings());
    let edit = PostEdit::settings_only(settings, draft.updated_at());
    let copy = store.update(draft.id(), edit, by).await;
    let copy = copy.map_err(|e| Problem::internal(&e))?;
    let location = format!("{BASE}/posts/{}/", copy.id());
    let created = CreatedPosts::linted(copy, store).await;
    Ok((StatusCode::CREATED, [(LOCATION, location)], Json(created)))
}

/// The copy of `original` as a bare draft, at the first free copy slug.
async fn made(store: &dyn PostStore, original: &AdminPost, by: &str) -> Result<AdminPost, Problem> {
    let title = format!("{} (copy)", original.title());
    for number in 1..=MOST_COPIES {
        let slug = match number {
            1 => format!("{}-copy", original.slug()),
            n => format!("{}-copy-{n}", original.slug()),
        };
        let slug = Slug::try_new_unicode(&slug).map_err(|e| Problem::internal(&e))?;
        let draft = PostDraft::new(&title, Some(slug), original.markdown())
            .with_author(original.author().cloned());
        match store.create(draft, by, None).await {
            Ok(post) => return Ok(post),
            Err(CreatePostError::SlugTaken(_)) => {}
            Err(error) => return Err(Problem::internal(&error)),
        }
    }
    let message = UserMessage::new(
        format!("This post already has {MOST_COPIES} copies."),
        "Each copy needs its own address, and every copy address is taken.",
        "Delete or rename an old copy, then copy the post again.",
    );
    Err(Problem::new(ProblemType::Conflict, &message).with_rule("slug"))
}
