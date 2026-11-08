//! The privacy choices page of each language, at `privacy-choices/`, which
//! the footer of every page links (privacy plan U4, R2, R6, R7). It lists
//! the five categories with every optional switch off; its form posts to
//! the consent route, as the banner's does.

use khalid_content::{EMBED_PROVIDERS, EmbedProvider};
use khalid_core::{Config, Locale, Mode, UserMessage};
use khalid_render::{
    ConsentKeeper, ConsentRow, PageBody, PageKind, RenderPrivacyChoices, Renderer,
};
use khalid_seo::PageKind as SeoKind;

use crate::build_lists::{self, text};
use crate::build_pages::{self, LanguagePages, Page};
use crate::build_seo;
use crate::page_code::CONSENT_FOLDER;
use crate::static_mode::consent::Asking;

/// The end of the page's address, after the mount and the language.
pub(crate) const REST: &str = "privacy-choices/";

/// The categories in the order the page lists them, by the name the form
/// sends (privacy plan KTD2). Only the first is always on.
const CATEGORIES: [&str; 5] = [
    "necessary",
    "preferences",
    "analytics",
    "marketing",
    "external-media",
];

/// Adds the privacy choices page to each of `languages`, in its language
/// from `renderer`'s strings. It is kept out of search results and
/// sitemaps, like the not-found page.
///
/// # Errors
///
/// Returns what, why and fix when the page's address cannot be made from
/// the site's settings.
pub(crate) fn add(
    config: &Config,
    renderer: &Renderer,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    // A static site asks only with its consent script, and only when its
    // code waits or a post shows an embed (privacy plan U6).
    let keeper = match config.mode() {
        Mode::Server => ConsentKeeper::Server,
        _ if Asking::of(config, languages) == Asking::Nothing => ConsentKeeper::Nobody,
        _ => ConsentKeeper::Script,
    };
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let title = text(renderer, &locale, "privacy_choices_title")?;
        let path = build_pages::path_of(config, &locale, REST);
        let meta = build_pages::meta(
            config,
            title.clone(),
            String::new(),
            path,
            PageKind::Website,
        )?;
        let mut seo = build_seo::page(SeoKind::NotFound, &locale, REST, &title)?;
        seo.translation = build_lists::list_key("privacy-choices")?;
        let body = PageBody::PrivacyChoices(RenderPrivacyChoices {
            title,
            action: format!("{}{CONSENT_FOLDER}/", config.mount()),
            // The privacy page comes with privacy plan U9.
            privacy: None,
            keeper,
            categories: rows(renderer, &locale)?,
        });
        language.pages.push(Page {
            file: format!("{}{REST}index.html", build_pages::prefix(config, &locale)),
            meta,
            body,
            seo,
            source: None,
            template: None,
            head_code: None,
            lead: None,
        });
    }
    Ok(())
}

/// The five categories in `locale`, each with its name and what it covers.
/// External media names every site an embed may load a player from.
fn rows(renderer: &Renderer, locale: &Locale) -> Result<Vec<ConsentRow>, UserMessage> {
    CATEGORIES
        .into_iter()
        .map(|key| {
            let strings = format!("consent_{}", key.replace('-', "_"));
            let vendors = if key == "external-media" {
                EMBED_PROVIDERS.into_iter().filter_map(name).collect()
            } else {
                Vec::new()
            };
            Ok(ConsentRow {
                key: key.to_owned(),
                name: text(renderer, locale, &strings)?,
                note: text(renderer, locale, &format!("{strings}_note"))?,
                vendors,
                optional: key != "necessary",
            })
        })
        .collect()
}

/// The name readers know `provider` by.
fn name(provider: EmbedProvider) -> Option<String> {
    let name = match provider {
        EmbedProvider::YouTube => "YouTube",
        EmbedProvider::Vimeo => "Vimeo",
        EmbedProvider::Spotify => "Spotify",
        EmbedProvider::SoundCloud => "SoundCloud",
        EmbedProvider::CodePen => "CodePen",
        EmbedProvider::Loom => "Loom",
        _ => return None,
    };
    Some(name.to_owned())
}
