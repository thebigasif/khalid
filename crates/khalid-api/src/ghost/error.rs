//! Errors in Ghost's own shape: `{"errors": [{"message", "context",
//! "type", ...}]}`, which Ghost's client turns into a thrown `Error`.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use khalid_core::UserMessage;
use serde_json::json;

use crate::query::ParseQueryError;

/// One error as Ghost sends it. The what, why and fix of a
/// [`UserMessage`] go into Ghost's `message`, `context` and `help`.
#[derive(Debug)]
pub(crate) struct GhostError {
    status: StatusCode,
    kind: &'static str,
    message: UserMessage,
}

impl GhostError {
    /// An error of Ghost's `kind`, such as `NotFoundError`, at `status`.
    pub(crate) fn new(status: StatusCode, kind: &'static str, message: UserMessage) -> Self {
        Self {
            status,
            kind,
            message,
        }
    }

    /// A 400 `BadRequestError`.
    pub(crate) fn bad_request(message: UserMessage) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "BadRequestError", message)
    }

    /// The 401 for a missing or wrong key, in Ghost's words, which clients
    /// may compare. The key is never repeated back.
    pub(crate) fn unauthorized() -> Self {
        let message = UserMessage::new(
            "Authorization failed",
            "This request has no valid content key, and Ghost's address answers only those.",
            "Add ?key= with a key made by khalid keys create --kind content.",
        );
        Self::new(StatusCode::UNAUTHORIZED, "UnauthorizedError", message)
    }

    /// The 404 for an address with nothing published at it.
    pub(crate) fn not_found(resource: &str) -> Self {
        let message = UserMessage::new(
            format!("Resource not found error, cannot read {resource}."),
            "It may still be a draft, or its slug may have changed.",
            format!("Check the address, or browse the list of {resource}s."),
        );
        Self::new(StatusCode::NOT_FOUND, "NotFoundError", message)
    }

    /// The 500 for a store that cannot be read, logged once here.
    pub(crate) fn internal(error: &(dyn std::error::Error + 'static)) -> Self {
        tracing::error!(error, "the Ghost-compatible API could not answer");
        let message = UserMessage::new(
            "The server could not read what it needed.",
            "Nothing was changed, but this request could not be answered.",
            "Try again in a minute, and check the server log if it keeps happening.",
        );
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "InternalServerError",
            message,
        )
    }
}

impl From<ParseQueryError> for GhostError {
    fn from(error: ParseQueryError) -> Self {
        let context = error.to_string();
        Self::bad_request(UserMessage::new(
            "The list could not be read.",
            format!("In this request, {context}."),
            "Fix that parameter, or leave it out to get the list as Ghost sends it.",
        ))
    }
}

impl IntoResponse for GhostError {
    fn into_response(self) -> Response {
        let message = &self.message;
        let body = json!({ "errors": [{
            "message": message.what(),
            "context": message.why(),
            "help": message.fix(),
            "type": self.kind,
            "details": null,
            "property": null,
            "code": null,
            "ghostErrorCode": null,
        }]});
        (self.status, Json(body)).into_response()
    }
}
