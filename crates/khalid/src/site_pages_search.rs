//! A live search's page, rendered into the search page the last full
//! render of [`SitePages`] made.

use std::sync::PoisonError;

use khalid_core::UserMessage;
use khalid_render::{HeadHtml, PageBody, RenderSearch, SearchMode, SearchResult};

use super::SitePages;
use crate::build_message;
use crate::page_code::PageCode;
use crate::site_html::SearchShell;
use crate::site_page::HeadNonce;

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
        let settings = self.settings();
        let Some(theme) = &settings.theme else {
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
        // The site's code and the banner, as every page carries them, so
        // the search route fills them for the reader as the router does.
        let code = PageCode::new(&settings.config, HeadNonce::PerRequest);
        let head = code.head(None);
        let mut meta = meta;
        if !head.is_empty() {
            let seo = meta.seo_head.as_ref().map_or("", HeadHtml::as_str);
            // Code from `[code]` was checked by HeadCode, which writes every
            // attribute quoted and keeps only head elements.
            meta.seo_head = Some(HeadHtml::from_trusted(format!("{seo}\n{head}")));
        }
        let html = theme.renderer.render(&site, &meta, &body);
        let page = (&theme.renderer, &site.locale);
        html.map(|html| Some(code.with_body_end(html, page, &meta.path)))
            .map_err(|error| build_message::render(&error))
    }
}
