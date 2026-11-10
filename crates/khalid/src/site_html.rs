//! The HTML of the pages `khalid serve` renders into memory, the
//! not-found page of each language, the search page each language keeps
//! without results, and why a render failed.

use std::error::Error;
use std::fmt;

use khalid_core::{Config, UserMessage};
use khalid_render::{PageBody, PageMeta, RenderSite, Renderer};
use khalid_server::{Moved, Page, PagePath, Rendered};

use crate::assets::PageStylesheets;
use crate::build_pages::{self, LanguagePages};
use crate::listed_post::ListedPost;
use crate::page_code::PageCode;
use crate::post_heads::PostHeads;
use crate::site_page::{HeadNonce, served_at};
use crate::unlock::GatedPosts;
use crate::{build_message, post_lock};

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

/// One render of the site: what [`Rendered`] holds, kept apart so that
/// `khalid serve` can answer before compressing.
#[derive(Debug)]
pub(crate) struct Served {
    pub(crate) pages: Vec<Page>,
    pub(crate) not_found: NotFound,
    pub(crate) moves: Vec<Moved>,
    /// The head of each post page, for the Content API: empty without one.
    pub(crate) heads: PostHeads,
}

impl Served {
    /// The pages, the not-found pages and the moves, as the server holds them.
    pub(crate) fn rendered(self) -> Rendered {
        let NotFound { default, under } = self.not_found;
        let rendered = Rendered::new(self.pages, default).with_not_found_under(under);
        rendered.with_moves(self.moves)
    }
}

/// What a render made beside its pages, which only the newest full
/// render hands on, so a render of older content changes nothing.
#[derive(Debug, Default)]
pub(crate) struct Made {
    /// The dated posts as lists show them.
    pub(crate) listed: Vec<ListedPost>,
    /// The password posts, which the password form checks.
    pub(crate) gated: GatedPosts,
    /// The search page of each language, empty unless every page was made.
    pub(crate) searches: Vec<SearchShell>,
}

/// The page `khalid serve` answers an address with nothing behind it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct NotFound {
    /// The default language's, for an address under no other language.
    pub(crate) default: String,
    /// Each other language's, under its own address, such as `/blog/de/`.
    pub(crate) under: Vec<(PagePath, String)>,
}

impl NotFound {
    /// Only `default`, as a headless site answers every miss.
    pub(crate) fn only(default: &str) -> Self {
        Self {
            default: default.to_owned(),
            under: Vec::new(),
        }
    }
}

/// Renders the `which` pages of `languages` with the theme's `renderer`
/// into `pages`, each linking its own stylesheet from `sheets`, which go
/// into `pages` too with every other one made so far, and returns the
/// not-found page of each language, empty unless `which` is [`Which::All`].
pub(crate) fn html_pages(
    (renderer, sheets): (&Renderer, &PageStylesheets),
    config: &Config,
    languages: &[LanguagePages],
    which: Which,
    pages: &mut Vec<Page>,
) -> Result<NotFound, UserMessage> {
    let mut not_found = NotFound::default();
    let code = PageCode::new(config, HeadNonce::PerRequest);
    for language in languages {
        for page in &language.pages {
            if which == Which::Posts && !matches!(page.body, PageBody::Post(_)) {
                continue;
            }
            let html = page.rendered(renderer, &language.site, &code)?;
            if matches!(page.body, PageBody::NotFound) {
                let html = sheets.fit(html).0;
                let locale = &language.site.locale;
                if locale == config.default_language() {
                    not_found.default = html;
                } else {
                    let under = build_pages::path_of(config, locale, "");
                    let at =
                        PagePath::try_new(&under).map_err(|_| build_message::address(&under))?;
                    not_found.under.push((at, html));
                }
                continue;
            }
            let (html, preload) = sheets.fit(html);
            pages.push(with_preload(served_at(&page.meta.path, html)?, preload));
        }
        for unlocked in &language.unlocked {
            let page = &unlocked.page;
            let html = page.rendered(renderer, &language.site, &code)?;
            let (html, preload) = sheets.fit(html);
            let path = post_lock::hidden(&page.meta.path);
            pages.push(with_preload(served_at(&path, html)?, preload));
        }
    }
    for sheet in sheets.made() {
        pages.push(sheets.page(&sheet)?);
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
