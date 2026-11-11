//! What the Content API answers with on a running site: the published posts
//! of the content index, the plain pages and the site's lists, held in
//! memory without the posts' bodies. A request renders the bodies of the
//! posts it returns from their files, so the store is small and quick to
//! build.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use khalid_api::content_store::{
    BoxFuture, ContentStore, ContentStoreError, Listed, MemoryContentStore, PostKey, SiteResources,
};
use khalid_api::query::ListQuery;
use khalid_api::resources::{Bodies, Collection, Post, PostSeries};
use khalid_content::{SiteContent, Wikilinks, parse_post, render_linked};
use khalid_core::{AbsoluteUrl, Config, Locale, SitePath, SlugMode, UserMessage};
use khalid_db::{ContentIndex, IndexedPost, PostChanges};
use khalid_routes::Routes;
use tokio::sync::watch;

use crate::entry_events::Entries;
use crate::listed_post::ListedTag;
use crate::post_heads::PostHeads;
use crate::post_lock::Lock;
use crate::tag_events::Tags;
use crate::{build_message, build_pages, index, routes_message, site_lists};

#[path = "content_page.rs"]
mod content_page;
use content_page::ContentPage;

/// What one render of the pages read for the Content API, without a single
/// post body: the content index, which posts are featured, and the lists.
#[derive(Debug, Clone, Default)]
pub(crate) struct Catalog {
    /// The rows a rebuild writes to `data/site.db`.
    pub(crate) index: ContentIndex,
    /// The paths of the posts marked `featured`.
    featured: BTreeSet<String>,
    /// The `updated:` date of each post that has one, by path, so the
    /// Content API says when a post last really changed and a new one can
    /// be sent to `IndexNow`.
    pub(crate) updated: BTreeMap<String, NaiveDate>,
    /// The series of each post that is part of one, by path.
    series: BTreeMap<String, PostSeries>,
    /// The slugs of the tags each post names, in its order, by path.
    post_tags: BTreeMap<String, Vec<String>>,
    /// The plain pages, such as About.
    pages: Vec<ContentPage>,
    lists: SiteResources,
    /// The pages, glossary terms, case studies and changelog entries.
    pub(crate) entries: Entries,
    /// The tags the posts name.
    pub(crate) tags: Tags,
    /// The head of each post page, by address, which the render that read
    /// this catalog made.
    pub(crate) heads: PostHeads,
    /// The pages wikilinks reach and the snippets, in each language, as
    /// the build renders bodies with them. Cloning shares them.
    links: Arc<BTreeMap<Locale, Wikilinks>>,
}

impl Catalog {
    /// The catalog of `content`, already read from the site `folder`.
    ///
    /// # Errors
    ///
    /// Returns what, why and fix when a post cannot be indexed or an
    /// address cannot be made from the site's settings.
    pub(crate) fn of(
        folder: &Path,
        config: &Config,
        content: &SiteContent,
    ) -> Result<Self, UserMessage> {
        let index = index::of_content(folder, config, content)?;
        let featured = content
            .posts
            .iter()
            .filter(|doc| doc.front.featured)
            .filter_map(|doc| index::slashed(&doc.path))
            .collect();
        let updated = content
            .posts
            .iter()
            .filter_map(|doc| Some((index::slashed(&doc.path)?, doc.front.last_updated()?)))
            .collect();
        let series = build_pages::series::by_path(config, content);
        let post_tags = content
            .posts
            .iter()
            .filter_map(|doc| {
                let tags = ListedTag::all(&doc.front.tags, &doc.locale, config.slugs());
                let slugs = tags.into_iter().map(|tag| tag.slug.to_string()).collect();
                Some((index::slashed(&doc.path)?, slugs))
            })
            .collect();
        let pages = ContentPage::all(folder, config, content)?;
        let lists = site_lists::of(config, &index, content)?;
        let routes = Routes::load(folder).map_err(|error| routes_message::of(&error))?;
        let links = (config.languages().iter())
            .map(|locale| {
                let links = Wikilinks::for_language(content, config, &routes, locale);
                (locale.clone(), links)
            })
            .collect();
        Ok(Self {
            index,
            featured,
            updated,
            series,
            post_tags,
            pages,
            lists,
            entries: Entries::of(content).with_drafts(folder),
            tags: Tags::of(config, content),
            heads: PostHeads::new(),
            links: Arc::new(links),
        })
    }
}

