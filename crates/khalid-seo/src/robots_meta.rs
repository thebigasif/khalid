//! The site's index policy per page type. The directives themselves are
//! [`khalid_core::RobotsDirectives`], shared with front matter.

use std::collections::BTreeMap;

use khalid_core::{ArchiveIndex, Indexing, RobotsDirectives};

use crate::{PageKind, SeoPage};

/// Which page types search engines may index, and when an archive is too thin.
///
/// By default search results, date archives and the not-found page are
/// `noindex`, and so is a tag or author archive with fewer than three posts.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct IndexPolicy {
    /// The setting per page type. A type not listed is indexed.
    pub by_kind: BTreeMap<PageKind, Indexing>,
    /// A tag or author archive listing fewer posts than this is `noindex`.
    pub thin_archive_below: u32,
    /// The whole site. `NoIndex` keeps every page out, over the setting per
    /// type and a page's own directives, as a demo or staging copy needs.
    pub whole_site: Indexing,
    /// The setting for the posts of one collection, by its name, over the
    /// setting for every post.
    pub by_collection: BTreeMap<String, Indexing>,
}

impl Default for IndexPolicy {
    fn default() -> Self {
        Self {
            by_kind: BTreeMap::from([
                (PageKind::Search, Indexing::NoIndex),
                (PageKind::Date, Indexing::NoIndex),
                (PageKind::NotFound, Indexing::NoIndex),
            ]),
            thin_archive_below: 3,
            whole_site: Indexing::Index,
            by_collection: BTreeMap::new(),
        }
    }
}

impl From<&ArchiveIndex> for IndexPolicy {
    /// The policy `[seo.index]` in `site.toml` asks for. The not-found page
    /// is never indexed, whatever the settings say.
    fn from(settings: &ArchiveIndex) -> Self {
        Self {
            by_kind: BTreeMap::from([
                (PageKind::Post, settings.posts),
                (PageKind::Page, settings.pages),
                (PageKind::Tag, settings.tags),
                (PageKind::Category, settings.categories),
                (PageKind::Author, settings.authors.indexing()),
                (PageKind::Date, settings.dates.indexing()),
                (PageKind::Search, settings.search),
                (PageKind::NotFound, Indexing::NoIndex),
            ]),
            thin_archive_below: settings.thin_archives_below,
            whole_site: settings.site,
            by_collection: settings.collections.clone(),
        }
    }
}

impl IndexPolicy {
    /// The directives for `page`: its own when it has them, otherwise the
    /// defaults with this policy's index setting. A site set to `noindex`
    /// keeps the page out either way.
    pub fn directives_for(&self, page: &SeoPage) -> RobotsDirectives {
        let mut directives = self.own_or_by_kind(page);
        if self.whole_site == Indexing::NoIndex {
            directives.indexing = Indexing::NoIndex;
        }
        directives
    }

    fn own_or_by_kind(&self, page: &SeoPage) -> RobotsDirectives {
        if let Some(own) = &page.robots {
            return own.clone();
        }
        let thin = matches!(page.kind, PageKind::Tag | PageKind::Author)
            && page
                .item_count
                .is_some_and(|count| count < self.thin_archive_below);
        let collection = (page.kind == PageKind::Post)
            .then(|| self.by_collection.get(page.collection.as_deref()?))
            .flatten();
        let indexing = if thin {
            Indexing::NoIndex
        } else if let Some(setting) = collection {
            *setting
        } else {
            self.by_kind.get(&page.kind).copied().unwrap_or_default()
        };
        let mut directives = RobotsDirectives::default();
        directives.indexing = indexing;
        directives
    }
}
