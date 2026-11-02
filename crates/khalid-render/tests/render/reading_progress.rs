//! The reading progress bar: a thin accent line at the top of a post that
//! fills as the reader scrolls. The stylesheet draws it with a scroll-driven
//! animation, the reader script fills it where browsers lack one, and the
//! template's `reading_progress` part turns it off.

use khalid_render::{PageBody, PageKind, RenderPack, TemplateOption, Tokens, stylesheet_text};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::parts::{pack_with, renderer};
use crate::post::sitemap_post;

const BAR: &str = "<div class=\"reading-progress\" aria-hidden=\"true\"></div>";

fn post(pack: &RenderPack<'_>) -> String {
    let meta = meta(
        "What a sitemap does",
        "/blog/what-a-sitemap-does/",
        PageKind::Article,
    );
    let page = renderer(pack)
        .render(
            &demo_site("en"),
            &meta,
            &PageBody::Post(Box::new(sitemap_post())),
        )
        .unwrap();
    assert_valid_page(&page);
    page
}

#[test]
fn a_post_carries_the_bar_before_its_article_in_classic() {
    let html = post(&RenderPack::classic());
    assert!(html.contains(&format!("{BAR}\n<article")), "{html}");
}

#[test]
fn the_template_can_turn_the_bar_off() {
    let off = pack_with(&[("reading_progress", TemplateOption::Word("none".to_owned()))]);
    assert!(!post(&off).contains("reading-progress"));
    let on = pack_with(&[("reading_progress", TemplateOption::Word("bar".to_owned()))]);
    assert!(post(&on).contains(BAR));
}

#[test]
fn other_pages_carry_no_bar() {
    let meta = meta("Not found", "/blog/missing/", PageKind::Website);
    let html = renderer(&RenderPack::classic())
        .render(&demo_site("en"), &meta, &PageBody::NotFound)
        .unwrap();
    assert!(!html.contains("reading-progress"));
}

#[test]
fn the_bar_fills_with_the_scroll_and_hides_for_reduced_motion_and_print() {
    let css = stylesheet_text(&Tokens::default());
    assert!(css.contains("animation-timeline: scroll(root)"), "{css}");
    assert!(css.contains("@keyframes reading-progress"));
    let hidden =
        "@media (prefers-reduced-motion: reduce), print { .reading-progress { display: none; } }";
    assert!(css.contains(hidden), "{css}");
}

#[test]
fn the_script_fills_the_bar_only_where_the_browser_cannot() {
    let script = khalid_render::reader_script_file();
    let text = std::str::from_utf8(script.body).unwrap();
    assert!(text.contains("CSS.supports(\"animation-timeline: scroll()\")"));
    assert!(text.contains(".reading-progress"));
}
