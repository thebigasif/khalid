//! A post as the admin's posts table lists it: the columns it shows and
//! the ones it filters by, without the body.

use chrono::{DateTime, NaiveDate, SecondsFormat, Utc};
use khalid_core::{Indexing, Slug};
use khalid_lint::{ClusterRole, Report, Severity};
use serde::Serialize;
use utoipa::ToSchema;

use super::{AdminPost, Listable, PostStatus, ReviewState, SeoScores, display, display_option};
use crate::query::{FieldValues, Literal};
use crate::staff_names::StaffNames;

/// The lint publish gate's answer for a post as it stands.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Gate {
    /// No blocking finding: the post may go live.
    #[default]
    Passes,
    /// At least one blocking finding: the post stays a draft.
    Refuses,
}

impl Gate {
    /// The name the API uses, such as `refuses`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passes => "passes",
            Self::Refuses => "refuses",
        }
    }
}

/// How many lint findings a post has, by severity. A post with any
/// `blocks` cannot be published.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, ToSchema)]
pub struct LintCounts {
    /// Findings that stop the post from going live.
    blocks: u32,
    /// Findings the writer should see, which do not stop it.
    warnings: u32,
}

impl LintCounts {
    /// `blocks` blocking findings and `warnings` warnings.
    pub fn new(blocks: u32, warnings: u32) -> Self {
        Self { blocks, warnings }
    }

    /// The counts in `report`.
    pub fn of(report: &Report) -> Self {
        let count = |severity| {
            let found = report
                .findings()
                .iter()
                .filter(|f| f.severity() == severity);
            u32::try_from(found.count()).unwrap_or(u32::MAX)
        };
        Self::new(count(Severity::Block), count(Severity::Warn))
    }

    /// Blocking findings.
    pub fn blocks(self) -> u32 {
        self.blocks
    }

    /// Warnings.
    pub fn warnings(self) -> u32 {
        self.warnings
    }

    /// What the gate answers for a post with these findings.
    pub fn gate(self) -> Gate {
        if self.blocks > 0 {
            Gate::Refuses
        } else {
            Gate::Passes
        }
    }
}

/// Whether search engines may list a post, as far as the site itself
/// says. Google's own answer comes with the Search Console connection
/// (plan section 5.11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IndexStatus {
    /// Live, and open to search engines.
    Indexable,
    /// Live, with robots set to `noindex`.
    Noindex,
    /// Live, with a canonical address that credits another page.
    CanonicalElsewhere,
    /// A draft or scheduled post, which search engines cannot reach.
    NotLive,
}

impl IndexStatus {
    /// What `post`'s status and settings say.
    pub fn of(post: &AdminPost) -> Self {
        let settings = post.settings();
        let noindex = |robots: &khalid_core::RobotsDirectives| robots.indexing == Indexing::NoIndex;
        if post.status() != PostStatus::Published {
            Self::NotLive
        } else if settings.robots.as_ref().is_some_and(noindex) {
            Self::Noindex
        } else if settings.canonical.is_some() {
            Self::CanonicalElsewhere
        } else {
            Self::Indexable
        }
    }

    /// The name the API uses, such as `not_live`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Indexable => "indexable",
            Self::Noindex => "noindex",
            Self::CanonicalElsewhere => "canonical_elsewhere",
            Self::NotLive => "not_live",
        }
    }
}

