//! The Admin API's subscriber routes: the list, labels, and forgetting a
//! reader (G39, plan 4.6). Readers join only through the sign-up form, so
//! no route adds one.

use std::sync::Arc;

use axum::extract::rejection::QueryRejection;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json, middleware};
use khalid_core::{Slug, UserMessage};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::pagination::Meta;
use crate::problem::{Problem, ProblemType};
use crate::query::Limit;
use crate::resources::{AdminSubscriber, SubscriberFilter, SubscriberStatus};
use crate::subscriber_desk::{LabelChange, SubscriberDesk, SubscriberError};

/// The desk every subscriber route calls, or `None` for a server that
/// keeps no subscribers.
pub(crate) type Subscribers = Option<Arc<dyn SubscriberDesk>>;

/// How many subscribers a page holds when it is not told.
const DEFAULT_LIMIT: u32 = 50;

/// The most subscribers one page holds.
const MAX_LIMIT: u32 = 100;

/// The longest text a search looks for.
const MAX_SEARCH: usize = 200;

/// The subscriber routes, under the Admin API's base. Each needs an
/// integration key with `subscribers:write`, or an owner or admin signed
/// in: addresses are readers' personal data.
pub(crate) fn routes(subscribers: Subscribers, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_subscribers))
        .routes(routes!(erase_subscriber))
        .routes(routes!(add_label, remove_label))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(subscribers)
}

/// One page of subscribers, and where it sits in the whole list.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct SubscriberList {
    /// The subscribers on this page, newest first.
    subscribers: Vec<AdminSubscriber>,
    /// The page, and how many subscribers match in all.
    meta: Meta,
}

/// One subscriber, wrapped the way the list wraps them.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminSubscribers {
    /// The one subscriber.
    subscribers: Vec<AdminSubscriber>,
}

/// What a subscriber list asks for. Every part left out matches everyone.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields)]
pub(crate) struct SubscriberQuery {
    /// Only subscribers who stand here.
    status: Option<SubscriberStatus>,
    /// Only subscribers who get this newsletter, such as `weekly`.
    #[param(value_type = Option<String>)]
    list: Option<Slug>,
    /// Only subscribers who carry this label, such as `paid`.
    #[param(value_type = Option<String>)]
    label: Option<Slug>,
    /// Only subscribers whose address holds this text, in any case.
    q: Option<String>,
    /// The page, from 1.
    page: Option<u32>,
    /// How many to a page: 50 when not given, at most 100.
    limit: Option<u32>,
}

