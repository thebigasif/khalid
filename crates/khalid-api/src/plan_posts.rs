//! What the publishing planner (GEO task M4) reads of each post, so the
//! publish door can plan from the posts and their schedules for the
//! admin's calendar.

use chrono::{DateTime, Utc};
use khalid_core::Slug;
use khalid_seo::{ClusterRole, PlanPost, PlanState};

use crate::content_store::BoxFuture;
use crate::post_store::PostStoreError;
use crate::resources::{AdminPost, Gate, PostStatus};

/// Reads every post as the publishing planner needs it. `khalid serve`
/// reads the site's files and asks the lint publish gate about each.
pub trait PlanPosts: Send + Sync {
    /// Every post, drafts and live ones, with the gate's answer.
    ///
    /// # Errors
    ///
    /// Returns [`PostStoreError`] when the posts or the lint settings
    /// cannot be read.
    fn planned(&self) -> BoxFuture<'_, Result<Vec<PlannedPost>, PostStoreError>>;
}

/// One post as the planner reads it: its Admin API shape, the gate's
/// answer, and its topic cluster.
#[derive(Debug, Clone)]
pub struct PlannedPost {
    post: AdminPost,
    gate: Gate,
    cluster: Option<(Slug, ClusterRole)>,
}

impl PlannedPost {
    /// `post`, whose gate answer is `gate`, in no cluster.
    pub fn new(post: AdminPost, gate: Gate) -> Self {
        Self {
            post,
            gate,
            cluster: None,
        }
    }

    /// The same post, in topic cluster `name` with `role`.
    #[must_use]
    pub fn in_cluster(self, name: Slug, role: ClusterRole) -> Self {
        Self {
            cluster: Some((name, role)),
            ..self
        }
    }

    /// The post in Admin API shape.
    pub fn post(&self) -> &AdminPost {
        &self.post
    }

    /// The post as the planner weighs it, a draft going live at
    /// `publish_at` when it has a schedule; `None` for a live post with no
    /// date, which is the linter's to report.
    pub fn plan_post(&self, publish_at: Option<DateTime<Utc>>) -> Option<PlanPost> {
        let state = match (self.post.status(), publish_at, self.gate) {
            (PostStatus::Published, ..) => PlanState::Live(self.post.published_at()?.date_naive()),
            (_, Some(at), _) => PlanState::Scheduled(at.date_naive()),
            (_, None, Gate::Passes) => PlanState::Ready,
            (_, None, Gate::Refuses) => PlanState::Blocked,
        };
        let post = PlanPost::new(self.post.id(), self.post.title(), state);
        Some(match &self.cluster {
            Some((name, role)) => post.in_cluster(name.clone(), *role),
            None => post,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn post(gate: Gate) -> PlannedPost {
        let slug = Slug::try_new("a").unwrap();
        let changed = Utc.with_ymd_and_hms(2026, 10, 1, 9, 0, 0).unwrap();
        PlannedPost::new(AdminPost::new("en.a", slug, "A", "Text.", changed), gate)
    }

    /// Post `en.a` as the planner weighs it in `state`.
    fn planned(state: PlanState) -> PlanPost {
        PlanPost::new("en.a", "A", state)
    }

    #[test]
    fn a_draft_is_scheduled_ready_or_blocked() {
        let at = Utc.with_ymd_and_hms(2026, 11, 3, 12, 0, 0).unwrap();
        let scheduled = planned(PlanState::Scheduled(at.date_naive()));
        assert_eq!(post(Gate::Refuses).plan_post(Some(at)), Some(scheduled));
        let ready = planned(PlanState::Ready);
        assert_eq!(post(Gate::Passes).plan_post(None), Some(ready));
        let blocked = planned(PlanState::Blocked);
        assert_eq!(post(Gate::Refuses).plan_post(None), Some(blocked));
    }

    #[test]
    fn a_live_post_counts_on_its_day() {
        let live = Utc.with_ymd_and_hms(2026, 10, 20, 0, 0, 0).unwrap();
        let mut one = post(Gate::Refuses);
        one.post = one.post.published(live);
        let want = planned(PlanState::Live(live.date_naive()));
        assert_eq!(one.plan_post(None), Some(want));
    }
}
