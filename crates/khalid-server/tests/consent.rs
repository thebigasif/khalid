//! A site that asks readers for consent per category (privacy plan U3):
//! a reader who has not answered gets a kept page, the GPC copy when their
//! browser sends the signal, each with its tag and Brotli copy; a reader
//! who answered gets a page of their own with the code of each category
//! they allowed, under the nonce of its policy.

use axum::body::{Body, to_bytes};
use axum::http::header::{self, HeaderName};
use axum::http::{Request, Response, StatusCode};
use khalid_server::{
    ASK_END, ASK_START, Answer, CHOSEN_END, CHOSEN_START, Categories, ConsentSite, GPC_END,
    GPC_START, HELD_END, HELD_START, HeldCode, MEDIA_END, MEDIA_START, MediaType, NO_GPC_END,
    NO_GPC_START, NONCE_SLOT, Page, PagePath, Pages, SAVED_END, SAVED_START, SITE_BODY_SLOT,
    SITE_HEAD_SLOT, SecurityHeaders, router_with_consent,
};
use tower::ServiceExt;

const REV: &str = "0badcafe";

/// A page as the build renders it for a site that asks: the slots, the
/// banner with its Marketing row and GPC note, a player, the saved notice
/// and the hidden form to change a choice, with text long enough to get a
/// Brotli copy.
#[cfg(test)]
fn page_html(head: &str) -> String {
    format!(
        "<html><head><title>Post</title>{head}{SITE_HEAD_SLOT}</head><body>{SAVED_START}<p>Your choices are saved.</p>{SAVED_END}{}{MEDIA_START}<iframe src=\"https://player.example/x\"></iframe>{MEDIA_END}{SITE_BODY_SLOT}{ASK_START}<aside>May we count visits?{NO_GPC_START}<label>Ads</label>{NO_GPC_END}{GPC_START}<p>Your browser asked us not to sell or share.</p>{GPC_END}</aside>{ASK_END}{CHOSEN_START}<aside>Change your choice</aside>{CHOSEN_END}</body></html>",
        "<p>A sitemap lists the pages you want found.</p>".repeat(40)
    )
}

#[cfg(test)]
fn pages() -> Pages {
    let path = |path: &str| PagePath::try_new(path).unwrap();
    let held = format!("{HELD_START}<script nonce=\"{NONCE_SLOT}\">post()</script>{HELD_END}");
    let pages = vec![
        Page::new(path("/blog/plain/"), MediaType::Html, page_html("")),
        Page::with_nonce_slot(path("/blog/tagged/"), MediaType::Html, page_html(&held)),
        Page::new(path("/blog/site.css"), MediaType::Css, "p{color:#123}"),
    ];
    Pages::new(pages, page_html("")).unwrap()
}

/// The cookie for an answer allowing `allowed`.
#[cfg(test)]
fn cookie(allowed: Categories) -> String {
    format!("khalid-consent={}", Answer::new(allowed, 20_400).value(REV))
}

#[cfg(test)]
async fn get(path: &str, headers: &[(&str, &str)]) -> (Response<Body>, String) {
    let held = HeldCode::new()
        .with(
            Categories::ANALYTICS,
            format!("<script nonce=\"{NONCE_SLOT}\">stats()</script>"),
            "",
        )
        .with(
            Categories::MARKETING,
            format!("<script nonce=\"{NONCE_SLOT}\">site()</script>"),
            format!("<script nonce=\"{NONCE_SLOT}\">chat()</script>"),
        );
    let site = ConsentSite::new(REV, held, Categories::MARKETING);
    let mut request = Request::get(path);
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    let request = request.body(Body::empty()).unwrap();
    let router = router_with_consent(pages(), SecurityHeaders::default(), site);
    let response = router.oneshot(request).await.unwrap();
    let (parts, body) = response.into_parts();
    let text = to_bytes(body, 1 << 20).await.unwrap();
    let text = String::from_utf8_lossy(&text).into_owned();
    (Response::from_parts(parts, Body::empty()), text)
}

#[cfg(test)]
fn header<'r>(response: &'r Response<Body>, name: &HeaderName) -> &'r str {
    response
        .headers()
        .get(name)
        .map_or("", |value| value.to_str().unwrap())
}

