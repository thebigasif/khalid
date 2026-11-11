//! A find-and-replace across the library's alt text,
//! `/api/admin/v1/media/alt-text/` (plan 5.1).

use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::{Extension, Json};
use khalid_core::UserMessage;
use khalid_media::{AltChange, AltReplace, ReplaceAltError};

use crate::admin_key::require;
use crate::keys::{Principal, Scope};
use crate::media_api::{MediaLibrary, no_library};
use crate::problem::{Problem, ProblemType};
use crate::resources::{AltChangeBody, AltChanges, AltReplaceBody};

/// Finds words in the alt text of every library item and puts others in
/// their place. Without `apply` it only lists what would change, so a
/// person can check before saving. Captions, folders and addresses stay
/// as they are. Needs `media:write`.
#[utoipa::path(
    post, path = "/media/alt-text/", tag = "admin media",
    security(("admin_key" = [])),
    request_body = AltReplaceBody,
    responses(
        (status = 200, description = "What changes, or changed", body = AltChanges),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks media:write", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The body could not be read, the words to find are empty, or a change would make alt text longer than 1000 characters", body = Problem, content_type = "application/problem+json"),
    )
)]
pub(crate) async fn replace_alt(
    State(media): State<Option<MediaLibrary>>,
    Extension(principal): Extension<Principal>,
    body: Result<Json<AltReplaceBody>, JsonRejection>,
) -> Result<Json<AltChanges>, Problem> {
    require(&principal, Scope::MediaWrite)?;
    let media = media.ok_or_else(no_library)?;
    let Json(body) = body.map_err(|rejection| unreadable(&rejection.body_text()))?;
    let replace = AltReplace::new(&body.find, &body.replace)
        .ok_or_else(|| unreadable("find must hold the words to look for"))?;
    let library = &media.library;
    let changes = if body.apply {
        library.replace_alt(&replace, principal.id().as_str()).await
    } else {
        library.alt_changes(&replace)
    };
    let changes = changes.map_err(|error| match error {
        ReplaceAltError::TooLong { id, limit } => {
            let name = library.get(&id).map(|item| item.name.as_str().to_owned());
            too_long(&name.unwrap_or_else(|| id.to_string()), limit)
        }
        error => Problem::internal(&error),
    })?;
    Ok(Json(AltChanges {
        applied: body.apply,
        changes: changes.into_iter().map(shown).collect(),
    }))
}

fn shown(change: AltChange) -> AltChangeBody {
    AltChangeBody {
        id: change.id.to_string(),
        name: change.name.as_str().to_owned(),
        before: change.before,
        after: change.after,
    }
}

fn unreadable(reason: &str) -> Problem {
    let message = UserMessage::new(
        format!("The replace could not be read: {reason}."),
        "No alt text was changed.",
        "Send find, replace and apply as in /api/openapi.json.",
    );
    Problem::new(ProblemType::InvalidBody, &message)
}

fn too_long(name: &str, limit: usize) -> Problem {
    let message = UserMessage::new(
        format!("This replace would make the alt text of {name} longer than {limit} characters."),
        "Screen readers read alt text whole, so it is kept short. No alt text was changed.",
        format!("Put shorter words in place, or shorten the alt text of {name} first."),
    );
    Problem::new(ProblemType::InvalidBody, &message).with_rule("alt")
}
