//! A small blog for the Ghost-compatible Content API's tests: posts with
//! authors and tags, one page, and the site lists they name.

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{HeaderMap, Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::content_store::{MemoryContentStore, SiteResources};
use khalid_api::ghost::{self, GhostSite};
use khalid_api::keys::{KeyToken, MemoryKeyStore, StoredKey};
use khalid_api::resources::{Author, Bodies, Collection, Post, Settings, Term};
use khalid_core::{AbsoluteUrl, Config, Environment, Locale, Slug};
use serde_json::Value;
use tower::ServiceExt as _;

/// A content key made up for these tests. It opens nothing real.
pub(crate) const KEY: &str =
    "kc_0123456789abcdef_00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

/// The site's settings, with a tagline, a logo and two menus.
const SITE: &str = r#"name = "Field Notes"
address = "https://example.com"
mount = "/blog/"
default_language = "en"
languages = ["en"]
ghost_content_api = true

[seo.titles]
tagline = "Plain notes on search."

[seo.publisher]
logo = { url = "https://example.com/logo.png", alt = "Field Notes", width = 600, height = 60 }

[[nav.primary]]
label = "About"
href = "/blog/about/"

[[nav.footer]]
label = "SerpDad"
href = "https://serpdad.com/"
"#;

/// A post: slug, title, day in October 2026, tags, authors, featured.
type Demo = (
    &'static str,
    &'static str,
    u32,
    &'static [&'static str],
    &'static [&'static str],
    bool,
);

/// The posts, oldest first.
const POSTS: [Demo; 4] = [
    (
        "anchor-text",
        "Choosing anchor text",
        1,
        &["links", "news"],
        &["asif"],
        true,
    ),
    (
        "slow-pages",
        "Fixing slow pages",
        2,
        &["speed"],
        &["lena"],
        false,
    ),
    (
        "title-tags",
        "Writing title tags",
        3,
        &["news"],
        &["lena", "asif"],
        false,
    ),
    (
        "sitemaps",
        "What a sitemap does",
        4,
        &["links"],
        &["asif"],
        false,
    ),
];

#[cfg(test)]
fn slug(text: &str) -> Slug {
    Slug::try_new(text).unwrap()
}

#[cfg(test)]
fn entry(slug_text: &str, title: &str, day: u32, words: usize) -> Post {
    let en = Locale::try_new("en").unwrap();
    let url = AbsoluteUrl::parse(&format!("https://example.com/blog/{slug_text}/")).unwrap();
    let at = Utc.with_ymd_and_hms(2026, 10, day, 9, 0, 0).unwrap();
    let text = vec!["word"; words].join(" ");
    let html = format!("<p>{text}</p>");
    Post::new(
        &format!("en.{slug_text}"),
        slug(slug_text),
        title,
        url,
        en,
        at,
    )
    .with_bodies(Bodies::new(&html, &text, &text))
}

/// The store behind every test.
#[cfg(test)]
pub(crate) fn store() -> MemoryContentStore {
    let owned = |names: &[&str]| names.iter().map(|name| (*name).to_owned()).collect();
    let posts = POSTS
        .iter()
        .map(|(slug, title, day, tags, authors, featured)| {
            let post = entry(slug, title, *day, 600)
                .with_tags(owned(tags))
                .with_authors(owned(authors));
            let post = if *featured {
                post.featured().with_excerpt("Pick the words with care.")
            } else {
                post
            };
            (Collection::Posts, post)
        });
    let about = (Collection::Pages, entry("about", "About", 1, 40));
    let en = Locale::try_new("en").unwrap();
    let url = |text: &str| AbsoluteUrl::parse(text).unwrap();
    let site = SiteResources::default()
        .with_authors(vec![
            Author::new(slug("asif"), "Asif", en.clone())
                .with_bio("Founder of SerpDad.")
                .with_url(url("https://example.com/blog/authors/asif/")),
            Author::new(slug("lena"), "Lena", en.clone()),
        ])
        .with_tags(vec![
            Term::new(slug("links"), "Links", en.clone()),
            Term::new(slug("news"), "News", en.clone()),
            Term::new(slug("speed"), "Speed", en.clone()),
        ])
        .with_settings(Settings::new(
            "Field Notes",
            url("https://example.com/blog/"),
            en,
            Vec::new(),
        ));
    MemoryContentStore::new(posts.chain([about]).collect()).with_site(site)
}

/// The settings in [`SITE`].
#[cfg(test)]
pub(crate) fn config() -> Config {
    Config::from_toml(Path::new("site.toml"), SITE, &Environment::empty()).unwrap()
}

/// The Ghost-compatible router over [`store`], with [`KEY`] stored.
#[cfg(test)]
pub(crate) fn router() -> Router {
    let token: KeyToken = KEY.parse().unwrap();
    let keys = MemoryKeyStore::new(vec![StoredKey::for_token(&token)]);
    ghost::router(
        Arc::new(store()),
        Arc::new(keys),
        GhostSite::from_config(&config()),
    )
}

/// `uri` sent with `headers`, as status, headers and JSON (or null).
#[cfg(test)]
pub(crate) async fn send(uri: &str, headers: &[(&str, &str)]) -> (StatusCode, HeaderMap, Value) {
    let mut request = Request::get(uri);
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    let response = router()
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (parts, body) = response.into_parts();
    let body = to_bytes(body, 4 * 1024 * 1024).await.unwrap();
    let json = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (parts.status, parts.headers, json)
}

/// `path` under `/ghost/api/content/` with the key, as Ghost's client
/// sends it: the key in the query and `Accept-Version: v5.0`.
#[cfg(test)]
pub(crate) async fn get(path: &str) -> (StatusCode, Value) {
    let join = if path.contains('?') { '&' } else { '?' };
    let uri = format!("/ghost/api/content/{path}{join}key={KEY}");
    let (status, _, json) = send(&uri, &[("accept-version", "v5.0")]).await;
    (status, json)
}

/// The first error in Ghost's error shape, as its type and message.
#[cfg(test)]
pub(crate) fn error(json: &Value) -> (&str, &str) {
    let first = &json["errors"][0];
    (
        first["type"].as_str().unwrap(),
        first["message"].as_str().unwrap(),
    )
}
