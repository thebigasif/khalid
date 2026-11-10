//! The policies a running `khalid serve` applies: the site's link policy
//! on body links, framed pages of allowed domains in the content security
//! policy, and the admin app's own policy.

use std::fs;
use std::path::PathBuf;

use super::{demo_copy, get, start};

#[test]
fn serve_gives_body_links_the_sites_link_policy() {
    let site = demo_copy();
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    let mut text = fs::read_to_string(&settings).unwrap();
    text.push_str("\n[seo.links]\nrel = [\"nofollow\"]\naffiliate_domains = [\"amazon.com\"]\n");
    fs::write(&settings, text).unwrap();
    let mut file = site.path().to_path_buf();
    file.push("content/en/posts/2026-09-28-what-a-sitemap-does.md");
    let mut post = fs::read_to_string(&file).unwrap();
    post.push_str("\nRead [the book](https://www.amazon.com/dp/B0C1234567) or ");
    post.push_str("[the guide](https://developers.google.com/search).\n");
    fs::write(&file, post).unwrap();
    let server = start(site.path());
    let page = get(&server, "/blog/what-a-sitemap-does/");
    assert_eq!(page.status, 200);
    let book = "<a href=\"https://www.amazon.com/dp/B0C1234567\" rel=\"nofollow sponsored\">";
    let guide = "<a href=\"https://developers.google.com/search\" rel=\"nofollow\">";
    assert!(page.body.contains(book), "{}", page.body);
    assert!(page.body.contains(guide), "{}", page.body);
}

#[test]
fn serve_frames_a_page_of_an_allowed_domain_and_lets_only_it_into_the_policy() {
    let site = demo_copy();
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    let mut text = fs::read_to_string(&settings).unwrap();
    text.push_str("\n[media]\nembed_domains = [\"tools.example.com\"]\n");
    fs::write(&settings, text).unwrap();
    let mut file = site.path().to_path_buf();
    file.push("content/en/posts/2026-09-28-what-a-sitemap-does.md");
    let mut post = fs::read_to_string(&file).unwrap();
    post.push_str("\n::embed{src=\"https://tools.example.com/roi\" title=\"ROI calculator\"}\n");
    post.push_str("\n::embed{src=\"https://other.example.com/x\" title=\"Elsewhere\"}\n");
    fs::write(&file, post).unwrap();
    let server = start(site.path());
    let page = get(&server, "/blog/what-a-sitemap-does/");
    assert_eq!(page.status, 200, "{}", page.body);
    let frame = "<iframe src=\"https://tools.example.com/roi\" title=\"ROI calculator\"";
    assert!(page.body.contains(frame), "{}", page.body);
    assert!(
        !page.body.contains("<iframe src=\"https://other"),
        "{}",
        page.body
    );
    assert!(
        page.policy.contains(" https://tools.example.com"),
        "{}",
        page.policy
    );
    assert!(
        !page.policy.contains("other.example.com"),
        "{}",
        page.policy
    );
}

#[test]
fn serve_answers_the_admin_app_at_admin_with_its_own_policy() {
    let site = demo_copy();
    let server = start(site.path());
    // The demo blog sits under /blog/, and so does its admin.
    let admin = get(&server, "/blog/admin/");
    // The pull request checks build Khalid without running npm, so the
    // binary holds no admin and says how to add it. The release builds
    // admin/dist first, and then only the app's page will do (G7).
    let mut bundle = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bundle.push("../../admin/dist/index.html");
    if bundle.is_file() {
        assert_eq!(admin.status, 200, "admin/dist is built: {}", admin.body);
        assert!(admin.body.contains("<div id=\"root\">"), "{}", admin.body);
    } else {
        assert_eq!(admin.status, 503, "admin/dist is not built: {}", admin.body);
        assert!(admin.body.contains("npm run build"), "{}", admin.body);
    }
    assert!(
        admin.policy.contains("script-src 'self'"),
        "{}",
        admin.policy
    );
    assert!(
        admin.policy.contains("frame-ancestors 'none'"),
        "{}",
        admin.policy
    );
    let blog = get(&server, "/blog/");
    assert_eq!(blog.status, 200, "the pages still answer beside the admin");
}
