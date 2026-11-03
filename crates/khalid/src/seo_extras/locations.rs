//! Local SEO, as Yoast Local has it: each `::location` block becomes the
//! card of the location it names in `[[seo.locations]]`, and the
//! locations page lists them all.

use khalid_core::{Config, UserMessage};
use khalid_render::{
    BodyHtml, LocationWords, LocationsPage, PageBody, PageKind, RenderPage, place_locations,
};
use khalid_seo::PageKind as SeoKind;

use crate::build_pages::{LanguagePages, meta, page, path_of};
use crate::{build_message, build_seo};

/// Where the locations page sits in each language's folder.
const PAGE: &str = "locations/";

/// Adds the locations page to each language of a site with locations,
/// unless `[seo.local] page = false` or the site has its own page there.
///
/// # Errors
///
/// What, why and fix when the built-in words cannot be read or the
/// page's address cannot be made.
pub(crate) fn add_page(
    config: &Config,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    let locations = &config.seo().locations;
    if locations.is_empty() || !config.seo().local.page {
        return Ok(());
    }
    for language in languages {
        let locale = &language.site.locale;
        let path = path_of(config, locale, PAGE);
        if language.pages.iter().any(|page| page.meta.path == path) {
            continue;
        }
        let made =
            LocationsPage::new(locations, locale).map_err(|error| build_message::render(&error))?;
        let mut seo = build_seo::page(SeoKind::Page, locale, PAGE, &made.title)?;
        seo.excerpt.clone_from(&made.description);
        // The page is the same list in every language, so its versions pair.
        seo.translation = build_seo::translation(Some("locations page"))?;
        let meta = meta(
            config,
            made.title.clone(),
            made.description,
            path,
            PageKind::Website,
        )?;
        let body = PageBody::Page(RenderPage {
            title: made.title,
            updated: None,
            body: BodyHtml::from_sanitised(made.body),
            term: None,
        });
        language
            .pages
            .push(page(locale, config, PAGE, meta, body, seo));
    }
    Ok(())
}

/// Swaps each location mark in the posts and pages of `languages` for its
/// card, in the page's language. A name the site does not list leaves no
/// card, and the build says which, so the person building can fix it.
///
/// # Errors
///
/// What, why and fix when the built-in words cannot be read.
pub(crate) fn fill(config: &Config, languages: &mut [LanguagePages]) -> Result<(), UserMessage> {
    let locations = &config.seo().locations;
    for language in languages {
        let words = LocationWords::for_locale(&language.site.locale)
            .map_err(|error| build_message::render(&error))?;
        for page in &mut language.pages {
            let body = match &mut page.body {
                PageBody::Post(post) => &mut post.body,
                PageBody::Page(shown) => &mut shown.body,
                _ => continue,
            };
            let Some((html, missing)) = place_locations(body.as_str(), locations, &words) else {
                continue;
            };
            *body = BodyHtml::from_sanitised(html);
            for name in missing {
                tracing::warn!(
                    page = %page.meta.path,
                    %name,
                    "a location block names a location site.toml does not list, so it shows nothing; add it under [[seo.locations]] or correct the name"
                );
            }
        }
    }
    Ok(())
}
