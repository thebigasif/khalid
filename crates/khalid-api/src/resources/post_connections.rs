//! How a post ties to the blog's other posts: its translations, the
//! posts pinned under it, its series and its topic cluster (plan 4.12).

use std::num::NonZeroU32;

use khalid_core::Slug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::ToSchema;

use super::settings_edit::Change;
use super::{ClusterPlace, NonBlank, display};

/// The most related posts a post may pin, the same limit front matter
/// keeps.
pub const MAX_RELATED: usize = 5;

/// How a post ties to the blog's other posts. Every part has a value:
/// `null`, an empty list or `false` when never set.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct PostConnections {
    /// The key shared by the versions of this post in other languages,
    /// such as `anchor-text`, or `null` when it has none.
    #[schema(value_type = Option<String>, example = "anchor-text")]
    pub translation: Option<NonBlank>,
    /// The slugs of the posts shown under this one, in place of the ones
    /// chosen at build time. At most [`MAX_RELATED`].
    #[serde(serialize_with = "slugs")]
    #[schema(value_type = Vec<String>, example = json!(["internal-links"]))]
    pub related: Vec<Slug>,
    /// The series the post is one part of, and its place in it.
    pub series: Option<SeriesPart>,
    /// The topic cluster the post belongs to, and its role in it.
    pub cluster: Option<ClusterMembership>,
    /// The topic group the post shares with others (P8), such as
    /// `internal-links`.
    #[schema(value_type = Option<String>, example = "internal-links")]
    pub topic_group: Option<NonBlank>,
    /// Whether this post is its topic group's main post, the one the
    /// group's other posts should link to.
    pub main_post: bool,
}

/// The series a post is one part of, as an edit sends it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SeriesPart {
    /// The series' name as readers see it, the same on every part.
    #[schema(value_type = String, example = "Topic clusters from scratch")]
    pub name: NonBlank,
    /// The post's place in the series, counted from 1, or `null` to follow
    /// the numbered parts in date order.
    #[serde(default)]
    #[schema(value_type = Option<u32>, minimum = 1, example = 2)]
    pub order: Option<NonZeroU32>,
}

impl SeriesPart {
    /// Part `order` of the series called `name`.
    pub fn new(name: NonBlank, order: Option<NonZeroU32>) -> Self {
        Self { name, order }
    }
}

/// The topic cluster a post belongs to, and its role in it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ClusterMembership {
    /// The cluster's name, the same on every post in it.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "internal-links")]
    pub name: Slug,
    /// `pillar` for the main post, or `member`, the default.
    #[serde(default = "member")]
    pub role: ClusterPlace,
}

impl ClusterMembership {
    /// A post with `role` in the cluster `name`.
    pub fn new(name: Slug, role: ClusterPlace) -> Self {
        Self { name, role }
    }
}

/// The role a cluster's post has when an edit names none.
fn member() -> ClusterPlace {
    ClusterPlace::Member
}

/// New values for some of a post's ties. A part left out keeps its value,
/// and `null` clears one that may be empty.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ConnectionsEdit {
    /// A new translation key, or `null` for none.
    #[serde(default)]
    #[schema(value_type = Option<String>, example = "anchor-text")]
    translation: Change<NonBlank>,
    /// Every pinned related post's slug, at most five; an empty list to
    /// let the build choose.
    #[serde(default, deserialize_with = "at_most_five")]
    #[schema(value_type = Option<Vec<String>>, example = json!(["internal-links"]))]
    related: Option<Vec<Slug>>,
    /// A new series and place, or `null` for none.
    #[serde(default)]
    #[schema(value_type = Option<SeriesPart>)]
    series: Change<SeriesPart>,
    /// A new topic cluster and role, or `null` for none.
    #[serde(default)]
    #[schema(value_type = Option<ClusterMembership>)]
    cluster: Change<ClusterMembership>,
    /// A new topic group, or `null` for none.
    #[serde(default)]
    #[schema(value_type = Option<String>, example = "internal-links")]
    topic_group: Change<NonBlank>,
    /// Whether the post is its topic group's main post.
    main_post: Option<bool>,
}

impl ConnectionsEdit {
    /// `connections` with the parts this edit names put in their place.
    #[must_use]
    pub fn applied_to(self, connections: PostConnections) -> PostConnections {
        PostConnections {
            translation: self.translation.applied_to(connections.translation),
            related: self.related.unwrap_or(connections.related),
            series: self.series.applied_to(connections.series),
            cluster: self.cluster.applied_to(connections.cluster),
            topic_group: self.topic_group.applied_to(connections.topic_group),
            main_post: self.main_post.unwrap_or(connections.main_post),
        }
    }
}

/// Reads the pinned related posts, refusing more than [`MAX_RELATED`] or
/// one listed twice. Serde calls it only for a field that is present.
fn at_most_five<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<Slug>>, D::Error> {
    let slugs = Vec::<Slug>::deserialize(deserializer)?;
    if slugs.len() > MAX_RELATED {
        return Err(serde::de::Error::custom(format!(
            "a post shows at most {MAX_RELATED} related posts, and this lists {}; keep the {MAX_RELATED} that matter most",
            slugs.len()
        )));
    }
    let twice = slugs
        .iter()
        .enumerate()
        .find(|(at, slug)| slugs.iter().take(*at).any(|seen| seen == *slug));
    if let Some((_, slug)) = twice {
        return Err(serde::de::Error::custom(format!(
            "the related post {slug} is listed twice; keep one"
        )));
    }
    Ok(Some(slugs))
}

/// Writes each slug as text.
fn slugs<S: Serializer>(slugs: &[Slug], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(slugs.iter().map(Slug::as_str))
}
