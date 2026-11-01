//! The back-link part: an "All posts" link to the blog's front page above
//! a post, in place of breadcrumbs, and on posts only.

use khalid_render::{PageBody, PageKind, RenderPack, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::{first_page, home};
use crate::parts::{pack_with, renderer};
use crate::post::sitemap_post;

/// The back link as the page shows it.
const BACK: &str = "<p class=\"back-link\"><a href=\"/blog/\">All posts</a></p>";

fn post(pack: &RenderPack<'_>) -> String {
    let post = sitemap_post();
    let meta = meta(&post.title, "/blog/what-a-sitemap-does/", PageKind::Article);
    let page = renderer(pack)
        .render(&demo_site("en"), &meta, &PageBody::Post(Box::new(post)))
        .unwrap();
    assert_valid_page(&page);
    page
}

#[test]
fn a_post_links_back_to_all_posts_when_the_template_turns_it_on() {
    let on = pack_with(&[("back_link", TemplateOption::Switch(true))]);
    let html = post(&on);
    assert!(
        html.contains(&format!("<main id=\"main\" tabindex=\"-1\">\n{BACK}")),
        "{html}"
    );
}

#[test]
fn classic_has_no_back_link() {
    let html = post(&RenderPack::classic());
    assert!(!html.contains("back-link"), "{html}");
}

#[test]
fn the_front_page_has_no_back_link() {
    let on = pack_with(&[("back_link", TemplateOption::Switch(true))]);
    let meta = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    let page = renderer(&on)
        .render(&demo_site("en"), &meta, &PageBody::Home(home(first_page())))
        .unwrap();
    assert!(!page.contains("back-link"), "{page}");
}
