//! A topic's hub, and the link to it from its category's list.

use khalid_render::{BodyHtml, Link, PageBody, PageKind, RenderHub, Renderer};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::{first_page, main_of};

fn render(body: &PageBody, path: &str) -> String {
    let meta = meta("Technical SEO", path, PageKind::Website);
    Renderer::new()
        .unwrap()
        .render(&demo_site("en"), &meta, body)
        .unwrap()
}

fn topic() -> Link {
    Link {
        name: "Technical SEO".to_owned(),
        href: "/blog/category/technical-seo/".to_owned(),
    }
}

#[test]
fn a_hub_shows_its_intro_its_chapters_in_order_and_the_newest_posts() {
    let posts = first_page().posts;
    let hub = RenderHub {
        title: "Technical SEO, from the ground up".to_owned(),
        body: BodyHtml::from_sanitised("<p>Start with how search finds pages.</p>".to_owned()),
        topic: Some(topic()),
        chapters: posts.iter().take(2).cloned().collect(),
        latest: posts.iter().skip(2).cloned().collect(),
    };
    let page = render(&PageBody::Hub(Box::new(hub)), "/blog/technical-seo/");
    assert_valid_page(&page);
    insta::assert_snapshot!(main_of(&page));
}

#[test]
fn a_category_list_links_to_its_hub_above_its_posts() {
    let mut listing = first_page();
    listing.hub = Some(Link {
        name: "Technical SEO, from the ground up".to_owned(),
        href: "/blog/technical-seo/".to_owned(),
    });
    let page = render(&PageBody::Category(listing), "/blog/category/technical-seo/");
    assert_valid_page(&page);
    let link = r#"<p class="hub-link"><a href="/blog/technical-seo/"><span>Start here</span> Technical SEO, from the ground up</a></p>"#;
    assert!(page.contains(link), "{page}");
}
