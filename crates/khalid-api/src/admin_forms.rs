//! The Admin API's form routes: which forms visitors answered, and each
//! form's answers, newest first (KH-GRO-1, plan section 4.6). Answers hold
//! what visitors typed, such as their email address, so both routes need
//! `subscribers:write`, the scope for what visitors send the site.

use std::sync::Arc;

use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json, middleware};
use khalid_core::UserMessage;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::form_answers::{FormAnswer, FormAnswers, FormSummary};
use crate::keys::{Principal, Scope};
use crate::pagination::Meta;
use crate::problem::{Problem, ProblemType};
use crate::query::Limit;

/// The answers the routes read, or `None` on a server that keeps none.
type Kept = Option<Arc<dyn FormAnswers>>;

/// How many answers a page holds when it is not told.
const DEFAULT_LIMIT: u32 = 25;

/// The most answers one page holds.
const MAX_LIMIT: u32 = 100;

/// The form routes, under the Admin API's base.
pub(crate) fn routes(forms: Kept, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_forms))
        .routes(routes!(list_answers))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(forms)
}

/// The forms visitors answered.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminForms {
    /// Each form with at least one answer, the one answered last first.
    forms: Vec<FormSummary>,
}

/// One page of a form's answers.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminAnswers {
    /// The answers on this page, newest first.
    answers: Vec<FormAnswer>,
    /// Pagination.
    meta: Meta,
}

/// Which page of answers to send.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields)]
pub(crate) struct AnswerQuery {
    /// The page, from 1.
    page: Option<u32>,
    /// Answers per page: 25 when not given, from 1 to 100.
    limit: Option<u32>,
}

/// Lists the forms visitors answered, with how many answers each has and
/// when the newest came. Spam caught by the trap is left out. Needs
/// `subscribers:write`.
#[utoipa::path(
    get, path = "/forms/", tag = "admin forms",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "Every form with answers", body = AdminForms),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server keeps no form answers", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_forms(
    State(forms): State<Kept>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<AdminForms>, Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let listed = kept(forms.as_deref())?.forms().await;
    let forms = listed.map_err(|error| Problem::internal(&error))?;
    Ok(Json(AdminForms { forms }))
}

/// Lists one page of a form's answers, newest first, each field in the
/// order the form sent them. A form with no answers lists none. Needs
/// `subscribers:write`.
#[utoipa::path(
    get, path = "/forms/{id}/answers/", tag = "admin forms",
    security(("admin_key" = [])),
    params(("id" = String, Path, description = "The form's id, as its block names it"), AnswerQuery),
    responses(
        (status = 200, description = "One page of the form's answers", body = AdminAnswers),
        (status = 400, description = "The page or limit is not a whole number in range", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server keeps no form answers", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_answers(
    State(forms): State<Kept>,
    Extension(principal): Extension<Principal>,
    Path(id): Path<String>,
    query: Result<Query<AnswerQuery>, QueryRejection>,
) -> Result<Json<AdminAnswers>, Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let Query(query) = query.map_err(|_| bad_query())?;
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT);
    if page == 0 || !(1..=MAX_LIMIT).contains(&limit) {
        return Err(bad_query());
    }
    let offset = u64::from(page - 1) * u64::from(limit);
    let read = kept(forms.as_deref())?.answers(&id, offset, limit).await;
    let read = read.map_err(|error| Problem::internal(&error))?;
    let meta = Meta::new(page, Limit::Count(limit), read.total);
    Ok(Json(AdminAnswers {
        answers: read.answers,
        meta,
    }))
}

/// The answers, or the 404 of a server that keeps none.
fn kept(forms: Option<&dyn FormAnswers>) -> Result<&dyn FormAnswers, Problem> {
    forms.ok_or_else(|| {
        let message = UserMessage::new(
            "This server keeps no form answers.",
            "Answers are kept in the site's database, which only khalid serve opens.",
            "Send the request to the running khalid serve.",
        );
        Problem::new(ProblemType::NotFound, &message)
    })
}

/// The 400 for a page or limit out of range.
fn bad_query() -> Problem {
    let message = UserMessage::new(
        "The answers list takes a page from 1 and a limit from 1 to 100.",
        "Answers come a page at a time, so a long list stays quick to send.",
        "Fix page or limit, or leave them out for the newest 25 answers.",
    );
    Problem::new(ProblemType::InvalidQuery, &message).with_rule("limit")
}
