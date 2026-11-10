//! The oEmbed provider on a running `khalid serve`, under the demo's mount
//! (plan 5.5, `api.md` Surfaces): no key, the card of a demo post.

#[expect(
    dead_code,
    reason = "these tests only read public addresses, so the key and admin helpers stay unused"
)]
mod common;

use common::{demo_copy, new_key, start};
use serde_json::Value;

/// A demo post's address, as a reader copies it.
const POST: &str = "https://www.example.com/blog/fixing-slow-pages/";

#[test]
fn another_site_gets_a_demo_post_as_a_card_under_the_mount() {
    let site = demo_copy();
    let key = new_key(
        site.path(),
        &["--name", "Reader", "--scope", "content:read"],
    );
    let server = start(site.path(), &key);
    let asked = format!("/blog/api/oembed?url={POST}");
    let (status, body) = server.raw("GET", &asked, &[], None);
    assert_eq!(status, 200, "{body}");
    let card: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(card["type"], "rich", "{card}");
    assert_eq!(card["provider_name"], "Field Notes on SEO", "{card}");
    assert!(
        card["title"]
            .as_str()
            .unwrap()
            .starts_with("Fixing slow pages"),
        "{card}"
    );
    let html = card["html"].as_str().unwrap();
    assert!(html.contains(&format!("<a href=\"{POST}\">")), "{html}");
    let (status, _) = server.raw(
        "GET",
        "/blog/api/oembed?url=https://www.example.com/blog/nothing/",
        &[],
        None,
    );
    assert_eq!(status, 404);
}
