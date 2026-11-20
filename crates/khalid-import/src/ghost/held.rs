//! The slugs of a Ghost post's people and tags, the fields a post's file
//! holds beyond its title and date, and what a post holds that its file
//! cannot.

use khalid_core::{AbsoluteUrl, Config};

use crate::Place;
use crate::front_matter::{FrontMatter, Parts, tag_list};
use crate::ghost::post_file::Source;
use crate::ghost::{Author, Held, Post, PostStatus, ShareCard, Visibility};
use crate::slug::post_slug;

/// An author's slug in Khalid: Ghost's own, kept when the site's rules
/// allow, else made from it or from the name.
pub(crate) fn author_slug(author: &Author, site: &Config) -> Option<String> {
    let locale = site.default_language();
    let made = post_slug(&author.slug, &author.name, &author.id, locale, site.slugs()).ok()?;
    Some(made.slug.as_str().to_owned())
}

/// The slugs of the authors of `post` whose ids the export knows, the main
/// one first.
pub(crate) fn author_slugs(post: &Post, from: Source<'_>) -> Vec<String> {
    post.authors
        .iter()
        .filter_map(|id| from.export.authors.iter().find(|author| &author.id == id))
        .filter_map(|author| author_slug(author, from.site))
        .collect()
}

/// `text` as a full address, with Ghost's own address filled in.
pub(crate) fn address(text: Option<&String>, from: Source<'_>) -> Option<AbsoluteUrl> {
    AbsoluteUrl::parse(&text?.replace("__GHOST_URL__", from.origin)).ok()
}

/// The feature image of `post` and what it shows, when both are known: its
/// own alt text, else the post's title, as an imported lead image gets.
pub(crate) fn lead_image<'a>(post: &'a Post, from: Source<'_>) -> Option<(AbsoluteUrl, &'a str)> {
    let alt = post.seo.feature_image_alt.as_deref().unwrap_or(&post.title);
    if alt.trim().is_empty() {
        return None;
    }
    Some((address(post.feature_image.as_ref(), from)?, alt))
}

/// Writes what a post or draft holds beyond its title, date and author
/// into `front`: its search title, the authors after the main one, its
/// public tags by name, its lead image and its share cards.
pub(crate) fn post_fields(front: &mut FrontMatter, post: &Post, from: Source<'_>, more: &[String]) {
    let tags = post
        .tags
        .iter()
        .filter_map(|id| from.export.tags.iter().find(|tag| &tag.id == id))
        .filter(|tag| !tag.internal)
        .map(|tag| tag.name.clone());
    front
        .maybe("seo_title", search_title(post))
        .list("co_authors", more)
        .list("tags", &tag_list(tags));
    if let Some((src, alt)) = lead_image(post, from) {
        front.map(
            "image",
            &[
                ("src", Some(src.to_string())),
                ("alt", Some(alt.to_owned())),
            ],
        );
    }
    let (facebook, x) = cards(post, from);
    let parts = |card: ShareCard| -> Parts {
        [
            ("title", card.title),
            ("description", card.description),
            ("image", card.image.map(|image| image.to_string())),
        ]
    };
    front.groups(
        "share_cards",
        &[("x", parts(x)), ("facebook", parts(facebook))],
    );
}

/// The title for search results, when it differs from the headline.
fn search_title(post: &Post) -> Option<String> {
    post.seo
        .meta_title
        .clone()
        .filter(|title| *title != post.title)
}

/// The post's Open Graph card and its card on X.
fn cards(post: &Post, from: Source<'_>) -> (ShareCard, ShareCard) {
    let seo = &post.seo;
    let card = |title: &Option<String>, description: &Option<String>, image| ShareCard {
        title: title.clone(),
        description: description.clone(),
        image: address(image, from),
    };
    (
        card(&seo.og_title, &seo.og_description, seo.og_image.as_ref()),
        card(
            &seo.twitter_title,
            &seo.twitter_description,
            seo.twitter_image.as_ref(),
        ),
    )
}

/// What `post`, placed in `place`, holds that its file cannot, in a fixed
/// order. A page's front matter has no search title, tags, image or share
/// cards, so a page lists them here; a post or draft writes them, and
/// lists only a feature image no alt text could be found for.
pub(crate) fn held(post: &Post, from: Source<'_>, place: Place) -> Vec<Held> {
    let mut held = Vec::new();
    if place == Place::Page {
        held.extend(search_title(post).map(Held::SearchTitle));
        let tags: Vec<String> = post
            .tags
            .iter()
            .filter_map(|id| from.export.tags.iter().find(|tag| &tag.id == id))
            .filter(|tag| !tag.internal)
            .map(|tag| tag.slug.clone())
            .collect();
        if !tags.is_empty() {
            held.push(Held::Tags(tags));
        }
    }
    if place == Place::Page || lead_image(post, from).is_none() {
        held.extend(address(post.feature_image.as_ref(), from).map(Held::FeatureImage));
    }
    if place == Place::Page {
        let (open_graph, twitter) = cards(post, from);
        if open_graph != ShareCard::default() {
            held.push(Held::OpenGraph(open_graph));
        }
        if twitter != ShareCard::default() {
            held.push(Held::Twitter(twitter));
        }
    }
    if let (PostStatus::Scheduled, Some(at)) = (&post.status, post.published) {
        held.push(Held::Scheduled(at));
    }
    if place == Place::Draft && post.visibility != Visibility::Public {
        held.push(Held::MembersOnly(post.visibility.clone()));
    }
    if place == Place::Page {
        held.extend(address(post.canonical_url.as_ref(), from).map(Held::Canonical));
    }
    held
}
