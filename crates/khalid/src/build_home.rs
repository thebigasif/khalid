//! The home page of each language on its own, made from the posts' front
//! matter without rendering any body: what `khalid serve` answers first
//! while the rest of the site renders behind it.

use std::cmp::Reverse;
use std::collections::BTreeMap;

use chrono::NaiveDate;
use khalid_content::{PostFront, SiteContent, Wikilinks};
use khalid_core::{Config, Locale, Slug, UserMessage};
use khalid_render::{Person, PostCard};
use khalid_routes::{PostAddress, Routes};

use crate::build_pages::{self, LanguagePages};
use crate::home_pictures::{self, Leads};
use crate::{build_reader, post_categories, site_chrome};

/// The home page of each language of `content`, in the order of
/// `site.toml`, linking the stylesheet at `stylesheet`: the page
/// [`build_pages::pages`] makes there, every dated post's card newest
/// first, and no other page. Also returns each card's lead image, which
/// [`home_pictures::draw`] puts on the cards.
pub(crate) fn home_pages(
    config: &Config,
    routes: &Routes,
    content: &SiteContent,
    stylesheet: &str,
) -> Result<(Vec<LanguagePages>, Leads), UserMessage> {
    let mut languages = Vec::with_capacity(config.languages().len());
    let mut leads = Leads::new();
    for locale in config.languages() {
        let links = Wikilinks::for_language(content, config, routes, locale);
        let people = build_reader::people(config, &links, content, locale)?;
        let mut cards = Vec::new();
        for doc in content.posts.iter().filter(|doc| &doc.locale == locale) {
            let Some(date) = doc.front.date else {
                continue;
            };
            let front = &doc.front;
            let (title, slug) =
                build_pages::named(config, doc, front.slug.as_ref(), front.title.as_deref())?;
            let entry = PostAddress::new(&doc.collection, locale, &slug, Some(date));
            let rest = build_pages::routed(config, routes, locale, &entry, &doc.path)?;
            let named = (title, rest.as_str(), date);
            let card = card((config, routes), locale, &people, named, front);
            if let Some(lead) = home_pictures::of(doc, &card.href) {
                leads.insert(card.href.clone(), lead);
            }
            cards.push(card);
        }
        // A stable sort, as the full render's, so posts of one day keep
        // their file order.
        cards.sort_by_key(|card| Reverse(card.date));
        languages.push(LanguagePages {
            site: site_chrome::render_site(config, locale, stylesheet),
            pages: vec![build_pages::home_page(config, locale, cards)?],
            listed: Vec::new(),
            unlocked: Vec::new(),
        });
    }
    Ok((languages, leads))
}

/// The card lists show of a dated post with this `title`, address end
/// `rest` and `date`, and the front matter `front`. Its category is named
/// by its slug until [`post_categories::fill`] names it as
/// `categories.toml` does.
pub(crate) fn card(
    (config, routes): (&Config, &Routes),
    locale: &Locale,
    people: &BTreeMap<Slug, Person>,
    (title, rest, date): (String, &str, NaiveDate),
    front: &PostFront,
) -> PostCard {
    PostCard {
        title,
        href: build_pages::path_of(config, locale, rest),
        date,
        excerpt: front.description.clone().unwrap_or_default(),
        image: None,
        author: front
            .author
            .as_ref()
            .map(|slug| build_reader::person(people, slug, config, locale)),
        category: (front.category.as_ref())
            .map(|slug| post_categories::link(config, routes, locale, slug)),
        featured: front.featured,
        reading_time: None,
    }
}
