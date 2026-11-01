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
use crate::{Brand, PageBody, PageMeta, RenderError, RenderSite};
use crate::{cta, enlarge, escape, share};

/// How many steps one page may take before rendering stops. The default
/// pages use a few thousand; the cap stops a theme template that loops
/// without end from holding a worker (RULES.md BL-SEC-10).
const FUEL_PER_PAGE: u64 = 1_000_000;

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
        Self::build(Vec::new(), &RenderPack::classic())
    }

    /// A renderer where the template pack `pack` replaces the default
    /// templates it names, and each file in `site_root/theme/templates/`
    /// replaces either. A pack may also add post templates such as
    /// `post-reference.html`, which a post asks for with `template:`.
    /// Templates read the pack's name and options as `template`.
    ///
    /// # Errors
    ///
    /// The errors of [`Renderer::with_theme`], and
    /// [`RenderError::UnknownTemplate`] for a pack file that is neither a
    /// default template nor a post template.
    pub fn with_pack(site_root: &Path, pack: &RenderPack<'_>) -> Result<Self, RenderError> {
        let mut sources = Vec::new();
        for &(name, text) in &pack.templates {
            if !templates::is_default(name) && !templates::is_post_template(name) {
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
        Self::build(sources, pack)
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

    fn build(overrides: Vec<Source>, pack: &RenderPack<'_>) -> Result<Self, RenderError> {
        let mut env = Environment::new();
        env.set_auto_escape_callback(|_| AutoEscape::Html);
        env.set_formatter(escape::format);
        env.set_undefined_behavior(UndefinedBehavior::Strict);
        env.set_fuel(Some(FUEL_PER_PAGE));
        env.add_filter("date", dates::format_date);
        env.add_filter("heading_links", heading_links::heading_links);
        env.add_filter("enlarge", enlarge::enlarge);
        env.add_function("share_links", share::share_links);
        env.add_filter("place_cta", cta::place_cta);
        for source in templates::defaults().into_iter().chain(overrides) {
            env.add_template_owned(source.name.clone(), source.text)
                .map_err(|error| RenderError::compile(&source.name, error))?;
        }
        Ok(Self {
            env,
            strings: Strings::load()?,
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
        let template = self
            .env
            .get_template(name)
            .map_err(|error| RenderError::render(name, error))?;
        let context = context! {
            site => Serde(site),
            meta => Serde(meta),
            t => Serde(self.strings.for_locale(&site.locale)),
            lang => site.locale.as_str(),
            dir => Serde(Direction::of(&site.locale)),
            page => body.value(),
            template => Serde(&self.pack),
            brand => Serde(&self.brand),
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
