//! `/api/oembed` over HTTP: another site given a post's address gets the
//! post as a card, with no key (plan 5.5).

use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use khalid_api::api;
use khalid_api::content_store::{MemoryContentStore, SiteResources};
use khalid_api::keys::MemoryKeyStore;
use khalid_api::post_store::MemoryPostStore;
use khalid_api::resources::{Author, Collection, Post, PostHead, Settings};
use khalid_core::{AbsoluteUrl, Locale, Slug};
use khalid_seo::{PageKind, SeoPage, SeoSite, ShareImage, UrlPath, head_tags, page_graph};
use serde_json::{Value, json};
use tower::ServiceExt;

/// The post's address.
const POST: &str = "https://example.com/blog/links-and-anchors/";

/// A post with an author, a summary that needs escaping and a share image,
/// on a site with its settings.
#[cfg(test)]
fn router() -> Router {
    let en = Locale::try_new("en").unwrap();
    let url = |text: &str| AbsoluteUrl::parse(text).unwrap();
    let mut seo = SeoSite::new("Field Notes", url("https://example.com/"), en.clone());
    seo.mount = UrlPath::try_new("/blog/").unwrap();
    let image = url("https://example.com/blog/media/links.png");
    seo.share_image = Some(ShareImage::new(image, 1200, 630, "Two linked pages"));
    let path = UrlPath::try_new("/links-and-anchors/").unwrap();
    let page = SeoPage::new(PageKind::Post, en.clone(), path, "Links & anchors");
    let head = PostHead::new(
        &head_tags(&seo, &page).unwrap(),
        &page_graph(&seo, &page).unwrap(),
    );
    let day = Utc.with_ymd_and_hms(2026, 10, 1, 9, 0, 0).unwrap();
    let slug = Slug::try_new("links-and-anchors").unwrap();
    let post = Post::new(
        "links-en",
        slug,
        "Links & anchors",
        url(POST),
        en.clone(),
        day,
    )
    .with_excerpt("Tom's <guide> to links.")
    .with_authors(vec!["asif".to_owned()])
    .with_head(Some(&head));
    let asif = Author::new(Slug::try_new("asif").unwrap(), "Asif", en.clone())
        .with_url(url("https://example.com/blog/authors/asif/"));
    let settings = Settings::new("Field Notes", url("https://example.com/blog/"), en, vec![]);
    let site = SiteResources::default()
        .with_authors(vec![asif])
        .with_settings(settings);
    let content = MemoryContentStore::new(vec![(Collection::Posts, post)]).with_site(site);
    let keys = MemoryKeyStore::new(vec![]);
    let posts = MemoryPostStore::default();
    api::router(Arc::new(content), Arc::new(posts), Arc::new(keys)).0
}

#[cfg(test)]
async fn get(uri: &str) -> (StatusCode, String, String) {
    let request = Request::get(uri).body(Body::empty()).unwrap();
    let response = router().oneshot(request).await.unwrap();
    let media = response.headers().get("content-type");
    let media = media.map(|value| value.to_str().unwrap().to_owned());
    let status = response.status();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        media.unwrap_or_default(),
        String::from_utf8(body.to_vec()).unwrap(),
    )
}

#[tokio::test]
async fn a_post_s_address_gets_its_card_with_no_key() {
    let copied = "https%3A%2F%2Fexample.com%2Fblog%2Flinks-and-anchors%3Futm_source%3Dx";
    let (status, media, body) = get(&format!("/api/oembed?url={copied}&maxwidth=400")).await;
    assert_eq!(
        (status, media.as_str()),
        (StatusCode::OK, "application/json"),
        "{body}"
    );
    let card: Value = serde_json::from_str(&body).unwrap();
    let html = "<blockquote class=\"khalid-embed\" cite=\"https://example.com/blog/links-and-anchors/\">\
        <p><a href=\"https://example.com/blog/links-and-anchors/\">Links &amp; anchors</a></p>\
        <p>Tom&#39;s &lt;guide&gt; to links.</p>\
        <p><a href=\"https://example.com/blog/\">Field Notes</a></p></blockquote>";
    let expected = json!({
        "type": "rich",
        "version": "1.0",
        "title": "Links & anchors",
        "author_name": "Asif",
        "author_url": "https://example.com/blog/authors/asif/",
        "provider_name": "Field Notes",
        "provider_url": "https://example.com/blog/",
        "thumbnail_url": "https://example.com/blog/media/links.png",
        "thumbnail_width": 1200,
        "thumbnail_height": 630,
        "html": html,
        "width": 400,
        "height": null,
    });
    assert_eq!(card, expected);
}

#[tokio::test]
async fn an_unknown_address_is_404_and_a_missing_one_400() {
    for unknown in [
        "https%3A%2F%2Fexample.com%2Fblog%2Fnot-a-post%2F",
        "https%3A%2F%2Felsewhere.example%2Fblog%2Flinks-and-anchors%2F",
        "https%3A%2F%2Fexample.com%2F",
    ] {
        let (status, media, body) = get(&format!("/api/oembed?url={unknown}")).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{unknown}: {body}");
        assert_eq!(media, "application/problem+json");
    }
    let (status, _, body) = get("/api/oembed").await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(body.contains("?url="), "{body}");
}

#[tokio::test]
async fn only_json_is_served() {
    let url = "https%3A%2F%2Fexample.com%2Fblog%2Flinks-and-anchors%2F";
    let (status, _, _) = get(&format!("/api/oembed?url={url}&format=xml")).await;
    assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    let (status, _, _) = get(&format!("/api/oembed?url={url}&format=json")).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn the_document_lists_the_provider_with_no_key() {
    let (_, _, body) = get("/api/openapi.json").await;
    let document: Value = serde_json::from_str(&body).unwrap();
    let operation = &document["paths"]["/api/oembed"]["get"];
    assert!(operation.is_object(), "{document}");
    assert_eq!(operation["security"], Value::Null, "{operation}");
}
