//! The header and footer menus through the Admin API on a running `khalid
//! serve`, as the admin's Navigation screen saves them into `site.toml`
//! (backlog row 21, plan 4.6).

#[expect(
    dead_code,
    reason = "these tests only read and change settings and pages, so the other HTTP helpers stay unused"
)]
mod common;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use common::{Server, demo_copy, new_key, start};
use serde_json::json;

const SETTINGS: &str = "/api/admin/v1/settings/";

/// Whether the home page holds `text` within 30 seconds.
#[cfg(test)]
fn shows(server: &Server, text: &str) -> bool {
    (0..300).any(|_| {
        thread::sleep(Duration::from_millis(100));
        server.raw("GET", "/blog/", &[], None).1.contains(text)
    })
}

#[test]
fn a_saved_menu_reaches_site_toml_and_the_pages_and_a_bad_link_saves_nothing() {
    let site = demo_copy();
    let owner = ["--name", "Owner", "--scope", "settings:write"];
    let server = start(site.path(), &new_key(site.path(), &owner));
    let (status, read) = server.admin("GET", SETTINGS, None);
    assert_eq!(status, 200, "{read}");

    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let primary = json!([{"label": "Start here", "href": "/blog/start-here/"}]);
    let change = json!({"settings": {"updated_at": seen, "nav": {"primary": primary}}});
    let (status, saved) = server.admin("PUT", SETTINGS, Some(&change));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["settings"]["nav"]["primary"], primary, "{saved}");
    let mut file = PathBuf::from(site.path());
    file.push("site.toml");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.starts_with("# The demo site's settings."), "{text}");
    let written = "[[nav.primary]]\nlabel = \"Start here\"\nhref = \"/blog/start-here/\"\n";
    assert!(text.contains(written), "{text}");
    let link = r#"<a href="/blog/start-here/">Start here</a>"#;
    assert!(shows(&server, link), "the header shows the new link");

    let seen = saved["settings"]["updated_at"].as_str().unwrap();
    let bad = json!({"settings": {"updated_at": seen,
        "nav": {"footer": [{"label": "Run", "href": "javascript:alert(1)"}]}}});
    let (status, body) = server.admin("PUT", SETTINGS, Some(&bad));
    assert_eq!(status, 422, "{body}");
    assert!(
        body["detail"].to_string().contains("/blog/about/"),
        "{body}"
    );
    let after = std::fs::read_to_string(&file).unwrap();
    assert_eq!(after, text, "nothing was written");
}
