//! Schema aggregation: the whole site's JSON-LD in one graph per language,
//! at `schema.json` beside each language's home page, so an AI agent or a
//! search engine reads every page's structured data in one request. robots.txt
//! names each file on a `Schemamap:` line, as Yoast does.

use std::collections::BTreeSet;

use khalid_core::{AbsoluteUrl, Locale};

use crate::sitemap::is_listed;
use crate::{SchemaGraph, SeoError, SeoPage, SeoSite, UrlPath, page_graph, site_graph};

/// Where each language's file lives, inside its language folder.
const SCHEMA_MAP_PATH: &str = "/schema.json";

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
