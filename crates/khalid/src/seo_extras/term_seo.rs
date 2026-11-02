//! Category and tag SEO, as Yoast has it: each term page's own title,
//! description, robots, share image and intro, from the `seo` tables of
//! `categories.toml` and `tags.toml` in the site folder.

use std::collections::BTreeMap;
use std::path::Path;

use khalid_content::render;
use khalid_core::{Config, TermSeo, UserMessage};
use khalid_render::{BodyHtml, PageBody, ShareImage};
use khalid_routes::Routes;
use khalid_seo::{MetaTemplate, PageKind, PageNumber};
use serde::Deserialize;

use crate::build_pages::LanguagePages;
use crate::seo_head_settings::share_image;
use crate::site_page::Page;
use crate::{build_seo, config_message};

/// One entry of either file: its slug and its `seo` tables by language.
/// The names are read by `categories`, and tags take theirs from posts.
#[derive(Deserialize)]
struct Entry {
    slug: String,
    #[serde(default)]
    seo: BTreeMap<String, TermSeo>,
    #[serde(flatten)]
    _names: BTreeMap<String, toml::Value>,
}

#[derive(Deserialize, Default)]
struct File {
    #[serde(default)]
    categories: Vec<Entry>,
    #[serde(default)]
    tags: Vec<Entry>,
}

/// Each term's settings by the page kind, its slug and its language code.
type Terms = BTreeMap<(PageKind, String, String), TermSeo>;

/// Gives each category and tag page in `languages` the settings its term
/// has in the `site` folder, and returns the address of every first term
/// page left without an intro, for the build's note.
///
/// # Errors
///
/// What, why and fix for a file that cannot be read, or a title, image or
/// intro that is not allowed.
pub(crate) fn fill(
    site: &Path,
    (config, routes): (&Config, &Routes),
    languages: &mut [LanguagePages],
) -> Result<Vec<String>, UserMessage> {
    let mut terms = Terms::new();
    for (name, kind) in [
        ("categories.toml", PageKind::Category),
        ("tags.toml", PageKind::Tag),
    ] {
        read(site, name, kind, &mut terms)?;
    }
    let mut bare = Vec::new();
    for language in languages {
        let locale = &language.site.locale;
        for page in &mut language.pages {
            let (noun, prefix) = match page.seo.kind {
                PageKind::Category => ("category", routes.category_prefix()),
                PageKind::Tag => ("tag", routes.tag_prefix()),
                _ => continue,
            };
            let slug = slug_after(page.seo.path.as_str(), prefix).to_owned();
            let first = page.seo.page_number.is_none_or(PageNumber::is_first);
            let key = (page.seo.kind, slug.clone(), locale.as_str().to_owned());
            let intro = match terms.get(&key) {
                Some(term) => {
                    let at = format!("{noun} {slug}, seo.{}", locale.as_str());
                    apply(&at, term, page)?;
                    term.intro
                        .as_deref()
                        .filter(|intro| !intro.trim().is_empty())
                }
                None => None,
            };
            if !first {
                continue;
            }
            let Some(intro) = intro else {
                bare.push(page.meta.path.clone());
                continue;
            };
            if let PageBody::Category(list) | PageBody::Tag(list) = &mut page.body {
                let at = format!("{noun} {slug}, seo.{}", locale.as_str());
                let html = render(intro, locale, config.slugs())
                    .map_err(|error| wrong(&at, &error))?
                    .html;
                let html = build_seo::with_link_policy(config, html);
                list.intro = Some(BodyHtml::from_sanitised(html));
            }
        }
    }
    Ok(bare)
}

/// The path segment after `prefix` in `path`, such as `seo` in
/// `/de/tag/seo/page/2/`. Empty when the path has no such segment.
fn slug_after<'a>(path: &'a str, prefix: &str) -> &'a str {
    let mut segments = path.split('/').skip_while(|segment| *segment != prefix);
    segments.nth(1).unwrap_or_default()
}

/// Puts `term`'s title, description, robots and share image on `page`.
fn apply(at: &str, term: &TermSeo, page: &mut Page) -> Result<(), UserMessage> {
    if let Some(title) = &term.title {
        let title = MetaTemplate::parse(title).map_err(|error| wrong(at, &error))?;
        page.seo.seo_title = Some(title);
    }
    if let Some(description) = &term.description {
        page.seo.excerpt.clone_from(description);
        page.seo.term_description = Some(description.clone());
        page.meta.description.clone_from(description);
    }
    if term.robots.is_some() {
        page.seo.robots.clone_from(&term.robots);
    }
    if let Some(image) = &term.image {
        let image = share_image(at, image).map_err(|mistake| mistake.message())?;
        page.meta.image = Some(ShareImage {
            url: image.url.clone(),
            alt: image.alt.clone(),
            width: image.width,
            height: image.height,
        });
        page.seo.share_image = Some(image);
    }
    Ok(())
}

/// Adds the `seo` tables of the `name` file in `site`, if it has one, as
/// settings of `kind` pages.
fn read(site: &Path, name: &str, kind: PageKind, terms: &mut Terms) -> Result<(), UserMessage> {
    let mut path = site.to_path_buf();
    path.push(name);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(wrong(name, &error)),
    };
    let file: File = toml::from_str(&text).map_err(|error| wrong(name, &error))?;
    let entries = if kind == PageKind::Category {
        file.categories
    } else {
        file.tags
    };
    for entry in entries {
        for (language, seo) in entry.seo {
            terms.insert((kind, entry.slug.clone(), language), seo);
        }
    }
    Ok(())
}

/// Tells the person building which category and tag pages have no intro.
/// A term page with only a list of posts gives search engines little to
/// rank, so Yoast flags it too; the build still goes on.
pub(crate) fn note(bare: &[String]) {
    if let [first, ..] = bare {
        tracing::info!(
            count = bare.len(),
            %first,
            "some category or tag pages have no intro, so search engines see only a list of links; add intro under seo in categories.toml or tags.toml"
        );
    }
}

fn wrong(at: &str, error: &dyn std::fmt::Display) -> UserMessage {
    UserMessage::new(
        format!("The search settings of {at} cannot be used: {error}."),
        config_message::WHY,
        "Correct the seo table in categories.toml or tags.toml, then run the command again.",
    )
}

#[cfg(test)]
mod tests {
    use super::slug_after;

    #[test]
    fn the_slug_is_found_after_the_prefix_in_any_language_folder() {
        assert_eq!(slug_after("/tag/seo/", "tag"), "seo");
        assert_eq!(slug_after("/de/tag/seo/page/2/", "tag"), "seo");
        assert_eq!(slug_after("/blog/", "tag"), "");
    }
}