/// The published content readers see now. Cloning shares it. Until the
/// first store is ready a request waits for it, and a refresh swaps the
/// store whole, so a request never sees half a rebuild.
#[derive(Debug, Clone)]
pub(crate) struct LiveContent(Arc<watch::Sender<State>>);

#[derive(Debug, Clone)]
enum State {
    Loading,
    Ready(Arc<Published>),
    Failed,
}

impl Default for LiveContent {
    fn default() -> Self {
        Self(Arc::new(watch::channel(State::Loading).0))
    }
}

impl LiveContent {
    /// Serves `published` until it is replaced.
    pub(crate) fn replace(&self, published: impl Into<Arc<Published>>) {
        self.0.send_replace(State::Ready(published.into()));
    }

    /// Answers waiting requests with an error when no store was ever ready.
    /// A store already served stays.
    pub(crate) fn fail(&self) {
        self.0.send_if_modified(|state| {
            let loading = matches!(state, State::Loading);
            if loading {
                *state = State::Failed;
            }
            loading
        });
    }

    async fn current(&self) -> Result<Arc<Published>, ContentStoreError> {
        let mut states = self.0.subscribe();
        let state = states
            .wait_for(|state| !matches!(state, State::Loading))
            .await
            .map_err(|_| ContentStoreError::new(NotLoaded))?;
        match &*state {
            State::Ready(published) => Ok(Arc::clone(published)),
            State::Loading | State::Failed => Err(ContentStoreError::new(NotLoaded)),
        }
    }
}

impl ContentStore for LiveContent {
    fn list<'a>(
        &'a self,
        collection: Collection,
        query: &'a ListQuery,
    ) -> BoxFuture<'a, Result<Listed<Post>, ContentStoreError>> {
        Box::pin(async move {
            let published = self.current().await?;
            let listed = published.store.list(collection, query).await?;
            if query.formats().is_empty() {
                return Ok(listed);
            }
            let total = listed.total();
            let posts = with_bodies(published, collection, listed.into_items()).await?;
            Ok(Listed::new(posts, total))
        })
    }

    fn find<'a>(
        &'a self,
        collection: Collection,
        key: &'a PostKey,
    ) -> BoxFuture<'a, Result<Option<Post>, ContentStoreError>> {
        Box::pin(async move {
            let published = self.current().await?;
            let Some(post) = published.store.find(collection, key).await? else {
                return Ok(None);
            };
            Ok(with_bodies(published, collection, vec![post]).await?.pop())
        })
    }

    fn site(&self) -> BoxFuture<'_, Result<SiteResources, ContentStoreError>> {
        Box::pin(async move { self.current().await?.store.site().await })
    }
}

/// The site's published content could not be read from its files, so the
/// Content API has nothing to answer with until the next save.
#[derive(Debug)]
struct NotLoaded;

impl fmt::Display for NotLoaded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the published posts could not be read from the site's files")
    }
}

impl std::error::Error for NotLoaded {}

/// One store of published posts, without bodies, and where each body is.
#[derive(Debug)]
pub(crate) struct Published {
    store: MemoryContentStore,
    /// Each published post, by the path of its file in the index.
    posts: BTreeMap<String, Listing>,
    /// The path of each post's file, by the post's identity.
    files: BTreeMap<String, String>,
    /// Each plain page, by its identity.
    pages: BTreeMap<String, ContentPage>,
    folder: PathBuf,
    slugs: SlugMode,
    links: Arc<BTreeMap<Locale, Wikilinks>>,
}