/// One row of the admin's posts table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct PostRow {
    /// The post's identity, as in `/posts/{id}/`.
    id: String,
    /// The last part of its address.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "choosing-anchor-text")]
    slug: Slug,
    /// The title.
    title: String,
    /// Where readers find it on the site, such as a redirect leads to.
    #[schema(example = "/blog/choosing-anchor-text/")]
    path: Option<String>,
    /// The author's slug, if any.
    #[serde(serialize_with = "display_option")]
    #[schema(value_type = Option<String>, example = "asif")]
    author: Option<Slug>,
    /// Draft, published or scheduled.
    status: PostStatus,
    /// Draft, in review or ready.
    review_state: ReviewState,
    /// The day it last came up for review (F3), if it ever has.
    #[serde(serialize_with = "display_option")]
    #[schema(value_type = Option<String>, format = Date, example = "2026-05-02")]
    reviewed: Option<NaiveDate>,
    /// Whether the post is due for review: more than six months since it
    /// was last reviewed or published, whichever came last. A list
    /// filters by this, such as `filter=review:due`.
    review_due: bool,
    /// Whether the post is featured.
    featured: bool,
    /// When it went or goes live; empty for a draft.
    published_at: Option<DateTime<Utc>>,
    /// When it last changed.
    updated_at: DateTime<Utc>,
    /// Who made the last change, as the post names them; `null` when not
    /// known, such as for a file changed by hand.
    updated_by: Option<String>,
    /// The name the admin shows for whoever made the last change: their
    /// account's name, the name their key was made with, or else
    /// `updated_by`; `null` when `updated_by` is.
    #[schema(example = "Maria Lopez")]
    updated_by_name: Option<String>,
    /// Whether the lint publish gate lets it go live. A list filters by
    /// this, such as `gate:refuses`.
    gate: Gate,
    /// Its lint findings, counted. Counting takes every rule, so a list
    /// counts only the rows on the page it sends, and cannot filter or
    /// order by the counts.
    lint: LintCounts,
    /// Its SEO and readability scores, counted with its findings, so only
    /// for the rows on the page a list sends.
    scores: Option<SeoScores>,
    /// How many words its text holds.
    words: u32,
    /// Its internal links to and from the blog's other posts. A list
    /// filters posts no other post links to by `orphan:true`.
    links: PostLinks,
    /// Whether it names a focus keyword. A list filters by this, such as
    /// `has_focus_keyword:false`.
    has_focus_keyword: bool,
    /// Its role in a topic cluster, or `null` outside one: the `pillar`,
    /// the main post the others link to, or a `member`. A list filters
    /// pillars by `pillar:true`, and either role by `cluster:member` or
    /// `cluster:pillar`.
    cluster: Option<ClusterPlace>,
    /// Whether search engines may list it.
    index_status: IndexStatus,
}

/// A post's role in its topic cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClusterPlace {
    /// The cluster's main post, which covers the topic as a whole.
    Pillar,
    /// A post on one part of the topic, which links to the pillar.
    Member,
}

/// How many of the blog's other posts a post links to, and how many link
/// to it, each counted once.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, ToSchema)]
pub struct PostLinks {
    /// Posts that link to this one.
    inbound: u32,
    /// Posts this one links to.
    outbound: u32,
}

impl PostLinks {
    /// `inbound` posts link here, and this links to `outbound` posts.
    pub fn new(inbound: usize, outbound: usize) -> Self {
        let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        Self {
            inbound: count(inbound),
            outbound: count(outbound),
        }
    }
}

impl PostRow {
    /// The row for `post`, whose gate answer is `gate`, with no findings
    /// counted yet; [`PostRow::with_lint`] adds them.
    pub fn new(post: &AdminPost, gate: Gate) -> Self {
        Self {
            id: post.id().to_owned(),
            slug: post.slug().clone(),
            title: post.title().to_owned(),
            path: None,
            author: post.author().cloned(),
            status: post.status(),
            review_state: post.settings().review_state,
            reviewed: post.settings().reviewed,
            review_due: false,
            featured: post.settings().featured,
            published_at: post.published_at(),
            updated_at: post.updated_at(),
            updated_by: post.updated_by().map(str::to_owned),
            updated_by_name: None,
            gate,
            lint: LintCounts::default(),
            scores: None,
            words: 0,
            links: PostLinks::default(),
            has_focus_keyword: post.settings().focus_keyword.is_some(),
            cluster: None,
            index_status: IndexStatus::of(post),
        }
    }

    /// The same row with the post's place on the site: its `words`, its
    /// `links`, and its `role` in a topic cluster, if it is in one.
    #[must_use]
    pub fn with_reach(self, words: usize, links: PostLinks, role: Option<ClusterRole>) -> Self {
        Self {
            words: u32::try_from(words).unwrap_or(u32::MAX),
            links,
            cluster: role.map(|role| match role {
                ClusterRole::Pillar => ClusterPlace::Pillar,
                ClusterRole::Member => ClusterPlace::Member,
            }),
            ..self
        }
    }

    /// The same row with the `path` readers find the post at, when known.
    #[must_use]
    pub fn with_path(self, path: Option<String>) -> Self {
        Self { path, ..self }
    }

