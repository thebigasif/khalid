//! Where a post's contents go and how they are numbered, as the
//! template's `contents` and `contents_numbers` parts say. Classic keeps
//! the folded box on phones and the side column at the end on wide
//! screens, numbered 1, 2, 3.

use khalid_render::{PageBody, PageKind, RenderPack, TemplateOption};

use crate::common::{demo_site, meta};
use crate::contents::long_post;
use crate::html_check::assert_valid_page;
use crate::parts::{pack_with, renderer};

fn render(pack: &RenderPack<'_>) -> String {
    let meta = meta(
        "What a sitemap does",
        "/blog/what-a-sitemap-does/",
        PageKind::Article,
    );
    let page = renderer(pack)
        .render(
            &demo_site("en"),
            &meta,
            &PageBody::Post(Box::new(long_post())),
        )
        .unwrap();
    assert_valid_page(&page);
    page
}

fn placed(placement: &str) -> String {
    render(&pack_with(&[(
        "contents",
        TemplateOption::Word(placement.to_owned()),
    )]))
}

#[test]
fn side_end_is_classic() {
    assert_eq!(placed("side-end"), render(&RenderPack::classic()));
}

#[test]
fn side_start_puts_the_column_on_the_start_side() {
    let html = placed("side-start");
    assert!(
        html.contains("<article class=\"post post-with-toc post-toc-start\">"),
        "{html}"
    );
    assert!(html.contains("<nav class=\"toc-side\""), "{html}");
}

#[test]
fn inline_is_an_open_box_and_no_column() {
    let html = placed("inline");
    assert!(html.contains("<article class=\"post\">"), "{html}");
    assert!(
        html.contains("<nav class=\"toc-box\" aria-label=\"Contents\">\n<p class=\"toc-title\">Contents</p>\n<ol>"),
        "{html}"
    );
    assert!(!html.contains("<details class=\"toc-inline\">"), "{html}");
    assert!(!html.contains("toc-side"), "{html}");
}

#[test]
fn inline_folded_is_the_folded_box_at_every_width() {
    let html = placed("inline-folded");
    assert!(
        html.contains("<article class=\"post post-toc-folded\">"),
        "{html}"
    );
    assert!(html.contains("<details class=\"toc-inline\">"), "{html}");
    assert!(!html.contains("toc-side"), "{html}");
}

#[test]
fn none_leaves_the_contents_out() {
    let html = placed("none");
    assert!(html.contains("<article class=\"post\">"), "{html}");
    assert!(!html.contains("toc-"), "{html}");
}

#[test]
fn the_numbers_can_nest_or_go() {
    let numbers = |style: &str| {
        render(&pack_with(&[(
            "contents_numbers",
            TemplateOption::Word(style.to_owned()),
        )]))
    };
    assert!(numbers("nested").contains("<ol class=\"toc-nested\">"));
    assert!(numbers("none").contains("<ol class=\"toc-plain\">"));
    assert_eq!(numbers("numbered"), render(&RenderPack::classic()));
}