#[cfg(test)]
fn directive(response: &Response<Body>, name: &str) -> String {
    let policy = header(response, &header::CONTENT_SECURITY_POLICY);
    let found = policy
        .split(';')
        .map(str::trim)
        .find(|p| p.starts_with(name));
    found
        .unwrap_or_else(|| panic!("no {name} in {policy}"))
        .to_owned()
}

/// The nonce of the first script in `html`.
#[cfg(test)]
fn nonce_in(html: &str) -> String {
    let (_, rest) = html.split_once("<script nonce=\"").unwrap();
    rest.split_once('"').unwrap().0.to_owned()
}

#[tokio::test]
async fn a_reader_who_has_not_answered_gets_the_kept_page_with_no_code() {
    let (answer, html) = get("/blog/plain/", &[]).await;
    assert_eq!(answer.status(), StatusCode::OK);
    assert!(html.contains("<label>Ads</label>"), "{html}");
    assert!(!html.contains("()"), "{html}");
    assert!(html.contains("<template><iframe"), "{html}");
    assert!(answer.headers().contains_key(header::ETAG), "the kept page");
    assert_eq!(
        header(&answer, &header::VARY),
        "accept-encoding, cookie, sec-gpc"
    );
    // A post's own code waits too.
    let (answer, html) = get("/blog/tagged/", &[]).await;
    assert!(
        html.contains("May we count visits?") && !html.contains("()"),
        "{html}"
    );
    assert_eq!(directive(&answer, "style-src"), "style-src 'self'");
}

#[tokio::test]
async fn a_reader_with_gpc_gets_the_kept_gpc_copy_with_its_own_tag_and_brotli() {
    let (plain, _) = get("/blog/plain/", &[]).await;
    let (answer, html) = get("/blog/plain/", &[("sec-gpc", "1")]).await;
    assert_eq!(answer.status(), StatusCode::OK);
    assert!(!html.contains("Ads"), "no Marketing row: {html}");
    assert!(
        html.contains("<p>Your browser asked us not to sell or share.</p>"),
        "{html}"
    );
    assert!(!html.contains("()"), "{html}");
    let tag = header(&answer, &header::ETAG);
    assert!(!tag.is_empty() && tag != header(&plain, &header::ETAG));
    assert_eq!(header(&answer, &header::CACHE_CONTROL), "no-cache");
    let (again, _) = get("/blog/plain/", &[("sec-gpc", "1"), ("if-none-match", tag)]).await;
    assert_eq!(again.status(), StatusCode::NOT_MODIFIED);
    let brotli = [("sec-gpc", "1"), ("accept-encoding", "br")];
    let (compressed, _) = get("/blog/plain/", &brotli).await;
    assert_eq!(header(&compressed, &header::CONTENT_ENCODING), "br");
    assert_eq!(
        header(&compressed, &header::VARY),
        "accept-encoding, cookie, sec-gpc"
    );
}

#[tokio::test]
async fn a_crawler_with_no_cookie_gets_the_plain_kept_page() {
    let (person, html) = get("/blog/plain/", &[]).await;
    let bot = [("user-agent", "Mozilla/5.0 (compatible; Googlebot/2.1)")];
    let (bot_answer, crawled) = get("/blog/plain/", &bot).await;
    assert_eq!(crawled, html);
    assert_eq!(
        header(&bot_answer, &header::ETAG),
        header(&person, &header::ETAG)
    );
}

#[tokio::test]
async fn a_reader_who_allowed_everything_gets_every_piece_with_the_policy_s_nonce() {
    let cookie = format!("a=1; {}", cookie(Categories::ALL));
    let (answer, html) = get("/blog/tagged/", &[("cookie", &cookie)]).await;
    assert!(!html.contains("May we count visits?"), "{html}");
    assert!(html.contains("<aside>Change your choice</aside>"), "{html}");
    assert!(html.contains("<iframe src"), "{html}");
    let nonce = nonce_in(&html);
    assert_eq!(nonce.len(), 32, "{html}");
    for code in ["post()", "stats()", "site()", "chat()"] {
        let script = format!("<script nonce=\"{nonce}\">{code}</script>");
        assert!(html.contains(&script), "{script} in {html}");
    }
    assert_eq!(
        directive(&answer, "script-src"),
        format!("script-src 'nonce-{nonce}' 'strict-dynamic'")
    );
    assert_eq!(header(&answer, &header::CACHE_CONTROL), "private, no-store");
    assert_eq!(header(&answer, &header::VARY), "cookie, sec-gpc");
    assert!(!answer.headers().contains_key(header::ETAG));
}

