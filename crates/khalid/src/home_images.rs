//! The lead images and author photos on the cards of the home pages that
//! `khalid serve` answers before the full render: drawn here from the
//! front matter, as the full render draws them, so the first answer is
//! the page the full render makes.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use khalid_content::{Document, LeadImage, SiteContent};
use khalid_core::{Config, UserMessage};
use khalid_render::{PageBody, PostCard};
use khalid_routes::Routes;

use crate::build_pages::{self, LanguagePages};
use crate::post_files::{self, PostFile};
use crate::post_images;
use crate::post_photos::{self, Shown};

/// Puts on the cards of the home pages in `languages` the lead image of
/// each post and the photo of each author they show, from `content` in
/// the `site` folder.
///
/// # Errors
///
/// Returns a message when an image cannot be read or made into sizes, or
/// a post's address cannot be made.
pub(crate) fn attach(
    site: &Path,
    (config, routes): (&Config, &Routes),
    content: &SiteContent,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    let wanted = shown_on(languages);
    let mut shown = Shown::default();
    for doc in &content.posts {
        let Some(image) = &doc.front.image else {
            continue;
        };
        let Some((_, rest, _)) = crate::build_home::dated((config, routes), doc)? else {
            continue;
        };
        let address = build_pages::path_of(config, &doc.locale, &rest);
        if !wanted.contains(&address) {
            continue;
        }
        if let Some(drawn) = drawn(site, config, (doc, &address), image)? {
            shown.card(address, drawn);
        }
    }
    for doc in &content.authors {
        let front = &doc.front;
        let (name, slug) =
            build_pages::named(config, doc, front.slug.as_ref(), front.name.as_deref())?;
        let address = build_pages::path_of(config, &doc.locale, &format!("authors/{slug}/"));
        let photo = front.photo.as_deref();
        let Some(image) = photo.and_then(|src| LeadImage::try_new(src, &name)) else {
            continue;
        };
        if !wanted.contains(&address) {
            continue;
        }
        if let Some(drawn) = drawn(site, config, (doc, &address), &image)? {
            shown.photo(address, drawn);
        }
    }
    post_photos::attach(languages, &shown);
    Ok(())
}

/// The addresses of every post and author the home pages in `languages`
/// show on a card.
fn shown_on(languages: &[LanguagePages]) -> BTreeSet<String> {
    let homes = languages
        .iter()
        .flat_map(|language| &language.pages)
        .filter_map(|page| match &page.body {
            PageBody::Home(home) => Some(home),
            _ => None,
        });
    let mut cards: Vec<&PostCard> = Vec::new();
    for home in homes {
        cards.extend(&home.listing.posts);
        cards.extend(&home.listing.featured);
        cards.extend(&home.picks);
        cards.extend(home.sections.iter().flat_map(|section| &section.posts));
    }
    let authors = cards.iter().filter_map(|card| card.author.as_ref());
    let people = authors.map(|person| person.href.clone());
    cards
        .iter()
        .map(|card| card.href.clone())
        .chain(people)
        .collect()
}

/// The lead image `image` of `doc`, whose page is at `address`, drawn as
/// the full render draws it: only when it is a file beside a folder
/// entry, as `post_files` publishes those alone.
fn drawn<F>(
    site: &Path,
    config: &Config,
    (doc, address): (&Document<F>, &str),
    image: &LeadImage,
) -> Result<Option<khalid_render::RenderImage>, UserMessage> {
    let folder = doc.path.parent().filter(|_| doc.path.ends_with("index.md"));
    let name = post_images::lead_name(image);
    let Some(folder) = folder.filter(|_| !name.starts_with('/')) else {
        return Ok(None);
    };
    let mut source = PathBuf::from(folder);
    source.push(name);
    let page = address.strip_prefix(config.mount()).unwrap_or(address);
    let file = PostFile {
        source,
        file: format!("{page}{name}"),
        address: format!("{address}{name}"),
    };
    let beside = post_files::on_disk(site, &file).is_ok_and(|path| path.is_file());
    if !beside {
        return Ok(None);
    }
    let picture = post_images::picture_of(site, config, &file)?;
    Ok(picture.map(|(picture, _)| post_images::lead_image(&picture, image)))
}
