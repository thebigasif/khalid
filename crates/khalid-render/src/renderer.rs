//! The sandboxed template engine that turns a page into HTML.

use std::path::Path;

use khalid_core::{Locale, TemplateName};
use minijinja::value::Serde;
use minijinja::{AutoEscape, Environment, UndefinedBehavior, context};

use crate::dates;
use crate::direction::Direction;
use crate::heading_links;
use crate::pack::{PackContext, RenderPack};
use crate::strings::Strings;
use crate::templates::{self, Source};
use crate::{
    Brand, ConsentBanner, ConsentPart, PageBody, PageMeta, PrivacyFacts, RenderError, RenderSite,
};
use crate::{cta, enlarge, escape, image, share};

/// How many steps one page may take before rendering stops. The default
/// pages use a few thousand; the cap stops a theme template that loops
/// without end from holding a worker (RULES.md BL-SEC-10).
const FUEL_PER_PAGE: u64 = 1_000_000;

/// How many steps a page may take for each post card it lists, once its
/// cards need more than [`FUEL_PER_PAGE`]: then it may take both. A card
/// takes a few hundred; the front page lists every post, so on a blog of
/// 10,000 posts the cap per page alone would stop it.
const FUEL_PER_CARD: u64 = 2_000;

/// Renders pages with the default templates and a theme's replacements.
///
/// The environment is sandboxed (RULES.md BL-SEC-10): it has no loader, so
/// templates can include only each other, and no function that reads files,
/// the network or the clock. Every value is escaped for HTML unless the
/// crate marked it safe, such as a post body that was sanitised before it
/// arrived. A missing value is an error, not an empty string.
#[derive(Debug)]
pub struct Renderer {
    env: Environment<'static>,
    strings: Strings,
    pack: PackContext,
    brand: BrandContext,
}

/// What templates read as `brand`: the logo and the icon as addresses.
#[derive(Debug, Default, serde::Serialize)]
struct BrandContext {
    logo: Option<String>,
    icon: Option<String>,
}

impl Renderer {
    /// A renderer with the default templates only.
    ///
    /// # Errors
    ///
    /// Returns a [`RenderError`] if a built-in template or strings file is
    /// broken, which this crate's tests rule out.
    pub fn new() -> Result<Self, RenderError> {
        Self::build(Vec::new(), &RenderPack::classic(), Strings::load()?)
    }

    /// A renderer where the template pack `pack` replaces the default
    /// templates it names, and each file in `site_root/theme/templates/`
    /// replaces either. A pack may also add post templates such as
    /// `post-reference.html`, which a post asks for with `template:`, and
    /// its own parts, such as `parts-locale.html`, which its other files
    /// import. A site cannot add or replace a part.
    /// Templates read the pack's name and options as `template`.
    /// Each file in `site_root/theme/strings/`, such as `fr.toml`, lays the
    /// site's own words over that language's built-in ones.
    ///
    /// # Errors
    ///
    /// The errors of [`Renderer::with_theme`];
    /// [`RenderError::SiteStrings`], [`RenderError::UnknownStringsKey`],
    /// [`RenderError::StringsFileName`] or [`RenderError::ReadStrings`] for
    /// a site strings file that is broken, names a key no template reads,
    /// is not named for a language or cannot be read; and
    /// [`RenderError::UnknownTemplate`] for a pack file that is neither a
    /// default template, a post template nor a part.
    pub fn with_pack(site_root: &Path, pack: &RenderPack<'_>) -> Result<Self, RenderError> {
        let mut sources = Vec::new();
        for &(name, text) in &pack.templates {
            let known = templates::is_default(name) || templates::is_post_template(name);
            if !known && !templates::is_pack_part(name) {
                return Err(RenderError::UnknownTemplate {
                    name: name.to_owned(),
                });
            }
            sources.push(Source {
                name: name.to_owned(),
                text: text.to_owned(),
            });
        }
        sources.extend(templates::overrides(site_root)?);
        Self::build(sources, pack, Strings::with_site(site_root)?)
    }

    /// A renderer where each file in `site_root/theme/templates/` replaces
    /// the default template of the same name.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::UnknownTemplate`] for a file that replaces
    /// nothing, [`RenderError::Compile`] for a template with a syntax
    /// mistake, and the read errors of [`RenderError`] when a file cannot be
    /// read or leads outside the site folder.
    pub fn with_theme(site_root: &Path) -> Result<Self, RenderError> {
        Self::with_pack(site_root, &RenderPack::classic())
    }

    fn build(
        overrides: Vec<Source>,
        pack: &RenderPack<'_>,
        strings: Strings,
    ) -> Result<Self, RenderError> {
        let mut env = Environment::new();
        env.set_auto_escape_callback(|_| AutoEscape::Html);
        env.set_formatter(escape::format);
        env.set_undefined_behavior(UndefinedBehavior::Strict);
        env.set_fuel(Some(FUEL_PER_PAGE));
        env.add_filter("date", dates::format_date);
        env.add_filter("heading_links", heading_links::heading_links);
        env.add_filter("enlarge", enlarge::enlarge);
        env.add_function("share_links", share::share_links);
        env.add_function("first_picture", image::first_picture);
        env.add_filter("place_cta", cta::place_cta);
        for source in templates::defaults().into_iter().chain(overrides) {
            env.add_template_owned(source.name.clone(), source.text)
                .map_err(|error| RenderError::compile(&source.name, error))?;
        }
        Ok(Self {
            env,
            strings,
            pack: PackContext::from(pack),
            brand: BrandContext::default(),
        })
    }

