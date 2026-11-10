//! Every post of the site as rows of the admin's posts table, each judged
//! by the lint publish gate against one view of the site.

use std::collections::BTreeMap;
use std::fs;

use khalid_api::content_store::{BoxFuture, Listed, select};
use khalid_api::link_map::LinkMap;
use khalid_api::plan_posts::{PlanPosts, PlannedPost};
use khalid_api::post_store::PostStoreError;
use khalid_api::query::ListQuery;
use khalid_api::resources::{Gate, PostLinks, PostRow, PostStatus};
use khalid_content::{PostFront, SiteContent, TopicRole, parse_post};
use khalid_core::Slug;
use khalid_seo::ClusterRole;

use super::{DRAFTS, Editors, Found, Site, SitePosts, blocking, failure};

/// Every post as the publishing planner reads it, for `khalid plan` and
/// the admin calendar's suggested days, on a blocking thread, writing
/// nothing.
impl PlanPosts for SitePosts {
    fn planned(&self) -> BoxFuture<'_, Result<Vec<PlannedPost>, PostStoreError>> {
        Box::pin(blocking(&self.0, Site::planned_now))
    }
}

/// `post` in the topic cluster `front` names, if any.
fn in_cluster(post: PlannedPost, front: &PostFront) -> PlannedPost {
    match &front.cluster {
        Some(cluster) => {
            let role = match cluster.role {
                TopicRole::Pillar => ClusterRole::Pillar,
                _ => ClusterRole::Member,
            };
            post.in_cluster(cluster.name.clone(), role)
        }
        None => post,
    }
}

impl Site {
    /// One page of the posts as `query` asks: every draft, and every
    /// published post that has no draft of the same identity. The linter
    /// and the site view are made once. Every post gets the gate's quick
    /// answer, which only runs the blocking rules; only the rows on the
    /// page get every rule and their findings counted, because several
    /// warnings compare a post with every other post.
    pub(super) fn list_now(&self, query: &ListQuery) -> Result<Listed<PostRow>, PostStoreError> {
        let content = self.content()?;
        let judge = self.judge(&content)?;
        let links = LinkMap::new(&judge.site.posts);
        let editors = Editors::read(&self.folder);
        let mut rows = Vec::new();
        let mut files = BTreeMap::new();
        for found in self.every_post(&content)? {
            let post = self.named_post(&found, &editors)?;
            let candidate = self.candidate_now(&judge, &found)?;
            let gate = if judge.linter.refuses(&candidate, &judge.site) {
                Gate::Refuses
            } else {
                Gate::Passes
            };
            let today = self.clock.now().date_naive();
            let counts = PostLinks::new(
                links.inbound(&candidate).len(),
                links.outbound(&candidate).len(),
            );
            let role = candidate.cluster.as_ref().map(|cluster| cluster.role);
            let row = PostRow::new(&post, gate).with_review_due(today);
            let row = row.with_path(candidate.path.clone());
            rows.push(row.with_reach(candidate.word_count, counts, role));
            files.insert(post.id().to_owned(), found);
        }
        let page = select(rows.iter(), query);
        let total = page.total();
        let mut counted = Vec::new();
        for row in page.into_items() {
            let found = files
                .get(row.id())
                .ok_or_else(|| failure("a listed post has no file"))?;
            counted.push(row.with_report(&self.report(&judge, found)?));
        }
        Ok(Listed::new(counted, total))
    }

    /// Every post as the publishing planner reads it: its Admin API shape,
    /// its front matter, and the gate's quick answer, the same one a list
    /// gives each row.
    pub(super) fn planned_now(&self) -> Result<Vec<PlannedPost>, PostStoreError> {
        let content = self.content()?;
        let judge = self.judge(&content)?;
        let mut planned = Vec::new();
        for found in self.every_post(&content)? {
            let post = Self::admin_post(&found)?;
            let front = parse_post(&found.path, &found.text.to_text())
                .map_err(PostStoreError::new)?
                .front;
            let candidate = self.candidate_now(&judge, &found)?;
            let gate = if judge.linter.refuses(&candidate, &judge.site) {
                Gate::Refuses
            } else {
                Gate::Passes
            };
            planned.push(in_cluster(PlannedPost::new(post, gate), &front));
        }
        Ok(planned)
    }

    /// The drafts in `drafts/<language>/`, then the published posts with a
    /// slug in their front matter, which their identity needs.
    fn every_post(&self, content: &SiteContent) -> Result<Vec<Found>, PostStoreError> {
        let mut found = Vec::new();
        for locale in self.config.languages() {
            let mut folder = self.folder.clone();
            folder.push(format!("{DRAFTS}/{locale}"));
            let Ok(entries) = fs::read_dir(&folder) else {
                continue;
            };
            let mut names: Vec<String> = entries
                .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
                .collect();
            names.sort();
            for name in names {
                let slug = name.strip_suffix(".md").map(Slug::try_new_unicode);
                if let Some(Ok(slug)) = slug {
                    let path = self.draft_path(locale, &slug);
                    found.push(Found::read(locale.clone(), slug, path, PostStatus::Draft)?);
                }
            }
        }
        for doc in &content.posts {
            let Some(slug) = doc.front.slug.clone() else {
                continue;
            };
            if self.draft_path(&doc.locale, &slug).is_file() {
                continue;
            }
            let mut path = self.folder.clone();
            path.push(&doc.path);
            let status = super::publish::content_status(doc.front.date);
            found.push(Found::read(doc.locale.clone(), slug, path, status)?);
        }
        Ok(found)
    }
}
