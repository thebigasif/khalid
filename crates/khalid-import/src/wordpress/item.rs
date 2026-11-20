//! One item of a `WordPress` export, with its text as written.

use chrono::NaiveDateTime;
use khalid_core::AbsoluteUrl;

/// One item of the export.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Item {
    /// The id, which other items use to point at this one.
    pub id: u64,
    /// What kind of item it is.
    pub kind: ItemKind,
    /// Whether it was published.
    pub status: ItemStatus,
    /// The headline.
    pub title: String,
    /// The slug, which may be percent-encoded, or empty for an unsaved draft.
    pub name: String,
    /// The address readers knew it by.
    pub link: Option<AbsoluteUrl>,
    /// The author's sign-in name.
    pub creator: String,
    /// When it was published, in the site's own time zone. None for no date.
    pub published: Option<NaiveDateTime>,
    /// The body, as HTML.
    pub content: String,
    /// The hand-written summary, which may be empty.
    pub excerpt: String,
    /// Whether the post was pinned to the top, which Khalid calls featured.
    pub sticky: bool,
    /// The item this one belongs to, such as the post an image was added to.
    pub parent: Option<u64>,
    /// The file's address, for an attachment.
    pub attachment_url: Option<AbsoluteUrl>,
    /// The slugs of its categories, in file order.
    pub categories: Vec<String>,
    /// The slugs of its tags, in file order.
    pub tags: Vec<String>,
    /// Every custom field in file order, where SEO plugins keep their data.
    pub meta: Vec<Meta>,
    /// Its comments, pingbacks and trackbacks, in file order.
    pub comments: Vec<Comment>,
}

impl Item {
    /// The value of the first custom field called `key`.
    pub fn meta(&self, key: &str) -> Option<&str> {
        self.meta
            .iter()
            .find(|meta| meta.key == key)
            .map(|meta| meta.value.as_str())
    }
}

/// One custom field of an item.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Meta {
    /// The field's name, such as `_yoast_wpseo_title`.
    pub key: String,
    /// The field's value, as stored.
    pub value: String,
}

/// One comment as the export holds it, a `wp:comment` of an item.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Comment {
    /// Its id, which a reply names as its parent.
    pub id: u64,
    /// The comment it answers, if any.
    pub parent: Option<u64>,
    /// The name shown with it.
    pub author: String,
    /// The writer's email, which may be empty.
    pub email: String,
    /// When it was sent, in UTC, or in the site's own time when the export
    /// has no UTC time.
    pub date: Option<NaiveDateTime>,
    /// What they wrote, as `WordPress` keeps it: text with some HTML.
    pub content: String,
    /// Where it stood in moderation.
    pub state: CommentState,
    /// Whether it is a pingback or trackback, a note another site left
    /// automatically, rather than a reader's comment.
    pub ping: bool,
}

/// Where a comment stood in `WordPress`'s moderation, `wp:comment_approved`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommentState {
    /// Shown under the post, `1`.
    Approved,
    /// Waiting for approval, `0`.
    Pending,
    /// Marked as spam.
    Spam,
    /// In the bin.
    Trash,
    /// Any other state, by its name in the file.
    Other(String),
}

impl CommentState {
    pub(crate) fn from_name(name: &str) -> Self {
        match name {
            "1" | "approve" => Self::Approved,
            "0" | "hold" => Self::Pending,
            "spam" => Self::Spam,
            "trash" | "post-trashed" => Self::Trash,
            other => Self::Other(other.to_owned()),
        }
    }
}

/// What kind of item an export holds.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ItemKind {
    /// A blog post.
    Post,
    /// A page, such as About.
    Page,
    /// An uploaded file, such as an image.
    Attachment,
    /// Anything else, such as a menu item, by its name in the file.
    Other(String),
}

impl ItemKind {
    pub(crate) fn from_name(name: &str) -> Self {
        match name {
            "post" => Self::Post,
            "page" => Self::Page,
            "attachment" => Self::Attachment,
            other => Self::Other(other.to_owned()),
        }
    }
}

/// Whether an item was published.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ItemStatus {
    /// Live on the site.
    Publish,
    /// Scheduled for later.
    Future,
    /// Being written.
    Draft,
    /// Waiting for review.
    Pending,
    /// Visible only to signed-in editors.
    Private,
    /// In the bin.
    Trash,
    /// Takes the status of its parent, as attachments do.
    Inherit,
    /// Any other status, by its name in the file.
    Other(String),
}

impl ItemStatus {
    pub(crate) fn from_name(name: &str) -> Self {
        match name {
            "publish" => Self::Publish,
            "future" => Self::Future,
            "draft" => Self::Draft,
            "pending" => Self::Pending,
            "private" => Self::Private,
            "trash" => Self::Trash,
            "inherit" => Self::Inherit,
            other => Self::Other(other.to_owned()),
        }
    }
}
