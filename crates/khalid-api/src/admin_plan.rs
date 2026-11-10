//! `GET /api/admin/v1/calendar/plan/`: the publishing planner's suggested
//! days for the admin's calendar (GEO task M4, KH-EDT-4). It only reads,
//! like `khalid plan`; editing a post to `scheduled` takes a suggestion.

use std::sync::Arc;

use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use chrono::NaiveDate;
use khalid_core::UserMessage;
use khalid_seo::{Pace, PaceFrom, Plan, PlanWarning, SlotKind, WaitReason};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::admin_key::require;
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::publish_door::PublishDoor;

/// The pace a plan is asked for.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub(crate) struct PlanQuery {
    /// Posts a week, from 1 to 14. Left out, the site's own pace over the
    /// last twelve weeks.
    #[param(minimum = 1, maximum = 14, example = 2)]
    per_week: Option<u8>,
}

/// Where a plan's pace came from.
#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum PlanPace {
    /// `per_week` was sent.
    Asked,
    /// The site's live posts of the last twelve weeks.
    History,
}

/// Whether a day is set or only suggested.
#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum PlanKind {
    /// The post is scheduled for the day.
    Scheduled,
    /// The planner suggests the day; nothing is set.
    Suggested,
}

/// One post on one day of the plan.
#[derive(Debug, Serialize, ToSchema)]
struct PlanSlot {
    /// The day, in UTC.
    #[schema(value_type = String, format = Date)]
    day: NaiveDate,
    /// The post's identity, as in `/posts/{id}/`.
    id: String,
    /// The title.
    title: String,
    /// Set or suggested.
    kind: PlanKind,
}

/// Why a ready draft has no day.
#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum PlanWhy {
    /// The lint publish gate refuses it.
    Blocked,
    /// Its topic cluster's main post, `pillar`, is refused.
    PillarBlocked,
}

/// A draft with no day, and why.
#[derive(Debug, Serialize, ToSchema)]
struct PlanWaiting {
    /// The post's identity.
    id: String,
    /// The title.
    title: String,
    /// Why it waits.
    why: PlanWhy,
    /// The main post it waits for, with `pillar_blocked`.
    pillar: Option<String>,
}

/// Something to act on: what, why and the fix.
#[derive(Debug, Serialize, ToSchema)]
struct PlanNote {
    /// What the planner found.
    what: String,
    /// Why it matters.
    why: String,
    /// What to do.
    fix: String,
}

/// The plan: the pace, the days from tomorrow, the drafts that wait, and
/// the warnings.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct PlanView {
    /// Posts a week.
    per_week: u8,
    /// Where the pace came from.
    pace_from: PlanPace,
    /// Scheduled and suggested posts, earliest first.
    calendar: Vec<PlanSlot>,
    /// Drafts the linter holds back.
    waiting: Vec<PlanWaiting>,
    /// What to act on.
    warnings: Vec<PlanNote>,
}

/// The plan as the Admin API wraps it.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct PlanBody {
    /// The plan.
    plan: PlanView,
}

impl From<Plan> for PlanView {
    fn from(plan: Plan) -> Self {
        let slot = |slot: khalid_seo::Slot| PlanSlot {
            kind: match slot.kind {
                SlotKind::Scheduled => PlanKind::Scheduled,
                _ => PlanKind::Suggested,
            },
            day: slot.day,
            id: slot.id,
            title: slot.title,
        };
        let waiting = |wait: khalid_seo::Waiting| {
            let (why, pillar) = match wait.why {
                WaitReason::PillarBlocked(pillar) => (PlanWhy::PillarBlocked, Some(pillar)),
                _ => (PlanWhy::Blocked, None),
            };
            PlanWaiting {
                id: wait.id,
                title: wait.title,
                why,
                pillar,
            }
        };
        let note = |warning: &PlanWarning| {
            let message = warning_message(warning);
            PlanNote {
                what: message.what().to_owned(),
                why: message.why().to_owned(),
                fix: message.fix().to_owned(),
            }
        };
        Self {
            per_week: plan.pace.get(),
            pace_from: match plan.pace_from {
                PaceFrom::Asked => PlanPace::Asked,
                _ => PlanPace::History,
            },
            warnings: plan.warnings.iter().map(note).collect(),
            calendar: plan.calendar.into_iter().map(slot).collect(),
            waiting: plan.waiting.into_iter().map(waiting).collect(),
        }
    }
}

