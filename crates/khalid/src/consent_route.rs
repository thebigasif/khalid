//! The consent form on a running site: the banner, the choices page and
//! each embed card post here, and the reader's categories come back as a
//! cookie on a 303 to the page they were reading (G45, privacy plan U3,
//! ADR "consent has categories").

use axum::Router;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Form, FromRequest, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::Response;
use axum::routing::{get, post};
use khalid_content::{EMBED_PROVIDERS, EmbedProvider};
use khalid_core::{Clock, CodeCategory, Config, ConsentCategory, ConsentRev, GpcMode, SystemClock};
use khalid_server::{
    Answer, CONSENT_COOKIE, Categories, ConsentSite, HeldCode, LivePages, NONCE_SLOT, Reader,
    SAVED_QUERY, SecurityHeaders, router_with_consent,
};
use serde::Deserialize;

use crate::page_code::CONSENT_FOLDER;

/// The most the form may send: a choice, four switches and the address of
/// a page.
const BODY_LIMIT: usize = 2 * 1024;

/// The days counted as one month of a choice's lifetime, so 13 months stay
/// inside the 13 the CNIL allows.
const DAYS_PER_MONTH: u32 = 30;

/// The seconds in a day.
const DAY: u32 = 24 * 60 * 60;

/// Where a browser looks for a site's answer to Global Privacy Control.
const GPC_JSON: &str = "/.well-known/gpc.json";

/// The answer at [`GPC_JSON`]: this site honours the signal.
pub(crate) const GPC_HONOURED: &str = "{\"gpc\": true}\n";

/// What the form route needs: where the blog lives, whether the cookie is
/// sent over HTTPS only, how long a choice lasts, the site's consent, and
/// the headers every answer carries.
#[derive(Debug, Clone)]
struct Answering {
    mount: String,
    secure: bool,
    days: u32,
    site: ConsentSite,
    security: SecurityHeaders,
}

/// The site's page router over `live`, each HTML page filled for the
/// reader's consent. Every site uses it: one with no code of its own
/// still has embed cards and the choices page.
pub(crate) fn pages(config: &Config, live: LivePages, security: SecurityHeaders) -> Router {
    router_with_consent(live, security, consent_site(config))
}

/// The short hash of the consent settings of the site with `config`
/// (privacy plan KTD3), which every cookie carries. External media counts
/// every provider Khalid can embed, so publishing a post never changes it.
/// Static mode's script and the consent proof read the same hash.
pub(crate) fn rev(config: &Config) -> ConsentRev {
    let embeds = EMBED_PROVIDERS.map(EmbedProvider::frame_origin);
    config.privacy().rev(config.code(), &embeds)
}

/// What the server needs to fill pages on the site with `config`: its
/// [`rev`], the code that waits, with the nonce slot, each piece under its
/// category, and what GPC keeps off. With an outside consent tool
/// declared, nothing waits, because that tool asks instead.
pub(crate) fn consent_site(config: &Config) -> ConsentSite {
    let privacy = config.privacy();
    let mut held = HeldCode::new();
    let waiting = config
        .code()
        .entries
        .iter()
        .filter(|entry| privacy.external_tool.is_none() && !entry.is_empty());
    for entry in waiting {
        let Some(category) = categories(entry.category) else {
            continue;
        };
        let head_code = entry.head.as_ref().map(|head| head.to_html(NONCE_SLOT));
        let body_code = entry.body.as_ref().map(|body| body.to_html(NONCE_SLOT));
        let (head_code, body_code) = (head_code.unwrap_or_default(), body_code.unwrap_or_default());
        held = held.with(category, head_code, body_code);
    }
    let locked = match privacy.gpc {
        GpcMode::Marketing => Categories::MARKETING,
        _ => Categories::ALL,
    };
    ConsentSite::new(rev(config).to_string(), held, locked)
}

/// The reader's category code in `category` waits for, or `None` for code
/// that tracks no one and waits for nothing.
fn categories(category: CodeCategory) -> Option<Categories> {
    match category.consent() {
        ConsentCategory::Necessary => None,
        ConsentCategory::Preferences => Some(Categories::PREFERENCES),
        ConsentCategory::Analytics => Some(Categories::ANALYTICS),
        ConsentCategory::ExternalMedia => Some(Categories::EXTERNAL_MEDIA),
        _ => Some(Categories::MARKETING),
    }
}

/// The route the consent form posts to, `<mount>_consent/`, and the
/// site's answer to Global Privacy Control at `/.well-known/gpc.json`.
pub(crate) fn router(config: &Config, security: SecurityHeaders) -> Router {
    let mount = config.mount().to_owned();
    let route = format!("{mount}{CONSENT_FOLDER}/");
    let answering = Answering {
        secure: config.address().as_str().starts_with("https://"),
        days: u32::from(config.privacy().lifetime_months) * DAYS_PER_MONTH,
        site: consent_site(config),
        mount,
        security: security.clone(),
    };
    let honoured = move || {
        let security = security.clone();
        async move { gpc_json(&security) }
    };
    Router::new()
        .route(
            &route,
            post(receive).layer(DefaultBodyLimit::max(BODY_LIMIT)),
        )
        .route(GPC_JSON, get(honoured))
        .with_state(answering)
}

