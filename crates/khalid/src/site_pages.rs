//! The pages `khalid serve` answers with: the ones `khalid build` writes,
//! rendered into memory instead of files, with the site's files and moved
//! addresses. A headless site has the files and moves and no HTML page.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError, RwLock};

use khalid_content::{ContentTree, Document, PostFront, SiteContent, check_bodies, read_content};
use khalid_core::{Clock, Config, Locale, UserMessage};
use khalid_render::{Renderer, Tokens};
use khalid_routes::Routes;
use khalid_server::{Page, Render, Rendered};

use crate::assets::PageStylesheets;
use crate::comment_pages::{self, ShownComments};
use crate::content_feed::ContentFeed;
use crate::form_pages::LiveForms;
use crate::listed_post::ListedPost;
use crate::post_files::Assets;
use crate::post_photos::Shown;
use crate::site_content::Catalog;
use crate::site_html::{Made, SearchShell, Served, Unrendered, Which, html_pages};
use crate::unlock::Gate;
use crate::{
    assets, build_gate, build_home, build_message, build_seo, categories, content_message,
    home_images, look, own_descriptions, post_categories, post_lock, search_page, share_images,
    site_outline,
};

/// Renders every page of one site from its files. Cloning shares it.
///
/// The settings, the routes and the theme, with the stylesheet built from
/// it, are read when `khalid serve` starts, and again before a full render
/// when one of their files changed (see `site_reload.rs`). The content is
/// read again on each render, so an edited post shows.
#[derive(Debug, Clone)]
pub(crate) struct SitePages(Arc<Site>);

#[derive(Debug)]
struct Site {
    folder: PathBuf,
    /// What the pages are made with now. Each render takes one copy, so a
    /// reload never mixes two sets of settings in one render.
    settings: Arc<RwLock<Arc<Settings>>>,
    /// How to read the settings again, set only by `khalid serve`.
    reload: OnceLock<reload::Reload>,
    feed: Option<ContentFeed>,
    /// What the last full render read of the site, so a saved post's page
    /// can be rendered alone.
    outline: Mutex<Outline>,
    /// The files beside folder posts as the last full render found them, so
    /// a saved post's page finds them without listing the folder again.
    /// `None` before the first full render.
    assets: Mutex<Option<Assets>>,
    /// How many times a full render has read the folder. Content passed to
    /// [`SitePages::render_content`] was read at start, as read 0.
    reads: AtomicU64,
    /// The newest read whose render handed on its outline, assets and
    /// catalog. A render of older content that finishes later, such as the
    /// start render after a save, hands on nothing, so it cannot undo the
    /// save or fire its entries' events again.
    handed_on: Mutex<u64>,
    /// The search page of each language as the last full render made it,
    /// which a search's results are rendered into.
    searches: Mutex<Vec<SearchShell>>,
    /// The comments post pages show, or `None` when the site takes none.
    comments: Option<ShownComments>,
    /// The page of each form, or `None` when the site takes none.
    forms: Option<LiveForms>,
    /// The password posts of the last full render, for the password form.
    gate: Gate,
    /// Dates a feed with no posts yet, and a draft's preview.
    clock: Arc<dyn Clock>,
}

/// What a saved post's page needs from the rest of the site, in a fraction
/// of the memory the whole content takes.
#[derive(Debug, Default)]
struct Outline {
    /// The posts without their bodies, and the authors whole, since a
    /// post's author box shows their bios.
    content: SiteContent,
    /// Every dated post, which related posts are chosen from.
    listed: Vec<ListedPost>,
    /// The lead images and author photos the full render drew, which cards
    /// and bylines on a saved post's page show.
    shown: Shown,
}

/// The settings, the routes and the theme the pages are made with.
#[derive(Debug)]
struct Settings {
    config: Config,
    routes: Routes,
    /// `None` on a headless site, whose theme is never read.
    theme: Option<Theme>,
}

