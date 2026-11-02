//! A hub: the front door to one topic, with its pillar intro, its chapters
//! in order and the topic's newest posts.

use serde::Serialize;

use crate::{BodyHtml, Link, PostCard};

/// The page of a topic's hub.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderHub {
    /// The heading, shown as the page's one `h1`.
    pub title: String,
    /// The pillar intro, the page's own text, already sanitised.
    pub body: BodyHtml,
    /// The category the hub opens, linked, for "View all".
    pub topic: Option<Link>,
    /// The posts to read first, in order.
    pub chapters: Vec<PostCard>,
    /// The topic's newest posts that are not chapters, newest first.
    pub latest: Vec<PostCard>,
}
