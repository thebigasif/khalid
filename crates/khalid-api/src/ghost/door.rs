//! What lets a request into the Ghost-compatible API: Ghost's `?key=`
//! and Ghost's `Accept-Version`.

use std::sync::Arc;

use axum::extract::{Query, Request, State};
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use khalid_core::UserMessage;
use serde::Deserialize;

use super::error::GhostError;
use crate::api_limits::{self, Api, ApiLimits};
use crate::keys::{KeyError, KeyKind, KeyStore, authenticate};

/// The version named on every answer: the Ghost Content API this copies.
const SERVED: &str = "v5.0";

/// The stored keys and the request limits, the same rules as the Content
/// API's own door.
#[derive(Clone)]
pub(super) struct GhostDoor {
    pub(super) keys: Arc<dyn KeyStore>,
    pub(super) limits: Arc<ApiLimits>,
}

/// The one query parameter this layer reads.
#[derive(Deserialize)]
struct KeyParameter {
    key: Option<String>,
}

/// Lets a request through only with a stored content key in `?key=`, as
/// Ghost's client sends it. Khalid's `Authorization` header is not read
/// here, so the two doors stay apart. Busy keys and addresses that sent
/// too many wrong keys get the Content API's own 429 (BL-SEC-5).
pub(super) async fn require_key(
    State(door): State<GhostDoor>,
    request: Request,
    next: Next,
) -> Response {
    let address = api_limits::client(&request);
    if let Some(wait) = address.and_then(|address| door.limits.paused(address)) {
        return api_limits::paused(wait);
    }
    let presented = Query::<KeyParameter>::try_from_uri(request.uri())
        .ok()
        .and_then(|Query(parameter)| parameter.key);
    let Some(text) = presented else {
        return GhostError::unauthorized().into_response();
    };
    match authenticate(door.keys.as_ref(), &text, KeyKind::Content).await {
        Ok(principal) => match door.limits.admit(principal.id(), Api::Content) {
            Ok(()) => next.run(request).await,
            Err(wait) => api_limits::busy(wait),
        },
        Err(KeyError::Unavailable { source }) => GhostError::internal(&source).into_response(),
        Err(_) => {
            if let Some(address) = address {
                door.limits.refused(address);
            }
            GhostError::unauthorized().into_response()
        }
    }
}

/// Serves a request that asks for no version or for Ghost 5, such as
/// `v5.0`, and names `v5.0` on every answer. Khalid's own Content API
/// keeps refusing `v5`: this is the only path that takes it.
pub(super) async fn negotiate_version(request: Request, next: Next) -> Response {
    let asked = request.headers().get("accept-version").map(|value| {
        let text = value.to_str().unwrap_or_default();
        is_five(text)
    });
    let mut response = match asked {
        None | Some(true) => next.run(request).await,
        Some(false) => refuse().into_response(),
    };
    let served = HeaderValue::from_static(SERVED);
    response.headers_mut().insert("content-version", served);
    response
}

/// `v5` or `v5.<minor>`, with either case of `v`.
fn is_five(text: &str) -> bool {
    let Some(number) = text.strip_prefix(['v', 'V']) else {
        return false;
    };
    let (major, minor) = number.split_once('.').unwrap_or((number, "0"));
    major == "5" && !minor.is_empty() && minor.bytes().all(|b| b.is_ascii_digit())
}

fn refuse() -> GhostError {
    GhostError::bad_request(UserMessage::new(
        "This address answers Ghost's Content API version v5 only.",
        "Khalid copies Ghost 5's Content API here, so another version could break the client.",
        "Set version: \"v5.0\" in the Ghost client, or leave Accept-Version out.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_ghost_five_is_a_version_this_path_takes() {
        for text in ["v5", "v5.0", "V5.82"] {
            assert!(is_five(text), "{text}");
        }
        for text in ["5.0", "v6.0", "v55.0", "v5.", "v5.x", ""] {
            assert!(!is_five(text), "{text}");
        }
    }
}
