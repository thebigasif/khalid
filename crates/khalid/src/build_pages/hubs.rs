//! Hubs: a page whose front matter has a `hub` table becomes the front
//! door to one topic. It shows its own text as the topic's intro, its
//! chapters in order and the topic's newest posts, and its category's list
//! links to it as the place to start.

use khalid_content::SiteContent;
use khalid_core::{AbsoluteUrl, Config};
use khalid_render::{Link, PageBody, PostCard, RenderHub};
use khalid_routes::Routes;
use khalid_seo::{ItemList, ListEntry, SchemaBlock};

use crate::build_pages::LanguagePages;
use crate::listed_post::ListedPost;
use crate::post_categories;

/// How many of the topic's newest posts a hub shows under its chapters:
/// two rows of three cards, enough to show the topic is alive.
const LATEST: usize = 6;

/// Turns each page of `languages` that `content` marks as a hub into one,
/// and links each hub's category list to it. Runs after the lists are
/// made and before categories are named.
pub(crate) fn fill(
    (config, routes): (&Config, &Routes),
    content: &SiteContent,
    languages: &mut [LanguagePages],
) {
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let mut links: Vec<(Link, Link)> = Vec::new();
        for page in &mut language.pages {
            let Some(source) = page.source.as_ref() else {
                continue;
            };
            let doc =
                (content.pages.iter()).find(|doc| doc.locale == locale && &doc.path == source);
            let Some(hub) = doc.and_then(|doc| doc.front.hub.as_ref()) else {
                continue;
            };
            let PageBody::Page(own) = &page.body else {
                continue;
            };
            let topic = post_categories::link(config, routes, &locale, &hub.category);
            let chapters: Vec<PostCard> = (hub.chapters.iter())
                .filter_map(|slug| {
                    language
                        .listed
                        .iter()
                        .find(|post| post.topics.slug == slug.as_str())
                })
                .map(|post| post.card.clone())
                .collect();
            let latest = latest(&language.listed, &topic, &chapters);
            let origin = config.address().as_str().trim_end_matches('/');
            let entries = (chapters.iter())
                .filter_map(|card| {
                    let url = AbsoluteUrl::parse(&format!("{origin}{}", card.href)).ok()?;
                    Some(ListEntry::new(&card.title, url))
                })
                .collect::<Vec<_>>();
            if !entries.is_empty() {
                let list = ItemList::new(&own.title, entries);
                page.seo.blocks.push(SchemaBlock::List(list));
            }
            let here = Link {
                name: own.title.clone(),
                href: page.meta.path.clone(),
            };
            links.push((topic.clone(), here));
            page.body = PageBody::Hub(Box::new(RenderHub {
                title: own.title.clone(),
                body: own.body.clone(),
                topic: Some(topic),
                chapters,
                latest,
            }));
        }
        for page in &mut language.pages {
            if let PageBody::Category(listing) = &mut page.body
                && let Some((_, hub)) = links.iter().find(|(topic, _)| topic.href == page.meta.path)
            {
                listing.hub = Some(hub.clone());
            }
        }
    }
}

/// The newest posts of `listed`, which is newest first, in the category
/// `topic`, leaving out the `chapters` the hub already lists.
fn latest(listed: &[ListedPost], topic: &Link, chapters: &[PostCard]) -> Vec<PostCard> {
    let in_topic = |post: &&ListedPost| {
        let category = post.card.category.as_ref();
        category.is_some_and(|category| category.href == topic.href)
    };
    let new = |post: &&ListedPost| !chapters.iter().any(|card| card.href == post.card.href);
    (listed.iter())
        .filter(in_topic)
        .filter(new)
        .take(LATEST)
        .map(|post| post.card.clone())
        .collect()
}
