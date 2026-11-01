//! The card shapes a template picks with its `cards` part: `image` (as
//! classic), `text`, `row`, `compact` and `ranked`, and the reading time
//! and author role a card may add to its meta line.

use khalid_render::{PageBody, PageKind, ReadingTime, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::image::sitemap_image;
use crate::listing::{first_page, home, main_of};
use crate::parts::{pack_with, renderer};

fn render_with(options: &[(&str, TemplateOption)]) -> String {
    let mut listing = first_page();
    listing.posts[0].image = Some(sitemap_image(khalid_render::Loading::Lazy));
    for post in &mut listing.posts {
        post.reading_time = Some(ReadingTime::from_minutes(4));
    }
    let meta = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    let page = renderer(&pack_with(options))
        .render(&demo_site("en"), &meta, &PageBody::Home(home(listing)))
        .unwrap();
    assert_valid_page(&page);
    main_of(&page).to_owned()
}

fn shape(name: &str) -> String {
    render_with(&[("cards", TemplateOption::Word(name.to_owned()))])
}

#[test]
fn image_cards_are_classic_cards() {
    let classic = render_with(&[]);
    assert_eq!(shape("image"), classic);
    assert!(
        classic.contains("<ul class=\"cards\" role=\"list\">"),
        "{classic}"
    );
}

#[test]
fn text_cards_leave_out_the_picture() {
    let html = shape("text");
    assert!(
        html.contains("<ul class=\"cards cards-text\" role=\"list\">"),
        "{html}"
    );
    assert!(!html.contains("<picture"), "{html}");
    assert!(html.contains("A sitemap lists the pages"), "{html}");
}

#[test]
fn row_cards_keep_the_picture_beside_the_text() {
    let html = shape("row");
    assert!(
        html.contains("<ul class=\"cards cards-row\" role=\"list\">"),
        "{html}"
    );
    assert!(html.contains("<picture"), "{html}");
}

#[test]
fn compact_cards_are_a_title_and_a_date() {
    let html = shape("compact");
    assert!(
        html.contains("<ul class=\"cards cards-compact\" role=\"list\">"),
        "{html}"
    );
    assert!(
        html.contains(
            "<article class=\"card\">\n<h2><a href=\"/blog/what-a-sitemap-does/\">What a sitemap does, and what it does not</a></h2>\n<p><time datetime=\"2026-09-28\">28 September 2026</time></p>\n</article>"
        ),
        "{html}"
    );
    assert!(!html.contains("<picture"), "{html}");
    assert!(!html.contains("A sitemap lists the pages"), "{html}");
}

#[test]
fn ranked_cards_are_a_numbered_list_without_excerpts() {
    let html = shape("ranked");
    assert!(
        html.contains("<ol class=\"cards cards-ranked\" role=\"list\">"),
        "{html}"
    );
    assert!(html.contains("</ol>"), "{html}");
    assert!(!html.contains("A sitemap lists the pages"), "{html}");
    assert!(!html.contains("<picture"), "{html}");
}

#[test]
fn a_card_adds_its_reading_time_and_author_role_when_the_template_says() {
    let plain = shape("text");
    assert!(!plain.contains("4 min read"), "{plain}");
    assert!(!plain.contains("Technical SEO editor"), "{plain}");
    let html = render_with(&[
        ("cards", TemplateOption::Word("text".to_owned())),
        ("card_reading_time", TemplateOption::Switch(true)),
        ("card_author_role", TemplateOption::Switch(true)),
    ]);
    assert!(
        html.contains("<span class=\"card-reading-time\">4 min read</span>"),
        "{html}"
    );
    assert!(
        html.contains("Lena Vogt<span class=\"person-role\">Technical SEO editor</span>"),
        "{html}"
    );
}
