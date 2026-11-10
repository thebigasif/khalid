//! The ways a key may be sent, as the `OpenAPI` document lists them. Kept
//! apart from `api.rs` so that file stays under 600 lines (BL-SIZE-1).

use utoipa::Modify;
use utoipa::openapi::OpenApi as Document;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

/// Adds the ways to send a key to the document.
pub(crate) struct KeySchemes;

impl Modify for KeySchemes {
    fn modify(&self, document: &mut Document) {
        let components = document.components.get_or_insert_with(Default::default);
        let schemes = [
            (
                "content_key",
                ApiKey::Header(ApiKeyValue::with_description(
                    "Authorization",
                    "The site's content key, as Khalid-Content followed by the key.",
                )),
            ),
            (
                "content_key_query",
                ApiKey::Query(ApiKeyValue::with_description(
                    "key",
                    "The site's content key.",
                )),
            ),
            (
                "admin_key",
                ApiKey::Header(ApiKeyValue::with_description(
                    "Authorization",
                    "An integration key or staff token, as Khalid followed by the key.",
                )),
            ),
            (
                "staff_session",
                ApiKey::Cookie(ApiKeyValue::with_description(
                    "__Host-khalid-session",
                    "A signed-in staff session. Every request with it also sends the session's \
                     CSRF token in the x-khalid-csrf header.",
                )),
            ),
        ];
        for (name, scheme) in schemes {
            components.add_security_scheme(name, SecurityScheme::ApiKey(scheme));
        }
    }
}
