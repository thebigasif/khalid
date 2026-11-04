//! A read-only copy of Ghost's Content API v5 under `/ghost/api/content/`
//! (R15, KTD13), so a front end built on Ghost's client moves to Khalid by
//! changing only its address and key. It answers only when the site sets
//! `ghost_content_api = true`; `docs/ghost-api.md` lists what it covers.
//!
//! It keeps Ghost's contract, not Khalid's: Ghost's paths, field names,
//! wrapper and error shape. So it is not in `/api/openapi.json`, which the
//! SDK and the MCP tools are made from (ADR "the Ghost-compatible API
//! keeps Ghost's contract").

mod door;
mod error;
mod people;
mod post;
mod query;
mod routes;
mod settings;

use std::sync::Arc;

use axum::Router;
use axum::middleware;
use khalid_core::SystemClock;

use crate::api_limits::ApiLimits;
use crate::content_store::ContentStore;
use crate::etag::tag_and_revalidate;
use crate::keys::KeyStore;
use door::GhostDoor;
use routes::GhostState;
pub use settings::GhostSite;

/// Where Ghost's Content API lives on a Ghost site, and so here.
pub const BASE: &str = "/ghost/api/content";

/// The Ghost-compatible routes over `content`, opened by the content keys
/// in `keys`, with `/settings/` from `site`. Every answer names `v5.0` and
/// carries an `ETag`, as the Content API's own answers do.
pub fn router(content: Arc<dyn ContentStore>, keys: Arc<dyn KeyStore>, site: GhostSite) -> Router {
    let door = GhostDoor {
        keys,
        limits: Arc::new(ApiLimits::new(SystemClock)),
    };
    let state = GhostState {
        content,
        site: Arc::new(site),
    };
    let routes = routes::routes(state)
        .route_layer(middleware::from_fn_with_state(door, door::require_key))
        .route_layer(middleware::from_fn(tag_and_revalidate))
        .route_layer(middleware::from_fn(door::negotiate_version));
    Router::new().nest(BASE, routes)
}
