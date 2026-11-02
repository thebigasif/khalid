//! The SEO features added to match Yoast (2 November 2026 comparison) that
//! need wiring beside the pages: archives switched off, the search
//! settings of category and tag pages, and location cards.

pub(crate) mod archives_off;
pub(crate) mod locations;
pub(crate) mod term_seo;

use std::path::Path;

use khalid_core::{Config, UserMessage};
use khalid_routes::Routes;

use crate::build_pages::LanguagePages;

/// Removes the archives switched off, gives category and tag pages their
/// own settings and draws each location block's card. Returns the pages removed, for their redirects,
/// and the first term pages that have no intro, for the build's note.
///
/// # Errors
///
/// What, why and fix for term settings that cannot be read or used.
pub(crate) fn arrange(
    site: &Path,
    (config, routes): (&Config, &Routes),
    languages: &mut [LanguagePages],
) -> Result<(Vec<archives_off::Gone>, Vec<String>), UserMessage> {
    let gone = archives_off::remove(config, languages);
    let bare = term_seo::fill(site, (config, routes), languages)?;
    locations::fill(config, languages)?;
    Ok((gone, bare))
}
