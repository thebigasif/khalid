//! The one render every page of `khalid serve` goes through, whole or for a
//! saved post alone.

use khalid_content::SiteContent;
use khalid_core::{Mode, UserMessage};
use khalid_render::Tokens;

use super::{Settings, SitePages};
use crate::listed_post::ListedPost;
use crate::newsletter_files::archive_pages;
use crate::post_files::{Assets, charts};
use crate::post_heads::{self, PostHeads};
use crate::post_photos::Shown;
use crate::site_files::HEADLESS_NOT_FOUND;
use crate::site_html::{Made, NotFound, Served, Which, html_pages, search_shells};
use crate::unlock::GatedPosts;
use crate::{
    assets, build_lists, build_pages, build_seo, build_sitemap, categories, comment_pages,
    form_pages, own_descriptions, post_categories, post_files, post_gate, post_lock, search_page,
    seo_extras, share_images, site_files, trails,
};

impl SitePages {
    /// The `which` pages made from `content` with `settings`, with the files `assets` lists
    /// beside folder posts, the site's files, the moved addresses and the
    /// not-found page of each language when `which` is [`Which::All`],
    /// and what else it [`Made`], which it keeps nowhere: the caller hands
    /// it on. Related posts are chosen among the dated posts and `others`. Only a full
    /// render serves the files; a saved post's page still points its
    /// images at them. A headless site has no HTML page, as `khalid build`
    /// writes none. Cards and bylines show the images in `shown`, which
    /// gains the ones these pages draw.
    pub(super) fn render_of(
        &self,
        settings: &Settings,
        assets: &Assets,
        (content, others): (&SiteContent, &[ListedPost]),
        (which, shown): (Which, &mut Shown),
    ) -> Result<(Served, Made), UserMessage> {
        let folder = &self.0.folder;
        let Settings {
            config,
            routes,
            theme,
        } = settings;
        let href = theme
            .as_ref()
            .map_or("", |theme| theme.stylesheet.path().as_str());
        let today = self.0.clock.now().date_naive();
        let content = &*charts::read(folder, content);
        let mut languages = build_pages::pages(config, routes, (content, others), href, today)?;
        let tokens = theme.as_ref().map(|theme| &theme.tokens);
        assets::preload_body_font(folder, config, tokens, &mut languages);
        let renderer = theme.as_ref().map(|theme| &theme.renderer);
        post_lock::lock(config, content, renderer, &mut languages);
        let categories = categories::of(folder)?;
        if let Some(theme) = theme {
            search_page::add(config, &theme.renderer, &mut languages)?;
            archive_pages::add(folder, config, (&theme.renderer, today), &mut languages)?;
            // A saved post's page alone needs no list, and its content
            // holds too few posts to make one.
            if which == Which::All {
                let sources = (content, categories.as_slice());
                build_lists::add((config, routes), &theme.renderer, sources, &mut languages)?;
                let found = (&theme.renderer, folder.as_path(), today);
                build_pages::discovery_pages((config, routes), found, content, &mut languages)?;
                build_sitemap::add(config, &theme.renderer, &mut languages)?;
            }
            own_descriptions::fill(config, &theme.renderer, &mut languages)?;
        }
        let (gone, _) = seo_extras::arrange(folder, (config, routes), &mut languages)?;
        // A headless site's heads carry the same trail as its pages would.
        trails::fill(folder, (config, routes), &mut languages)?;
        post_categories::fill((config, routes), &categories, &mut languages);
        comment_pages::fill(config, self.0.comments.as_ref(), &mut languages);
        form_pages::fill(config, self.0.forms.as_ref(), &mut languages);
        form_pages::under_mount(config, &mut languages);
        post_gate::open(config, routes, content, &mut languages)?;
        let beside = post_files::served(folder, config, assets, &mut languages, shown)?;
        let mut seo_site = build_seo::site_on(config, self.0.clock.now().date_naive())?;
        // Only a page this server shows names `/api/oembed`, which it
        // answers; `khalid build` output may sit on a host with no server.
        seo_site.oembed = config.mode() == Mode::Server;
        let default_tokens = Tokens::default();
        let tokens = theme.as_ref().map_or(&default_tokens, |t| &t.tokens);
        let cards = share_images::attach(folder, config, tokens, &seo_site, &mut languages)?;
        let seo_pages = build_seo::heads(&seo_site, &mut languages)?;
        build_sitemap::fill(config, &seo_site, &seo_pages, &mut languages)?;
        post_lock::share_heads(&mut languages);
        let mut served = Served {
            pages: Vec::new(),
            not_found: NotFound::only(HEADLESS_NOT_FOUND),
            moves: Vec::new(),
            heads: PostHeads::new(),
        };
        let mut made = Made::default();
        if which == Which::All {
            made.gated = GatedPosts::of(&languages);
            if self.0.feed.is_some() {
                served.heads = post_heads::of(&seo_site, &seo_pages)?;
            }
            served.pages = site_files::files(config, &seo_site, &seo_pages, self.0.clock.now())?;
            served.pages.extend(beside);
            served.moves = site_files::moves(config, routes, content)?;
            served.moves.extend(seo_extras::archives_off::moves(&gone)?);
        }
        if let Some(theme) = theme {
            if which == Which::All {
                (served.pages).extend([theme.stylesheet.clone(), theme.script.clone()]);
                (served.pages).extend(theme.fonts.iter().cloned());
                made.searches = search_shells(&languages);
            }
            let parts = (&theme.renderer, &theme.sheets);
            served.not_found = html_pages(parts, config, &languages, which, &mut served.pages)?;
        }
        // The cards draw behind the answers: their addresses are already
        // baked into the heads, so the pages are complete without the
        // files, and the cache warms while the server answers (G14, ADR
        // "khalid serve answers before the full render").
        if !cards.is_empty() {
            let site = folder.clone();
            std::thread::spawn(move || share_images::warm(&site, cards));
        }
        let listed = languages.into_iter().flat_map(|language| language.listed);
        made.listed = listed.collect();
        Ok((served, made))
    }
}
