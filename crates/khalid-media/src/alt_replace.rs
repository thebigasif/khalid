//! A find-and-replace across the alt text of every library item (plan 5.1).

use crate::media_edit::MAX_ALT;
use crate::{ContentKey, FileName, MediaError, MediaItem};

/// Words to find in alt text, and what to put in their place. Matching is
/// exact, letter case included, so a replace never touches more than the
/// preview showed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AltReplace {
    find: String,
    replace: String,
}

/// One item's alt text before and after a replace.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct AltChange {
    /// The item.
    pub id: ContentKey,
    /// Its file name, so a person can tell the items apart.
    pub name: FileName,
    /// Its alt text now.
    pub before: String,
    /// Its alt text after the replace, trimmed.
    pub after: String,
}

/// Why a replace was not made.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReplaceAltError {
    /// The replace would make one item's alt text too long. Nothing was
    /// changed.
    #[error("the alt text of {id} would be longer than {limit} characters")]
    TooLong {
        /// The first item it would make too long.
        id: ContentKey,
        /// The most characters alt text may have.
        limit: usize,
    },
    /// A change could not be written. Items before it in the list were
    /// changed; the rest were not.
    #[error(transparent)]
    Media(#[from] MediaError),
}

impl AltReplace {
    /// Find `find` and put `replace` in its place, or `None` when `find`
    /// is empty or only spaces, which would match every item.
    pub fn new(find: &str, replace: &str) -> Option<Self> {
        if find.trim().is_empty() {
            return None;
        }
        Some(Self {
            find: find.to_owned(),
            replace: replace.to_owned(),
        })
    }

    /// The change to `item`'s alt text, or `None` when it does not hold
    /// the words.
    ///
    /// # Errors
    ///
    /// Returns [`ReplaceAltError::TooLong`] when the new alt text is over
    /// its limit.
    pub(crate) fn change(&self, item: &MediaItem) -> Result<Option<AltChange>, ReplaceAltError> {
        if !item.alt.contains(&self.find) {
            return Ok(None);
        }
        let after = item
            .alt
            .replace(&self.find, &self.replace)
            .trim()
            .to_owned();
        if after.chars().count() > MAX_ALT {
            return Err(ReplaceAltError::TooLong {
                id: item.id.clone(),
                limit: MAX_ALT,
            });
        }
        Ok(Some(AltChange {
            id: item.id.clone(),
            name: item.name.clone(),
            before: item.alt.clone(),
            after,
        }))
    }
}