#[tokio::test]
async fn gpc_keeps_marketing_held_for_a_reader_who_accepted_all() {
    let cookie = cookie(Categories::ALL);
    let (_, html) = get("/blog/tagged/", &[("cookie", &cookie), ("sec-gpc", "1")]).await;
    assert!(html.contains("stats()"), "{html}");
    for marketing in ["post()", "site()", "chat()"] {
        assert!(!html.contains(marketing), "{marketing} in {html}");
    }
    let mut chosen = Answer::new(Categories::MARKETING, 20_400);
    chosen.over_gpc = true;
    let cookie = format!("khalid-consent={}", chosen.value(REV));
    let (_, html) = get("/blog/tagged/", &[("cookie", &cookie), ("sec-gpc", "1")]).await;
    assert!(html.contains("site()") && html.contains("post()"), "{html}");
}

#[tokio::test]
async fn allowing_external_media_alone_loads_players_and_keeps_the_banner() {
    let media = Answer {
        media_only: true,
        ..Answer::new(Categories::EXTERNAL_MEDIA, 20_400)
    };
    let cookie = format!("khalid-consent={}", media.value(REV));
    let (answer, html) = get("/blog/plain/", &[("cookie", &cookie)]).await;
    assert!(html.contains("</p><iframe src"), "{html}");
    assert!(html.contains("May we count visits?"), "{html}");
    assert!(!html.contains("()"), "{html}");
    assert_eq!(header(&answer, &header::CACHE_CONTROL), "private, no-store");
}

#[tokio::test]
async fn a_cookie_for_older_settings_or_an_old_yes_asks_again() {
    let stale = "khalid-consent=2.12345678.15.20400.0";
    for cookie in [stale, "khalid-consent=yes"] {
        let (answer, html) = get("/blog/plain/", &[("cookie", cookie)]).await;
        assert!(html.contains("May we count visits?"), "{cookie}: {html}");
        assert!(!html.contains("()"), "{cookie}: {html}");
        assert!(
            !html.contains("Change your choice</aside></body>"),
            "{html}"
        );
        assert!(answer.headers().contains_key(header::ETAG), "the kept page");
    }
}

#[tokio::test]
async fn an_old_no_gets_neither_code_nor_banner() {
    let (answer, html) = get("/blog/tagged/", &[("cookie", "khalid-consent=no")]).await;
    assert!(!html.contains("()"), "{html}");
    assert!(!html.contains("May we count visits?"), "{html}");
    assert!(html.contains("<aside>Change your choice</aside>"), "{html}");
    assert_eq!(directive(&answer, "connect-src"), "connect-src 'self'");
    assert_eq!(header(&answer, &header::CACHE_CONTROL), "private, no-store");
}

#[tokio::test]
async fn the_page_a_reader_returns_to_after_saving_says_so() {
    let cookie = cookie(Categories::NONE);
    let (_, html) = get("/blog/plain/?consent=saved", &[("cookie", &cookie)]).await;
    assert!(
        html.contains("<body><p>Your choices are saved.</p>"),
        "{html}"
    );
    let (_, html) = get("/blog/plain/", &[("cookie", &cookie)]).await;
    assert!(!html.contains("<body><p>Your choices"), "{html}");
}

#[tokio::test]
async fn files_stay_as_they_are_and_the_not_found_page_follows_the_choice() {
    let cookie = cookie(Categories::ALL);
    let (answer, body) = get("/blog/site.css", &[("cookie", &cookie)]).await;
    assert_eq!(body, "p{color:#123}");
    assert!(answer.headers().contains_key(header::ETAG));
    let (answer, html) = get("/blog/no-such-post/", &[("cookie", &cookie)]).await;
    assert_eq!(answer.status(), StatusCode::NOT_FOUND);
    assert!(
        html.contains("site()") && !html.contains("May we"),
        "{html}"
    );
    let (_, html) = get("/blog/no-such-post/", &[("sec-gpc", "1")]).await;
    assert!(!html.contains("<label>Ads"), "{html}");
}