/// The theme's renderer, and the stylesheet and reader script every page links.
#[derive(Debug)]
struct Theme {
    renderer: Renderer,
    stylesheet: Page,
    /// Each page's own stylesheet, made from the whole one.
    sheets: PageStylesheets,
    script: Page,
    /// The self-hosted font files the stylesheet names, read at start.
    fonts: Vec<Page>,
    /// The tokens the stylesheet was built from, for the share cards.
    tokens: Tokens,
}

impl Theme {
    /// The theme of the site in `folder` with `config`, read from its
    /// files, or `None` on a headless site.
    fn read(folder: &Path, config: &Config) -> Result<Option<Self>, UserMessage> {
        if !config.mode().has_pages() {
            return Ok(None);
        }
        let look = look::of(folder, config)?;
        let tokens = look::tokens(folder, &look)?;
        let renderer = Renderer::with_pack(folder, &look::render_pack(&look, config))
            .map_err(|error| build_message::render(&error))?
            .with_brand(&tokens.brand);
        let own = assets::own_rules(folder)?;
        let rules = (look.pack().stylesheet(), own.as_str());
        let (stylesheet, sheets) = assets::stylesheets(config, &tokens, rules)?;
        Ok(Some(Self {
            renderer,
            stylesheet,
            sheets,
            script: assets::script_page(config)?,
            fonts: assets::font_pages(config, assets::fonts(folder, &tokens))?,
            tokens,
        }))
    }
}

impl SitePages {
    /// The pages of the site in `folder`, with its checked `config` and
    /// `routes`. With a `feed`, each render also hands the Content API what
    /// it read. `clock` dates the feeds and a draft's preview.
    pub(crate) fn new(
        folder: &Path,
        config: Config,
        routes: Routes,
        feed: Option<ContentFeed>,
        clock: Arc<dyn Clock>,
    ) -> Result<Self, UserMessage> {
        let theme = Theme::read(folder, &config)?;
        let comments =
            (theme.is_some() && comment_pages::taken(config.mode())).then(ShownComments::default);
        let forms = comments.is_some().then(LiveForms::default);
        let settings = Settings {
            config,
            routes,
            theme,
        };
        Ok(Self(Arc::new(Site {
            folder: folder.to_path_buf(),
            settings: Arc::new(RwLock::new(Arc::new(settings))),
            reload: OnceLock::new(),
            feed,
            outline: Mutex::default(),
            assets: Mutex::default(),
            reads: AtomicU64::default(),
            handed_on: Mutex::default(),
            searches: Mutex::default(),
            gate: Gate::default(),
            comments,
            forms,
            clock,
        })))
    }

    /// The settings, the routes and the theme the pages are made with now.
    fn settings(&self) -> Arc<Settings> {
        let settings = self.0.settings.read();
        Arc::clone(&settings.unwrap_or_else(PoisonError::into_inner))
    }

    /// The text `key` of the strings file for `locale`, as the theme's
    /// templates read it. `None` on a headless site.
    pub(crate) fn text(&self, locale: &Locale, key: &str) -> Option<String> {
        let settings = self.settings();
        let theme = settings.theme.as_ref()?;
        theme.renderer.text(locale, key).map(str::to_owned)
    }

    /// The comments the post pages show, or `None` when the site takes none.
    pub(crate) fn comments(&self) -> Option<ShownComments> {
        self.0.comments.clone()
    }

    /// The page of each form, or `None` when the site takes none.
    pub(crate) fn forms(&self) -> Option<LiveForms> {
        self.0.forms.clone()
    }

    /// The address of the stylesheet every page links; `None` when headless.
    pub(crate) fn stylesheet(&self) -> Option<String> {
        let settings = self.settings();
        (settings.theme.as_ref()).map(|theme| theme.stylesheet.path().as_str().to_owned())
    }

