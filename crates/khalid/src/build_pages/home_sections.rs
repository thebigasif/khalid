//! The home page's sections from `[reader] home_sections` (audit brief
//! G19), in the owner's order. A section of a category, a collection, a
//! channel or a series is made from that list's own first page, so its
//! name, its "View all" link and its posts match the page it links to. A
//! section whose list has no page in a language, such as a category with
//! no posts in German yet, is left out of that language's home page, and
//! so is the newsletter on a site without one.

use khalid_core::{Config, HomeSection, Locale, UserMessage};
use khalid_render::{
    HomeSectionKind, HomeSectionPosts, PageBody, PostCard, RenderListing, Renderer,
};
use khalid_routes::Routes;

use crate::build_lists;
use crate::build_pages::{self, LanguagePages, Page};

/// How many posts a section shows: two rows of three on a wide screen.
/// A template may show fewer.
const PER_SECTION: usize = 6;

/// Fills the sections of each home page of `languages`, named in its
/// language by `renderer`'s strings. Without `home_sections` the home
/// pages stay as they are: one list of the newest posts.
///
/// # Errors
///
/// Returns what, why and fix when the strings file lacks a section's name,
/// which the shipped strings never do.
pub(crate) fn fill(
    (config, routes): (&Config, &Routes),
    renderer: &Renderer,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    let wanted = &config.reader().home_sections;
    if wanted.is_empty() {
        return Ok(());
    }
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let mut sections = Vec::with_capacity(wanted.len());
        for section in wanted {
            let made = match section {
                HomeSection::Newsletter => {
                    (language.site.subscribe.is_some()).then(HomeSectionPosts::newsletter)
                }
                HomeSection::Latest | HomeSection::Featured => {
                    newest((config, routes), renderer, language, section)?
                }
                listed => listed_rest(routes, listed).and_then(|rest| {
                    let path = build_pages::path_of(config, &locale, &rest);
                    from_list(&language.pages, &path)
                }),
            };
            sections.extend(made);
        }
        for page in &mut language.pages {
            if let PageBody::Home(home) = &mut page.body {
                home.sections.clone_from(&sections);
            }
        }
    }
    Ok(())
}

/// The newest posts of `language`, or its featured ones, linked to the
/// archive. Featured posts link to a channel of only featured posts when
/// `routes.toml` declares one. `None` when there are no such posts.
fn newest(
    (config, routes): (&Config, &Routes),
    renderer: &Renderer,
    language: &LanguagePages,
    section: &HomeSection,
) -> Result<Option<HomeSectionPosts>, UserMessage> {
    let locale: &Locale = &language.site.locale;
    let featured = matches!(section, HomeSection::Featured);
    let key = if featured {
        "editors_picks"
    } else {
        "latest_posts"
    };
    let channel = routes
        .channels()
        .iter()
        .find(|channel| channel.filter().featured == Some(true))
        .filter(|_| featured)
        .map(|channel| channel.path().trim_start_matches('/').to_owned());
    let rest = channel.unwrap_or_else(|| {
        if config.reader().archive {
            "archive/".to_owned()
        } else {
            String::new()
        }
    });
    let posts: Vec<PostCard> = (language.listed.iter())
        .filter(|post| !featured || post.featured)
        .take(PER_SECTION)
        .map(|post| post.card.clone())
        .collect();
    if posts.is_empty() {
        return Ok(None);
    }
    Ok(Some(HomeSectionPosts {
        title: build_lists::text(renderer, locale, key)?,
        href: build_pages::path_of(config, locale, &rest),
        posts,
        kind: HomeSectionKind::Posts,
    }))
}

/// Where the list of a category, a collection, a channel or a series sits,
/// after the mount and the language prefix. `None` for a collection whose
/// page is the home page, a channel `routes.toml` does not declare, and
/// the sections no list holds.
fn listed_rest(routes: &Routes, section: &HomeSection) -> Option<String> {
    match section {
        HomeSection::Category(slug) => Some(format!("{}/{slug}/", routes.category_prefix())),
        HomeSection::Collection(name) => build_lists::collection_rest(routes, name.as_str()),
        HomeSection::Channel(name) => routes
            .channels()
            .iter()
            .find(|channel| channel.name() == name.as_str())
            .map(|channel| channel.path().trim_start_matches('/').to_owned()),
        HomeSection::Series(slug) => Some(format!("series/{slug}/")),
        _ => None,
    }
}

/// The section made from the list page at `path` among `pages`: its title
/// and its first posts. `None` when no list page sits there.
fn from_list(pages: &[Page], path: &str) -> Option<HomeSectionPosts> {
    let listing: &RenderListing = pages.iter().find_map(|page| match &page.body {
        PageBody::Category(listing)
        | PageBody::Collection(listing)
        | PageBody::Channel(listing)
            if page.meta.path == path =>
        {
            Some(listing)
        }
        _ => None,
    })?;
    Some(HomeSectionPosts {
        title: listing.title.clone(),
        href: path.to_owned(),
        posts: listing.posts.iter().take(PER_SECTION).cloned().collect(),
        kind: HomeSectionKind::Posts,
    })
}
