//! The Ghost-compatible Content API's tags and authors (R15, KTD13), and
//! posts with `include=tags,authors` and `fields`.

mod ghost;

use axum::http::StatusCode;
use ghost::{error, get};
use serde_json::Value;

/// The values of `field` in the entries under `name` in `json`.
#[cfg(test)]
fn each<'a>(json: &'a Value, name: &str, field: &str) -> Vec<&'a str> {
    let entries = json[name].as_array().unwrap();
    let value = |entry: &'a Value| entry[field].as_str().unwrap();
    entries.iter().map(value).collect()
}

#[tokio::test]
async fn tags_are_listed_and_read_in_ghosts_shape() {
    let (status, json) = get("tags/").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(each(&json, "tags", "slug"), ["links", "news", "speed"]);
    assert_eq!(json["meta"]["pagination"]["total"], 3);
    let tag = &json["tags"][1];
    assert_eq!(tag["id"], "en.news");
    assert_eq!(tag["name"], "News");
    assert_eq!(tag["visibility"], "public");
    assert!(tag["description"].is_null() && tag["feature_image"].is_null());
    let (status, json) = get("tags/slug/speed/").await;
    assert_eq!(
        (status, each(&json, "tags", "name")),
        (StatusCode::OK, vec!["Speed"])
    );
    let (status, json) = get("tags/en.links/").await;
    assert_eq!(
        (status, each(&json, "tags", "slug")),
        (StatusCode::OK, vec!["links"])
    );
}

#[tokio::test]
async fn authors_are_listed_and_read_in_ghosts_shape() {
    let (status, json) = get("authors/?limit=1").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(each(&json, "authors", "slug"), ["asif"]);
    assert_eq!(json["meta"]["pagination"]["next"], 2);
    let author = &json["authors"][0];
    assert_eq!(author["name"], "Asif");
    assert_eq!(author["bio"], "Founder of SerpDad.");
    assert_eq!(author["url"], "https://example.com/blog/authors/asif/");
    assert!(author["profile_image"].is_null() && author["website"].is_null());
    let (status, json) = get("authors/slug/lena/").await;
    assert_eq!(
        (status, each(&json, "authors", "name")),
        (StatusCode::OK, vec!["Lena"])
    );
    assert!(json["authors"][0]["bio"].is_null());
    let (status, json) = get("authors/slug/nobody/").await;
    assert_eq!(
        (status, error(&json).0),
        (StatusCode::NOT_FOUND, "NotFoundError")
    );
}

#[tokio::test]
async fn include_adds_tags_and_authors_with_the_primary_ones() {
    let (status, json) = get("posts/slug/title-tags/?include=tags,authors").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let post = &json["posts"][0];
    assert_eq!(each(post, "tags", "slug"), ["news"]);
    assert_eq!(each(post, "authors", "slug"), ["lena", "asif"]);
    assert_eq!(post["primary_tag"]["name"], "News");
    assert_eq!(post["primary_author"]["slug"], "lena");
    assert!(post["authors"][1]["profile_image"].is_null());
}

#[tokio::test]
async fn include_works_on_lists_and_leaves_out_what_was_not_asked() {
    let (status, json) = get("posts/?include=authors&limit=all").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    for post in json["posts"].as_array().unwrap() {
        assert!(post["primary_author"]["name"].is_string(), "{post}");
        assert!(post.get("tags").is_none() && post.get("primary_tag").is_none());
    }
    let (_, json) = get("pages/?include=tags").await;
    let page = &json["pages"][0];
    assert_eq!(page["tags"], serde_json::json!([]));
    assert!(page["primary_tag"].is_null());
}

#[tokio::test]
async fn fields_keeps_only_the_fields_asked_for() {
    let (status, json) = get("posts/?fields=title,url&include=tags").await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let post = json["posts"][0].as_object().unwrap();
    let mut keys: Vec<&str> = post.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["primary_tag", "tags", "title", "url"]);
    let (_, json) = get("tags/?fields=slug").await;
    let tag = json["tags"][0].as_object().unwrap();
    assert_eq!(tag.keys().collect::<Vec<_>>(), ["slug"]);
}

#[tokio::test]
async fn an_include_this_api_cannot_answer_is_ghosts_400() {
    for path in ["posts/?include=tiers", "tags/?include=count.posts"] {
        let (status, json) = get(path).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{path}");
        let (kind, message) = error(&json);
        assert_eq!(kind, "BadRequestError", "{path}");
        assert!(message.contains("include"), "{message}");
    }
}
