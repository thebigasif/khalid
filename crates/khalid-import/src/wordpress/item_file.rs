//! The content file of one `WordPress` post or page.

use std::path::PathBuf;

use chrono::NaiveDate;
use khalid_core::{AbsoluteUrl, Config, Slug};
use khalid_routes::{PostAddress, Routes};

use crate::Place;
use crate::PlannedFile;
use crate::front_matter::{FrontMatter, Parts};
use crate::markdown::body_markdown;
use crate::redirects::path_of;
use crate::slug::{PostSlug, post_slug};
use crate::wordpress::names::{author_slug, category_slug, decoded};
use crate::wordpress::{
    Export, Held, ImportedItem, Item, ItemError, ItemStatus, SeoMeta, ShareWords,
};
use khalid_content::MAX_SECONDARY_KEYWORDS;

/// What every item's file is made for: the export and the site.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Source<'a> {
    pub(crate) export: &'a Export,
    pub(crate) site: &'a Config,
    pub(crate) routes: &'a Routes,
}

/// One file as it is put together.
struct File<'a> {
    item: &'a Item,
    from: Source<'a>,
    slug: Slug,
    seo: SeoMeta,
    front: FrontMatter,
    held: Vec<Held>,
}

/// The file for `item` in `place`, and what became of it. Every file is
/// read back as a post first; a page writes only fields a page also has.
pub(crate) fn item_file(
    item: &Item,
    from: Source<'_>,
    place: Place,
) -> Result<(PlannedFile, ImportedItem), ItemError> {
    let site = from.site;
    let PostSlug { slug, origin } = post_slug(
        &item.name,
        &item.title,
        item.id,
        site.default_language(),
        site.slugs(),
    )?;
    let body = body_markdown(&item.content).map_err(ItemError::Body)?;
    let mut file = File {
        item,
        from,
        seo: SeoMeta::read(item, &from.export.site),
        front: FrontMatter::default(),
        held: Vec::new(),
        slug,
    };
    file.front
        .text("title", &decoded(&item.title))
        .text("slug", file.slug.as_str());
    let title = file
        .seo
        .title
        .as_deref()
        .map(|title| Held::SearchTitle(decoded(title)));
    file.held.extend(title);
    let (path, address) = match place {
        Place::Page => file.page(),
        Place::Post | Place::Draft => file.post(place)?,
    };
    let File {
        slug,
        seo,
        front,
        mut held,
        ..
    } = file;
    if !seo.focus_keywords.is_empty() {
        held.push(Held::FocusKeywords(seo.focus_keywords));
    }
    if !body.shortcodes.is_empty() {
        held.push(Held::Shortcodes(body.shortcodes));
    }
    held.extend(seo.unread.into_iter().map(Held::Unread));
    if let (ItemStatus::Future, Some(at)) = (&item.status, item.published) {
        held.push(Held::Scheduled(at));
    }
    let path = PathBuf::from(path);
    let text = front.file(&body.markdown);
    khalid_content::parse_post(&path, &text).map_err(ItemError::Content)?;
    let imported = ImportedItem {
        id: item.id,
        path: path.clone(),
        address,
        slug,
        slug_origin: origin,
        held,
        place,
    };
    Ok((PlannedFile { path, text }, imported))
}

