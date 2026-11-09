//! The common part of Ghost's filter language on the Ghost-compatible
//! Content API (R15, KTD13), and Ghost's 400 for the rest.

mod ghost;

use axum::http::StatusCode;
use ghost::{error, get};
use serde_json::Value;

/// The slugs `path` answers with under `name`, after checking it is a 200.
#[cfg(test)]
async fn slugs(path: &str, name: &str) -> Vec<String> {
    let (status, json) = get(path).await;
    assert_eq!(status, StatusCode::OK, "{path}: {json}");
    let entries = json[name].as_array().unwrap();
    let slug = |entry: &Value| entry["slug"].as_str().unwrap().to_owned();
    entries.iter().map(slug).collect()
}

#[tokio::test]
async fn and_or_lists_and_not_return_what_ghost_would() {
    let cases: [(&str, &[&str]); 7] = [
        ("tag:news+featured:true", &["anchor-text"]),
        (
            "tag:[speed,news]",
            &["title-tags", "slow-pages", "anchor-text"],
        ),
        (
            "primary_tag.slug:-news",
            &["sitemaps", "slow-pages", "anchor-text"],
        ),
        ("tag:speed,featured:true", &["slow-pages", "anchor-text"]),
        ("authors.slug:lena", &["title-tags", "slow-pages"]),
        ("primary_author:asif+tags.slug:-[links]", &[]),
        ("slug:[anchor-text,sitemaps]", &["sitemaps", "anchor-text"]),
    ];
    for (filter, expected) in cases {
        // Ghost's client sends the filter through encodeURIComponent.
        let sent = filter
            .replace('+', "%2B")
            .replace('[', "%5B")
            .replace(']', "%5D");
        let found = slugs(&format!("posts/?filter={sent}"), "posts").await;
        assert_eq!(found, expected, "{filter}");
    }
}

#[tokio::test]
async fn dates_compare_in_either_of_ghosts_forms() {
    let found = slugs(
        "posts/?filter=published_at:%3E'2026-10-02%2012:00:00'",
        "posts",
    )
    .await;
    assert_eq!(found, ["sitemaps", "title-tags"]);
    let found = slugs(
        "posts/?filter=published_at:%3C='2026-10-02T09:00:00.000Z'",
        "posts",
    )
    .await;
    assert_eq!(found, ["slow-pages", "anchor-text"]);
}

#[tokio::test]
async fn status_and_id_filters_answer_as_on_a_ghost_site() {
    assert_eq!(
        slugs("posts/?filter=status:published", "posts").await.len(),
        4
    );
    let public = slugs("posts/?filter=visibility:public", "posts").await;
    assert_eq!(public.len(), 4);
    let drafts = slugs("posts/?filter=status:draft", "posts").await;
    assert_eq!(drafts, Vec::<String>::new());
    let found = slugs("posts/?filter=id:-'en.sitemaps'%2Bfeatured:false", "posts").await;
    assert_eq!(found, ["title-tags", "slow-pages"]);
}

#[tokio::test]
async fn filters_work_on_reads_pages_tags_and_authors() {
    assert_eq!(slugs("pages/?filter=slug:about", "pages").await, ["about"]);
    let tags = slugs("tags/?filter=visibility:public%2Bslug:-speed", "tags").await;
    assert_eq!(tags, ["links", "news"]);
    assert_eq!(
        slugs("authors/?filter=slug:lena", "authors").await,
        ["lena"]
    );
}

#[tokio::test]
async fn a_filter_outside_the_common_part_is_ghosts_400_naming_it() {
    let cases = [
        ("posts/?filter=title:~'slow'", "title"),
        ("posts/?filter=tag:~'ne'", "~"),
        ("posts/?filter=featured:%3Etrue", "featured"),
        ("posts/?filter=custom_excerpt:null", "custom_excerpt"),
        ("posts/?filter=published_at:%3E'soon'", "soon"),
        ("tags/?filter=description:x", "description"),
        ("posts/?filter=tag:", "filter"),
    ];
    for (path, named) in cases {
        let (status, json) = get(path).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{path}: {json}");
        let (kind, message) = error(&json);
        assert_eq!(kind, "BadRequestError", "{path}");
        let context = json["errors"][0]["context"].as_str().unwrap();
        let said = format!("{message} {context}");
        assert!(said.contains(named), "{path}: {said}");
    }
}
