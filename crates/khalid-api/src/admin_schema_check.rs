//! The schema check route of the Admin API, `POST /api/admin/v1/schema/checks/`:
//! JSON-LD measured twice, once against the schema.org vocabulary and once
//! against what Google needs for a rich result, with a fix for each finding
//! that has one. It saves nothing. It needs `posts:write`, since its answer
//! goes into a post's custom JSON-LD field.

use axum::extract::rejection::JsonRejection;
use axum::{Extension, Json, middleware};
use khalid_core::UserMessage;
use khalid_seo::{
    AuditContext, FindingLevel, FixTarget, SchemaFinding, SchemaFix, SchemaReports, SeoError,
    schema_reports,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};

/// The longest markup the route reads, in bytes, as for the custom field.
const MAX_BYTES: usize = 64 * 1024;

/// The markup to check.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SchemaCheckAsk {
    /// The JSON-LD, as written: one node, a `@graph` or a list.
    #[schema(example = "{\"@type\": \"Offer\", \"price\": 5, \"pricecurrency\": \"USD\"}")]
    json_ld: String,
    /// The text a reader sees on the page, to flag values the markup gives
    /// that the page never shows. Leave it out to skip that check.
    #[serde(default)]
    visible_text: Option<String>,
    /// `@id`s defined elsewhere on the page that the markup may point to,
    /// such as the site's organisation.
    #[serde(default)]
    known_ids: Vec<String>,
}

/// How much a finding matters.
#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CheckLevel {
    /// Search engines skip or misread what it is about.
    Error,
    /// Search engines may still read it, but it is weaker or risky.
    Warning,
}

/// What a fix changes.
#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CheckFixTarget {
    /// The name of the property at the path.
    Property,
    /// The `@type` at the path.
    Type,
    /// The text value at the path.
    Value,
}

/// One change that settles a finding.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub(crate) struct CheckFix {
    /// What the fix changes.
    target: CheckFixTarget,
    /// Where, as in the finding's path.
    #[schema(example = "offers.pricecurrency")]
    path: String,
    /// The name or value as written.
    #[schema(example = "pricecurrency")]
    from: String,
    /// The name or value it becomes.
    #[schema(example = "priceCurrency")]
    to: String,
    /// The fix as a button says it.
    #[schema(example = "Change pricecurrency to priceCurrency")]
    label: String,
}

/// One thing a check found.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub(crate) struct CheckFinding {
    /// How much it matters.
    level: CheckLevel,
    /// Where it is, such as `@graph[0].offers.priceCurrency`; empty for the
    /// whole markup.
    #[schema(example = "offers.pricecurrency")]
    path: String,
    /// What is wrong, in one sentence.
    #[schema(example = "pricecurrency is not a schema.org property; it looks like priceCurrency.")]
    what: String,
    /// Why it matters.
    why: String,
    /// What to do.
    fix: String,
    /// The one change that settles it, or `null` when there is none.
    change: Option<CheckFix>,
}

/// The two reports on one piece of markup.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub(crate) struct SchemaChecks {
    /// The `@type` of each node at the top, in the order written.
    #[schema(example = json!(["Product"]))]
    types: Vec<String>,
    /// Whether the markup is correct schema.org.
    schema_org: Vec<CheckFinding>,
    /// Whether Google can show it as a rich result.
    google: Vec<CheckFinding>,
}

impl From<SchemaReports> for SchemaChecks {
    fn from(reports: SchemaReports) -> Self {
        let findings = |found: Vec<SchemaFinding>| found.into_iter().map(finding).collect();
        Self {
            types: reports.types,
            schema_org: findings(reports.schema_org),
            google: findings(reports.google),
        }
    }
}

fn finding(found: SchemaFinding) -> CheckFinding {
    CheckFinding {
        level: match found.level {
            FindingLevel::Error => CheckLevel::Error,
            _ => CheckLevel::Warning,
        },
        path: found.path,
        what: found.message.what().to_owned(),
        why: found.message.why().to_owned(),
        fix: found.message.fix().to_owned(),
        change: found.fix.map(fix),
    }
}

fn fix(fix: SchemaFix) -> CheckFix {
    let label = fix.label();
    CheckFix {
        target: match fix.target {
            FixTarget::Property => CheckFixTarget::Property,
            FixTarget::Type => CheckFixTarget::Type,
            _ => CheckFixTarget::Value,
        },
        path: fix.path,
        from: fix.from,
        to: fix.to,
        label,
    }
}

/// The check route, declared with `utoipa::path` (RULES.md BL-API-4),
/// behind an integration key or staff token.
pub(crate) fn routes(door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(check_schema))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
}

/// Checks JSON-LD twice: whether it is correct schema.org, and whether
/// Google can show it as a rich result. Each finding says where it is, what
/// is wrong and how to fix it, and carries the one change that settles it
/// when there is one, such as renaming `pricecurrency` to `priceCurrency`.
/// Nothing is saved. Needs `posts:write`.
#[utoipa::path(
    post, path = "/schema/checks/", tag = "admin posts",
    security(("admin_key" = [])),
    request_body = SchemaCheckAsk,
    responses(
        (status = 200, description = "The two reports", body = SchemaChecks),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The markup is not JSON, or is longer than 64 KB", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn check_schema(
    Extension(principal): Extension<Principal>,
    body: Result<Json<SchemaCheckAsk>, JsonRejection>,
) -> Result<Json<SchemaChecks>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let Json(ask) = body.map_err(|rejection| unreadable(&rejection.body_text()))?;
    if ask.json_ld.len() > MAX_BYTES {
        return Err(unreadable("it is longer than 64 KB"));
    }
    let context = AuditContext {
        visible_text: ask.visible_text.as_deref(),
        known_ids: &ask.known_ids,
    };
    let reports = schema_reports(&ask.json_ld, &context).map_err(|error| not_json(&error))?;
    Ok(Json(reports.into()))
}

fn not_json(error: &SeoError) -> Problem {
    let message = UserMessage::new(
        format!("The markup is not JSON: {error}."),
        "Search engines skip JSON-LD they cannot read, so nothing in it counts.",
        "Fix the place the message names, such as a missing comma or quote.",
    );
    Problem::new(ProblemType::InvalidBody, &message)
}

fn unreadable(why: &str) -> Problem {
    let message = UserMessage::new(
        "The request could not be read as markup to check.",
        format!("The check needs a json_ld field with the markup as text ({why})."),
        "Send {\"json_ld\": \"...\"} with the JSON-LD as one string.",
    );
    Problem::new(ProblemType::InvalidBody, &message)
}
