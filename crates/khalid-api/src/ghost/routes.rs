//! The routes under `/ghost/api/content/`: browse, read by id and read by
//! slug for posts, pages, tags and authors, and the settings.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use khalid_core::Slug;
use serde::Serialize;
use serde_json::{Map, Value, json};

use super::error::GhostError;
use super::filter::{self, Names};
use super::people::{GhostAuthor, GhostTag, Include, add_included};
use super::post::GhostPost;
use super::query::GhostQuery;
use super::settings::GhostSite;
use crate::content_store::{ContentStore, PostKey, SiteResources, select};
use crate::pagination::Meta;
use crate::query::{Condition, ListQuery};
use crate::resources::{Collection, Listable, Post};

/// What every route reads: the published content and the site.
#[derive(Clone)]
pub(super) struct GhostState {
    pub(super) content: Arc<dyn ContentStore>,
    pub(super) site: Arc<GhostSite>,
}

/// The resources with browse and read routes.
#[derive(Debug, Clone, Copy)]
enum Kind {
    Posts,
    Pages,
    Tags,
    Authors,
}

impl Kind {
    /// The plural in the path and the wrapper, and one entry's name.
    fn names(self) -> (&'static str, &'static str) {
        match self {
            Self::Posts => ("posts", "post"),
            Self::Pages => ("pages", "page"),
            Self::Tags => ("tags", "tag"),
            Self::Authors => ("authors", "author"),
        }
    }
}

/// Every route, with `state`.
pub(super) fn routes(state: GhostState) -> Router {
    let router = [Kind::Posts, Kind::Pages, Kind::Tags, Kind::Authors]
        .into_iter()
        .fold(Router::new(), resource);
    router
        .route("/settings/", get(read_settings))
        .with_state(state)
}

/// `router` with the three routes of `kind`.
fn resource(router: Router<GhostState>, kind: Kind) -> Router<GhostState> {
    let (name, _) = kind.names();
    let browse_one = move |State(state): State<GhostState>, Query(raw): Query<GhostQuery>| async move {
        browse(&state, kind, &raw).await
    };
    let by_id = move |State(state): State<GhostState>,
                      Path(id): Path<String>,
                      Query(raw): Query<GhostQuery>| async move {
        read(&state, kind, Some(PostKey::Id(id)), &raw).await
    };
    let by_slug = move |State(state): State<GhostState>,
                        Path(slug): Path<String>,
                        Query(raw): Query<GhostQuery>| async move {
        let key = Slug::try_new(&slug).ok().map(PostKey::Slug);
        read(&state, kind, key, &raw).await
    };
    router
        .route(&format!("/{name}/"), get(browse_one))
        .route(&format!("/{name}/{{id}}/"), get(by_id))
        .route(&format!("/{name}/slug/{{slug}}/"), get(by_slug))
}

/// What a route answers: Ghost's JSON, or Ghost's error.
type Answer = Result<Json<Value>, GhostError>;

/// Which of the site's lists a route reads.
type Pick<T> = fn(&SiteResources) -> &[T];

impl GhostState {
    /// What every list holds to: the site's default language. A Ghost site
    /// has one language, so posts in the others, which may share a slug
    /// with their translation, stay out.
    fn scope(&self) -> Vec<Condition> {
        vec![filter::is("language", self.site.language())]
    }

    /// [`Self::scope`] and the entry at `key`.
    fn scope_at(&self, key: &PostKey) -> Vec<Condition> {
        let mut scope = self.scope();
        scope.push(match key {
            PostKey::Id(id) => filter::is("id", id),
            PostKey::Slug(slug) => filter::is("slug", slug.as_str()),
        });
        scope
    }
}

/// The settings, as Ghost sends them: one object, not a list.
async fn read_settings(State(state): State<GhostState>) -> Answer {
    Ok(Json(json!({ "settings": *state.site, "meta": {} })))
}

/// One page of `kind` under its name, with Ghost's pagination.
async fn browse(state: &GhostState, kind: Kind, raw: &GhostQuery) -> Answer {
    let (entries, meta) = match kind {
        Kind::Posts => browse_posts(state, Collection::Posts, raw).await?,
        Kind::Pages => browse_posts(state, Collection::Pages, raw).await?,
        Kind::Tags => {
            browse_site(
                state,
                (SiteResources::tags, GhostTag::new),
                filter::TAGS,
                raw,
            )
            .await?
        }
        Kind::Authors => {
            browse_site(
                state,
                (SiteResources::authors, GhostAuthor::new),
                filter::AUTHORS,
                raw,
            )
            .await?
        }
    };
    Ok(wrapped(
        kind.names().0,
        Value::Array(entries),
        Some(json!(meta)),
    ))
}

/// The entry of `kind` at `key`, as a list of one with no `meta`, or
/// Ghost's 404. A slug that cannot be a slug is `None`: nothing is at it.
async fn read(state: &GhostState, kind: Kind, key: Option<PostKey>, raw: &GhostQuery) -> Answer {
    let (name, one) = kind.names();
    let not_found = || GhostError::not_found(one);
    let key = key.ok_or_else(not_found)?;
    let entry = match kind {
        Kind::Posts => read_post(state, Collection::Posts, &key, raw).await?,
        Kind::Pages => read_post(state, Collection::Pages, &key, raw).await?,
        Kind::Tags => {
            read_site(
                state,
                (SiteResources::tags, GhostTag::new),
                filter::TAGS,
                &key,
                raw,
            )
            .await?
        }
        Kind::Authors => {
            read_site(
                state,
                (SiteResources::authors, GhostAuthor::new),
                filter::AUTHORS,
                &key,
                raw,
            )
            .await?
        }
    };
    let entry = entry.ok_or_else(not_found)?;
    Ok(wrapped(name, Value::Array(vec![entry]), None))
}

