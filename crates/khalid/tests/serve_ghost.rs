//! Ghost's Content API on a running `khalid serve` (R15, KTD13): off by
//! default, so `/ghost/api/content/` is any unknown address; on, it answers
//! under the blog's mount with a key from `khalid keys create --kind
//! content`, and Khalid's own APIs are unchanged.

mod common;

use std::fs;
use std::path::Path;

use common::{Server, demo_copy, new_key, start};
use serde_json::Value;

/// A content key, as a Ghost front end holds.
const CONTENT: [&str; 4] = ["--name", "Front end", "--kind", "content"];

/// An editor's key, so a test can read the same post through the Admin API.
const EDITOR: [&str; 4] = ["--name", "Editor", "--scope", "posts:write"];

/// Switches Ghost's Content API on in the site at `site`. The setting is a
/// top-level key, so it goes above the first table.
#[cfg(test)]
fn switch_on(site: &Path) {
    let mut file = site.to_path_buf();
    file.push("site.toml");
    let text = fs::read_to_string(&file).unwrap();
    fs::write(&file, format!("ghost_content_api = true\n{text}")).unwrap();
}

/// `path` with `key`, as Ghost's client sends it, as status and JSON.
#[cfg(test)]
fn ghost(server: &Server, path: &str, key: &str) -> (u16, Value) {
    let path = format!("{path}?key={key}");
    let (status, body) = server.raw("GET", &path, &[("Accept-Version", "v5.0")], None);
    (status, serde_json::from_str(&body).unwrap_or(Value::Null))
}

#[test]
fn ghosts_api_is_an_unknown_address_until_the_site_switches_it_on() {
    let site = demo_copy();
    let key = new_key(site.path(), &CONTENT);
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let own = format!("/blog/api/content/v1/posts/?key={key}");
    assert!(server.answers(&own, 200));
    for path in [
        "/blog/ghost/api/content/posts/",
        "/ghost/api/content/posts/",
        "/blog/ghost/api/content/settings/",
    ] {
        assert_eq!(ghost(&server, path, &key).0, 404, "{path}");
    }
}

#[test]
fn switched_on_it_answers_ghosts_shape_under_the_mount_and_at_the_root() {
    let site = demo_copy();
    switch_on(site.path());
    let key = new_key(site.path(), &CONTENT);
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let list = format!("/blog/ghost/api/content/posts/?key={key}");
    assert!(server.answers(&list, 200));
    let (_, json) = ghost(&server, "/blog/ghost/api/content/posts/", &key);
    let posts = json["posts"].as_array().unwrap();
    assert!(posts[0]["html"].is_string() && posts[0]["reading_time"].is_u64());
    assert!(json["meta"]["pagination"]["total"].as_u64().unwrap() >= 1);

    let (status, admin) = server.admin("GET", "/api/admin/v1/posts/en.fixing-slow-pages/", None);
    assert_eq!(status, 200, "{admin}");
    let read = "/ghost/api/content/posts/slug/fixing-slow-pages/";
    let (status, json) = ghost(&server, read, &key);
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["posts"][0]["title"], admin["posts"][0]["title"]);

    let (status, json) = ghost(&server, "/blog/ghost/api/content/settings/", &key);
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["settings"]["title"], "Field Notes on SEO");
    let (status, json) = ghost(&server, "/blog/ghost/api/content/posts/", "wrong");
    assert_eq!(status, 401, "{json}");
    assert_eq!(json["errors"][0]["type"], "UnauthorizedError");
    let own = format!("/blog/api/content/v1/posts/?key={key}");
    let (status, _) = server.raw("GET", &own, &[("Accept-Version", "v5.0")], None);
    assert_eq!(status, 406, "Khalid's own API keeps refusing v5");
}

#[test]
fn ghosts_path_answers_in_the_default_language_only() {
    let site = demo_copy();
    switch_on(site.path());
    let key = new_key(site.path(), &CONTENT);
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let (status, json) = ghost(&server, "/blog/ghost/api/content/posts/", &key);
    assert_eq!(status, 200, "{json}");
    let posts = json["posts"].as_array().unwrap();
    assert!(!posts.is_empty(), "{json}");
    let english = |post: &Value| post["id"].as_str().is_some_and(|id| id.starts_with("en."));
    assert!(posts.iter().all(english), "{json}");
    let german = "/blog/ghost/api/content/posts/slug/langsame-seiten-schneller-machen/";
    assert_eq!(ghost(&server, german, &key).0, 404);
}
