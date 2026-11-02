//! Category and tag SEO, as Yoast has it: each term page's own title,
//! description, robots, share image and intro, from the `seo` tables of
//! `categories.toml` and `tags.toml` in the site folder.

use std::collections::BTreeMap;
use std::path::Path;

use khalid_content::render;
use khalid_core::{Config, TermSeo, UserMessage};
use khalid_render::{BodyHtml, PageBody, ShareImage};
use khalid_routes::Routes;
use khalid_seo::{MetaTemplate, PageKind};
use serde::Deserialize;

use crate::build_pages::LanguagePages;
use crate::seo_head_settings::share_image;
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
    for (name, kind) in [("categories.toml", PageKind::Category), ("tags.toml", PageKind::Tag)] {
        read(site, name, kind, &mut terms)?;
    }
    let mut bare = Vec::new();
    for language in languages {
        let code = language.site.locale.as_str().to_owned();
        for page in &mut language.pages {
            let prefix = match page.seo.kind {
                PageKind::Category => routes.category_prefix(),
                PageKind::Tag => routes.tag_prefix(),
                _ => continue,
            };
            let path = page.seo.path.as_str().to_owned();
            let slug = (path.strip_prefix(&format!("/{prefix}/")))
                .and_then(|rest| rest.split('/').next())
                .unwrap_or_default();
            let first = page.seo.page_number.is_none_or(|number| number.is_first());
            let term = terms.get(&(page.seo.kind, slug.to_owned(), code.clone()));
            let Some(term) = term else {
                if first {
                    bare.push(page.meta.path.clone());
                }
                continue;
            };
            let at = format!("{} {slug}, seo.{code}", page.seo.kind.noun());
            if let Some(title) = &term.title {
                let title = MetaTemplate::parse(title).map_err(|error| wrong(&at, &error))?;
                page.seo.seo_title = Some(title);
            }
            if let Some(description) = &term.description {
                page.seo.excerpt.clone_from(description);
                page.seo.term_description = Some(description.clone());
                page.meta.description.clone_from(description);
            }
            page.seo.robots.clone_from(&term.robots);
            if let Some(image) = &term.image {
                let image = share_image(&at, image).map_err(|mistake| mistake.message())?;
                page.meta.image = Some(ShareImage {
                    url: image.url.clone(),
                    alt: image.alt.clone(),
                    width: image.width,
                    height: image.height,
                });
                page.seo.share_image = Some(image);
            }
            let intro = term.intro.as_deref().filter(|intro| !intro.trim().is_empty());
            match (intro, &mut page.body) {
                (Some(intro), PageBody::Category(list) | PageBody::Tag(list)) if first => {
                    let html = render(intro, &language.site.locale, config.slugs())
                        .map_err(|error| wrong(&at, &error))?
                        .html;
                    list.intro = Some(BodyHtml::from_sanitised(build_seo::with_link_policy(config, html)));
                }
                (None, _) if first => bare.push(page.meta.path.clone()),
                _ => {}
            }
        }
    }
    Ok(bare)
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
    let entries = if kind == PageKind::Category { file.categories } else { file.tags };
    for entry in entries {
        for (language, seo) in entry.seo {
            terms.insert((kind, entry.slug.clone(), language), seo);
        }
    }
    Ok(())
}

fn wrong(at: &str, error: &dyn std::fmt::Display) -> UserMessage {
    UserMessage::new(
        format!("The search settings of {at} cannot be used: {error}."),
        config_message::WHY,
        "Correct the seo table in categories.toml or tags.toml, then run the command again.",
    )
}
