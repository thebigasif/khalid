//! The trash: a deleted post or page is kept until someone restores it
//! or deletes it for good (KH-EDT-6).

use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::content_store::BoxFuture;
use crate::post_store::PostStoreError;
use crate::resources::{AdminPost, PostStatus};

/// What a copy in the trash was before it was deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TrashedKind {
    /// A post or a draft.
    Post,
    /// A page, such as About, which is live once saved.
    Page,
}

/// A post or page in the trash.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Trashed {
    /// Names this copy in the trash: when it was deleted, then the post's
    /// identity, such as `20261031T093000Z-en.hello`.
    pub key: String,
    /// The post's identity when it was deleted.
    pub id: String,
    /// Its title.
    pub title: String,
    /// Its status when it was deleted.
    pub status: PostStatus,
    /// When it was deleted.
    pub trashed_at: DateTime<Utc>,
    /// Whether it was a post or a page.
    pub kind: TrashedKind,
}

impl Trashed {
    /// The copy of `post` deleted at `at`.
    pub fn new(post: &AdminPost, at: DateTime<Utc>) -> Self {
        Self::listed(
            &key(at, post.id()),
            post.id(),
            post.title(),
            post.status(),
            at,
        )
    }

    /// A post read back from the trash; [`Trashed::of_kind`] makes it a
    /// page.
    pub fn listed(
        key: &str,
        id: &str,
        title: &str,
        status: PostStatus,
        trashed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            key: key.to_owned(),
            id: id.to_owned(),
            title: title.to_owned(),
            status,
            trashed_at,
            kind: TrashedKind::Post,
        }
    }

    /// The same copy, as one that was a `kind`.
    #[must_use]
    pub fn of_kind(self, kind: TrashedKind) -> Self {
        Self { kind, ..self }
    }
}

/// What came back from the trash.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Restored {
    /// A post, as it is now.
    Post(Box<AdminPost>),
    /// A page, which the page routes read.
    Page,
}

/// The key of post `id`'s copy deleted at `at`: the time to the second,
/// then the identity, so keys sort by when the post was deleted.
pub fn key(at: DateTime<Utc>, id: &str) -> String {
    format!("{}-{id}", at.format("%Y%m%dT%H%M%SZ"))
}

/// Where deleted posts and pages wait. `khalid serve` keeps them as files
/// under `trash/`; `MemoryPostStore`, from the `test-util` feature, keeps
/// the posts it deletes.
pub trait PostTrash: Send + Sync {
    /// Every post and page in the trash, the most recently deleted first.
    ///
    /// # Errors
    ///
    /// Returns [`PostStoreError`] when the trash cannot be read.
    fn trashed(&self) -> BoxFuture<'_, Result<Vec<Trashed>, PostStoreError>>;

    /// Puts the copy at `key` back where it was, in the status it had, and
    /// answers what came back.
    ///
    /// # Errors
    ///
    /// Returns [`TrashError::NotFound`] when nothing in the trash has that
    /// key, [`TrashError::Taken`] when a post or page has its address now,
    /// and [`TrashError::Store`] when the files cannot be read or written.
    fn restore<'a>(&'a self, key: &'a str) -> BoxFuture<'a, Result<Restored, TrashError>>;

    /// Deletes the copy at `key` for good.
    ///
    /// # Errors
    ///
    /// Returns [`TrashError::NotFound`] when nothing in the trash has that
    /// key, and [`TrashError::Store`] when the trash cannot be written.
    fn purge<'a>(&'a self, key: &'a str) -> BoxFuture<'a, Result<(), TrashError>>;
}

/// Why a copy in the trash was not restored or deleted.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TrashError {
    /// Nothing in the trash has that key.
    #[error("nothing in the trash has that key")]
    NotFound,
    /// Another post or page has the address the restored one would take.
    #[error("{id} has the address the restored copy would take")]
    Taken {
        /// The identity both share.
        id: String,
    },
    /// The files could not be read or written.
    #[error(transparent)]
    Store(#[from] PostStoreError),
}
