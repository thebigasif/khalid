//! Schema aggregation: the whole site's JSON-LD in one graph per language,
//! at `schema.json` beside each language's home page, so an AI agent or a
//! search engine reads every page's structured data in one request. Each
//! language's home page links its file. robots.txt names the files on a
//! `Schemamap:` line, as Yoast does, only when the site asks for it.

use std::collections::BTreeSet;

use khalid_core::{AbsoluteUrl, Locale};

use crate::sitemap::is_listed;
use crate::{SchemaGraph, SeoError, SeoPage, SeoSite, UrlPath, page_graph, site_graph};

/// Where each language's file lives, inside its language folder.
const SCHEMA_MAP_PATH: &str = "/schema.json";

/// Whether each language's whole-site graph is written, and where it is named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SchemaMap {
    /// No `schema.json` files.
    Off,
    /// The files, each linked from its language's home page. The default.
    Files,
    /// The files, linked from the home pages and also named in robots.txt on
    /// a `Schemamap:` line, as Yoast does. Lighthouse marks such a
    /// robots.txt invalid, because `Schemamap` is not a directive it knows.
    FilesAndRobotsTxt,
}

impl SchemaMap {
    /// Whether the `schema.json` files are written.
    pub fn writes_files(self) -> bool {
        self != Self::Off
    }
}

/// The address of the schema file of `language`.
///
/// # Errors
///
/// [`SeoError::Url`] when the address cannot be built.
pub fn schema_map_url(site: &SeoSite, language: &Locale) -> Result<AbsoluteUrl, SeoError> {
    site.url_for(language, &UrlPath::try_new(SCHEMA_MAP_PATH)?)
}

/// The graph of every page in `language` that search engines may list: the
/// site's own nodes first, then each page's nodes in the order given, each
/// node with an `@id` written once.
///
/// # Errors
///
/// [`SeoError::Url`] when an address cannot be built.
pub fn site_schema(
    site: &SeoSite,
    pages: &[SeoPage],
    language: &Locale,
) -> Result<SchemaGraph, SeoError> {
    let mut graph = site_graph(site)?;
    let mut seen: BTreeSet<AbsoluteUrl> = (graph.nodes().iter())
        .filter_map(|node| node.id().cloned())
        .collect();
    let listed = pages
        .iter()
        .filter(|page| &page.language == language && is_listed(site, page));
    for page in listed {
        for node in page_graph(site, page)?.nodes() {
            let new = node.id().is_none_or(|id| seen.insert(id.clone()));
            if new {
                graph.push(node.clone());
            }
        }
    }
    Ok(graph)
}
