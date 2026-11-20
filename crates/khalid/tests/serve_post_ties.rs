//! A post's ties and listing choices set through the Admin API on a
//! running `khalid serve`: written into the post's file and read back
//! (plan 4.12, 5.2 and 5.4).

mod common;

use std::fs;

use serde_json::json;

#[test]
fn ties_and_listing_choices_reach_the_file_and_read_back() {
    let site = common::demo_copy();
    let key = common::new_key(
        site.path(),
        &[
            "--name",
            "Desk",
            "--scope",
            "posts:write",
            "--scope",
            "posts:publish",
        ],
    );
    let server = common::start(site.path(), &key);
    let path = "/api/admin/v1/posts/en.what-a-sitemap-does/";
    let (status, read) = server.admin("GET", path, None);
    assert_eq!(status, 200, "{read}");
    let before = &read["posts"][0]["settings"];
    assert_eq!(before["connections"]["translation"], "sitemap", "{before}");
    assert_eq!(before["listing"]["sitemap"], true, "{before}");
    let edit = json!({"posts": [{
        "settings": {
            "connections": {"related": ["why-internal-links-matter"], "translation": "sitemaps"},
            "listing": {"sitemap": false, "page_type": "FAQPage", "breadcrumb_title": "Sitemaps"},
        },
        "updated_at": read["posts"][0]["updated_at"],
    }]});
    let (status, saved) = server.admin("PUT", path, Some(&edit));
    assert_eq!(status, 200, "{saved}");

    let mut file = site.path().to_path_buf();
    file.push("content/en/posts/2026-09-28-what-a-sitemap-does.md");
    let text = fs::read_to_string(&file).unwrap();
    for line in [
        "related: [why-internal-links-matter]\n",
        "translation: \"sitemaps\"\n",
        "sitemap: false\n",
        "page_type: FAQPage\n",
        "breadcrumb_title: \"Sitemaps\"\n",
    ] {
        assert!(text.contains(line), "{line} in {text}");
    }
    let (status, again) = server.admin("GET", path, None);
    assert_eq!(status, 200, "{again}");
    let settings = &again["posts"][0]["settings"];
    assert_eq!(settings["listing"]["sitemap"], false, "{settings}");
    assert_eq!(settings["listing"]["page_type"], "FAQPage", "{settings}");
    assert_eq!(
        settings["connections"]["translation"], "sitemaps",
        "{settings}"
    );
    assert!(
        server.answers("/blog/what-a-sitemap-does/", 200),
        "a post left out of the sitemap is still served"
    );
}