/// The warning in words for the admin.
fn warning_message(warning: &PlanWarning) -> UserMessage {
    match warning {
        PlanWarning::PaceDropped { recent, before } => UserMessage::new(
            format!(
                "{recent} {} went live in the last four weeks, against {before} in the four weeks before.",
                if *recent == 1 { "post" } else { "posts" }
            ),
            "Search engines read a steady pace as a sign a site keeps covering its subject.",
            "Schedule the suggested drafts on their days to get back to your pace.",
        ),
        PlanWarning::MemberBeforePillar { member, pillar } => UserMessage::new(
            format!("{member} is scheduled before {pillar}, the main post of its topic."),
            "The post links to its main post, so readers would meet a missing page.",
            format!("Move {member} to a day after {pillar} goes live."),
        ),
        PlanWarning::NoneReady { last } => UserMessage::new(
            last.map_or(
                "No draft is ready and nothing is scheduled.".to_owned(),
                |day| format!("No draft is ready, so nothing goes live after {day}."),
            ),
            "A gap in publishing breaks the steady pace search engines look for.",
            "Finish a draft until its checks pass, and it gets a day here.",
        ),
        _ => UserMessage::new(
            "The planner found something this version cannot describe.",
            "The suggested days still hold.",
            "Update Khalid to read it.",
        ),
    }
}

/// The 400 for a pace that is not a number from 1 to 14.
fn bad_pace() -> Problem {
    let message = UserMessage::new(
        "The pace must be a whole number of posts a week, from 1 to 14.",
        "The planner spaces drafts out at most two a day.",
        "Send per_week from 1 to 14, or leave it out to keep the site's own pace.",
    );
    Problem::new(ProblemType::InvalidQuery, &message).with_rule("per_week")
}

/// The 404 from a server with no planner.
fn no_planner() -> Problem {
    let message = UserMessage::new(
        "This server makes no publishing plan.",
        "The plan reads the site's own posts and checks, which only its server has.",
        "Send the request to the running khalid serve.",
    );
    Problem::new(ProblemType::NotFound, &message)
}

/// Suggests a day for each ready draft from tomorrow, at `per_week` or the
/// site's own pace, with each topic's main post before its members, and
/// says what to act on. Scheduled posts keep their days. Nothing is
/// changed: to take a suggestion, edit the post to `scheduled` with that
/// day's `published_at`. Needs `posts:write`.
#[utoipa::path(
    get, path = "/calendar/plan/", tag = "admin posts",
    security(("admin_key" = [])),
    params(PlanQuery),
    responses(
        (status = 200, description = "The plan", body = PlanBody),
        (status = 400, description = "per_week is not a number from 1 to 14", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "This server makes no plan", body = Problem, content_type = "application/problem+json"),
    )
)]
pub(crate) async fn read_plan(
    State(door): State<Arc<PublishDoor>>,
    Extension(principal): Extension<Principal>,
    query: Result<Query<PlanQuery>, QueryRejection>,
) -> Result<Json<PlanBody>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let Query(PlanQuery { per_week }) = query.map_err(|_| bad_pace())?;
    let pace = match per_week {
        Some(posts) => Some(Pace::per_week(posts).ok_or_else(bad_pace)?),
        None => None,
    };
    let plan = door
        .plan(pace)
        .await
        .map_err(|error| Problem::internal(&error))?
        .ok_or_else(no_planner)?;
    Ok(Json(PlanBody { plan: plan.into() }))
}
