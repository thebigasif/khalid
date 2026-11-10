//! What one webhook sent, `GET /api/admin/v1/webhooks/{id}/deliveries/`
//! (task 4.10, KH-API-3). The webhook routes serve it.

use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use khalid_core::UserMessage;
use khalid_webhooks::{Attempt, Delivery, DeliveryState, Outcome};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::admin_key::require;
use crate::admin_webhooks::{Webhooks, not_on};
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};

/// How many deliveries a page holds when none is asked for.
const DEFAULT_LIMIT: u32 = 50;

/// The most deliveries one page holds.
const MAX_LIMIT: u32 = 100;

/// Which page of the log to list.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields)]
pub(crate) struct DeliveriesQuery {
    /// Only deliveries older than this one, by its `id`: the `next` of
    /// the page before. Left out, the list starts at the newest.
    before: Option<i64>,
    /// From 1 to 100; 50 when left out.
    limit: Option<u32>,
}

/// One try at sending a delivery: the receiver's status, or why it could
/// not be reached. Exactly one of the two is there.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminAttempt {
    /// When it was made.
    at: DateTime<Utc>,
    /// The HTTP status the receiver answered; 200 to 299 is a success.
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    /// Why the receiver could not be reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

/// Where a delivery stands: waiting for its next try, delivered, or given
/// up after every try failed.
#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StateName {
    Pending,
    Delivered,
    GaveUp,
}

/// One message a webhook sent, with every try.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminDelivery {
    /// Its place in the log: a later delivery has a larger number.
    id: i64,
    /// The event it announced, such as `post.published`.
    event: &'static str,
    /// Where it stands.
    state: StateName,
    /// When the next try is due, or when it was delivered or given up.
    state_at: DateTime<Utc>,
    /// Every try so far, oldest first.
    attempts: Vec<AdminAttempt>,
}

/// One page of one webhook's deliveries.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminDeliveries {
    /// The deliveries, newest first.
    deliveries: Vec<AdminDelivery>,
    /// The `before` that lists the next, older page, or null on the last
    /// page.
    next: Option<i64>,
}

/// A delivery this version does not know how to show.
#[derive(Debug, thiserror::Error)]
#[error("a webhook delivery is in a shape this version cannot show")]
struct UnknownShape;

/// Lists the messages one webhook that is on sent, newest first, a page
/// at a time, each with every try and where it stands. The bodies sent
/// are left out. Needs `webhooks:manage`.
#[utoipa::path(
    get, path = "/webhooks/{id}/deliveries/", tag = "admin webhooks",
    security(("admin_key" = [])),
    params(("id" = i64, Path, description = "The webhook's identity"), DeliveriesQuery),
    responses(
        (status = 200, description = "One page of the webhook's deliveries", body = AdminDeliveries),
        (status = 400, description = "The limit is not from 1 to 100, or before is not a number", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks webhooks:manage", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No webhook that is on has this identity", body = Problem, content_type = "application/problem+json"),
    )
)]
pub(crate) async fn list_deliveries(
    State(webhooks): State<Webhooks>,
    Extension(principal): Extension<Principal>,
    Path(id): Path<String>,
    query: Result<Query<DeliveriesQuery>, QueryRejection>,
) -> Result<Json<AdminDeliveries>, Problem> {
    require(&principal, Scope::WebhooksManage)?;
    let Query(query) = query.map_err(|_| bad_page())?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT);
    if !(1..=MAX_LIMIT).contains(&limit) {
        return Err(bad_page());
    }
    let (Some(store), Ok(id)) = (webhooks, id.parse::<i64>()) else {
        return Err(not_on());
    };
    let on = store.list().await.map_err(|e| Problem::internal(&e))?;
    if !on.iter().any(|webhook| webhook.id == id) {
        return Err(not_on());
    }
    let log = store.deliveries(id, query.before, limit).await;
    let log = log.map_err(|e| Problem::internal(&e))?;
    let full = log.len() == usize::try_from(limit).unwrap_or(usize::MAX);
    let next = log.last().filter(|_| full).map(|last| last.id().get());
    let deliveries = log.iter().map(shown).collect::<Result<_, _>>();
    let deliveries = deliveries.map_err(|e| Problem::internal(&e))?;
    Ok(Json(AdminDeliveries { deliveries, next }))
}

/// The 400 for a page that cannot be read.
fn bad_page() -> Problem {
    let message = UserMessage::new(
        "The page of the delivery log asked for cannot be read: limit must be from 1 to 100, and before a delivery's id.",
        "A page that large would be slow to read and to send.",
        "Ask for limit=50, and pass the next of the page before as before.",
    );
    Problem::new(ProblemType::InvalidQuery, &message).with_rule("limit")
}

/// `delivery` as the Admin API shows it.
fn shown(delivery: &Delivery) -> Result<AdminDelivery, UnknownShape> {
    let (state, state_at) = match *delivery.state() {
        DeliveryState::Pending { next_at } => (StateName::Pending, next_at),
        DeliveryState::Delivered { at } => (StateName::Delivered, at),
        DeliveryState::GaveUp { at } => (StateName::GaveUp, at),
        _ => return Err(UnknownShape),
    };
    let attempts = delivery.attempts().iter().map(attempt);
    Ok(AdminDelivery {
        id: delivery.id().get(),
        event: delivery.event().as_str(),
        state,
        state_at,
        attempts: attempts.collect::<Result<_, _>>()?,
    })
}

/// `attempt` as the Admin API shows it.
fn attempt(attempt: &Attempt) -> Result<AdminAttempt, UnknownShape> {
    let (status, reason) = match attempt.outcome() {
        Outcome::Answered { status } => (Some(*status), None),
        Outcome::Unreachable { reason } => (None, Some(reason.clone())),
        _ => return Err(UnknownShape),
    };
    let at = attempt.at();
    Ok(AdminAttempt { at, status, reason })
}
