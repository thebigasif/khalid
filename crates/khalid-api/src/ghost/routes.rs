//! The routes under `/ghost/api/content/`.

use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

use super::error::GhostError;
use super::settings::GhostSite;
use crate::content_store::ContentStore;

/// What every route reads: the published content and the site.
#[derive(Clone)]
pub(super) struct GhostState {
    #[expect(
        dead_code,
        reason = "the posts and pages routes read it in the next part"
    )]
    pub(super) content: Arc<dyn ContentStore>,
    pub(super) site: Arc<GhostSite>,
}

/// Every route, with `state`.
pub(super) fn routes(state: GhostState) -> Router {
    Router::new()
        .route("/settings/", get(read_settings))
        .with_state(state)
}

/// What a route answers: Ghost's JSON, or Ghost's error.
type Answer = Result<Json<Value>, GhostError>;

/// The settings, as Ghost sends them: one object, not a list.
async fn read_settings(State(state): State<GhostState>) -> Answer {
    Ok(Json(json!({ "settings": *state.site, "meta": {} })))
}
