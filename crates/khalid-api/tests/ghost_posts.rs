//! The Ghost-compatible Content API's posts and pages (R15, KTD13): Ghost's
//! wrapper and field names, read by slug or id, pages, order and formats.

mod ghost;

use axum::http::StatusCode;
use ghost::{error, get};
use serde_json::Value;

/// The slugs of the entries under `name` in `json`.
#[cfg(test)]
fn slugs<'a>(json: &'a Value, name: &str) -> Vec<&'a str> {
    let entries = json[name].as_array().unwrap();
    let slug = |entry: &'a Value| entry["slug"].as_str().unwrap();
    entries.iter().map(slug).collect()
}

#[tokio::test]
async fn posts_come_newest_first_in_ghosts_wrapper() {
    let (status, json) = get("posts/").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(
        slugs(&json, "posts"),
        ["sitemaps", "title-tags", "slow-pages", "anchor-text"]
    );
    let pagination = &json["meta"]["pagination"];
    assert_eq!(pagination["page"], 1);
    assert_eq!(pagination["limit"], 15);
    assert_eq!(pagination["pages"], 1);
    assert_eq!(pagination["total"], 4);
    assert!(pagination["next"].is_null() && pagination["prev"].is_null());
}

#[tokio::test]
async fn a_post_carries_ghosts_field_names_and_neutral_defaults() {
    let (_, json) = get("posts/").await;
    let post = &json["posts"][3];
    assert_eq!(post["id"], "en.anchor-text");
    assert_eq!(post["title"], "Choosing anchor text");
    assert_eq!(post["url"], "https://example.com/blog/anchor-text/");
    assert_eq!(post["featured"], true);
    assert_eq!(post["custom_excerpt"], "Pick the words with care.");
    assert_eq!(post["excerpt"], "Pick the words with care.");
    assert_eq!(post["published_at"], "2026-10-01T09:00:00.000Z");
    assert_eq!(post["updated_at"], "2026-10-01T09:00:00.000Z");
    assert_eq!(post["reading_time"], 2);
    assert_eq!(
        post["html"],
        format!("<p>{}</p>", vec!["word"; 600].join(" "))
    );
    assert_eq!(post["visibility"], "public");
    assert_eq!(post["access"], true);
    assert_eq!(post["comments"], false);
    assert!(post["codeinjection_head"].is_null() && post["codeinjection_foot"].is_null());
    assert!(post["feature_image"].is_null());
    assert_eq!(post["uuid"].as_str().map(str::len), Some(36));
    assert!(
        post.get("plaintext").is_none() && post.get("tags").is_none(),
        "{post}"
    );
}

#[tokio::test]
async fn an_excerpt_without_a_custom_one_is_the_text_cut_short() {
    let (_, json) = get("posts/slug/slow-pages/").await;
    let post = &json["posts"][0];
    assert!(post["custom_excerpt"].is_null());
    let excerpt = post["excerpt"].as_str().unwrap();
    assert!(
        excerpt.starts_with("word word") && excerpt.len() <= 500,
        "{excerpt}"
    );
}

#[tokio::test]
async fn one_post_is_read_by_slug_or_id_as_ghosts_client_asks() {
    // The client repeats the slug in the query; it must not get in the way.
    let (status, by_slug) = get("posts/slug/title-tags/?slug=title-tags").await;
    assert_eq!(status, StatusCode::OK, "{by_slug}");
    assert_eq!(slugs(&by_slug, "posts"), ["title-tags"]);
    assert!(by_slug.get("meta").is_none());
    let (status, by_id) = get("posts/en.title-tags/").await;
    assert_eq!(
        (status, slugs(&by_id, "posts")),
        (StatusCode::OK, vec!["title-tags"])
    );
}

#[tokio::test]
async fn pages_have_the_same_shape_as_posts() {
    let (status, json) = get("pages/").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(slugs(&json, "pages"), ["about"]);
    assert_eq!(json["pages"][0]["reading_time"], 1);
    let (status, json) = get("pages/slug/about/").await;
    assert_eq!(
        (status, slugs(&json, "pages")),
        (StatusCode::OK, vec!["about"])
    );
}

#[tokio::test]
async fn nothing_at_an_address_is_ghosts_404() {
    for path in [
        "posts/slug/no-such-post/",
        "posts/en.nothing/",
        "pages/slug/anchor-text/",
        "posts/slug/Not%20A%20Slug/",
    ] {
        let (status, json) = get(path).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{path}");
        assert_eq!(error(&json).0, "NotFoundError", "{path}");
    }
}

#[tokio::test]
async fn limit_and_page_split_the_list_as_ghost_does() {
    let (status, json) = get("posts/?limit=2&page=2").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(slugs(&json, "posts"), ["slow-pages", "anchor-text"]);
    let pagination = &json["meta"]["pagination"];
    assert_eq!(
        (pagination["pages"].as_u64(), pagination["prev"].as_u64()),
        (Some(2), Some(1))
    );
    let (_, json) = get("posts/?limit=all").await;
    assert_eq!(json["meta"]["pagination"]["limit"], "all");
    assert_eq!(slugs(&json, "posts").len(), 4);
}

#[tokio::test]
async fn order_and_plaintext_are_answered() {
    let (status, json) = get("posts/?order=published_at%20asc&formats=plaintext").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(slugs(&json, "posts")[0], "anchor-text");
    let post = &json["posts"][0];
    assert!(post["plaintext"].as_str().unwrap().starts_with("word word"));
    assert!(post.get("html").is_none(), "{post}");
}

#[tokio::test]
async fn a_list_parameter_that_cannot_be_read_is_ghosts_400() {
    for path in [
        "posts/?limit=many",
        "posts/?order=colour%20asc",
        "posts/?formats=mobiledoc",
    ] {
        let (status, json) = get(path).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{path}");
        assert_eq!(error(&json).0, "BadRequestError", "{path}");
    }
}
