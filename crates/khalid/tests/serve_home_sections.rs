//! The home page's sections through the Admin API on a running `khalid
//! serve`, as the admin's Site settings save them into `site.toml` (gap
//! G19).

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
fn home_sections_are_saved_into_site_toml_and_a_bad_slug_saves_nothing() {
    let site = demo_copy();
    let owner = ["--name", "Owner", "--scope", "settings:write"];
    let key = new_key(site.path(), &owner);
    let server = start(site.path(), &key);
    let (status, read) = server.admin("GET", SETTINGS, None);
    assert_eq!(status, 200, "{read}");
    assert_eq!(read["settings"]["home_sections"], json!([]), "{read}");

    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let sections = json!([
        {"kind": "category", "slug": "technical-seo"},
        {"kind": "newsletter"},
        {"kind": "latest"},
    ]);
    let change = json!({"settings": {"updated_at": seen, "home_sections": sections}});
    let (status, saved) = server.admin("PUT", SETTINGS, Some(&change));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["settings"]["home_sections"], sections, "{saved}");
    let mut file = PathBuf::from(site.path());
    file.push("site.toml");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.starts_with("# The demo site's settings."), "{text}");
    let written = "[reader]\nhome_sections = [\n  { category = \"technical-seo\" },\n  \
                   \"newsletter\",\n  \"latest\",\n]\n";
    assert!(text.contains(written), "{text}");

    let seen = saved["settings"]["updated_at"].as_str().unwrap();
    let bad = json!({"settings": {"updated_at": seen,
        "home_sections": [{"kind": "category", "slug": "Technical SEO"}]}});
    let (status, body) = server.admin("PUT", SETTINGS, Some(&bad));
    assert_eq!(status, 422, "{body}");
    let after = std::fs::read_to_string(&file).unwrap();
    assert_eq!(after, text, "nothing was written");
}