/// One published post, and the language and day that order it.
#[derive(Debug, Clone)]
struct Listing {
    post: Post,
    locale: Locale,
    date: NaiveDate,
}

impl Published {
    /// The published posts of `catalog`, read from the site `folder` with
    /// its `config`, in every language. A post is published when it has a
    /// date, as on the public site.
    ///
    /// # Errors
    ///
    /// Returns what, why and fix when an address cannot be made from the
    /// site's settings.
    pub(crate) fn of(
        folder: &Path,
        config: &Config,
        catalog: &Catalog,
    ) -> Result<Self, UserMessage> {
        let posts = listings(config, catalog, catalog.index.posts.iter())?;
        Ok(Self::assemble(
            folder,
            config,
            posts.into_iter().collect(),
            catalog,
        ))
    }

    /// This store after a save: the posts `changes` names are made again
    /// from `catalog` or removed, every other post is kept as it is, and
    /// the lists come from `catalog`.
    ///
    /// # Errors
    ///
    /// Returns what, why and fix when an address cannot be made from the
    /// site's settings.
    pub(crate) fn changed(
        &self,
        config: &Config,
        catalog: &Catalog,
        changes: &PostChanges,
    ) -> Result<Self, UserMessage> {
        let mut posts = self.posts.clone();
        for path in changes.written.iter().chain(&changes.removed) {
            posts.remove(path);
        }
        let written = catalog
            .index
            .posts
            .iter()
            .filter(|row| changes.written.contains(&row.path));
        posts.extend(listings(config, catalog, written)?);
        Ok(Self::assemble(&self.folder, config, posts, catalog))
    }

    /// The store of `posts`, in each language of `config` newest first, and
    /// posts of one day in slug order, as the index lists them.
    fn assemble(
        folder: &Path,
        config: &Config,
        posts: BTreeMap<String, Listing>,
        catalog: &Catalog,
    ) -> Self {
        let mut entries = Vec::with_capacity(posts.len());
        let mut files = BTreeMap::new();
        for locale in config.languages() {
            let mut rows: Vec<(&String, &Listing)> = posts
                .iter()
                .filter(|(_, listing)| &listing.locale == locale)
                .collect();
            rows.sort_by(|(_, a), (_, b)| {
                let (a_slug, b_slug) = (a.post.slug().as_str(), b.post.slug().as_str());
                b.date.cmp(&a.date).then_with(|| a_slug.cmp(b_slug))
            });
            for (path, listing) in rows {
                let id = listing.post.id().to_owned();
                files.entry(id).or_insert_with(|| path.clone());
                // Every post takes its head from this render, since a new
                // translation changes the heads of posts left as they were.
                let head = catalog.heads.get(listing.post.url().as_str());
                entries.push((Collection::Posts, listing.post.clone().with_head(head)));
            }
        }
        let mut pages = BTreeMap::new();
        for page in &catalog.pages {
            entries.push((Collection::Pages, page.post.clone()));
            pages.insert(page.post.id().to_owned(), page.clone());
        }
        Self {
            store: MemoryContentStore::new(entries).with_site(catalog.lists.clone()),
            posts,
            files,
            pages,
            folder: folder.to_path_buf(),
            slugs: config.slugs(),
            links: Arc::clone(&catalog.links),
        }
    }

    /// The body of the entry of `collection` with identity `id` in every
    /// format, rendered now: a post's read from its file, a page's from the
    /// markdown the store keeps.
    fn bodies(&self, collection: Collection, id: &str) -> Result<Bodies, ContentStoreError> {
        if collection == Collection::Pages {
            let Some(page) = self.pages.get(id) else {
                return Ok(Bodies::default());
            };
            return self.rendered(&page.markdown, &page.locale);
        }
        let Some((path, listing)) = self
            .files
            .get(id)
            .and_then(|path| Some((path, self.posts.get(path)?)))
        else {
            return Ok(Bodies::default());
        };
        let locale = &listing.locale;
        let file = SitePath::within(&self.folder, path).map_err(ContentStoreError::new)?;
        let text = std::fs::read_to_string(file.as_path()).map_err(ContentStoreError::new)?;
        let post = parse_post(file.relative(), &text).map_err(ContentStoreError::new)?;
        // A locked post's body stays on the server, as on its page.
        if Lock::of(&post.front).is_some() {
            return Ok(Bodies::default());
        }
        self.rendered(&post.body, locale)
    }

