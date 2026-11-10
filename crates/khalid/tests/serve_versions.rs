//! Earlier versions on a running `khalid serve`: an Admin API edit keeps
//! the text it replaced in the site's database, the versions outlast a
//! restart, and bringing one back is an edit that can be undone (plan 4.6,
//! KH-EDT-2).

mod common;

use serde_json::{Value, json};

/// The demo post every edit here changes.
const POST: &str = "/api/admin/v1/posts/en.why-internal-links-matter/";

/// Its earlier versions.
const VERSIONS: &str = "/api/admin/v1/posts/en.why-internal-links-matter/versions/";

/// The post as the server holds it now.
#[cfg(test)]
fn current(server: &common::Server) -> Value {
    let (status, json) = server.admin("GET", POST, None);
    assert_eq!(status, 200, "{json}");
    json["posts"][0].clone()
}

/// Saves `markdown` as the post's text, on the copy it holds now.
#[cfg(test)]
fn save(server: &common::Server, markdown: &str) {
    let updated_at = current(server)["updated_at"].clone();
    let edit = json!({ "posts": [{ "markdown": markdown, "updated_at": updated_at }] });
    let (status, json) = server.admin("PUT", POST, Some(&edit));
    assert_eq!(status, 200, "{json}");
}

/// The text of each earlier version, the newest first.
#[cfg(test)]
fn kept(server: &common::Server) -> Vec<String> {
    let (status, json) = server.admin("GET", VERSIONS, None);
    assert_eq!(status, 200, "{json}");
    let versions = json["versions"].as_array().unwrap();
    let text = |version: &Value| version["markdown"].as_str().unwrap().to_owned();
    versions.iter().map(text).collect()
}

#[test]
fn versions_and_a_stale_edit_name_who_saved_through_a_restart() {
    let site = common::demo_copy();
    let scopes = ["--scope", "posts:write", "--scope", "posts:publish"];
    let key = common::new_key(site.path(), &[&["--name", "Maria"][..], &scopes].concat());
    let server = common::start(site.path(), &key);
    let before = current(&server);
    assert_eq!(before["updated_by"], Value::Null, "a file from the demo");
    let original = before["markdown"].as_str().unwrap().to_owned();
    save(&server, &format!("{original}\nA line by Maria."));
    let by = current(&server)["updated_by"].clone();
    assert!(by.is_string(), "the key that saved it: {by}");
    drop(server);

    let server = common::start(site.path(), &key);
    assert_eq!(current(&server)["updated_by"], by, "kept through a restart");
    save(&server, &original);
    let (status, json) = server.admin("GET", VERSIONS, None);
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["versions"][0]["saved_by"], by, "{json}");
    assert_eq!(json["versions"][0]["saved_by_name"], "Maria", "{json}");
    let rows = "/api/admin/v1/posts/?filter=status:published&limit=100";
    let (status, json) = server.admin("GET", rows, None);
    assert_eq!(status, 200, "{json}");
    let id = "en.why-internal-links-matter";
    let row = json["posts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == id);
    assert_eq!(row.unwrap()["updated_by_name"], "Maria", "{json}");
    let old = before["updated_at"].clone();
    let stale = json!({ "posts": [{ "markdown": "Lost.", "updated_at": old }] });
    let (status, json) = server.admin("PUT", POST, Some(&stale));
    assert_eq!(status, 409, "{json}");
    let said = json["detail"].as_str().unwrap_or_default();
    assert!(said.starts_with("Maria changed this post"), "{json}");
}

#[test]
fn an_edit_keeps_the_old_text_through_a_restart_and_a_restore_can_be_undone() {
    let site = common::demo_copy();
    let scopes = ["--scope", "posts:write", "--scope", "posts:publish"];
    let key = common::new_key(site.path(), &[&["--name", "Editor"][..], &scopes].concat());
    let server = common::start(site.path(), &key);
    assert_eq!(kept(&server), Vec::<String>::new());
    let original = current(&server)["markdown"].as_str().unwrap().to_owned();
    let changed = format!("{original}\nOne more line about anchor text.");
    save(&server, &changed);
    assert_eq!(kept(&server), std::slice::from_ref(&original));
    drop(server);

    let server = common::start(site.path(), &key);
    assert_eq!(
        kept(&server),
        std::slice::from_ref(&original),
        "kept through a restart"
    );
    save(&server, &original);
    assert_eq!(current(&server)["markdown"], original.as_str());
    assert!(server.answers("/blog/why-internal-links-matter/", 200));
    assert_eq!(
        kept(&server),
        [changed, original],
        "the restore can be undone"
    );
}
