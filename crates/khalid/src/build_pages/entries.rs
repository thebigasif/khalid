//! The case studies and the changelog of each language (plan section
//! 5.13): a page per entry at the address `routes.toml` gives its
//! collection, and a list of each collection at `/case-studies/` and
//! `/changelog/`, the changelog grouped by month.

use std::cmp::Reverse;

use chrono::{Datelike, NaiveDate};
use khalid_content::{CaseStudyFront, ChangelogFront, Document, SiteContent, Wikilinks};
use khalid_core::{Config, Locale, UserMessage};
use khalid_render::{
    BodyHtml, EntryCard, EntryGroup, EntryKind, Link, PageBody, PageKind, RenderEntries,
    RenderEntry, Renderer,
};
use khalid_routes::{PostAddress, Routes};
use khalid_seo::{ItemList, ListEntry, PageKind as SeoKind, SchemaBlock};

use crate::build_pages::{self, LanguagePages, Page};
use crate::{build_lists, build_seo, definition_text};

/// One entry, read and rendered, before its list is made.
struct Made {
    collection: String,
    card: EntryCard,
    page: Page,
}

/// What every entry of one language is made with.
struct Language<'a> {
    config: &'a Config,
    routes: &'a Routes,
    links: Wikilinks,
    locale: &'a Locale,
}

/// Adds, to each of `languages`, the page of each case study and
/// changelog entry of `content` in its language and the list of each
/// collection that has one, titled by `renderer`'s strings or the
/// collection's `_index.md`.
///
/// # Errors
///
/// Returns what, why and fix when an entry has no title to make its
/// address from, an address cannot be made, or a body cannot be rendered.
pub(crate) fn add(
    (config, routes): (&Config, &Routes),
    renderer: &Renderer,
    content: &SiteContent,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let mine = |doc_locale: &Locale| doc_locale == &locale;
        let studies: Vec<_> = (content.case_studies.iter())
            .filter(|d| mine(&d.locale))
            .collect();
        let changes: Vec<_> = (content.changelog.iter())
            .filter(|d| mine(&d.locale))
            .collect();
        if studies.is_empty() && changes.is_empty() {
            continue;
        }
        let links = Wikilinks::for_language(content, config, routes, &locale);
        let made = Language {
            config,
            routes,
            links,
            locale: &locale,
        };
        let studies: Vec<Made> =
            (studies.iter().map(|doc| made.study(doc))).collect::<Result<_, _>>()?;
        let changes: Vec<Made> =
            (changes.iter().map(|doc| made.change(doc))).collect::<Result<_, _>>()?;
        let lists = [
            (EntryKind::CaseStudies, studies),
            (EntryKind::Changelog, changes),
        ];
        for (kind, entries) in lists {
            if let Some(list) = made.list(renderer, content, kind, entries, &mut language.pages)? {
                language.pages.push(list);
            }
        }
    }
    Ok(())
}

