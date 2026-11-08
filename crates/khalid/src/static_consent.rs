//! Consent on a static site (privacy plan U6, KTD7, KTD9): the banner the
//! consent script shows, the choices page of each language, the tag that
//! loads the script and the file that says Global Privacy Control is
//! honoured. With scripts off, the banner stays hidden and nothing
//! optional ever loads.

use std::fmt::Write as _;

use khalid_content::EMBED_PROVIDERS;
use khalid_core::{Config, Locale, RobotsDirectives, UserMessage};
use khalid_render::{
    Asset, BodyHtml, PageBody, PageKind, RenderPage, Renderer, Robots, consent_script,
};
use khalid_seo::{PageKind as SeoKind, SitemapListing};
use sha2::{Digest, Sha256};

use crate::build_pages::{self, LanguagePages, Page};
use crate::post_lock::escaped;
use crate::{assets, build_files, build_message, build_seo, serve_message};

/// The end of the choices page's address, after the mount and the language.
pub(crate) const CHOICES: &str = "privacy-choices/";

/// The category the site's own code waits for until U2 tags each entry:
/// untagged owner code counts as Marketing (KTD2).
pub(crate) const MARKETING: &str = "marketing";

/// How long a choice lasts, in days, until `[privacy]` sets it (U2): six
/// months, as the server's cookie has always lasted.
const DAYS: u16 = 180;

/// The file that tells browsers the site honours Global Privacy Control,
/// at the root of the host.
pub(crate) const GPC_FILE: &str = ".well-known/gpc.json";

/// What [`GPC_FILE`] says. The day is when Khalid began to honour the
/// signal on static sites, so a build never changes it.
pub(crate) const GPC_JSON: &str = "{\"gpc\": true, \"lastUpdate\": \"2026-11-08\"}\n";

/// How an embed's player starts in a page.
const EMBED: &str = "<figure class=\"embed";

/// The marks the consent script looks for in a page: the banner, an embed
/// and the choices form. A page with none of them does not load it.
const NEEDS_SCRIPT: [&str; 3] = ["data-khalid-banner", EMBED, "data-khalid-choices"];

/// Whether the page `html` needs the consent script.
pub(crate) fn needs_script(html: &str) -> bool {
    NEEDS_SCRIPT.iter().any(|mark| html.contains(mark))
}

/// What a static site asks its readers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Asking {
    /// Nothing: no owner code waits and no page shows an embed, so no page
    /// loads the consent script.
    Nothing,
    /// Only whether to load the players of embeds.
    Media,
    /// Whether to run the site's own code, and to load the players of any
    /// embeds.
    Code,
}

impl Asking {
    /// What the site with `config` and `languages` asks: about its own
    /// code when it waits for consent, else about embeds when a post or
    /// page shows one.
    pub(crate) fn of(config: &Config, languages: &[LanguagePages]) -> Self {
        let embeds = |body: &BodyHtml| body.as_str().contains(EMBED);
        let shows_embed = (languages.iter())
            .flat_map(|language| &language.pages)
            .any(|page| match &page.body {
                PageBody::Post(post) => embeds(&post.body),
                PageBody::Page(page) => embeds(&page.body),
                _ => false,
            });
        if config.code().waits_for_consent() {
            Self::Code
        } else if shows_embed {
            Self::Media
        } else {
            Self::Nothing
        }
    }
}

/// The consent script and the tag that loads it, for the site with
/// `config` that asks as `asking` says, or `None` when it asks nothing.
///
/// # Errors
///
/// Returns what, why and fix when the script cannot be compressed.
pub(crate) fn script(
    config: &Config,
    asking: Asking,
) -> Result<Option<(Asset, String)>, UserMessage> {
    if asking == Asking::Nothing {
        return Ok(None);
    }
    let script = consent_script().map_err(|error| build_message::render(&error))?;
    let tag = format!(
        "<script src=\"{src}\" integrity=\"{integrity}\" data-khalid-rev=\"{rev}\" data-khalid-days=\"{DAYS}\" data-khalid-path=\"{path}\" defer></script>",
        src = escaped(&assets::href(config, &script)),
        integrity = script.integrity(),
        rev = rev(asking),
        path = escaped(config.mount()),
    );
    Ok(Some((script, tag)))
}

/// Where [`GPC_FILE`] goes in the output of the site with `config`: at the
/// top for a blog at the root of its host, else in the folder of files for
/// the root.
pub(crate) fn gpc_file(config: &Config) -> String {
    if config.mount() == "/" {
        GPC_FILE.to_owned()
    } else {
        format!("{}/{GPC_FILE}", build_files::ROOT_FOLDER)
    }
}

