//! Readers in and out of the Admin API as CSV (backlog row 9): Khalid's
//! own export, Substack's `email_list` file or Ghost's members export in,
//! and Khalid's export out. Imported readers keep the status the file gave
//! them, and no email goes out to them. Both routes need
//! `subscribers:write`: the file is readers' personal data.

use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::http::header;
use axum::{Extension, Json};
use khalid_core::{Slug, UserMessage};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::require;
use crate::admin_subscribers::{Subscribers, no_store, problem};
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::resources::SubscriberImport;
use crate::subscriber_desk::SubscriberError;

/// The import and export routes, for the subscriber router to guard.
pub(crate) fn routes() -> OpenApiRouter<Subscribers> {
    OpenApiRouter::new()
        .routes(routes!(import_subscribers))
        .routes(routes!(export_subscribers))
}

/// Where imported readers go when the file names no newsletter.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields)]
pub(crate) struct ImportQuery {
    /// The newsletter readers join when the file names none, such as
    /// `weekly`. Left out, they join main.
    #[param(value_type = Option<String>)]
    list: Option<Slug>,
}

/// Brings in a list of readers from Khalid, Substack or Ghost. Each keeps
/// the status the file gave them, and no email goes out. An address the
/// site has is left as it is. Needs `subscribers:write`.
#[utoipa::path(
    post, path = "/subscribers/import/", tag = "admin subscribers",
    security(("admin_key" = [])),
    params(ImportQuery),
    request_body(
        content = String, content_type = "text/csv",
        description = "Khalid's export (email,status,created_at,lists,labels), Substack's email_list file, or Ghost's members export",
        example = "email,status,created_at,lists,labels\nlena@example.com,confirmed,2026-11-01T09:00:00Z,main,paid\n"
    ),
    responses(
        (status = 200, description = "What came in, what was there already, and each row left out", body = SubscriberImport),
        (status = 400, description = "The newsletter in the query is not a slug", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The site keeps no subscribers", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The file is no reader list, or a newsletter does not exist", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn import_subscribers(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
    query: Result<Query<ImportQuery>, QueryRejection>,
    body: String,
) -> Result<Json<SubscriberImport>, Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let Query(query) = query.map_err(|_| bad_query())?;
    let desk = subscribers.ok_or_else(no_store)?;
    let imported = desk.import(&body, query.list.as_ref()).await;
    let imported = imported.map_err(|error| match error {
        SubscriberError::NotReaders(why) => not_readers(&why),
        error => problem(error, ""),
    })?;
    Ok(Json(imported))
}

/// Every reader as Khalid's CSV export, oldest first, to keep or to bring
/// into another site. Needs `subscribers:write`.
#[utoipa::path(
    get, path = "/subscribers/export/", tag = "admin subscribers",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The readers, header first: email,status,created_at,lists,labels", body = String, content_type = "text/csv"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn export_subscribers(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
) -> Result<([(header::HeaderName, &'static str); 3], String), Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let text = match subscribers {
        Some(desk) => desk.export().await.map_err(|error| problem(error, ""))?,
        None => "email,status,created_at,lists,labels\n".to_owned(),
    };
    let headers = [
        (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"subscribers.csv\"",
        ),
        (header::CACHE_CONTROL, "no-store"),
    ];
    Ok((headers, text))
}

fn not_readers(why: &str) -> Problem {
    let message = UserMessage::new(
        format!("This file cannot be brought in: {why}."),
        "Nothing from it was added, so the list is as it was.",
        "Send Khalid's export, Substack's email_list file or Ghost's members export as it came.",
    );
    Problem::new(ProblemType::InvalidBody, &message).with_rule("file")
}

fn bad_query() -> Problem {
    let message = UserMessage::new(
        "The newsletter to import into is not a name the site could have.",
        "A newsletter's name is a slug, such as weekly.",
        "Fix list in the query, or leave it out for the newsletter main.",
    );
    Problem::new(ProblemType::InvalidQuery, &message)
}
