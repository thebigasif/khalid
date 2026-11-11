//! The mail hook on `khalid serve` (gap audit 5, row 7): Resend or Postmark
//! tells the blog that an address bounced for good or that its owner marked
//! an email as spam, and the newsletter sends that address nothing again.
//! Mail sent to dead addresses and to people who complain is what makes a
//! sending account look like a spammer's.

use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::routing::post;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use hmac::{Hmac, KeyInit, Mac};
use khalid_core::{Config, MailTransport, Secret};
use khalid_newsletter::{EmailAddress, Newsletter, Stop};
use serde::Deserialize;
use sha2::Sha256;
use subtle::ConstantTimeEq;

/// Where the hook answers, beside git mode's push hook.
pub(crate) const HOOK_PATH: &str = "/api/hooks/mail";

/// The services that send a site's mail and report what became of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Service {
    /// Signs each event the way Svix does, with the hook's signing secret.
    Resend,
    /// Sends each event to an address with a user name and password in it.
    Postmark,
}

/// What the hook needs: who sends the events, the newsletter that stops
/// mail, and the secret that proves an event came from the service.
#[derive(Clone)]
struct Hook {
    service: Service,
    newsletter: Newsletter,
    secret: Arc<Secret<String>>,
}

/// One Resend event: its type, and the email it is about.
#[derive(Deserialize)]
struct ResendEvent {
    #[serde(rename = "type")]
    kind: String,
    data: ResendData,
}

#[derive(Deserialize)]
struct ResendData {
    #[serde(default)]
    to: Vec<String>,
    bounce: Option<ResendBounce>,
}

/// `Permanent` for an address that never takes mail.
#[derive(Deserialize)]
struct ResendBounce {
    #[serde(rename = "type")]
    kind: String,
}

/// One Postmark bounce or spam complaint.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PostmarkEvent {
    record_type: String,
    #[serde(rename = "Type", default)]
    kind: String,
    #[serde(default)]
    email: String,
}

/// The mail hook of the site with `config`, stopping mail through
/// `newsletter`, or no route at all unless the site sends through Resend
/// or Postmark and the environment holds the hook's `secret`.
pub(crate) fn router(
    config: &Config,
    newsletter: Option<Newsletter>,
    secret: Option<Secret<String>>,
) -> Router {
    let service = match config.mail().map(|mail| &mail.transport) {
        Some(MailTransport::Resend) => Service::Resend,
        Some(MailTransport::Postmark(_)) => Service::Postmark,
        _ => return Router::new(),
    };
    let (Some(newsletter), Some(secret)) = (newsletter, secret) else {
        return Router::new();
    };
    hook(service, newsletter, secret)
}

fn hook(service: Service, newsletter: Newsletter, secret: Secret<String>) -> Router {
    let hook = Hook {
        service,
        newsletter,
        secret: Arc::new(secret),
    };
    Router::new()
        .route(HOOK_PATH, post(reported))
        .with_state(hook)
}

/// Stops mail to each address a trusted event names. Anything without the
/// secret is a 401, and a store that fails is a 503, which the service
/// sends again later. Running an event twice changes nothing more.
async fn reported(
    State(hook): State<Hook>,
    headers: HeaderMap,
    body: Bytes,
) -> (StatusCode, &'static str) {
    let secret = hook.secret.expose();
    let trusted = match hook.service {
        Service::Resend => signed(&headers, &body, secret),
        Service::Postmark => allowed(&headers, secret),
    };
    if !trusted {
        return (
            StatusCode::UNAUTHORIZED,
            "The event did not carry KHALID_MAIL_HOOK_SECRET. Set the same secret on the mail service's webhook.\n",
        );
    }
    let Some(stops) = stops(hook.service, &body) else {
        return (
            StatusCode::BAD_REQUEST,
            "This is not an event the mail service sends, so nothing changed.\n",
        );
    };
    for (email, why) in &stops {
        // The log never holds the address (BL-RUN-5).
        if let Err(error) = hook.newsletter.stop(email, *why).await {
            tracing::warn!(%error, "the mail hook could not stop mail to an address; the service will send the event again");
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Not stored. Send it again later.\n",
            );
        }
    }
    if !stops.is_empty() {
        tracing::info!(
            addresses = stops.len(),
            "the mail hook stopped mail to bounced or complaining addresses"
        );
    }
    (StatusCode::OK, "Noted.\n")
}

