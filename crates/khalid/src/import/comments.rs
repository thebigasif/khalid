//! `khalid import wordpress` stores the comments that come in under their
//! posts in `data/site.db` (decision D-4), each with the time it was sent,
//! shown or waiting for approval as it was on `WordPress`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use khalid_comments::{
    AuthorName, CommentAuthor, CommentId, CommentStatus, CommentStore, CommentText, NewComment,
    PostFile, ReaderEmail,
};
use khalid_core::UserMessage;
use khalid_db::CommentTable;
use khalid_import::wordpress::ImportedComment;

use crate::exit::Exit;
use crate::redirects;

/// What became of the comments an import brought.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Stored {
    /// How many were stored.
    pub(crate) stored: usize,
    /// How many were longer than a comment may be, and stayed behind.
    pub(crate) too_long: usize,
}

/// Stores `comments` in the site at `site`, each under the file its post
/// was written to: its planned file, or where `moved` says the file went.
/// A reply to a comment that stayed behind is stored as a comment of its
/// own.
pub(crate) fn store(
    site: &Path,
    comments: &[ImportedComment],
    moved: &BTreeMap<PathBuf, PathBuf>,
) -> Result<Stored, (UserMessage, Exit)> {
    if comments.is_empty() {
        return Ok(Stored::default());
    }
    let written = redirects::on_main_site(site, async |database, id| {
        let table = CommentTable::new(database.clone(), id);
        let mut ids: BTreeMap<u64, CommentId> = BTreeMap::new();
        let mut done = Stored::default();
        for comment in comments {
            let parent = comment.parent.and_then(|old| ids.get(&old).copied());
            let Some(new) = new_comment(comment, moved, parent) else {
                done.too_long = done.too_long.saturating_add(1);
                continue;
            };
            let status = if comment.approved {
                CommentStatus::Approved
            } else {
                CommentStatus::Pending
            };
            let stored = table.insert(new, status, comment.at.and_utc()).await?;
            ids.insert(comment.id, stored.id);
            done.stored = done.stored.saturating_add(1);
        }
        Ok(done)
    })?;
    written.map_err(|error: khalid_comments::CommentStoreError| {
        let message = UserMessage::new(
            format!("The posts came in, but their comments were not stored: {error}."),
            "Readers do not see the old comments under the imported posts.",
            "Check that data/site.db can be written to, move the imported posts away, then run the import again.",
        );
        (message, Exit::Failure)
    })
}

/// `comment` as Khalid stores it, under `parent`, or `None` when its text
/// is longer than a comment may be. A name too long for Khalid is cut
/// short, and an email Khalid could not write to is left out.
fn new_comment(
    comment: &ImportedComment,
    moved: &BTreeMap<PathBuf, PathBuf>,
    parent: Option<CommentId>,
) -> Option<NewComment> {
    let text = CommentText::parse(&comment.text).ok()?;
    let short: String = comment.author.chars().take(AuthorName::MAX).collect();
    let name = AuthorName::parse(&short)
        .or_else(|_| AuthorName::parse("Anonymous"))
        .ok()?;
    let email = comment
        .email
        .as_deref()
        .and_then(|email| ReaderEmail::parse(email).ok());
    let file = moved.get(&comment.post).unwrap_or(&comment.post);
    Some(NewComment {
        post: PostFile::new(file.to_string_lossy().replace('\\', "/")),
        parent,
        author: CommentAuthor::Reader { name, email },
        text,
    })
}
