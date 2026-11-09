//! Series (plan section 4.12): posts whose front matter names the same
//! `series` are read in order. Each gets "Part 2 of 5", the parts on
//! either side and the whole list, and each series a page at
//! `/series/<name>/` that lists its parts in order, an `ItemList` in its
//! JSON-LD.

use std::collections::BTreeMap;

use chrono::NaiveDate;
use khalid_api::resources::PostSeries;
use khalid_content::{Document, PostFront, SiteContent};
use khalid_core::{AbsoluteUrl, Config, Locale, Slug, UserMessage};
use khalid_render::{
    BodyHtml, Link, PageBody, PageKind, PostCard, RenderListing, RenderSeries, Renderer,
};
use khalid_seo::{ItemList, ListEntry, PageKind as SeoKind, SchemaBlock};

use crate::build_pages::{self, LanguagePages, Page};
use crate::{build_lists, build_message, build_seo, index};

/// One part of a series, before the series is put in order.
struct Part {
    /// Its place from the front matter; parts without one come after.
    order: Option<u32>,
    date: NaiveDate,
    card: PostCard,
}

/// The parts of one series, by the series' name.
type Series = BTreeMap<String, Vec<Part>>;

/// The series of each post of `content` that is part of one, as the
/// Content API shows it, by the post's path in the content index. A
/// series whose name makes no address is left out, as `khalid build`
/// refuses it.
pub(crate) fn by_path(config: &Config, content: &SiteContent) -> BTreeMap<String, PostSeries> {
    let of = |doc: &Document<PostFront>| {
        let series = doc.front.series.as_ref()?;
        let slug = Slug::from_title(&series.name, &doc.locale, config.slugs()).ok()?;
        let order = series.order.map(u32::from);
        Some((
            index::slashed(&doc.path)?,
            PostSeries::new(&series.name, slug, order),
        ))
    };
    content.posts.iter().filter_map(of).collect()
}

/// Gives each post of `languages` that `content` puts in a series its
/// place in it, and adds the page of each series, introduced in its
/// language by `renderer`'s strings.
///
/// # Errors
///
/// Returns what, why and fix when a series' name makes no address, or its
/// page's address cannot be made from the site's settings.
pub(crate) fn add(
    config: &Config,
    renderer: &Renderer,
    content: &SiteContent,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let mut all = Series::new();
        for listed in &language.listed {
            let doc = (content.posts.iter()).find(|doc| doc.path == listed.path);
            let Some(series) = doc.and_then(|doc| doc.front.series.as_ref()) else {
                continue;
            };
            all.entry(series.name.clone()).or_default().push(Part {
                order: series.order.map(u32::from),
                date: listed.card.date,
                card: listed.card.clone(),
            });
        }
        for (name, mut parts) in all {
            parts.sort_by_key(|part| (part.order.is_none(), part.order, part.date));
            let slug = Slug::from_title(&name, &locale, config.slugs())
                .map_err(|_| build_message::no_series_slug(&name))?;
            let rest = format!("series/{slug}/");
            let href = build_pages::path_of(config, &locale, &rest);
            let links: Vec<Link> = (parts.iter())
                .map(|part| Link {
                    name: part.card.title.clone(),
                    href: part.card.href.clone(),
                })
                .collect();
            fill(&mut language.pages, (&name, &href), &links);
            let intro = build_lists::text(renderer, &locale, "series_intro")?
                .replace("{total}", &parts.len().to_string());
            let cards = parts.into_iter().map(|part| part.card).collect();
            let page = series_page(config, (&locale, &rest), (name, intro), cards)?;
            language.pages.push(page);
        }
    }
    Ok(())
}

/// Gives each post page among `pages` that `links` names its place in the
/// series `name`, whose page is at `href`.
fn fill(pages: &mut [Page], (name, href): (&str, &str), links: &[Link]) {
    for page in pages.iter_mut() {
        let PageBody::Post(post) = &mut page.body else {
            continue;
        };
        let Some(at) = links.iter().position(|link| link.href == page.meta.path) else {
            continue;
        };
        post.series = Some(RenderSeries {
            name: name.to_owned(),
            href: href.to_owned(),
            part: at + 1,
            total: links.len(),
            previous: at
                .checked_sub(1)
                .and_then(|before| links.get(before))
                .cloned(),
            next: links.get(at + 1).cloned(),
            parts: links.to_vec(),
        });
    }
}

/// The page of the series `name` at `rest`, introduced by `intro`, listing
/// `cards` in order.
fn series_page(
    config: &Config,
    (locale, rest): (&Locale, &str),
    (name, intro): (String, String),
    cards: Vec<PostCard>,
) -> Result<Page, UserMessage> {
    let path = build_pages::path_of(config, locale, rest);
    let meta = build_pages::meta(config, name.clone(), intro.clone(), path, PageKind::Website)?;
    let mut seo = build_seo::page(SeoKind::Collection, locale, rest, &name)?;
    seo.excerpt.clone_from(&intro);
    seo.item_count = u32::try_from(cards.len()).ok();
    let origin = config.address().as_str().trim_end_matches('/');
    let entries = (cards.iter())
        .filter_map(|card| {
            let url = AbsoluteUrl::parse(&format!("{origin}{}", card.href)).ok()?;
            Some(ListEntry::new(&card.title, url))
        })
        .collect();
    seo.blocks
        .push(SchemaBlock::List(ItemList::new(&name, entries)));
    // The words come from the strings file, never from a writer, so they
    // need no sanitising beyond escaping the one paragraph.
    let intro = BodyHtml::from_sanitised(format!("<p>{}</p>", escape(&intro)));
    let body = PageBody::Collection(RenderListing {
        hub: None,
        numbered: true,
        title: name,
        intro: Some(intro),
        posts: cards,
        pagination: None,
        featured: None,
    });
    Ok(build_pages::page(locale, config, rest, meta, body, seo))
}

/// `text` with the characters HTML gives meaning to written as entities.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
