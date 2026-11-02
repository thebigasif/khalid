//! The author profile a staff account writes as: an author file in the
//! site's default language, found or made through the same store the Admin
//! API's author routes use (plan sections 4.4 and 4.6).

use std::path::Path;

use khalid_api::content_store::BoxFuture;
use khalid_api::entry_store::{Entry, EntryStore};
use khalid_api::resources::{AdminAuthor, NewAuthor};
use khalid_auth::AuthorProfiles;
use khalid_core::{Config, Locale, Slug, SlugMode};

use crate::site_entries::SiteEntries;

/// The author files of one site, as profiles for its staff accounts.
pub(crate) struct SiteAuthorProfiles {
    authors: SiteEntries<AdminAuthor>,
    language: Locale,
    mode: SlugMode,
}

impl SiteAuthorProfiles {
    /// The author files of the `site` folder, whose settings are `config`.
    pub(crate) fn new(site: &Path, config: &Config) -> Self {
        Self {
            authors: SiteEntries::new(site, config),
            language: config.default_language().clone(),
            mode: config.slugs(),
        }
    }
}

impl AuthorProfiles for SiteAuthorProfiles {
    fn profile_for<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Option<String>> {
        Box::pin(async move {
            let slug = Slug::from_title(name, &self.language, self.mode).ok()?;
            let id = format!("{}.{slug}", self.language);
            match self.authors.get(&id).await {
                Ok(Some(found)) => return Some(found.id().to_owned()),
                Ok(None) => {}
                Err(error) => {
                    tracing::warn!(%error, "the author files could not be read, so the account has no author profile yet");
                    return None;
                }
            }
            match self.authors.create(NewAuthor::named(slug, name)).await {
                Ok(made) => Some(made.id().to_owned()),
                Err(error) => {
                    tracing::warn!(%error, "the author file could not be made, so the account has no author profile yet");
                    None
                }
            }
        })
    }
}