/// `/.well-known/gpc.json`, which says the site honours the signal.
fn gpc_json(security: &SecurityHeaders) -> Response {
    let mut response = Response::new(Body::from(GPC_HONOURED));
    let headers = response.headers_mut();
    let json = HeaderValue::from_static("application/json");
    headers.insert(header::CONTENT_TYPE, json);
    security.apply(headers);
    response
}

/// The consent form as a browser sends it: the button pressed, the page to
/// go back to, and on the choices page, a switch per category, sent only
/// when on.
#[derive(Debug, Deserialize)]
struct ConsentForm {
    choice: String,
    back: String,
    preferences: Option<String>,
    analytics: Option<String>,
    marketing: Option<String>,
    #[serde(rename = "external-media")]
    external_media: Option<String>,
}

impl ConsentForm {
    /// The categories whose switches were on.
    fn switched_on(&self) -> Categories {
        [
            (&self.preferences, Categories::PREFERENCES),
            (&self.analytics, Categories::ANALYTICS),
            (&self.marketing, Categories::MARKETING),
            (&self.external_media, Categories::EXTERNAL_MEDIA),
        ]
        .into_iter()
        .filter(|(switch, _)| switch.is_some())
        .fold(Categories::NONE, |set, (_, category)| set.with(category))
    }
}