    /// The markdown `body` in `locale` in every format, with its snippets
    /// placed and its wikilinks linked as on its page.
    fn rendered(&self, body: &str, locale: &Locale) -> Result<Bodies, ContentStoreError> {
        let none = Wikilinks::new();
        let links = self.links.get(locale).unwrap_or(&none);
        let rendered =
            render_linked(body, locale, self.slugs, links).map_err(ContentStoreError::new)?;
        Ok(Bodies::new(
            &rendered.html,
            body,
            &plaintext(&rendered.html),
        ))
    }
}

/// `posts` of `collection` with their bodies, rendered on a blocking
/// thread, since each one reads a file (RULES.md BL-RUN-7).
async fn with_bodies(
    published: Arc<Published>,
    collection: Collection,
    posts: Vec<Post>,
) -> Result<Vec<Post>, ContentStoreError> {
    let work = move || {
        posts
            .into_iter()
            .map(|post| {
                let bodies = published.bodies(collection, post.id())?;
                Ok(post.with_bodies(bodies))
            })
            .collect()
    };
    tokio::task::spawn_blocking(work)
        .await
        .map_err(ContentStoreError::new)?
}

/// The published posts among `rows` of the index of `catalog`, by path.
fn listings<'a>(
    config: &Config,
    catalog: &Catalog,
    rows: impl Iterator<Item = &'a IndexedPost>,
) -> Result<Vec<(String, Listing)>, UserMessage> {
    let mut found = Vec::new();
    for row in rows {
        let Some(date) = row.date else {
            continue;
        };
        let mut post = post_of(config, row, date)?;
        if catalog.featured.contains(&row.path) {
            post = post.featured();
        }
        if let Some(day) = catalog.updated.get(&row.path).filter(|day| **day > date) {
            post = post.with_updated_at(midnight(*day));
        }
        post = post.with_series(catalog.series.get(&row.path).cloned());
        if let Some(tags) = catalog.post_tags.get(&row.path) {
            post = post.with_tags(tags.clone());
        }
        let locale = row.locale.clone();
        found.push((row.path.clone(), Listing { post, locale, date }));
    }
    Ok(found)
}

/// The index `row` of a post published on `date`, in API shape, without
/// its body.
fn post_of(config: &Config, row: &IndexedPost, date: NaiveDate) -> Result<Post, UserMessage> {
    let locale = &row.locale;
    let path = build_pages::path_of(config, locale, &format!("{}/", row.slug));
    let address = format!("{}{path}", config.address().as_str().trim_end_matches('/'));
    let url = AbsoluteUrl::parse(&address).map_err(|_| build_message::address(&address))?;
    let at = midnight(date);
    let id = format!("{}.{}", locale.as_str(), row.slug);
    let title = row.title.clone().unwrap_or_else(|| row.slug.to_string());
    let mut post = Post::new(&id, row.slug.clone(), &title, url, locale.clone(), at)
        .with_authors(row.author.iter().map(ToString::to_string).collect());
    if let Some(excerpt) = &row.description {
        post = post.with_excerpt(excerpt);
    }
    Ok(post)
}

/// The first second of `day`, in UTC.
fn midnight(day: NaiveDate) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(day.and_time(NaiveTime::MIN), Utc)
}

/// `html` with its tags removed, the common entities read back, and runs
/// of white space made one space.
fn plaintext(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    let text = text
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
#[path = "site_content_tests.rs"]
mod tests;
