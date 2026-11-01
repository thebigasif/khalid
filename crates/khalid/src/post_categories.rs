//! The category of each post where a page shows it: on the post, on every
//! card that lists it, and in the row of topic links on the home page.
//! Templates such as category pills and a category bar read these
//! (templates spec 3.1 and 3.2).

use std::collections::BTreeMap;

use khalid_core::{Config, Locale, Slug};
use khalid_db::IndexedTerm;
use khalid_render::{Link, PageBody, PostCard, TopicGroup};
use khalid_routes::Routes;

use crate::build_pages::{LanguagePages, path_of};

/// The link to the page of the category `slug` in `locale`, named by its
/// slug until [`fill`] names it.
pub(crate) fn link(config: &Config, routes: &Routes, locale: &Locale, slug: &Slug) -> Link {
    Link {
        name: slug.to_string(),
        href: path_of(
            config,
            locale,
            &format!("{}/{slug}/", routes.category_prefix()),
        ),
    }
}

/// Names each category the pages of `languages` show as `categories`
/// names it in the page's language, and gives each home page one group of
/// topic links: every category of its posts, in the order of
/// `categories`, then any category the file does not name.
pub(crate) fn fill(
    (config, routes): (&Config, &Routes),
    categories: &[IndexedTerm],
    languages: &mut [LanguagePages],
) {
    for language in languages {
        let locale = &language.site.locale;
        // Each named category's page address, its name and its place in
        // the file.
        let named: BTreeMap<String, (usize, &str)> = (categories.iter().enumerate())
            .filter(|(_, term)| &term.locale == locale)
            .map(|(at, term)| {
                let href = link(config, routes, locale, &term.slug).href;
                (href, (at, term.name.as_str()))
            })
            .collect();
        for page in &mut language.pages {
            name_page(&mut page.body, &named);
        }
    }
}

/// Names the categories `body` shows by `named`, and fills the topic links
/// of a home page that has none from its posts.
fn name_page(body: &mut PageBody, named: &BTreeMap<String, (usize, &str)>) {
    let name = |category: &mut Option<Link>| {
        if let Some(link) = category
            && let Some((_, name)) = named.get(&link.href)
        {
            (*name).clone_into(&mut link.name);
        }
    };
    let name_card = |card: &mut PostCard| name(&mut card.category);
    match body {
        PageBody::Home(home) => {
            home.listing.posts.iter_mut().for_each(name_card);
            home.listing.featured.iter_mut().for_each(name_card);
            home.picks.iter_mut().for_each(name_card);
            (home.sections.iter_mut())
                .for_each(|section| section.posts.iter_mut().for_each(name_card));
            if home.topics.is_empty() {
                let links = topics(&home.listing.posts, named);
                if !links.is_empty() {
                    home.topics = vec![TopicGroup { name: None, links }];
                }
            }
        }
        PageBody::Collection(listing)
        | PageBody::Tag(listing)
        | PageBody::Category(listing)
        | PageBody::Archive(listing)
        | PageBody::Channel(listing)
        | PageBody::Date(listing) => {
            listing.posts.iter_mut().for_each(name_card);
            listing.featured.iter_mut().for_each(name_card);
        }
        PageBody::Post(post) => {
            name(&mut post.category);
            post.related.iter_mut().for_each(name_card);
        }
        PageBody::Author(author) => author.posts.iter_mut().for_each(name_card),
        _ => {}
    }
}

/// Each category of `posts` once, in the order of the file `named` comes
/// from, then the ones it does not name in the order their first post
/// comes.
fn topics(posts: &[PostCard], named: &BTreeMap<String, (usize, &str)>) -> Vec<Link> {
    let mut found: Vec<(usize, Link)> = Vec::new();
    for category in posts.iter().filter_map(|post| post.category.as_ref()) {
        if !found.iter().any(|(_, link)| link.href == category.href) {
            let at = named.get(&category.href).map_or(usize::MAX, |(at, _)| *at);
            found.push((at, category.clone()));
        }
    }
    // A stable sort keeps the unnamed ones in the order they came.
    found.sort_by_key(|(at, _)| *at);
    found.into_iter().map(|(_, link)| link).collect()
}
