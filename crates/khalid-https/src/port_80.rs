//! What plain HTTP on port 80 answers when the site serves HTTPS.

use axum::Router;
use axum::extract::{Path, State};
use axum::http::header::{CONTENT_TYPE, HOST, LOCATION};
use axum::http::uri::PathAndQuery;
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;

use crate::{Certificates, Challenges, Domain};

/// Port 80's answers: Let's Encrypt's HTTP-01 checks from `challenges`,
/// and a permanent move to the same address on `https://domain` for every
/// other request, so a visitor who types the plain address lands on the
/// secure one.
pub fn port_80(domain: &Domain, challenges: Challenges) -> Router {
    let domain = domain.clone();
    checks()
        .fallback(move |uri: Uri| async move { moved(domain.as_str(), &uri) })
        .with_state(challenges)
}

/// Port 80's answers for many domains behind one listener: Let's
/// Encrypt's HTTP-01 checks, and a permanent move to HTTPS on the name the
/// visitor asked for once `certificates` has one for it, or on `home`
/// otherwise.
pub fn port_80_among(home: &Domain, certificates: Certificates) -> Router {
    let home = home.clone();
    let challenges = certificates.challenges().clone();
    checks()
        .fallback(move |headers: HeaderMap, uri: Uri| async move {
            let asked = headers
                .get(HOST)
                .and_then(|value| value.to_str().ok())
                .map(host_name)
                .filter(|name| certificates.get(name).is_some());
            moved(asked.as_deref().unwrap_or(home.as_str()), &uri)
        })
        .with_state(challenges)
}

/// The route Let's Encrypt's file check reads.
fn checks() -> Router<Challenges> {
    Router::new().route("/.well-known/acme-challenge/{token}", get(answer))
}

/// The name in a `Host` header, in lower case without the port or a last
/// dot.
fn host_name(host: &str) -> String {
    let name = match host.rsplit_once(':') {
        Some((name, port)) if port.bytes().all(|b| b.is_ascii_digit()) => name,
        _ => host,
    };
    name.trim_end_matches('.').to_ascii_lowercase()
}

/// The key authorization for an open check, or 404.
async fn answer(State(challenges): State<Challenges>, Path(token): Path<String>) -> Response {
    match challenges.http_answer(&token) {
        Some(text) => ([(CONTENT_TYPE, "text/plain")], text).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// A 308 to the same path and query over HTTPS on `domain`, which keeps a
/// form's POST a POST.
fn moved(domain: &str, uri: &Uri) -> Response {
    let path = uri.path_and_query().map_or("/", PathAndQuery::as_str);
    let to = format!("https://{domain}{path}");
    (StatusCode::PERMANENT_REDIRECT, [(LOCATION, to)]).into_response()
}
