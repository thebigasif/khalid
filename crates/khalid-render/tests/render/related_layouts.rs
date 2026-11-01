//! The related posts under a post, laid out as the template's `related`
//! part says (grid, list, row, scroll or links) and cut to its
//! `related_count`, five in classic.

use khalid_render::{PageBody, PageKind, RenderPack, RenderPost, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::card;
use crate::parts::{pack_with, renderer};
use crate::post::{part, sitemap_post};

fn post() -> RenderPost {
    let mut post = sitemap_post();
    post.related = (1..=8)
        .map(|day| {
            card(
                &format!("Note {day}"),
                &format!("note-{day}"),
                day,
                "One line.",
            )
        })
        .collect();
    post
}

fn related(pack: &RenderPack<'_>) -> String {
    let meta = meta(
        "What a sitemap does",
        "/blog/what-a-sitemap-does/",
        PageKind::Article,
    );
    let page = renderer(pack)
        .render(&demo_site("en"), &meta, &PageBody::Post(Box::new(post())))
        .unwrap();
    assert_valid_page(&page);
    part(&page, "<section class=\"related", "</section>")
}

fn layout(name: &str) -> String {
    related(&pack_with(&[(
        "related",
        TemplateOption::Word(name.to_owned()),
    )]))
}

#[test]
fn classic_shows_five_cards_in_its_grid() {
    let html = related(&RenderPack::classic());
    assert!(html.starts_with("<section class=\"related\">"), "{html}");
    assert_eq!(
        html.matches("<article class=\"card\">").count(),
        5,
        "{html}"
    );
    assert_eq!(layout("grid"), html);
}

#[test]
fn the_count_cuts_the_list() {
    let pack = pack_with(&[("related_count", TemplateOption::Number(3))]);
    let html = related(&pack);
    assert_eq!(
        html.matches("<article class=\"card\">").count(),
        3,
        "{html}"
    );
    let pack = pack_with(&[("related_count", TemplateOption::Number(8))]);
    assert_eq!(
        related(&pack).matches("<article class=\"card\">").count(),
        8
    );
}

#[test]
fn each_layout_has_its_own_shape() {
    assert!(layout("list").contains("<ul class=\"cards cards-text\""));
    assert!(layout("row").contains("<ul class=\"cards cards-row\""));
    let scroll = layout("scroll");
    assert!(
        scroll.starts_with("<section class=\"related related-scroll\">"),
        "{scroll}"
    );
    let links = layout("links");
    assert!(
        links.contains(
            "<ul class=\"related-links\">\n<li><a href=\"/blog/note-1/\">Note 1</a></li>"
        ),
        "{links}"
    );
    assert!(!links.contains("<article"), "{links}");
}