/// A short hash of what a reader agrees to: the categories with code, the
/// players External media may load and the lifetime. A change asks every
/// reader again (KTD3). U2 moves it into `[privacy]`.
fn rev(asking: Asking) -> String {
    let mut shown = format!("days={DAYS};");
    if asking == Asking::Code {
        shown.push_str(MARKETING);
        shown.push(';');
    }
    for provider in EMBED_PROVIDERS {
        shown.push_str(provider.frame_origin());
        shown.push(';');
    }
    Sha256::digest(shown.as_bytes()).iter().take(4).fold(
        String::with_capacity(8),
        |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        },
    )
}

/// The banner in `locale`, hidden until the consent script shows it, with
/// the status line that says a choice was saved.
pub(crate) fn banner(renderer: &Renderer, locale: &Locale, choices: &str) -> String {
    let words = |key: &str| escaped(renderer.text(locale, key).unwrap_or_default());
    format!(
        "<section class=\"consent\" aria-labelledby=\"khalid-consent-title\" data-khalid-banner hidden>\
         <h2 id=\"khalid-consent-title\">{title}</h2><p>{note}</p><p data-khalid-gpc hidden>{gpc}</p>\
         <p class=\"consent-answers\"><button type=\"button\" value=\"accept-all\">{accept}</button>\
         <button type=\"button\" value=\"reject-all\">{reject}</button>\
         <a href=\"{choices}\">{choose}</a></p></section>\n<p role=\"status\" data-khalid-saved=\"{saved}\"></p>",
        title = words("static_consent_title"),
        note = words("static_consent_note"),
        gpc = words("static_consent_gpc"),
        accept = words("static_consent_accept"),
        reject = words("static_consent_reject"),
        choices = escaped(choices),
        choose = words("static_consent_choose"),
        saved = words("static_consent_saved"),
    )
}

/// The choices page's body in `locale`. A site that asks nothing says so.
/// Any other has a note for readers with scripts off and the form the
/// consent script shows, with a Marketing row when the site's own code
/// waits.
fn choices_body(renderer: &Renderer, locale: &Locale, asking: Asking) -> String {
    let words = |key: &str| escaped(renderer.text(locale, key).unwrap_or_default());
    if asking == Asking::Nothing {
        return format!("<p>{}</p>\n", words("static_choices_none"));
    }
    let row = |name: &str, key: &str, state: &str| {
        format!(
            "<p><label><input type=\"checkbox\" name=\"{name}\"{state}> <strong>{label}</strong></label><br>{note}</p>\n",
            label = words(&format!("static_choices_{key}")),
            note = words(&format!("static_choices_{key}_note")),
        )
    };
    let mut rows = row("necessary", "necessary", " checked disabled");
    if asking == Asking::Code {
        rows.push_str(&row(MARKETING, "marketing", ""));
    }
    rows.push_str(&row("external-media", "media", ""));
    format!(
        "<p>{intro}</p>\n<p data-khalid-noscript>{noscript}</p>\n<p data-khalid-gpc hidden>{gpc}</p>\n\
         <div class=\"choices\" data-khalid-choices hidden>\n{rows}\
         <button type=\"button\" value=\"save\">{save}</button>\n</div>\n\
         <p role=\"status\" data-khalid-saved=\"{saved}\"></p>\n",
        intro = words("static_choices_intro"),
        noscript = words("static_choices_noscript"),
        gpc = words("static_consent_gpc"),
        save = words("static_choices_save"),
        saved = words("static_consent_saved"),
    )
}

/// Adds the choices page to each of `languages`, kept out of search
/// engines and the sitemap, with the choices `asking` offers.
///
/// # Errors
///
/// Returns what, why and fix when the page's address cannot be made from
/// the site's settings.
pub(crate) fn add_choices(
    config: &Config,
    renderer: &Renderer,
    languages: &mut [LanguagePages],
    asking: Asking,
) -> Result<(), UserMessage> {
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let title = renderer
            .text(&locale, "static_consent_title")
            .ok_or_else(serve_message::internal)?
            .to_owned();
        let path = build_pages::path_of(config, &locale, CHOICES);
        let mut meta = build_pages::meta(
            config,
            title.clone(),
            String::new(),
            path,
            PageKind::Website,
        )?;
        meta.robots = Robots::NoIndex;
        let mut seo = build_seo::page(SeoKind::Page, &locale, CHOICES, &title)?;
        seo.robots = "noindex".parse::<RobotsDirectives>().ok();
        seo.sitemap = SitemapListing::Excluded;
        let body = BodyHtml::from_sanitised(choices_body(renderer, &locale, asking));
        language.pages.push(Page {
            file: format!(
                "{}{CHOICES}index.html",
                build_pages::prefix(config, &locale)
            ),
            meta,
            body: PageBody::Page(RenderPage {
                title,
                updated: None,
                body,
                term: None,
            }),
            seo,
            source: None,
            template: None,
            head_code: None,
            lead: None,
        });
    }
    Ok(())
}