/// The tags and authors `include` asks for, offered on posts and pages.
const POST_INCLUDE: Include = Include {
    tags: true,
    authors: true,
};

async fn browse_posts(
    state: &GhostState,
    collection: Collection,
    raw: &GhostQuery,
) -> Result<(Vec<Value>, Meta), GhostError> {
    let include = raw.include(POST_INCLUDE)?;
    let query = raw.list_query(filter::POSTS, &Post::FILTER_FIELDS, state.scope())?;
    let listed = state
        .content
        .list(collection, &query)
        .await
        .map_err(|error| GhostError::internal(&error))?;
    let meta = Meta::new(query.page(), query.limit(), listed.total());
    let site = site_for(state, include).await?;
    let shaped = (&query, site.as_ref(), include);
    let posts = listed.into_items();
    let entries = posts.iter().map(|post| shape_post(post, shaped, raw));
    Ok((entries.collect(), meta))
}

async fn read_post(
    state: &GhostState,
    collection: Collection,
    key: &PostKey,
    raw: &GhostQuery,
) -> Result<Option<Value>, GhostError> {
    let include = raw.include(POST_INCLUDE)?;
    let scope = state.scope_at(key);
    let query = raw.list_query(filter::POSTS, &Post::FILTER_FIELDS, scope)?;
    let listed = state
        .content
        .list(collection, &query)
        .await
        .map_err(|error| GhostError::internal(&error))?;
    let site = site_for(state, include).await?;
    let shaped = (&query, site.as_ref(), include);
    let found = listed.into_items().into_iter().next();
    Ok(found.map(|post| shape_post(&post, shaped, raw)))
}

/// The site's lists, when `include` needs them to name tags or authors.
async fn site_for(
    state: &GhostState,
    include: Include,
) -> Result<Option<SiteResources>, GhostError> {
    if !include.any() {
        return Ok(None);
    }
    let site = state.content.site().await;
    site.map(Some).map_err(|error| GhostError::internal(&error))
}

/// `post` in Ghost's shape, in the formats the query asks for, with what
/// `include` adds from `site`, cut to the fields `raw` asks for.
fn shape_post(
    post: &Post,
    (query, site, include): (&ListQuery, Option<&SiteResources>, Include),
    raw: &GhostQuery,
) -> Value {
    let mut entry = object(&GhostPost::new(post, query.formats()));
    if let Some(site) = site {
        add_included(&mut entry, post, site, include);
    }
    raw.keep_fields(entry, include)
}

/// One page of a site list `pick`ed from the store and `shape`d for Ghost,
/// filtered by the Ghost fields in `names`.
async fn browse_site<T: Listable, G: Serialize>(
    state: &GhostState,
    (pick, shape): (Pick<T>, fn(&T) -> G),
    names: Names,
    raw: &GhostQuery,
) -> Result<(Vec<Value>, Meta), GhostError> {
    let include = raw.include(Include::default())?;
    let query = raw.list_query(names, T::FIELDS, state.scope())?;
    let site = state
        .content
        .site()
        .await
        .map_err(|error| GhostError::internal(&error))?;
    let listed = select(pick(&site).iter(), &query);
    let meta = Meta::new(query.page(), query.limit(), listed.total());
    let entries = listed.into_items().into_iter();
    let entries = entries.map(|entry| raw.keep_fields(object(&shape(&entry)), include));
    Ok((entries.collect(), meta))
}

async fn read_site<T: Listable, G: Serialize>(
    state: &GhostState,
    (pick, shape): (Pick<T>, fn(&T) -> G),
    names: Names,
    key: &PostKey,
    raw: &GhostQuery,
) -> Result<Option<Value>, GhostError> {
    let include = raw.include(Include::default())?;
    let query = raw.list_query(names, T::FIELDS, state.scope_at(key))?;
    let site = state
        .content
        .site()
        .await
        .map_err(|error| GhostError::internal(&error))?;
    let found = select(pick(&site).iter(), &query)
        .into_items()
        .into_iter()
        .next();
    Ok(found.map(|entry| raw.keep_fields(object(&shape(&entry)), include)))
}

/// `entry` as a JSON object; anything else is an empty one.
fn object(entry: &impl Serialize) -> Map<String, Value> {
    match serde_json::to_value(entry) {
        Ok(Value::Object(fields)) => fields,
        _ => Map::new(),
    }
}

/// `entries` under `name`, with `meta` beside them on a list.
fn wrapped(name: &str, entries: Value, meta: Option<Value>) -> Json<Value> {
    let mut body = Map::new();
    body.insert(name.to_owned(), entries);
    if let Some(meta) = meta {
        body.insert("meta".to_owned(), meta);
    }
    Json(Value::Object(body))
}