impl File<'_> {
    /// The search description, or else the hand-written summary.
    fn description(&self) -> Option<String> {
        let excerpt = Some(self.item.excerpt.as_str()).filter(|text| !text.is_empty());
        self.seo.description.as_deref().or(excerpt).map(decoded)
    }

    /// A page's fields, its file and its address.
    fn page(&mut self) -> (String, Option<String>) {
        let (slug, site) = (&self.slug, self.from.site);
        self.front.maybe("description", self.description());
        self.held.extend(self.seo.robots.clone().map(Held::Robots));
        self.held
            .extend(self.seo.canonical.clone().map(Held::Canonical));
        let path = format!("content/{}/pages/{slug}.md", site.default_language());
        (path, Some(format!("{}{slug}/", site.mount())))
    }

    /// A post's or a draft's fields, its file and, once published, its
    /// address.
    fn post(&mut self, place: Place) -> Result<(String, Option<String>), ItemError> {
        let (
            item,
            Source {
                export,
                site,
                routes,
            },
        ) = (self.item, self.from);
        let locale = site.default_language();
        let date = item.published.map(|at| at.date());
        let address = match (place, date) {
            (Place::Post, Some(date)) => {
                let post = PostAddress::new("posts", locale, &self.slug, Some(date));
                self.front.text("date", &date.to_string());
                Some(routes.address(site, &post).map_err(ItemError::Address)?)
            }
            (Place::Post, None) => return Err(ItemError::NoDate),
            _ => None,
        };
        let (category, more) = categories(item, export, self.seo.primary_category, site);
        let group = category.as_ref().unwrap_or(&self.slug).to_string();
        let description = self.description();
        self.front
            .maybe("author", author(item, export, site))
            .maybe("category", category)
            .maybe("description", description);
        let live = date.filter(|_| address.is_some());
        if let Some(alias) = live.and_then(|date| self.alias(date)) {
            self.front.list("aliases", &[alias]);
        }
        if item.sticky {
            self.front.yes("featured");
        }
        self.front
            .maybe("robots", self.seo.robots.as_ref())
            .maybe("canonical", self.seo.canonical.as_ref());
        self.social(&group);
        post_held(item, export, more, &mut self.held);
        let path = match (address.is_some(), date) {
            (true, Some(date)) => format!("content/{locale}/posts/{date}-{}.md", self.slug),
            _ => format!("drafts/{locale}/{}.md", self.slug),
        };
        Ok((path, address))
    }

    /// The keywords, share cards and pillar mark the plugins kept, as post
    /// front matter holds them. A cornerstone post becomes the main post of
    /// the topic `group`, its category's slug or else its own. Keywords
    /// beyond the four secondary ones a post holds stay in the report.
    fn social(&mut self, group: &str) {
        let mut keywords = std::mem::take(&mut self.seo.focus_keywords).into_iter();
        let focus = keywords.next();
        let mut secondary: Vec<String> = Vec::new();
        for word in keywords.chain(self.seo.related_keywords.iter().cloned()) {
            if Some(&word) != focus.as_ref() && !secondary.contains(&word) {
                secondary.push(word);
            }
        }
        let beyond = secondary.split_off(secondary.len().min(MAX_SECONDARY_KEYWORDS));
        if !beyond.is_empty() {
            self.held.push(Held::FocusKeywords(beyond));
        }
        self.front
            .maybe("focus_keyword", focus)
            .list("secondary_keywords", &secondary);
        let card = |card: &ShareWords| -> Parts {
            [
                ("title", card.title.as_deref().map(decoded)),
                ("description", card.description.as_deref().map(decoded)),
                ("image", card.image.clone()),
            ]
        };
        let (x, facebook) = (card(&self.seo.x), card(&self.seo.facebook));
        self.front
            .groups("share_cards", &[("x", x), ("facebook", facebook)]);
        if self.seo.pillar {
            self.front.text("topic_group", group).yes("main_post");
        }
    }

    /// The post's old path as an alias, when the old site and the blog share
    /// a host, the path lies inside the blog and the address changed.
    fn alias(&self, date: NaiveDate) -> Option<String> {
        let Source { site, routes, .. } = self.from;
        let old = self
            .item
            .link
            .as_ref()
            .filter(|old| old.host() == site.address().host())?;
        let post = PostAddress::new("posts", site.default_language(), &self.slug, Some(date));
        let redirect = routes.redirect(site, path_of(old), &post).ok()?;
        Some(redirect.from().to_owned())
    }
}

/// What a post had that its front matter cannot hold: tags, its feature
/// image and categories beyond the first.
fn post_held(item: &Item, export: &Export, more: Vec<String>, held: &mut Vec<Held>) {
    if !item.tags.is_empty() {
        held.push(Held::Tags(item.tags.clone()));
    }
    held.extend(feature_image(item, export).map(Held::FeatureImage));
    if !more.is_empty() {
        held.push(Held::MoreCategories(more));
    }
}

/// The author's slug, the same as their own file's.
fn author(item: &Item, export: &Export, site: &Config) -> Option<Slug> {
    match export
        .authors
        .iter()
        .find(|author| author.login == item.creator)
    {
        Some(author) => author_slug(author, site),
        None => Slug::from_title(&item.creator, site.default_language(), site.slugs()).ok(),
    }
}

/// The category front matter holds, the primary one when a plugin named it,
/// and the slugs of the rest.
fn categories(
    item: &Item,
    export: &Export,
    primary: Option<u64>,
    site: &Config,
) -> (Option<Slug>, Vec<String>) {
    let primary = export
        .categories
        .iter()
        .find(|term| term.id.is_some() && term.id == primary)
        .map(|term| term.slug.as_str())
        .filter(|slug| item.categories.iter().any(|own| own == slug));
    let main = primary.or_else(|| item.categories.first().map(String::as_str));
    let more = item
        .categories
        .iter()
        .filter(|slug| Some(slug.as_str()) != main)
        .cloned()
        .collect();
    (
        main.and_then(|slug| category_slug(slug, export, site)),
        more,
    )
}

/// The address of the image a post's `_thumbnail_id` names.
fn feature_image(item: &Item, export: &Export) -> Option<AbsoluteUrl> {
    let id = item.meta("_thumbnail_id")?.parse().ok()?;
    export.item(id)?.attachment_url.clone()
}
