//! The routes under `/ghost/api/content/`: browse and read for posts and
//! pages, and the settings.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use khalid_core::{Slug, UserMessage};
use serde::Deserialize;
use serde_json::{Map, Value, json};

use super::error::GhostError;
use super::post::GhostPost;
use super::settings::GhostSite;
use crate::content_store::{ContentStore, PostKey};
use crate::pagination::Meta;
use crate::query::{ListQuery, RawListQuery};
use crate::resources::{Collection, Post};

/// What every route reads: the published content and the site.
#[derive(Clone)]
pub(super) struct GhostState {
    pub(super) content: Arc<dyn ContentStore>,
    pub(super) site: Arc<GhostSite>,
}

/// The parameters Ghost's client sends. Others, such as `key` and the
/// `slug` the client repeats on a read, are left alone.
#[derive(Debug, Default, Deserialize)]
pub(super) struct GhostQuery {
    filter: Option<String>,
    order: Option<String>,
    formats: Option<String>,
    limit: Option<String>,
    page: Option<String>,
}

impl GhostQuery {
    /// The checked list query, with fields the posts do not have refused.
    fn list_query(&self) -> Result<ListQuery, GhostError> {
        let raw = RawListQuery {
            filter: self.filter.clone(),
            include: None,
            fields: None,
            order: self.order.clone(),
            formats: self.formats.clone(),
            limit: self.limit.clone(),
            page: self.page.clone(),
        };
        let query = ListQuery::try_from(&raw)?;
        let fields: &[&str] = &Post::FILTER_FIELDS;
        let unknown = |parameter: &str, field: &str| {
            GhostError::bad_request(UserMessage::new(
                format!("The {parameter} uses {field}, which this API cannot sort or filter by."),
                "The list cannot be sent until it is fixed.",
                format!("Use one of these fields: {}.", fields.join(", ")),
            ))
        };
        if let Some(filter) = query.filter() {
            filter
                .check_fields(fields)
                .map_err(|error| unknown("filter", error.field()))?;
        }
        if let Some(key) = query
            .order()
            .iter()
            .find(|key| !fields.contains(&key.field()))
        {
            return Err(unknown("order", key.field()));
        }
        Ok(query)
    }
}

/// Every route, with `state`.
pub(super) fn routes(state: GhostState) -> Router {
    Router::new()
        .route("/posts/", get(browse_posts))
        .route("/posts/{id}/", get(read_post))
        .route("/posts/slug/{slug}/", get(read_post_by_slug))
        .route("/pages/", get(browse_pages))
        .route("/pages/{id}/", get(read_page))
        .route("/pages/slug/{slug}/", get(read_page_by_slug))
        .route("/settings/", get(read_settings))
        .with_state(state)
}

/// What a route answers: Ghost's JSON, or Ghost's error.
type Answer = Result<Json<Value>, GhostError>;

async fn browse_posts(State(state): State<GhostState>, Query(raw): Query<GhostQuery>) -> Answer {
    browse(&state, Collection::Posts, "posts", &raw).await
}

async fn read_post(
    State(state): State<GhostState>,
    Path(id): Path<String>,
    Query(raw): Query<GhostQuery>,
) -> Answer {
    read(&state, Collection::Posts, Some(PostKey::Id(id)), &raw).await
}

async fn read_post_by_slug(
    State(state): State<GhostState>,
    Path(slug): Path<String>,
    Query(raw): Query<GhostQuery>,
) -> Answer {
    let key = Slug::try_new(&slug).ok().map(PostKey::Slug);
    read(&state, Collection::Posts, key, &raw).await
}

async fn browse_pages(State(state): State<GhostState>, Query(raw): Query<GhostQuery>) -> Answer {
    browse(&state, Collection::Pages, "pages", &raw).await
}

async fn read_page(
    State(state): State<GhostState>,
    Path(id): Path<String>,
    Query(raw): Query<GhostQuery>,
) -> Answer {
    read(&state, Collection::Pages, Some(PostKey::Id(id)), &raw).await
}

async fn read_page_by_slug(
    State(state): State<GhostState>,
    Path(slug): Path<String>,
    Query(raw): Query<GhostQuery>,
) -> Answer {
    let key = Slug::try_new(&slug).ok().map(PostKey::Slug);
    read(&state, Collection::Pages, key, &raw).await
}

/// The settings, as Ghost sends them: one object, not a list.
async fn read_settings(State(state): State<GhostState>) -> Answer {
    Ok(Json(json!({ "settings": *state.site, "meta": {} })))
}

/// One page of `collection` under `name`, with Ghost's pagination.
async fn browse(
    state: &GhostState,
    collection: Collection,
    name: &str,
    raw: &GhostQuery,
) -> Answer {
    let query = raw.list_query()?;
    let listed = state
        .content
        .list(collection, &query)
        .await
        .map_err(|error| GhostError::internal(&error))?;
    let meta = Meta::new(query.page(), query.limit(), listed.total());
    let entries: Vec<GhostPost> = listed
        .into_items()
        .iter()
        .map(|post| GhostPost::new(post, query.formats()))
        .collect();
    Ok(wrapped(name, json!(entries), Some(json!(meta))))
}

/// The entry of `collection` at `key`, as a list of one with no `meta`,
/// or Ghost's 404. A slug that cannot be a slug is `None`: nothing is at it.
async fn read(
    state: &GhostState,
    collection: Collection,
    key: Option<PostKey>,
    raw: &GhostQuery,
) -> Answer {
    let (name, one) = match collection {
        Collection::Pages => ("pages", "page"),
        _ => ("posts", "post"),
    };
    let query = raw.list_query()?;
    let Some(key) = key else {
        return Err(GhostError::not_found(one));
    };
    let found = state
        .content
        .find(collection, &key)
        .await
        .map_err(|error| GhostError::internal(&error))?;
    let post = found.ok_or_else(|| GhostError::not_found(one))?;
    let entry = GhostPost::new(&post, query.formats());
    Ok(wrapped(name, json!([entry]), None))
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
