//! The SEO settings of a site, from the `[seo]` table of `site.toml`.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{
    AbsoluteUrl, AiPageSettings, BreadcrumbSettings, CrawlerSettings, FeedSettings, IndexNowKey,
    Indexing, LinkSettings, LocationSettings, PublisherSettings, SchemaTemplateSettings,
    SocialSettings, TitleSettings, VerificationSettings,
};

/// The SEO settings. Every one has a default, so the table may be left out:
///
/// ```toml
/// [seo.index]
/// tags = "noindex"
/// authors = "off"
/// thin_archives_below = 5
///
/// [seo.indexnow]
/// key = "3f9c0b5e2a8d4c71"
///
/// [seo.links]
/// affiliate_domains = ["amazon.com"]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct SeoSettings {
    /// Which archive pages search engines may list, from `[seo.index]`.
    pub index: ArchiveIndex,
    /// Where changed pages are sent, from `[seo.indexnow]`. `None`, the
    /// default, sends nothing.
    pub indexnow: Option<IndexNowSettings>,
    /// Title and description templates, from `[seo.titles]`.
    pub titles: TitleSettings,
    /// The Twitter account and default share image, from `[seo.social]`.
    pub social: SocialSettings,
    /// Verification codes and files, from `[seo.verification]`.
    pub verification: VerificationSettings,
    /// Which crawlers may read which paths, from `[seo.crawlers]`.
    pub crawlers: CrawlerSettings,
    /// What every feed carries, from `[seo.feeds]`.
    pub feeds: FeedSettings,
    /// What links to other sites get, from `[seo.links]`.
    pub links: LinkSettings,
    /// Who publishes the site, from `[seo.publisher]`.
    pub publisher: PublisherSettings,
    /// The site's shops, offices or clinics, from `[[seo.locations]]`.
    pub locations: Vec<LocationSettings>,
    /// Schema nodes per category or collection, from `[[seo.schema]]`.
    pub schema: Vec<SchemaTemplateSettings>,
    /// The AI info page, from `[seo.ai_page]` (P3). On by default.
    pub ai_page: AiPageSettings,
    /// How the breadcrumb trail reads, from `[seo.breadcrumbs]`.
    pub breadcrumbs: BreadcrumbSettings,
}

/// `IndexNow`, switched on by its table in `site.toml`.
///
/// `IndexNow` tells Bing, Yandex, Seznam, Naver and Yep about a changed page at
/// once, instead of waiting for their next crawl. The key is required; the
/// endpoint defaults to the shared one, which passes each request on to every
/// search engine that takes part.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IndexNowSettings {
    /// The key the site serves in its key file.
    pub key: IndexNowKey,
    /// Where the changed addresses are sent. `None` for
    /// [`IndexNowSettings::SHARED_ENDPOINT`].
    #[serde(default)]
    pub endpoint: Option<AbsoluteUrl>,
}

impl IndexNowSettings {
    /// The shared `IndexNow` endpoint, used when `endpoint` is left out.
    pub const SHARED_ENDPOINT: &str = "https://api.indexnow.org/indexnow";

    /// `IndexNow` with `key`, sending to the shared endpoint.
    pub fn new(key: IndexNowKey) -> Self {
        Self {
            key,
            endpoint: None,
        }
    }
}

/// Whether search engines may list each kind of archive page: `"index"` or
/// `"noindex"`.
///
/// By default tag, category and author archives are indexed, and date
/// archives and search results are not. A tag or author archive with fewer
/// posts than `thin_archives_below` is never indexed, because a page with
/// one or two links looks thin to a search engine. `site = "noindex"` keeps
/// the whole site out, as Khalid's demo blogs are.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct ArchiveIndex {
    /// Tag archives, such as `/tag/crawl-budget/`.
    pub tags: Indexing,
    /// Category archives, such as `/category/technical-seo/`.
    pub categories: Indexing,
    /// Author archives, such as `/authors/asif/`. `"off"` builds none and
    /// sends their addresses to the home page, as a one-author blog wants.
    pub authors: ArchiveSetting,
    /// Date archives, such as `/2026/10/`. `"off"` builds none and sends
    /// their addresses to the home page.
    pub dates: ArchiveSetting,
    /// Search results pages.
    pub search: Indexing,
    /// A tag or author archive listing fewer posts than this is `noindex`.
    /// `0` turns the rule off.
    pub thin_archives_below: u32,
    /// The whole site. `"noindex"` keeps every page out of search results
    /// and sitemaps, whatever the other settings and a post's own `robots`
    /// say, for a sample or staging copy. Links are still followed.
    pub site: Indexing,
    /// Every post, Yoast's "show posts in search results". A post's own
    /// `robots` still wins.
    pub posts: Indexing,
    /// Every standalone page, such as "About".
    pub pages: Indexing,
    /// The posts of one collection, by its name, such as `news`, over
    /// `posts`.
    pub collections: BTreeMap<String, Indexing>,
}

impl Default for ArchiveIndex {
    fn default() -> Self {
        Self {
            tags: Indexing::Index,
            categories: Indexing::Index,
            authors: ArchiveSetting::Index,
            dates: ArchiveSetting::NoIndex,
            search: Indexing::NoIndex,
            thin_archives_below: 3,
            site: Indexing::Index,
            posts: Indexing::Index,
            pages: Indexing::Index,
            collections: BTreeMap::new(),
        }
    }
}

/// Whether an archive is built, and if it is, whether search engines may
/// list it: `"index"`, `"noindex"` or `"off"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArchiveSetting {
    /// Built, and search engines may list it.
    Index,
    /// Built, but kept out of search results.
    NoIndex,
    /// Not built: each of its addresses sends readers to the home page.
    Off,
}

impl ArchiveSetting {
    /// Whether search engines may list the archive. One that is off has no
    /// page to list.
    pub fn indexing(self) -> Indexing {
        match self {
            Self::Index => Indexing::Index,
            Self::NoIndex | Self::Off => Indexing::NoIndex,
        }
    }
}