    /// The same row with whoever made the last change shown by their name
    /// in `names`.
    #[must_use]
    pub fn named(self, names: &StaffNames) -> Self {
        let updated_by_name = self.updated_by.as_deref().map(|by| names.of(by).to_owned());
        Self {
            updated_by_name,
            ..self
        }
    }

    /// The same row with its findings counted as `lint`.
    #[must_use]
    pub fn with_lint(self, lint: LintCounts) -> Self {
        Self { lint, ..self }
    }

    /// The same row with its findings counted and scored from `report`.
    #[must_use]
    pub fn with_report(self, report: &Report) -> Self {
        Self {
            lint: LintCounts::of(report),
            scores: Some(SeoScores::of(report)),
            ..self
        }
    }

    /// The same row marked due for review when `today` is more than six
    /// months past its last review or its publish day, whichever came
    /// last, or past its last change when it has neither (F3, decision D4).
    #[must_use]
    pub fn with_review_due(self, today: NaiveDate) -> Self {
        let published = self.published_at.map(|at| at.date_naive());
        let last = self
            .reviewed
            .max(published)
            .unwrap_or(self.updated_at.date_naive());
        let due = last
            .checked_add_months(chrono::Months::new(6))
            .is_some_and(|until| until < today);
        Self {
            review_due: due,
            ..self
        }
    }

    /// The post's identity.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The lint publish gate's answer.
    pub fn gate(&self) -> Gate {
        self.gate
    }

    /// Its lint findings, counted.
    pub fn lint(&self) -> LintCounts {
        self.lint
    }

    /// Whether search engines may list it.
    pub fn index_status(&self) -> IndexStatus {
        self.index_status
    }

    /// Whether the post is due for review (F3).
    pub fn review_due(&self) -> bool {
        self.review_due
    }

    /// When it went or goes live.
    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        self.published_at
    }

    /// When it last changed.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl FieldValues for PostRow {
    fn values(&self, field: &str) -> Vec<Literal> {
        let text = |value: &str| vec![Literal::Text(value.to_owned())];
        let time = |at: &DateTime<Utc>| text(&at.to_rfc3339_opts(SecondsFormat::Secs, true));
        match field {
            "id" => text(&self.id),
            "slug" => text(self.slug.as_str()),
            "title" => text(&self.title),
            "author" => self.author.iter().flat_map(|a| text(a.as_str())).collect(),
            "status" => status_text(self.status),
            "review_state" => text(self.review_state.as_str()),
            "reviewed" => self
                .reviewed
                .iter()
                .flat_map(|day| text(&day.format("%Y-%m-%d").to_string()))
                .collect(),
            "review_due" => vec![Literal::Text(
                if self.review_due { "due" } else { "fresh" }.to_owned(),
            )],
            "featured" => vec![Literal::Bool(self.featured)],
            "published_at" => self.published_at.iter().flat_map(time).collect(),
            "updated_at" => time(&self.updated_at),
            "gate" => text(self.gate.as_str()),
            "index_status" => text(self.index_status.as_str()),
            "has_focus_keyword" => vec![Literal::Bool(self.has_focus_keyword)],
            "orphan" => vec![Literal::Bool(self.links.inbound == 0)],
            "pillar" => vec![Literal::Bool(self.cluster == Some(ClusterPlace::Pillar))],
            "cluster" => match self.cluster {
                Some(ClusterPlace::Pillar) => text("pillar"),
                Some(ClusterPlace::Member) => text("member"),
                None => Vec::new(),
            },
            _ => Vec::new(),
        }
    }
}

impl Listable for PostRow {
    const FIELDS: &'static [&'static str] = &[
        "id",
        "slug",
        "title",
        "author",
        "status",
        "review_state",
        "reviewed",
        "review_due",
        "featured",
        "published_at",
        "updated_at",
        "gate",
        "index_status",
        "has_focus_keyword",
        "orphan",
        "pillar",
        "cluster",
    ];

    fn id(&self) -> &str {
        &self.id
    }

    fn slug(&self) -> &Slug {
        &self.slug
    }
}

/// A status as the API writes it, such as `draft`, for a filter to match.
fn status_text(status: PostStatus) -> Vec<Literal> {
    let name = match status {
        PostStatus::Draft => "draft",
        PostStatus::Published => "published",
        PostStatus::Scheduled => "scheduled",
    };
    vec![Literal::Text(name.to_owned())]
}
