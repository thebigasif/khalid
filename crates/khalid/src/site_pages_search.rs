//! A live search's page, rendered into the search page the last full
//! render of [`SitePages`] made.

use std::sync::PoisonError;

use khalid_core::UserMessage;
use khalid_render::{PageBody, RenderSearch, SearchMode, SearchResult};

use super::SitePages;
use crate::build_message;
use crate::site_html::SearchShell;

impl SitePages {
    /// The page of a search for `query` that found `results`, rendered into
    /// the search page at `path` as the last full render made it. `None`
    /// before the first full render, or when `path` has no search page.
    pub(crate) fn render_search(
        &self,
        path: &str,
        query: Option<String>,
        results: Vec<SearchResult>,
    ) -> Result<Option<String>, UserMessage> {
        let Some(theme) = &self.0.theme else {
            return Ok(None);
        };
        let shell = self
            .0
            .searches
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .iter()
            .find(|shell| shell.meta.path == path)
            .cloned();
        let Some(SearchShell { site, meta }) = shell else {
            return Ok(None);
        };
        let body = PageBody::Search(RenderSearch {
            title: meta.title.clone(),
            mode: SearchMode::Live,
            action: meta.path.clone(),
            query,
            results,
        });
        let html = theme.renderer.render(&site, &meta, &body);
        html.map(Some)
            .map_err(|error| build_message::render(&error))
    }
}