impl Language<'_> {
    fn study(&self, doc: &Document<CaseStudyFront>) -> Result<Made, UserMessage> {
        let front = &doc.front;
        let title = front.title.as_deref();
        let (title, slug) = build_pages::named(self.config, doc, front.slug.as_ref(), title)?;
        let body = build_pages::body_html(self.config, &self.links, doc)?;
        let summary = (front.description.clone())
            .filter(|text| !text.trim().is_empty())
            .unwrap_or_else(|| definition_text::summary(body.as_str()));
        let entry = PostAddress::new(&doc.collection, self.locale, &slug, front.date);
        let rest = build_pages::routed(self.config, self.routes, self.locale, &entry, &doc.path)?;
        let card = EntryCard {
            title,
            href: build_pages::path_of(self.config, self.locale, &rest),
            date: front.date,
            summary,
            client: front.client.clone(),
            outcome: front.outcome.clone(),
            version: None,
            body: None,
        };
        let page = self.page(doc, (&rest, &card), EntryKind::CaseStudies, body)?;
        Ok(Made {
            collection: doc.collection.clone(),
            card,
            page,
        })
    }

    fn change(&self, doc: &Document<ChangelogFront>) -> Result<Made, UserMessage> {
        let front = &doc.front;
        let title = front.title.as_deref().or(front.version.as_deref());
        let (title, slug) = build_pages::named(self.config, doc, None, title)?;
        let body = build_pages::body_html(self.config, &self.links, doc)?;
        let entry = PostAddress::new(&doc.collection, self.locale, &slug, front.date);
        let rest = build_pages::routed(self.config, self.routes, self.locale, &entry, &doc.path)?;
        let card = EntryCard {
            title,
            href: build_pages::path_of(self.config, self.locale, &rest),
            date: front.date,
            summary: definition_text::summary(body.as_str()),
            client: None,
            outcome: None,
            version: front.version.clone(),
            body: Some(body.clone()),
        };
        let page = self.page(doc, (&rest, &card), EntryKind::Changelog, body)?;
        Ok(Made {
            collection: doc.collection.clone(),
            card,
            page,
        })
    }

    /// The page of one entry at `rest`, shown as `card` shows it, with its
    /// whole `body`.
    fn page<F>(
        &self,
        doc: &Document<F>,
        (rest, card): (&str, &EntryCard),
        kind: EntryKind,
        body: BodyHtml,
    ) -> Result<Page, UserMessage> {
        let (config, locale) = (self.config, self.locale);
        let mut seo = build_seo::page(SeoKind::Page, locale, rest, &card.title)?;
        seo.excerpt.clone_from(&card.summary);
        seo.published = card.date.map(build_seo::midnight);
        let back = format!("{}/", doc.collection);
        let meta = build_pages::meta(
            config,
            card.title.clone(),
            card.summary.clone(),
            card.href.clone(),
            PageKind::Article,
        )?;
        let entry = RenderEntry {
            title: card.title.clone(),
            kind,
            date: card.date,
            client: card.client.clone(),
            outcome: card.outcome.clone(),
            version: card.version.clone(),
            back: Link {
                name: doc.collection.clone(),
                href: build_pages::path_of(config, locale, &back),
            },
            body,
        };
        let body = PageBody::Entry(Box::new(entry));
        Ok(build_pages::page(locale, config, rest, meta, body, seo).made_from(&doc.path))
    }

    /// The list of `entries`, newest first, after their pages are added to
    /// `pages`. `None` when there is no entry.
    fn list(
        &self,
        renderer: &Renderer,
        content: &SiteContent,
        kind: EntryKind,
        mut entries: Vec<Made>,
        pages: &mut Vec<Page>,
    ) -> Result<Option<Page>, UserMessage> {
        let (config, locale) = (self.config, self.locale);
        let Some(collection) = entries.first().map(|made| made.collection.clone()) else {
            return Ok(None);
        };
        entries.sort_by_key(|made| Reverse(made.card.date));
        let index = (content.indexes.iter())
            .find(|doc| &doc.locale == locale && doc.collection == collection);
        let front = index.map(|doc| &doc.front);
        let title = match front.and_then(|front| front.title.clone()) {
            Some(title) => title,
            None => build_lists::text(renderer, locale, title_key(kind))?,
        };
        let description = (front.and_then(|front| front.description.clone())).unwrap_or_default();
        let intro = match index.filter(|doc| !doc.body.trim().is_empty()) {
            Some(doc) => Some(build_pages::body_html(config, &self.links, doc)?),
            None => None,
        };
        let rest = format!("{collection}/");
        let path = build_pages::path_of(config, locale, &rest);
        let meta = build_pages::meta(
            config,
            title.clone(),
            description.clone(),
            path,
            PageKind::Website,
        )?;
        let mut seo = build_seo::page(SeoKind::Collection, locale, &rest, &title)?;
        seo.excerpt = description;
        seo.translation = build_lists::list_key(&collection)?;
        seo.item_count = u32::try_from(entries.len()).ok();
        let items = (entries.iter())
            .map(|made| ListEntry::new(&made.card.title, made.page.meta.canonical.clone()));
        seo.blocks
            .push(SchemaBlock::List(ItemList::new(&title, items.collect())));
        let cards: Vec<EntryCard> = entries.iter().map(|made| made.card.clone()).collect();
        pages.extend(entries.into_iter().map(|made| made.page));
        let groups = match kind {
            EntryKind::Changelog => by_month(cards),
            _ => vec![EntryGroup {
                year: None,
                month: None,
                entries: cards,
            }],
        };
        let body = PageBody::Entries(RenderEntries {
            title,
            intro,
            kind,
            groups,
        });
        Ok(Some(build_pages::page(
            locale, config, &rest, meta, body, seo,
        )))
    }
}

/// The strings key of a list's title.
fn title_key(kind: EntryKind) -> &'static str {
    match kind {
        EntryKind::Changelog => "changelog_title",
        _ => "case_studies_title",
    }
}

/// `cards`, newest first, in one group per month. Undated ones come last,
/// under no month.
fn by_month(cards: Vec<EntryCard>) -> Vec<EntryGroup> {
    let month = |date: Option<NaiveDate>| date.map(|date| (date.year(), date.month()));
    let mut groups: Vec<EntryGroup> = Vec::new();
    for card in cards {
        let (year, month) = month(card.date).unzip();
        match groups.last_mut() {
            Some(group) if group.year == year && group.month == month => group.entries.push(card),
            _ => groups.push(EntryGroup {
                year,
                month,
                entries: vec![card],
            }),
        }
    }
    groups
}
