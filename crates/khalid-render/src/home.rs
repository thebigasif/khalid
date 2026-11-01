//! The blog's front page: its newest posts, or sections set in `site.toml`,
//! then a row of links to every topic.

use serde::Serialize;

use crate::{Link, PostCard, RenderListing};

/// The most editor's picks the home page carries, the most the `picks`
/// part may show.
pub const MOST_PICKS: usize = 8;

/// The front page in one language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderHome {
    /// The title, intro and newest posts, shown when there are no sections.
    #[serde(flatten)]
    pub listing: RenderListing,
    /// The sections from `[[reader.home_sections]]`, in order. When there
    /// are any, they replace the one list of newest posts.
    pub sections: Vec<HomeSectionPosts>,
    /// The posts marked `featured`, newest first, at most [`MOST_PICKS`],
    /// for the editor's picks a template with its `picks` part on shows.
    pub picks: Vec<PostCard>,
    /// The links to every category, grouped the way the menu groups them.
    pub topics: Vec<TopicGroup>,
    /// Each category with posts in this language, in the order
    /// `categories.toml` lists them, for the tabs a template with its
    /// `category_tabs` part on shows.
    pub categories: Vec<Link>,
}

/// One section of the front page: the newest posts of a category, a
/// collection or a channel, and a link to all of them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HomeSectionPosts {
    /// The section's name, such as `Technical SEO`.
    pub title: String,
    /// The page with all of its posts, for "View all".
    pub href: String,
    /// Its newest posts.
    pub posts: Vec<PostCard>,
}

/// A group of topic links. A group without a name is shown without a
/// heading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopicGroup {
    /// The group's name, such as `Guides`.
    pub name: Option<String>,
    /// Its categories, linked.
    pub links: Vec<Link>,
}