    /// Every page of the site at its address, the stylesheet they link, the
    /// site's files, the not-found page of the default language and the
    /// moved addresses, read from the folder. Checks every body again,
    /// as `khalid --site` does at start, so a saved edit with a broken
    /// wikilink is refused with its file and line, and the pages already
    /// served stay. Reads files and renders,
    /// so it runs off the async runtime (RULES.md BL-RUN-7).
    pub(crate) fn render_all(&self) -> Result<Served, UserMessage> {
        self.reload_if_changed();
        let (folder, settings) = (&self.0.folder, self.settings());
        let Settings { config, routes, .. } = &*settings;
        let read = self
            .0
            .reads
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);
        let (content, assets) = ContentTree::walk(folder)
            .and_then(|tree| Ok((read_content(&tree)?, Assets::of(&tree))))
            .and_then(|read| check_bodies(&read.0, config, routes).map(|()| read))
            .map_err(|error| content_message::of(folder, &error))?;
        self.render_read(&settings, &content, (assets, read))
    }

    /// The first pages `khalid serve` answers with while the rest of the
    /// site renders: the home page of each language, made from the front
    /// matter of `content` without rendering a single body, and the
    /// stylesheet they link, byte for byte the full render's. None for a
    /// headless site, nor with home sections: list pages fill those (G19).
    pub(crate) fn render_first(&self, content: &SiteContent) -> Result<Vec<Page>, UserMessage> {
        let (folder, settings) = (&self.0.folder, self.settings());
        let Settings {
            config,
            routes,
            theme,
        } = &*settings;
        let sections = !config.reader().home_sections.is_empty();
        let Some(theme) = theme.as_ref().filter(|_| !sections) else {
            return Ok(Vec::new());
        };
        let href = theme.stylesheet.path().as_str();
        let mut languages = build_home::home_pages(config, routes, content, href)?;
        let categories = categories::of(folder)?;
        post_categories::fill((config, routes), &categories, &mut languages);
        home_images::attach(folder, (config, routes), content, &mut languages)?;
        search_page::link(config, &mut languages);
        own_descriptions::fill(config, &theme.renderer, &mut languages)?;
        let seo_site = build_seo::site_on(config, self.0.clock.now().date_naive())?;
        let cards = share_images::attach(folder, config, &theme.tokens, &seo_site, &mut languages)?;
        // The home pages answer first, so their cards are drawn now; the
        // rest warm in the background once the pages are live (G14).
        share_images::draw_now(folder, cards)?;
        build_seo::heads(&seo_site, &mut languages)?;
        let mut pages = vec![theme.stylesheet.clone(), theme.script.clone()];
        pages.extend(theme.fonts.iter().cloned());
        let theme_parts = (&theme.renderer, &theme.sheets);
        html_pages(theme_parts, config, &languages, Which::All, &mut pages)?;
        Ok(pages)
    }

    /// Keeps what a full render's `content`, once the publish gate has
    /// left out the posts it refuses, and `assets` say about the site, so a
    /// post saved before the first full render ends shows at once: on a
    /// 10,000-post site that render takes most of a minute. Until it ends,
    /// a saved post's page has no related posts and no author photo, which
    /// only a full render draws; the full render that follows each save
    /// adds them. Once a render has kept an outline, this keeps nothing.
    fn outline_start(&self, content: &SiteContent, assets: &Assets) {
        let mut kept = self.0.assets.lock().unwrap_or_else(PoisonError::into_inner);
        if kept.is_some() {
            return;
        }
        *kept = Some(assets.clone());
        let mut outline = self
            .0
            .outline
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        outline.content = site_outline::outline_of(content);
    }

    /// The password posts the password form checks, as the last full
    /// render found them.
    pub(crate) fn gate(&self) -> Gate {
        self.0.gate.clone()
    }

    /// Every page made from `content`, already read from the site folder,
    /// with the files `assets` lists beside folder posts, the site's files,
    /// the not-found page and the moved addresses, then hands the feed its
    /// catalog when there is one. At start, `khalid serve` passes what it
    /// checked, so the folder is read once, not twice. `content` must have
    /// passed `check_bodies`. Only a full render hands the feed its
    /// catalog, so the Content API never sees part of the site, and only
    /// when no render of content read later has handed on its own.
    pub(crate) fn render_content(
        &self,
        content: &SiteContent,
        assets: Assets,
    ) -> Result<Served, UserMessage> {
        self.render_read(&self.settings(), content, (assets, 0))
    }

    /// Renders `content`, which the folder's `read` gave, with `settings`,
    /// as [`SitePages::render_content`] says.
    fn render_read(
        &self,
        settings: &Settings,
        content: &SiteContent,
        (assets, read): (Assets, u64),
    ) -> Result<Served, UserMessage> {
        let Settings { config, routes, .. } = settings;
        let Site {
            folder,
            feed,
            outline,
            assets: kept,
            handed_on,
            clock,
            ..
        } = &*self.0;
        // Readers never see a post `khalid build` leaves out (KH-LNT-1).
        let checked = (config, routes, content);
        let content = &*build_gate::for_readers(folder, checked, clock.as_ref())?;
        self.outline_start(content, &assets);
        // The catalog holds no post body, so it is quick to make here, and
        // the Content API's store is built from it in the background.
        let catalog = match feed {
            Some(_) => Some(Catalog::of(folder, config, content)?),
            None => None,
        };
        let mut shown = Shown::default();
        let (mut rendered, made) =
            self.render_of(settings, &assets, (content, &[]), (Which::All, &mut shown))?;
        let mut handed_on = handed_on.lock().unwrap_or_else(PoisonError::into_inner);
        if read < *handed_on {
            tracing::debug!(read, "a render of newer content already handed on its own");
            return Ok(rendered);
        }
        *handed_on = read;
        let Made {
            listed,
            gated,
            searches,
        } = made;
        self.0.gate.replace(gated);
        *self
            .0
            .searches
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = searches;
        *outline.lock().unwrap_or_else(PoisonError::into_inner) = Outline {
            content: site_outline::outline_of(content),
            listed,
            shown,
        };
        *kept.lock().unwrap_or_else(PoisonError::into_inner) = Some(assets);
        if let (Some(feed), Some(mut catalog)) = (feed, catalog) {
            catalog.heads = std::mem::take(&mut rendered.heads);
            feed.send(catalog);
        }
        Ok(rendered)
    }
}

