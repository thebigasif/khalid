//! Which of a site's files still name a media item, so the Admin API never
//! deletes a file a post, a page or the theme still shows.

use std::error::Error;

use khalid_media::MediaItem;

use crate::content_store::BoxFuture;

/// Finds the site files that name a media item. `khalid serve` reads the
/// site's content and theme; a router with none deletes without asking.
pub trait MediaUses: std::fmt::Debug + Send + Sync {
    /// The site files that name `item`'s file, as paths from the site
    /// folder such as `content/posts/choosing-a-crm.md`, in order, or none
    /// when nothing names it.
    ///
    /// # Errors
    ///
    /// Returns [`MediaUsesError`] when the site's files cannot be read.
    fn using<'a>(
        &'a self,
        item: &'a MediaItem,
    ) -> BoxFuture<'a, Result<Vec<String>, MediaUsesError>>;
}

/// The site's files could not be read to look for a media item.
#[derive(Debug, thiserror::Error)]
#[error("the site's files could not be read")]
pub struct MediaUsesError(#[source] pub Box<dyn Error + Send + Sync>);
