//! The `[seo]` table of `site.toml` through the Admin API on a running
//! `khalid serve`, as the admin's SEO settings screen saves it (Yoast
//! package WP3, KH-API-2).

#[expect(
    dead_code,
    reason = "these tests only read and change settings, so the raw HTTP helpers stay unused"
)]
mod common;

use std::path::PathBuf;

use common::{demo_copy, new_key, start};
use serde_json::json;

#[test]
fn seo_parts_are_saved_into_site_toml_and_a_bad_one_saves_nothing() {
    let site = demo_copy();
    let owner = ["--name", "Owner", "--scope", "settings:write"];
    let key = new_key(site.path(), &owner);
    let server = start(site.path(), &key);
    let (status, read) = server.admin("GET", "/api/admin/v1/settings/", None);
    assert_eq!(status, 200, "{read}");
    assert!(read["settings"]["seo"].is_object(), "{read}");

    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let change = json!({"settings": {"updated_at": seen, "seo": {
        "titles": {"separator": "·", "post": {"title": "{title} {sep} {site}"}},
        "crawlers": {"block": ["GPTBot", "CCBot"]},
    }}});
    let (status, saved) = server.admin("PUT", "/api/admin/v1/settings/", Some(&change));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(
        saved["settings"]["seo"]["titles"]["separator"], "·",
        "{saved}"
    );
    assert_eq!(saved["settings"]["restart_needed"], false, "{saved}");
    let mut file = PathBuf::from(site.path());
    file.push("site.toml");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.starts_with("# The demo site's settings."), "{text}");
    assert!(
        text.contains("[seo.crawlers]\nblock = [\"GPTBot\", \"CCBot\"]\n"),
        "{text}"
    );

    let seen = saved["settings"]["updated_at"].as_str().unwrap();
    let unknown = json!({"settings": {"updated_at": seen, "seo": {"feeds": {"length": 5}}}});
    let (status, body) = server.admin("PUT", "/api/admin/v1/settings/", Some(&unknown));
    assert_eq!(status, 422, "{body}");
    let after = std::fs::read_to_string(&file).unwrap();
    assert_eq!(after, text, "nothing was written");
}

#[test]
fn a_bluesky_did_saved_in_settings_is_served_for_the_handle() {
    let site = demo_copy();
    let owner = ["--name", "Owner", "--scope", "settings:write"];
    let key = new_key(site.path(), &owner);
    let server = start(site.path(), &key);
    let (status, _) = server.raw("GET", "/.well-known/atproto-did", &[], None);
    assert_eq!(status, 404);
    let (_, read) = server.admin("GET", "/api/admin/v1/settings/", None);
    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let handle = json!({"settings": {"updated_at": seen,
        "seo": {"verification": {"bluesky": "fieldnotes.bsky.social"}}}});
    let (status, body) = server.admin("PUT", "/api/admin/v1/settings/", Some(&handle));
    assert_eq!(status, 422, "{body}");

    let did = "did:plc:z72i7hdynmk6r22z27h6tvur";
    let change = json!({"settings": {"updated_at": seen,
        "seo": {"verification": {"bluesky": did}}}});
    let (status, saved) = server.admin("PUT", "/api/admin/v1/settings/", Some(&change));
    assert_eq!(status, 200, "{saved}");
    // The pages render again behind the save, a moment later.
    assert!(server.answers("/.well-known/atproto-did", 200));
    let (_, file) = server.raw("GET", "/.well-known/atproto-did", &[], None);
    assert_eq!(file, did);
}

#[test]
fn the_ai_training_switch_round_trips_and_reaches_robots_txt() {
    let site = demo_copy();
    let owner = ["--name", "Owner", "--scope", "settings:write"];
    let key = new_key(site.path(), &owner);
    let server = start(site.path(), &key);
    let (_, read) = server.admin("GET", "/api/admin/v1/settings/", None);
    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let wrong = json!({"settings": {"updated_at": seen,
        "seo": {"crawlers": {"ai_training": "maybe"}}}});
    let (status, body) = server.admin("PUT", "/api/admin/v1/settings/", Some(&wrong));
    assert_eq!(status, 422, "{body}");

    let change = json!({"settings": {"updated_at": seen,
        "seo": {"crawlers": {"ai_training": "block"}}}});
    let (status, saved) = server.admin("PUT", "/api/admin/v1/settings/", Some(&change));
    assert_eq!(status, 200, "{saved}");
    let (_, read) = server.admin("GET", "/api/admin/v1/settings/", None);
    assert_eq!(
        read["settings"]["seo"]["crawlers"],
        json!({"ai_training": "block"}),
        "{read}"
    );
    let (status, robots) = server.raw("GET", "/robots.txt", &[], None);
    assert_eq!(status, 200, "{robots}");
    assert!(robots.contains("User-agent: GPTBot\n"), "{robots}");
    assert!(!robots.contains("OAI-SearchBot"), "{robots}");
}