/// Whether `headers` sign `body` with `secret` the way Resend does: the
/// HMAC-SHA256 of `id.timestamp.body`, keyed with the bytes the secret
/// spells in base64 after `whsec_`, as one `v1,` entry of `svix-signature`.
fn signed(headers: &HeaderMap, body: &[u8], secret: &str) -> bool {
    let header = |name| headers.get(name).and_then(|value| value.to_str().ok());
    let (Some(id), Some(at), Some(signatures)) = (
        header("svix-id"),
        header("svix-timestamp"),
        header("svix-signature"),
    ) else {
        return false;
    };
    let key = STANDARD.decode(secret.strip_prefix("whsec_").unwrap_or(secret));
    let Some(Ok(mut mac)) = key.ok().map(|key| Hmac::<Sha256>::new_from_slice(&key)) else {
        return false;
    };
    for part in [id.as_bytes(), b".", at.as_bytes(), b".", body] {
        mac.update(part);
    }
    signatures
        .split(' ')
        .filter_map(|entry| entry.strip_prefix("v1,"))
        .filter_map(|sent| STANDARD.decode(sent).ok())
        .any(|sent| mac.clone().verify_slice(&sent).is_ok())
}

/// Whether `headers` carry the user name and password Postmark was given
/// in the hook's address, with `secret` as the password, compared in
/// constant time. Any user name does.
fn allowed(headers: &HeaderMap, secret: &str) -> bool {
    let basic = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Basic "));
    let Some(Ok(pair)) = basic.map(|basic| STANDARD.decode(basic.trim())) else {
        return false;
    };
    let password = pair
        .iter()
        .position(|&byte| byte == b':')
        .and_then(|colon| pair.get(colon + 1..));
    password.is_some_and(|password| bool::from(password.ct_eq(secret.as_bytes())))
}

/// The addresses `body` says to stop and why, or `None` when it is not an
/// event `service` sends. A bounce that may pass, such as a full mailbox,
/// stops nothing, and nor does any other event.
fn stops(service: Service, body: &[u8]) -> Option<Vec<(EmailAddress, Stop)>> {
    let (addresses, why) = match service {
        Service::Resend => {
            let event: ResendEvent = serde_json::from_slice(body).ok()?;
            let permanent = event
                .data
                .bounce
                .is_some_and(|bounce| bounce.kind == "Permanent");
            let why = match event.kind.as_str() {
                "email.bounced" if permanent => Some(Stop::Bounced),
                "email.complained" => Some(Stop::Complained),
                _ => None,
            };
            (event.data.to, why)
        }
        Service::Postmark => {
            let event: PostmarkEvent = serde_json::from_slice(body).ok()?;
            let why = match (event.record_type.as_str(), event.kind.as_str()) {
                ("Bounce", "HardBounce" | "BadEmailAddress") => Some(Stop::Bounced),
                ("SpamComplaint", _) => Some(Stop::Complained),
                _ => None,
            };
            (vec![event.email], why)
        }
    };
    let Some(why) = why else {
        return Some(Vec::new());
    };
    // `Name <address>` names the address inside the brackets.
    let bare = |text: &str| {
        let text = text.rsplit_once('<').map_or(text, |(_, inside)| inside);
        EmailAddress::parse(text.trim_end_matches('>')).ok()
    };
    Some(
        addresses
            .iter()
            .filter_map(|text| bare(text))
            .map(|email| (email, why))
            .collect(),
    )
}

#[cfg(test)]
#[path = "mail_hook_tests.rs"]
mod tests;
