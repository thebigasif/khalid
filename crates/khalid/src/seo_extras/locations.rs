//! Local SEO, as Yoast Local has it: each `::location` block becomes the
//! card of the location it names in `[[seo.locations]]`.

use khalid_core::{Config, UserMessage};
use khalid_render::{BodyHtml, LocationWords, PageBody, place_locations};

use crate::build_message;
use crate::build_pages::LanguagePages;

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
