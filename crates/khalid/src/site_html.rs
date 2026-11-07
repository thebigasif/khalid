//! The HTML of the pages `khalid serve` renders into memory, the search
//! page each language keeps without results, and why a render failed.

use std::error::Error;
use std::fmt;

use khalid_core::{Config, UserMessage};
use khalid_render::{PageBody, PageMeta, RenderSite, Renderer};
use khalid_server::Page;

use crate::assets::PageStylesheets;
use crate::build_pages::LanguagePages;
use crate::page_code::PageCode;
use crate::post_lock;
use crate::site_page::{HeadNonce, served_at};

/// Why the site could not be rendered after an edit, as a person reads it.
#[derive(Debug)]
pub(crate) struct Unrendered(pub(crate) UserMessage);

impl fmt::Display for Unrendered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for Unrendered {}

/// Which pages a render makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Which {
    /// Every page, the stylesheet and the not-found page.
    All,
    /// Only the page of each dated post.
    Posts,
}

/// A language's search page without results: the site as its pages show it
/// and the page's head, with its title and its SEO tags.
#[derive(Debug, Clone)]
pub(crate) struct SearchShell {
    pub(crate) site: RenderSite,
    pub(crate) meta: PageMeta,
}

/// The search page of each of `languages`.
pub(crate) fn search_shells(languages: &[LanguagePages]) -> Vec<SearchShell> {
    let mut shells = Vec::new();
    for language in languages {
        let searches = language
            .pages
            .iter()
            .filter(|page| matches!(page.body, PageBody::Search(_)));
        shells.extend(searches.map(|page| SearchShell {
            site: language.site.clone(),
            meta: page.meta.clone(),
        }));
    }
    shells
}

/// Renders the `which` pages of `languages` with the theme's `renderer`
/// into `pages`, each linking its own stylesheet from `sheets`, which go
/// into `pages` too, and returns the not-found page of the default
/// language, empty unless `which` is [`Which::All`].
pub(crate) fn html_pages(
    (renderer, sheets): (&Renderer, &PageStylesheets),
    config: &Config,
    languages: &[LanguagePages],
    which: Which,
    pages: &mut Vec<Page>,
) -> Result<String, UserMessage> {
    let mut not_found = String::new();
    let mut linked = Linked::new(sheets);
    let code = PageCode::new(config, HeadNonce::PerRequest);
    for language in languages {
        for page in &language.pages {
            if which == Which::Posts && !matches!(page.body, PageBody::Post(_)) {
                continue;
            }
            let html = page.rendered(renderer, &language.site, &code)?;
            if matches!(page.body, PageBody::NotFound) {
                if &language.site.locale == config.default_language() {
                    not_found = linked.fit(html, pages)?.0;
                }
                continue;
            }
            let (html, preload) = linked.fit(html, pages)?;
            pages.push(with_preload(served_at(&page.meta.path, html)?, preload));
        }
        for unlocked in &language.unlocked {
            let page = &unlocked.page;
            let html = page.rendered(renderer, &language.site, &code)?;
            let (html, preload) = linked.fit(html, pages)?;
            let path = post_lock::hidden(&page.meta.path);
            pages.push(with_preload(served_at(&path, html)?, preload));
        }
    }
    Ok(not_found)
}

/// `page`, whose answers say to preload its own stylesheet at `href`.
fn with_preload(page: Page, href: Option<String>) -> Page {
    match href {
        Some(href) => page.with_preload(&href),
        None => page,
    }
}

/// The pages' own stylesheets one render has added so far, each once.
struct Linked<'s> {
    sheets: &'s PageStylesheets,
    added: Vec<String>,
}

impl<'s> Linked<'s> {
    fn new(sheets: &'s PageStylesheets) -> Self {
        Self {
            sheets,
            added: Vec::new(),
        }
    }

    /// `html` linking its own stylesheet, and that stylesheet's address.
    /// The stylesheet goes into `pages` the first time a page links it.
    fn fit(
        &mut self,
        html: String,
        pages: &mut Vec<Page>,
    ) -> Result<(String, Option<String>), UserMessage> {
        let (html, sheet) = self.sheets.fit(html);
        let Some(sheet) = sheet else {
            return Ok((html, None));
        };
        if !self.added.iter().any(|name| name == sheet.name()) {
            self.added.push(sheet.name().to_owned());
            pages.push(self.sheets.page(&sheet)?);
        }
        Ok((html, Some(self.sheets.href(&sheet))))
    }
}