impl Render for SitePages {
    type Error = Unrendered;

    fn render(&self) -> Result<Rendered, Unrendered> {
        let served = (self.render_all())
            .inspect_err(reload::keeps_last_good)
            .map_err(Unrendered)?;
        Ok(served.rendered())
    }

    /// A saved post's page alone: its own file read again, with the authors
    /// and the files beside it as the last full render read them. Only
    /// when nothing else can reach the page (see `saved_posts`), so it is
    /// the page the full render will make, but for what a save before the
    /// first full render ends leaves out (see `outline_start`).
    fn render_saved(&self, files: &[PathBuf]) -> Result<Option<Vec<Page>>, Unrendered> {
        let kept = self
            .0
            .assets
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        let (Some(assets), Some((saved, others))) = (kept, self.saved_posts(files)) else {
            return Ok(None);
        };
        // A post that is or was locked or gated changes the posts the
        // gate checks, which only the full render keeps.
        let locked = |doc: &Document<PostFront>| {
            let gated = doc.body.contains(":::gated");
            post_lock::Lock::of(&doc.front).is_some() || gated || self.0.gate.guards(&doc.path)
        };
        if saved.posts.iter().any(locked) {
            return Ok(None);
        }
        let outline = self.0.outline.lock();
        let mut shown = outline
            .unwrap_or_else(PoisonError::into_inner)
            .shown
            .clone();
        let (settings, saved) = (self.settings(), (&saved, others.as_slice()));
        let (served, _) = (self.render_of(&settings, &assets, saved, (Which::Posts, &mut shown)))
            .map_err(Unrendered)?;
        Ok(Some(served.pages))
    }
}

#[cfg(test)]
#[path = "site_pages_tests.rs"]
mod tests;

#[path = "site_pages_search.rs"]
mod search;

#[path = "site_pages_saved.rs"]
mod saved;

#[path = "site_pages_preview.rs"]
mod preview;

#[path = "site_pages_render.rs"]
mod render;

#[path = "site_reload.rs"]
pub(crate) mod reload;
