//! The HTML sitemap: a page for readers that links every listed page of one
//! language, grouped by section.

use serde::Serialize;

use crate::Link;

/// The HTML sitemap of one language, at `/sitemap/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderSitemap {
    /// The heading, shown as the page's one `h1`, such as `Sitemap`.
    pub title: String,
    /// The sections that have links, in the order they are shown.
    pub groups: Vec<SitemapGroup>,
}

/// The links of one section of the HTML sitemap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SitemapGroup {
    /// Which section, which picks the heading from the strings file.
    pub section: SitemapSection,
    /// The pages, in the order they are shown.
    pub links: Vec<Link>,
}

/// A section of the HTML sitemap, in the order the page shows them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SitemapSection {
    /// Posts.
    Posts,
    /// The home page, plain pages and glossary terms.
    Pages,
    /// Categories, apart from tags, so a category and a tag with the same
    /// name are told apart.
    Categories,
    /// Tags.
    Tags,
    /// Authors, collections and other lists of posts.
    Archives,
}
