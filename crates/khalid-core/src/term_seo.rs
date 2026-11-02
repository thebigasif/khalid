//! A category's or tag's own search settings, from `categories.toml` and
//! `tags.toml` in the site folder.

use serde::Deserialize;

use crate::{ImageSettings, RobotsDirectives};

/// What one category or tag page says in one language, in place of the
/// site's defaults. Each entry of `categories.toml` or `tags.toml` holds
/// one table per language under `seo`:
///
/// ```toml
/// [[tags]]
/// slug = "crawl-budget"
///
/// [tags.seo.en]
/// title = "Crawl budget guides {sep} {site}"
/// description = "How search engines decide how much of a site to read."
/// intro = "Every guide on **crawl budget**, newest first."
/// robots = "noindex"
///
/// [tags.seo.en.image]
/// url = "https://www.example.com/media/crawl.png"
/// alt = "A spider on a sitemap"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct TermSeo {
    /// The title template, with the same variables as `[seo.titles]`.
    pub title: Option<String>,
    /// The meta description, also the `{term_description}` variable.
    pub description: Option<String>,
    /// The page's own robots directives, such as `noindex`.
    pub robots: Option<RobotsDirectives>,
    /// Markdown shown above the posts on the term's first page.
    pub intro: Option<String>,
    /// The image link previews show for the term's page.
    pub image: Option<ImageSettings>,
}
