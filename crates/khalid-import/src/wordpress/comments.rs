//! Which of the comments under each `WordPress` post come in (decision
//! D-4: a move from `WordPress` brings its comments).

use std::path::PathBuf;

use chrono::NaiveDateTime;

use crate::Place;
use crate::wordpress::{Comment, CommentState, Export, ImportedItem};

/// A comment that comes in, under the published post whose file it names.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ImportedComment {
    /// Its id in the export.
    pub id: u64,
    /// The post's file, from the site folder.
    pub post: PathBuf,
    /// The comment it answers, by its id in the export. Khalid's threads
    /// are one level deep, so a reply to a reply answers the first comment
    /// of its thread that came in.
    pub parent: Option<u64>,
    /// The name shown with it, or `Anonymous` when it had none, as
    /// `WordPress` shows it.
    pub author: String,
    /// The writer's email, when they gave one.
    pub email: Option<String>,
    /// When it was sent, in UTC.
    pub at: NaiveDateTime,
    /// What they wrote, as plain text.
    pub text: String,
    /// Whether it was shown under the post, or still waited for approval.
    pub approved: bool,
}

/// A comment that stays behind, with the reason.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LeftComment {
    /// Its id in the export.
    pub id: u64,
    /// Why it stays behind.
    pub reason: CommentLeftReason,
}

/// Why a comment stays behind.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommentLeftReason {
    /// It was marked as spam.
    Spam,
    /// It was in the bin, or in another state Khalid has no place for.
    Removed,
    /// A pingback or trackback, which Khalid does not show.
    Ping,
    /// It is under a page, a draft or an item that did not come in, and
    /// only published posts have comments.
    NotOnPost,
    /// It has no text, or no date.
    Empty,
}

/// The comments of every published post that came in as `items`, in
/// export order, and those that stay behind with the reason.
pub(crate) fn plan_comments(
    export: &Export,
    items: &[ImportedItem],
) -> (Vec<ImportedComment>, Vec<LeftComment>) {
    let (mut came, mut left) = (Vec::new(), Vec::new());
    for item in &export.items {
        let post = items
            .iter()
            .find(|imported| imported.id == item.id && imported.place == Place::Post);
        // A reply comes after the comment it answers, whatever the file's
        // order.
        let mut comments: Vec<&Comment> = item.comments.iter().collect();
        comments.sort_by_key(|comment| comment.id);
        for comment in comments {
            let text = plain_text(&comment.content);
            let reason = match (&comment.state, post, comment.date) {
                _ if comment.ping => Some(CommentLeftReason::Ping),
                (CommentState::Spam, ..) => Some(CommentLeftReason::Spam),
                (CommentState::Trash | CommentState::Other(_), ..) => {
                    Some(CommentLeftReason::Removed)
                }
                (_, None, _) => Some(CommentLeftReason::NotOnPost),
                (_, _, None) => Some(CommentLeftReason::Empty),
                _ if text.is_empty() => Some(CommentLeftReason::Empty),
                _ => None,
            };
            if let Some(reason) = reason {
                left.push(LeftComment {
                    id: comment.id,
                    reason,
                });
                continue;
            }
            let (Some(post), Some(at)) = (post, comment.date) else {
                continue;
            };
            let author = comment.author.trim();
            came.push(ImportedComment {
                id: comment.id,
                post: post.path.clone(),
                parent: thread_root(comment.parent, &item.comments, &came),
                author: if author.is_empty() {
                    "Anonymous"
                } else {
                    author
                }
                .to_owned(),
                email: Some(comment.email.trim().to_owned()).filter(|email| !email.is_empty()),
                at,
                text,
                approved: comment.state == CommentState::Approved,
            });
        }
    }
    (came, left)
}

/// The first comment of the thread that `parent` is in, among those that
/// came in, walking up past the ones that stayed behind.
fn thread_root(parent: Option<u64>, all: &[Comment], came: &[ImportedComment]) -> Option<u64> {
    let mut found = None;
    let mut next = parent;
    // A thread cannot be deeper than the comments it holds, which also
    // stops a loop of parents in a broken export.
    for _ in 0..=all.len() {
        let Some(id) = next else { break };
        if came.iter().any(|comment| comment.id == id) {
            found = Some(id);
        }
        next = all
            .iter()
            .find(|comment| comment.id == id)
            .and_then(|comment| comment.parent);
    }
    found.and_then(|id| {
        let first = came.iter().find(|comment| comment.id == id)?;
        Some(first.parent.unwrap_or(first.id))
    })
}

/// `html` as plain text: its tags gone, its entities read, and its blank
/// lines kept as `WordPress` keeps them.
fn plain_text(html: &str) -> String {
    let fragment = scraper::Html::parse_fragment(html);
    let text: String = fragment.root_element().text().collect();
    text.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_go_and_entities_are_read() {
        assert_eq!(
            plain_text("<p>Rye &amp; <a href=\"x\">spelt</a></p>\n\nYes."),
            "Rye & spelt\n\nYes."
        );
    }
}
