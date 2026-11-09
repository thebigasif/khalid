//! Related posts under a post: the cards chosen at build time, in a section
//! of their own after the article.

use khalid_render::{Feature, Loading, RenderPost};

use crate::html_check::assert_valid_page;
use crate::image::sitemap_image;
use crate::listing::card;
use crate::post::{part, render, render_on, site_without, sitemap_post};

/// The demo sitemap post with the three posts chosen for it.
fn post_with_related() -> RenderPost {
    let mut post = sitemap_post();
    post.related = vec![
        card(
            "What robots.txt can and cannot block",
            "robots-txt",
            10,
            "A robots file asks crawlers to stay out. It does not hide a page.",
        ),
        card(
            "How crawl budget works on small sites",
            "crawl-budget",
            20,
            "Most small sites never run out of crawl budget.",
        ),
        card(
            "Why internal links matter",
            "internal-links",
            14,
            "Internal links help readers and search engines find the rest of your site.",
        ),
    ];
    post
}

#[test]
fn a_post_ends_with_its_related_posts() {
    let page = render("en", post_with_related());
    assert_valid_page(&page);
    insta::assert_snapshot!(part(&page, "<section class=\"related\">", "</section>"));
}

#[test]
fn a_post_with_none_has_no_empty_section() {
    let page = render("en", sitemap_post());
    assert!(!page.contains("class=\"related\""));
}

#[test]
fn switched_off_there_are_no_related_posts() {
    let page = render_on(&site_without(Feature::RelatedPosts), post_with_related());
    assert_valid_page(&page);
    assert!(!page.contains("class=\"related\""));
    assert!(!page.contains("robots-txt"));
}

#[test]
fn related_card_pictures_wait_even_on_a_post_with_no_lead_image() {
    let mut post = post_with_related();
    post.image = None;
    for card in &mut post.related {
        card.image = Some(sitemap_image(Loading::Lazy));
    }
    let page = render("en", post);
    assert_valid_page(&page);
    assert_eq!(page.matches(r#"loading="lazy""#).count(), 3);
    assert!(!page.contains("fetchpriority"));
}
