//! The `[privacy]` table and the tagged `[[code]]` list of `site.toml`
//! through the Admin API on a running `khalid serve`, as the admin's
//! Privacy screen saves them (privacy plan U8, R17, R19, R24).

#[expect(
    dead_code,
    reason = "these tests only read and change settings, so the raw HTTP helpers stay unused"
)]
mod common;

use std::path::PathBuf;

use common::{demo_copy, new_key, start};
use serde_json::json;

const SETTINGS: &str = "/api/admin/v1/settings/";

#[test]
fn privacy_and_tagged_code_are_saved_into_site_toml_and_a_bad_lifetime_saves_nothing() {
    let site = demo_copy();
    let owner = ["--name", "Owner", "--scope", "admin:all"];
    let key = new_key(site.path(), &owner);
    let server = start(site.path(), &key);
    let (status, read) = server.admin("GET", SETTINGS, None);
    assert_eq!(status, 200, "{read}");
    assert_eq!(read["settings"]["mode"], "server", "{read}");
    assert!(read["settings"]["privacy"].is_object(), "{read}");

    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let script = json!({"head": "<script src=\"https://stats.example.net/a.js\"></script>",
        "category": "no-tracking", "vendor": "Example Stats"});
    let privacy = json!({"settings": {"updated_at": seen, "privacy": {
        "lifetime_months": 12,
        "contact": {"name": "Field Notes Ltd", "email": "privacy@example.com"},
    }}});
    let (status, saved) = server.admin("PUT", SETTINGS, Some(&privacy));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["settings"]["restart_needed"], false, "{saved}");
    let seen = saved["settings"]["updated_at"].as_str().unwrap();
    let change = json!({"settings": {"updated_at": seen, "code": [script]}});
    let (status, saved) = server.admin("PUT", SETTINGS, Some(&change));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(
        saved["settings"]["privacy"]["lifetime_months"], 12,
        "{saved}"
    );
    assert_eq!(
        saved["settings"]["code"][0]["category"], "no-tracking",
        "{saved}"
    );
    // The consent route reads the tagged code when khalid serve starts.
    assert_eq!(saved["settings"]["restart_needed"], true, "{saved}");
    let mut file = PathBuf::from(site.path());
    file.push("site.toml");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.starts_with("# The demo site's settings."), "{text}");
    assert!(text.contains("[privacy]\nlifetime_months = 12\n"), "{text}");
    assert!(text.contains("[[code]]\n"), "{text}");

    let seen = saved["settings"]["updated_at"].as_str().unwrap();
    let long = json!({"settings": {"updated_at": seen, "privacy": {"lifetime_months": 14}}});
    let (status, body) = server.admin("PUT", SETTINGS, Some(&long));
    assert_eq!(status, 422, "{body}");
    assert_eq!(body["rule"], "lifetime_months", "{body}");
    let after = std::fs::read_to_string(&file).unwrap();
    assert_eq!(after, text, "nothing was written");
}

#[test]
fn body_code_added_on_the_scripts_tab_is_checked_and_an_empty_list_removes_code() {
    let site = demo_copy();
    let owner = ["--name", "Owner", "--scope", "settings:write"];
    let key = new_key(site.path(), &owner);
    let server = start(site.path(), &key);
    let (_, read) = server.admin("GET", SETTINGS, None);
    let mut file = PathBuf::from(site.path());
    file.push("site.toml");
    let before = std::fs::read_to_string(&file).unwrap();

    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let div = json!({"body": "<div>Ads</div>", "category": "marketing"});
    let change = json!({"settings": {"updated_at": seen, "code": [div]}});
    let (status, body) = server.admin("PUT", SETTINGS, Some(&change));
    assert_eq!(status, 422, "{body}");
    let detail = body["detail"].as_str().unwrap_or_default();
    assert!(detail.contains("may hold only script, not div"), "{body}");
    let after = std::fs::read_to_string(&file).unwrap();
    assert_eq!(after, before, "nothing was written");

    let tag = json!({"body": "<script src=\"https://cdn.example.org/tag.js\"></script>",
        "category": "analytics"});
    let change = json!({"settings": {"updated_at": seen, "code": [tag]}});
    let (status, saved) = server.admin("PUT", SETTINGS, Some(&change));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["settings"]["code"][0]["category"], "analytics");
    let seen = saved["settings"]["updated_at"].as_str().unwrap();
    let none = json!({"settings": {"updated_at": seen, "code": []}});
    let (status, saved) = server.admin("PUT", SETTINGS, Some(&none));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["settings"]["code"], json!([]), "{saved}");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(
        !text.contains("[[code]]") && !text.contains("[code]"),
        "{text}"
    );
}
