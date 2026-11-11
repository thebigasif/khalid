//! The newsletter's addresses on a running site: the form posts to
//! `subscribe`, and the links in emails open `subscribe/confirm` and
//! `unsubscribe`, all under the mount path. A counted link in a newsletter
//! email opens `subscribe/click` (G39).

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::rejection::FormRejection;
use axum::extract::{ConnectInfo, DefaultBodyLimit, Form, Query, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, Response, StatusCode, header};
use axum::routing::{get, post};
use axum::{Extension, Router};
use serde::Deserialize;

use crate::{
    BackPath, Confirmed, EmailAddress, Newsletter, Page, Pages, ReaderPass, SendId, SignUpCheck,
    SignUpLimits, SubscriberToken, Unsubscribed,
};

/// The largest form body read. An address and a token fit many times over.
const LARGEST_FORM: usize = 4 * 1024;

/// Pages hold links with tokens and answers about one reader, so nothing
/// keeps them, no other site may frame them, and no link leaks the address.
/// The last two match what `khalid-server` sends with every other page.
const HEADERS: [(HeaderName, &str); 8] = [
    (header::CACHE_CONTROL, "no-store"),
    (header::REFERRER_POLICY, "no-referrer"),
    (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
    (
        header::CONTENT_SECURITY_POLICY,
        "default-src 'none'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'",
    ),
    (HeaderName::from_static("x-robots-tag"), "noindex"),
    (header::CONTENT_TYPE, "text/html; charset=utf-8"),
    (
        HeaderName::from_static("permissions-policy"),
        "camera=(), microphone=(), geolocation=(), payment=(), usb=(), browsing-topics=()",
    ),
    (
        HeaderName::from_static("cross-origin-opener-policy"),
        "same-origin",
    ),
];

/// What every route reads. `newsletter` is `None` while the site cannot
/// send email, and then every page says sign-ups are not open.
#[derive(Debug)]
struct Shared {
    pages: Pages,
    newsletter: Option<Newsletter>,
    limits: Option<Box<dyn SignUpLimits>>,
}

/// The fields of the sign-up form. `homepage` is the trap a person never
/// sees; a program that fills it gets the same page and nothing happens.
/// `list` names the newsletter, `main` when it is left out.
#[derive(Debug, Deserialize)]
struct SignUpForm {
    email: Option<String>,
    homepage: Option<String>,
    /// The page of gated content the form was on.
    back: Option<String>,
    list: Option<String>,
}

/// The token of a link, from its query or from the page's one button, and
/// the page to come back to.
#[derive(Debug, Deserialize)]
struct Link {
    token: Option<String>,
    back: Option<String>,
}

/// A pass link's code and the page to come back to.
#[derive(Debug, Deserialize)]
struct PassLink {
    code: Option<String>,
    back: Option<String>,
}

/// A counted link's email and its number there, read as text so a link
/// someone changed by hand gets a page, not an error.
#[derive(Debug, Deserialize)]
struct ClickLink {
    send: Option<String>,
    link: Option<String>,
}

/// The routes of the newsletter, with `pages` in the site's language.
/// The sign-up form of the `::newsletter` block posts to `/subscribe`, so
/// on a site under a mount path that address answers too.
///
/// With `limits`, each sign-up is counted against the reader's network
/// address first, so serve the router with `ConnectInfo<SocketAddr>`; a
/// sign-up whose address is not known then sends nothing.
pub fn router(
    pages: Pages,
    newsletter: Option<Newsletter>,
    limits: Option<Box<dyn SignUpLimits>>,
) -> Router {
    let mount = pages.mount().to_owned();
    let shared = Arc::new(Shared {
        pages,
        newsletter,
        limits,
    });
    let mut router = Router::new()
        .route(&format!("{mount}subscribe"), post(sign_up))
        .route(
            &format!("{mount}subscribe/confirm"),
            get(confirm_page).post(confirm),
        )
        .route(&format!("{mount}subscribe/pass"), get(pass))
        .route(&format!("{mount}subscribe/click"), get(click))
        .route(&format!("{mount}unsubscribe"), get(leave_page).post(leave));
    if mount != "/" {
        router = router.route("/subscribe", post(sign_up));
    }
    router
        .layer(DefaultBodyLimit::max(LARGEST_FORM))
        .with_state(shared)
}

async fn sign_up(
    State(shared): State<Arc<Shared>>,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    form: Result<Form<SignUpForm>, FormRejection>,
) -> Response<Body> {
    let Some(newsletter) = &shared.newsletter else {
        return page(&shared, StatusCode::SERVICE_UNAVAILABLE, Page::Closed);
    };
    let Ok(Form(form)) = form else {
        return page(&shared, StatusCode::UNPROCESSABLE_ENTITY, Page::WrongEmail);
    };
    if form.homepage.is_some_and(|trap| !trap.is_empty()) {
        return page(&shared, StatusCode::OK, Page::Check);
    }
    let email = form.email.as_deref().map(EmailAddress::parse);
    let Some(Ok(email)) = email else {
        return page(&shared, StatusCode::UNPROCESSABLE_ENTITY, Page::WrongEmail);
    };
    let Some(list) = newsletter.lists().find(form.list.as_deref()) else {
        return page(&shared, StatusCode::NOT_FOUND, Page::NoSuchList);
    };
    let peer = peer.map(|Extension(ConnectInfo(peer))| peer);
    if let Some(refused) = shared.over_limit(peer, &headers) {
        return refused;
    }
    let signed = match shared.back(form.back.as_deref()) {
        Some(back) => newsletter.sign_up_to_read(email, list, &back).await,
        None => newsletter.sign_up(email, list).await,
    };
    match signed {
        Ok(_) => page(&shared, StatusCode::OK, Page::Check),
        Err(error) => failed(&shared, &error),
    }
}

async fn confirm_page(
    State(shared): State<Arc<Shared>>,
    Query(link): Query<Link>,
) -> Response<Body> {
    let back = shared.back(link.back.as_deref());
    link_page(&shared, &link, |token| Page::Confirm {
        token,
        back: back.as_ref(),
    })
}

async fn leave_page(State(shared): State<Arc<Shared>>, Query(link): Query<Link>) -> Response<Body> {
    link_page(&shared, &link, Page::Leave)
}

/// The page a link opens: one button that posts its token. Opening it
/// changes nothing.
fn link_page<'a>(
    shared: &Shared,
    link: &'a Link,
    ask: impl FnOnce(&'a str) -> Page<'a>,
) -> Response<Body> {
    if shared.newsletter.is_none() {
        return page(shared, StatusCode::SERVICE_UNAVAILABLE, Page::Closed);
    }
    match link
        .token
        .as_deref()
        .filter(|token| SubscriberToken::parse(token).is_ok())
    {
        Some(token) => page(shared, StatusCode::OK, ask(token)),
        None => page(shared, StatusCode::NOT_FOUND, Page::Unknown),
    }
}

async fn confirm(
    State(shared): State<Arc<Shared>>,
    form: Result<Form<Link>, FormRejection>,
) -> Response<Body> {
    let Some(newsletter) = &shared.newsletter else {
        return page(&shared, StatusCode::SERVICE_UNAVAILABLE, Page::Closed);
    };
    let (token, back) = form
        .ok()
        .map(|Form(link)| (link.token.unwrap_or_default(), link.back))
        .unwrap_or_default();
    match newsletter.confirm(&token).await {
        Ok(Confirmed::Now(_) | Confirmed::Before) => {
            let back = shared.back(back.as_deref());
            let_in(&shared, newsletter, back.as_ref())
        }
        Ok(Confirmed::Expired) => page(&shared, StatusCode::GONE, Page::Expired),
        Ok(_) => page(&shared, StatusCode::NOT_FOUND, Page::Unknown),
        Err(error) => failed(&shared, &error),
    }
}

/// A pass link from an email: a code this site signed, still in time,
/// gets the reader cookie and goes back to the page it names. Opening it
/// stores nothing, so a mail program that scans it does no harm.
async fn pass(State(shared): State<Arc<Shared>>, Query(link): Query<PassLink>) -> Response<Body> {
    let Some(newsletter) = &shared.newsletter else {
        return page(&shared, StatusCode::SERVICE_UNAVAILABLE, Page::Closed);
    };
    let code = link.code.as_deref().unwrap_or_default();
    if !newsletter.pass().is_some_and(|pass| pass.opens(code)) {
        return page(&shared, StatusCode::NOT_FOUND, Page::Unknown);
    }
    let back = shared.back(link.back.as_deref());
    let_in(&shared, newsletter, back.as_ref())
}

/// A counted link from an email: one more click on it, then a 302 to where
/// it leads. Only links stored with an email lead anywhere, so the route
/// sends no one to an address of their choosing.
async fn click(State(shared): State<Arc<Shared>>, Query(link): Query<ClickLink>) -> Response<Body> {
    let Some(newsletter) = &shared.newsletter else {
        return page(&shared, StatusCode::SERVICE_UNAVAILABLE, Page::Closed);
    };
    let send = link.send.as_deref().and_then(|send| send.parse().ok());
    let position = link.link.as_deref().and_then(|link| link.parse().ok());
    let (Some(send), Some(position)) = (send, position) else {
        return page(&shared, StatusCode::NOT_FOUND, Page::Unknown);
    };
    match newsletter.click(SendId::from_row(send), position).await {
        Ok(Some(url)) => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::FOUND;
            let headers = response.headers_mut();
            headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
            let policy = HeaderValue::from_static("no-referrer");
            headers.insert(header::REFERRER_POLICY, policy);
            if let Ok(location) = HeaderValue::try_from(url.as_str()) {
                headers.insert(header::LOCATION, location);
            }
            response
        }
        Ok(None) => page(&shared, StatusCode::NOT_FOUND, Page::Unknown),
        Err(error) => failed(&shared, &error),
    }
}