    /// This renderer with the main site's logo and icon from the tokens'
    /// `[brand]`: every page links the icon for the browser tab, and a
    /// template that turns on its `logo` part shows the logo in the header.
    #[must_use]
    pub fn with_brand(mut self, brand: &Brand) -> Self {
        self.brand = BrandContext {
            logo: brand.logo.as_ref().map(ToString::to_string),
            icon: brand.icon.as_ref().map(ToString::to_string),
        };
        self
    }

    /// Renders one page of `site` with the head `meta` and the main content
    /// `body`.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::Render`] when the template fails, for example
    /// a theme template that names a value the page does not have.
    pub fn render(
        &self,
        site: &RenderSite,
        meta: &PageMeta,
        body: &PageBody,
    ) -> Result<String, RenderError> {
        self.render_named(body.template(), site, meta, body)
    }

    /// Renders one page like [`render`](Self::render), but with the theme's
    /// post template `template`, such as `post-wide.html` for `wide`, in
    /// place of the page type's own.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::Render`] when the theme has no such template,
    /// which [`has_template`](Self::has_template) tells beforehand, or when
    /// it fails.
    pub fn render_as(
        &self,
        template: &TemplateName,
        site: &RenderSite,
        meta: &PageMeta,
        body: &PageBody,
    ) -> Result<String, RenderError> {
        self.render_named(&template.file_name(), site, meta, body)
    }

    /// Whether the theme has the post template `template`.
    pub fn has_template(&self, template: &TemplateName) -> bool {
        self.env.get_template(&template.file_name()).is_ok()
    }

    fn render_named(
        &self,
        name: &str,
        site: &RenderSite,
        meta: &PageMeta,
        body: &PageBody,
    ) -> Result<String, RenderError> {
        let cards = u64::try_from(body.cards()).unwrap_or(u64::MAX);
        let for_cards = cards.saturating_mul(FUEL_PER_CARD);
        // Fuel is set on the environment, so a list longer than the cap
        // per page covers renders in a copy with more. The copy shares the
        // compiled templates.
        let wider;
        let env = if for_cards > FUEL_PER_PAGE {
            let mut copy = self.env.clone();
            copy.set_fuel(Some(for_cards.saturating_add(FUEL_PER_PAGE)));
            wider = copy;
            &wider
        } else {
            &self.env
        };
        let template = env
            .get_template(name)
            .map_err(|error| RenderError::render(name, error))?;
        let placed;
        let pack = match body {
            PageBody::Home(home) if home.places_newsletter() => {
                placed = self.pack.with_newsletter_placed();
                &placed
            }
            _ => &self.pack,
        };
        let context = context! {
            site => Serde(site),
            meta => Serde(meta),
            t => Serde(self.strings.for_locale(&site.locale)),
            lang => site.locale.as_str(),
            dir => Serde(Direction::of(&site.locale)),
            page => body.value(),
            lists_posts => body.lists_posts(),
            template => Serde(pack),
            brand => Serde(&self.brand),
        };
        template
            .render(context)
            .map_err(|error| RenderError::render(name, error))
    }

    /// The `part` of the consent banner in `locale`, from `consent.html`,
    /// for the page splice to put right after the skip link.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::Render`] when the template fails, for example
    /// a theme's `consent.html` that names a value the banner does not have.
    pub fn consent(
        &self,
        locale: &Locale,
        banner: &ConsentBanner,
        part: ConsentPart,
    ) -> Result<String, RenderError> {
        let name = "consent.html";
        let template = (self.env)
            .get_template(name)
            .map_err(|error| RenderError::render(name, error))?;
        let context = context! {
            t => Serde(self.strings.for_locale(locale)),
            banner => Serde(banner),
            part => Serde(part),
        };
        template
            .render(context)
            .map_err(|error| RenderError::render(name, error))
    }

    /// The privacy facts as `privacy_facts.html` draws them in `locale`, for
    /// an owner's own privacy page to show where it writes
    /// `::privacy-facts`. The owner's words are never read as a template.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::Render`] when the template fails.
    pub fn privacy_facts(
        &self,
        locale: &Locale,
        facts: &PrivacyFacts,
    ) -> Result<String, RenderError> {
        let name = "privacy_facts.html";
        let template = (self.env)
            .get_template(name)
            .map_err(|error| RenderError::render(name, error))?;
        let context = context! {
            t => Serde(self.strings.for_locale(locale)),
            lang => locale.as_str(),
            facts => Serde(facts),
        };
        template
            .render(context)
            .map_err(|error| RenderError::render(name, error))
    }

    /// The text `key` of the strings file for `locale`, as templates read it
    /// under `t`, such as `search_link` for the search page's title. `None`
    /// for a key the strings file does not have.
    pub fn text(&self, locale: &Locale, key: &str) -> Option<&str> {
        self.strings
            .for_locale(locale)
            .and_then(|strings| strings.get(key))
            .map(String::as_str)
    }
}
