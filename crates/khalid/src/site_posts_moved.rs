//! A live post's new slug (KH-SEO-8, ADR 0010). Its file stays where it
//! is, `slug:` changes, and its old address joins `aliases`, so the old
//! address answers with a 301 to the new one on `khalid serve` and in the
//! `_redirects` of a static build.

use chrono::NaiveDate;
use khalid_api::post_store::PostStoreError;
use khalid_api::resources::{Alias, PostSettings};
use khalid_core::Slug;
use khalid_routes::{PostAddress, Routes};

use super::{Found, Site};

impl Site {
    /// `after` with the post's address under its old slug and `before`'s
    /// date added to `aliases`, and its address under `slug` and `after`'s
    /// date taken out, so moving back to an earlier slug makes no loop.
    pub(super) fn moved_aliases(
        &self,
        found: &Found,
        slug: &Slug,
        (before, after): (&PostSettings, &PostSettings),
    ) -> Result<PostSettings, PostStoreError> {
        let routes = Routes::load(&self.folder).map_err(PostStoreError::new)?;
        let relative = found.path.strip_prefix(&self.folder).ok();
        let collection = relative.and_then(|path| path.iter().nth(2)?.to_str());
        let collection = collection.unwrap_or("posts");
        let address = |slug: &Slug, date: Option<NaiveDate>| {
            let post = PostAddress::new(collection, &found.locale, slug, date);
            let address = routes.address(&self.config, &post).ok();
            address.and_then(|address| Alias::try_new(&address))
        };
        let mut settings = after.clone();
        let new = address(slug, after.date);
        settings.aliases.retain(|alias| Some(alias) != new.as_ref());
        if let Some(old) = address(&found.slug, before.date)
            && !settings.aliases.contains(&old)
        {
            settings.aliases.push(old);
        }
        Ok(settings)
    }
}