/// Lists the subscribers, newest first, with how many match. Needs
/// `subscribers:write`.
#[utoipa::path(
    get, path = "/subscribers/", tag = "admin subscribers",
    security(("admin_key" = [])),
    params(SubscriberQuery),
    responses(
        (status = 200, description = "A page of subscribers, newest first", body = SubscriberList),
        (status = 400, description = "A filter or the page is not one the API knows", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_subscribers(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
    query: Result<Query<SubscriberQuery>, QueryRejection>,
) -> Result<Json<SubscriberList>, Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let Query(query) = query.map_err(|_| bad_query())?;
    let search = query.q.filter(|q| !q.trim().is_empty());
    if search.as_ref().is_some_and(|q| q.len() > MAX_SEARCH) {
        return Err(bad_query());
    }
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let filter = SubscriberFilter {
        status: query.status,
        list: query.list,
        label: query.label,
        search: search.map(|q| q.trim().to_owned()),
    };
    let Some(desk) = subscribers else {
        let meta = Meta::new(page, Limit::Count(limit), 0);
        let subscribers = Vec::new();
        return Ok(Json(SubscriberList { subscribers, meta }));
    };
    let listed = desk.list(&filter, (page, limit)).await;
    let listed = listed.map_err(|error| problem(error, ""))?;
    let meta = Meta::new(page, Limit::Count(limit), listed.total);
    Ok(Json(SubscriberList {
        subscribers: listed.subscribers,
        meta,
    }))
}

/// Gives a subscriber a label, such as `paid`. Needs `subscribers:write`.
#[utoipa::path(
    put, path = "/subscribers/{id}/labels/{label}/", tag = "admin subscribers",
    security(("admin_key" = [])),
    params(
        ("id" = i64, Path, description = "The subscriber's identity"),
        ("label" = String, Path, description = "The label, in lowercase letters, digits and hyphens"),
    ),
    responses(
        (status = 200, description = "The subscriber, with the label", body = AdminSubscribers),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No subscriber has this identity", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The label is not a slug, or the subscriber unsubscribed", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn add_label(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
    Path((id, label)): Path<(String, String)>,
    OriginalUri(uri): OriginalUri,
) -> Result<Json<AdminSubscribers>, Problem> {
    let change = (id.as_str(), label.as_str(), LabelChange::Add);
    labelled(subscribers, &principal, change, uri.path()).await
}

/// Takes a label away from a subscriber. Needs `subscribers:write`.
#[utoipa::path(
    delete, path = "/subscribers/{id}/labels/{label}/", tag = "admin subscribers",
    security(("admin_key" = [])),
    params(
        ("id" = i64, Path, description = "The subscriber's identity"),
        ("label" = String, Path, description = "The label to take away"),
    ),
    responses(
        (status = 200, description = "The subscriber, without the label", body = AdminSubscribers),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No subscriber has this identity", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The label is not a slug", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn remove_label(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
    Path((id, label)): Path<(String, String)>,
    OriginalUri(uri): OriginalUri,
) -> Result<Json<AdminSubscribers>, Problem> {
    let change = (id.as_str(), label.as_str(), LabelChange::Remove);
    labelled(subscribers, &principal, change, uri.path()).await
}

/// Gives subscriber `id`, asked for at `path`, the label `label`, or
/// takes it away, as `change` says.
async fn labelled(
    subscribers: Subscribers,
    principal: &Principal,
    (id, label, change): (&str, &str, LabelChange),
    path: &str,
) -> Result<Json<AdminSubscribers>, Problem> {
    require(principal, Scope::SubscribersWrite)?;
    let id: i64 = id.parse().map_err(|_| missing(path))?;
    let label = Slug::try_new(label).map_err(|_| bad_label(label))?;
    let desk = subscribers.ok_or_else(|| missing(path))?;
    let changed = desk.label(id, &label, change).await;
    let subscriber = changed.map_err(|error| problem(error, path))?;
    Ok(Json(AdminSubscribers {
        subscribers: vec![subscriber],
    }))
}

/// Removes a subscriber and everything kept about them, for a reader who
/// asks to be forgotten. Needs `subscribers:write`.
#[utoipa::path(
    delete, path = "/subscribers/{id}/", tag = "admin subscribers",
    security(("admin_key" = [])),
    params(("id" = i64, Path, description = "The subscriber's identity")),
    responses(
        (status = 204, description = "The subscriber and their labels are gone"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No subscriber has this identity", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn erase_subscriber(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
    Path(id): Path<String>,
    OriginalUri(uri): OriginalUri,
) -> Result<StatusCode, Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let id: i64 = id.parse().map_err(|_| missing(uri.path()))?;
    let desk = subscribers.ok_or_else(|| missing(uri.path()))?;
    let erased = desk.erase(id).await;
    erased.map_err(|error| problem(error, uri.path()))?;
    Ok(StatusCode::NO_CONTENT)
}

/// The problem a desk error at `path` answers with.
fn problem(error: SubscriberError, path: &str) -> Problem {
    match error {
        SubscriberError::NotFound => missing(path),
        SubscriberError::Unsubscribed => left(),
        error => Problem::internal(&error),
    }
}

fn missing(path: &str) -> Problem {
    let message = UserMessage::new(
        "No subscriber has this identity.",
        "They may have been removed already, or the identity may be mistyped.",
        "List the subscribers to find them.",
    );
    Problem::new(ProblemType::NotFound, &message).with_instance(path)
}

fn left() -> Problem {
    let message = UserMessage::new(
        "This reader unsubscribed, so they get no new label.",
        "A label picks who gets an email, and they get none.",
        "Leave them as they are, or remove them if they asked to be forgotten.",
    );
    Problem::new(ProblemType::InvalidBody, &message).with_rule("status")
}

fn bad_label(label: &str) -> Problem {
    let message = UserMessage::new(
        format!("{label} cannot be a label."),
        "A label is written in lowercase letters, digits and hyphens, such as paid.",
        "Pick a label like paid or from-launch.",
    );
    Problem::new(ProblemType::InvalidBody, &message).with_rule("label")
}

fn bad_query() -> Problem {
    let message = UserMessage::new(
        "The subscriber list was asked for in a way the API does not know.",
        "status takes pending, confirmed or unsubscribed, list and label take a slug, \
         q takes at most 200 characters, and page and limit take whole numbers.",
        "Fix the query, or leave it out to list the newest 50 subscribers.",
    );
    Problem::new(ProblemType::InvalidQuery, &message)
}