/// Keeps the choice in a cookie and sends the reader back. A form posted
/// from another site gets 403, so no other site can choose for a reader;
/// a form too long gets 413.
async fn receive(State(answering): State<Answering>, request: Request) -> Response {
    let from_elsewhere = request
        .headers()
        .get("sec-fetch-site")
        .is_some_and(|site| site == "cross-site");
    let mut response = Response::new(Body::empty());
    answering.security.apply(response.headers_mut());
    if from_elsewhere {
        *response.status_mut() = StatusCode::FORBIDDEN;
        return response;
    }
    let reader = Reader::of(request.headers(), answering.site.rev());
    let form = match Form::<ConsentForm>::from_request(request, &()).await {
        Ok(Form(form)) => form,
        Err(rejection) => {
            *response.status_mut() = rejection.status();
            return response;
        }
    };
    let day = SystemClock.now().timestamp() / i64::from(DAY);
    let day = u32::try_from(day).unwrap_or_default();
    let Some(answer) = answer_for(&form, reader, answering.site.locked(), day) else {
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return response;
    };
    *response.status_mut() = StatusCode::SEE_OTHER;
    let headers = response.headers_mut();
    if let Ok(location) = HeaderValue::try_from(saved_at(back_to(&answering.mount, &form.back))) {
        headers.insert(header::LOCATION, location);
    }
    if let Ok(cookie) = HeaderValue::try_from(cookie(&answering, answer)) {
        headers.insert(header::SET_COOKIE, cookie);
    }
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

/// The answer `form` gives on `day`, from `reader`, whose browser's GPC
/// keeps `locked` off, or `None` for a button the form never has.
///
/// Accept all allows every category GPC leaves on. Save allows the
/// switches turned on, and when one is locked by GPC, records that the
/// reader chose it despite the signal. Allowing external media from a card
/// adds it to a reader's answer, or answers it alone, so the banner still
/// asks about the rest.
fn answer_for(form: &ConsentForm, reader: Reader, locked: Categories, day: u32) -> Option<Answer> {
    let locked = if reader.gpc() {
        locked
    } else {
        Categories::NONE
    };
    let answer = match form.choice.as_str() {
        "accept-all" => Answer::new(Categories::ALL.without(locked), day),
        "reject-all" => Answer::new(Categories::NONE, day),
        "save" => {
            let allowed = form.switched_on();
            Answer {
                over_gpc: !Categories::ALL.without(locked).contains(allowed),
                ..Answer::new(allowed, day)
            }
        }
        "allow-external-media" => match reader.answer() {
            Some(answer) if !answer.media_only => Answer {
                allowed: answer.allowed.with(Categories::EXTERNAL_MEDIA),
                day,
                ..answer
            },
            _ => Answer {
                media_only: true,
                ..Answer::new(Categories::EXTERNAL_MEDIA, day)
            },
        },
        _ => return None,
    };
    Some(answer)
}

/// Where to send the reader: `back` when it is a page of the blog, else
/// the blog's home. Anything else could send the reader to another site.
fn back_to<'a>(mount: &'a str, back: &'a str) -> &'a str {
    let back = back.split_once('#').map_or(back, |(page, _)| page);
    let safe = back.starts_with(mount)
        && !back.contains("//")
        && back
            .bytes()
            .all(|b| b.is_ascii_graphic() && !matches!(b, b'\\' | b'"' | b'<' | b'>'));
    if safe { back } else { mount }
}

/// `page` with the query that shows the saved notice.
fn saved_at(page: &str) -> String {
    let joint = if page.contains('?') { '&' } else { '?' };
    format!("{page}{joint}{SAVED_QUERY}")
}

/// The cookie that keeps `answer`, for as long as the site keeps a choice.
fn cookie(answering: &Answering, answer: Answer) -> String {
    let secure = if answering.secure { "; Secure" } else { "" };
    format!(
        "{CONSENT_COOKIE}={}; Path={}; Max-Age={}; SameSite=Lax; HttpOnly{secure}",
        answer.value(answering.site.rev()),
        answering.mount,
        answering.days.saturating_mul(DAY),
    )
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderMap;

    use super::*;

    fn form(choice: &str, on: &[&str]) -> ConsentForm {
        let switch = |name: &str| on.contains(&name).then(|| "on".to_owned());
        ConsentForm {
            choice: choice.to_owned(),
            back: "/blog/".to_owned(),
            preferences: switch("preferences"),
            analytics: switch("analytics"),
            marketing: switch("marketing"),
            external_media: switch("external-media"),
        }
    }

    fn headers(cookie: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(cookie) = cookie {
            headers.insert(header::COOKIE, HeaderValue::from_str(cookie).unwrap());
        }
        headers
    }

    fn reader(cookie: Option<&str>) -> Reader {
        Reader::of(&headers(cookie), "r")
    }

    fn with_gpc(cookie: Option<&str>) -> Reader {
        let mut headers = headers(cookie);
        headers.insert("sec-gpc", HeaderValue::from_static("1"));
        Reader::of(&headers, "r")
    }

    fn allowed(choice: &str, on: &[&str], reader: Reader) -> Answer {
        answer_for(&form(choice, on), reader, Categories::MARKETING, 9).unwrap()
    }

    #[test]
    fn accept_all_under_gpc_allows_every_category_but_marketing() {
        let all = allowed("accept-all", &[], reader(None));
        assert_eq!(all, Answer::new(Categories::ALL, 9));
        let gpc = allowed("accept-all", &[], with_gpc(None));
        let but_marketing = Categories::ALL.without(Categories::MARKETING);
        assert_eq!(gpc, Answer::new(but_marketing, 9));
        let none = allowed("reject-all", &[], with_gpc(None));
        assert_eq!(none, Answer::new(Categories::NONE, 9));
    }

    #[test]
    fn turning_marketing_on_under_gpc_records_the_reader_s_own_choice() {
        let on = ["analytics", "marketing"];
        let chosen = allowed("save", &on, with_gpc(None));
        let both = Categories::ANALYTICS.with(Categories::MARKETING);
        assert_eq!(chosen.allowed, both);
        assert!(chosen.over_gpc);
        assert!(!allowed("save", &on, reader(None)).over_gpc);
        assert!(!allowed("save", &["analytics"], with_gpc(None)).over_gpc);
    }

    #[test]
    fn allowing_external_media_adds_it_or_answers_it_alone() {
        let alone = allowed("allow-external-media", &[], reader(None));
        assert!(alone.media_only);
        assert_eq!(alone.allowed, Categories::EXTERNAL_MEDIA);
        let analytics = Answer::new(Categories::ANALYTICS, 3).value("r");
        let cookie = format!("{CONSENT_COOKIE}={analytics}");
        let added = allowed("allow-external-media", &[], reader(Some(&cookie)));
        assert!(!added.media_only);
        let both = Categories::ANALYTICS.with(Categories::EXTERNAL_MEDIA);
        assert_eq!(added.allowed, both);
        assert!(answer_for(&form("maybe", &[]), reader(None), Categories::NONE, 9).is_none());
    }

    /// `khalid-server` has no `khalid-content` dependency, so it keeps its
    /// own copy of the embed marks; both must stay the same.
    #[test]
    fn the_server_releases_the_marks_an_embed_is_written_with() {
        assert_eq!(khalid_server::MEDIA_START, khalid_content::MEDIA_START);
        assert_eq!(khalid_server::MEDIA_END, khalid_content::MEDIA_END);
        assert_eq!(
            khalid_server::MEDIA_CARD_START,
            khalid_content::MEDIA_CARD_START
        );
        assert_eq!(
            khalid_server::MEDIA_CARD_END,
            khalid_content::MEDIA_CARD_END
        );
    }

    #[test]
    fn the_reader_goes_back_to_a_page_of_the_blog_and_is_told_it_saved() {
        assert_eq!(
            saved_at(back_to("/blog/", "/blog/a/")),
            "/blog/a/?consent=saved"
        );
        assert_eq!(
            saved_at(back_to("/blog/", "/blog/search/?q=x#top")),
            "/blog/search/?q=x&consent=saved"
        );
        for away in [
            "https://evil.example/",
            "/elsewhere/",
            "/blog//evil.example",
        ] {
            assert_eq!(back_to("/blog/", away), "/blog/", "{away}");
        }
    }
}