/// A confirmed reader: the reader cookie when gated content opens, then
/// a 303 to `back`, or the page that says they are subscribed.
fn let_in(shared: &Shared, newsletter: &Newsletter, back: Option<&BackPath>) -> Response<Body> {
    let mut response = match back {
        Some(back) => {
            let mut response = page(shared, StatusCode::SEE_OTHER, Page::Confirmed);
            if let Ok(location) = HeaderValue::try_from(back.as_str()) {
                response.headers_mut().insert(header::LOCATION, location);
            }
            response
        }
        None => page(shared, StatusCode::OK, Page::Confirmed),
    };
    let cookie = newsletter.pass().map(ReaderPass::cookie);
    if let Some(Ok(cookie)) = cookie.map(HeaderValue::try_from) {
        response.headers_mut().insert(header::SET_COOKIE, cookie);
    }
    response
}

impl Shared {
    /// `text` as a page on this blog to come back to, if it is one.
    fn back(&self, text: Option<&str>) -> Option<BackPath> {
        text.and_then(|text| BackPath::parse(self.pages.mount(), text))
    }

    /// The page that refuses a sign-up from `peer` over the limits: 429
    /// with when to try again, or 503 when the limits cannot be checked.
    /// `None` when the sign-up may go ahead.
    fn over_limit(&self, peer: Option<SocketAddr>, headers: &HeaderMap) -> Option<Response<Body>> {
        let limits = self.limits.as_ref()?;
        let check = peer.map_or(SignUpCheck::Unknown, |peer| {
            limits.check(peer.ip(), headers)
        });
        match check {
            SignUpCheck::Allowed => None,
            SignUpCheck::Wait(wait) => {
                let minutes = wait.as_secs().div_ceil(60).max(1);
                let refused = Page::TooMany { minutes };
                let mut response = page(self, StatusCode::TOO_MANY_REQUESTS, refused);
                let retry = HeaderValue::from(minutes.saturating_mul(60));
                response.headers_mut().insert(header::RETRY_AFTER, retry);
                Some(response)
            }
            SignUpCheck::Unknown => {
                tracing::warn!("a sign-up could not be checked against the limits");
                Some(page(self, StatusCode::SERVICE_UNAVAILABLE, Page::Failed))
            }
        }
    }
}

/// One click: the token comes from the page's button, or from the link's
/// query when a mail program posts to it (RFC 8058, one-click unsubscribe).
async fn leave(
    State(shared): State<Arc<Shared>>,
    Query(query): Query<Link>,
    form: Result<Form<Link>, FormRejection>,
) -> Response<Body> {
    let Some(newsletter) = &shared.newsletter else {
        return page(&shared, StatusCode::SERVICE_UNAVAILABLE, Page::Closed);
    };
    let token = query
        .token
        .or_else(|| form.ok().and_then(|Form(link)| link.token))
        .unwrap_or_default();
    match newsletter.unsubscribe(&token).await {
        Ok(Unsubscribed::Done) => {
            // A reader who leaves in this browser no longer reads gated
            // content in it. Other devices keep their cookie until it runs
            // out, as no request checks who holds one.
            let mut response = page(&shared, StatusCode::OK, Page::Left);
            let forget = newsletter.pass().map(ReaderPass::forget);
            if let Some(Ok(forget)) = forget.map(HeaderValue::try_from) {
                response.headers_mut().insert(header::SET_COOKIE, forget);
            }
            response
        }
        Ok(_) => page(&shared, StatusCode::NOT_FOUND, Page::Unknown),
        Err(error) => failed(&shared, &error),
    }
}

/// 503 with the page that says nothing was sent. The cause goes to the log,
/// which never holds an address or a token.
fn failed(shared: &Shared, why: &dyn std::fmt::Display) -> Response<Body> {
    tracing::warn!(%why, "a newsletter step failed");
    page(shared, StatusCode::SERVICE_UNAVAILABLE, Page::Failed)
}

fn page(shared: &Shared, status: StatusCode, page: Page<'_>) -> Response<Body> {
    let mut response = Response::new(Body::from(shared.pages.html(page)));
    *response.status_mut() = status;
    let headers = response.headers_mut();
    for (name, value) in HEADERS {
        headers.insert(name, HeaderValue::from_static(value));
    }
    response
}
