//! The dek part, a post's subtitle under its title, and the updated-note
//! part, which says why a post was updated next to the date.

use chrono::NaiveDate;
use khalid_render::{PageBody, PageKind, RenderPack, RenderPost, Renderer, TemplateOption};

use crate::common::{demo_site, meta};
use crate::parts::{pack_with, renderer};
use crate::post::sitemap_post;

fn post() -> RenderPost {
    let mut post = sitemap_post();
    post.dek = Some("Plain facts about a file search engines read.".to_owned());
    post.updated = NaiveDate::from_ymd_opt(2026, 10, 20);
    post.updated_note = Some("Added the part on image sitemaps.".to_owned());
    post
}

fn render(renderer: &Renderer, post: RenderPost) -> String {
    let meta = meta(&post.title, "/blog/what-a-sitemap-does/", PageKind::Article);
    renderer
        .render(&demo_site("en"), &meta, &PageBody::Post(Box::new(post)))
        .unwrap()
}

#[test]
fn the_dek_shows_under_the_title_when_the_template_turns_it_on() {
    let on = pack_with(&[("dek", TemplateOption::Switch(true))]);
    let html = render(&renderer(&on), post());
    assert!(
        html.contains(
            "<h1>What a sitemap does, and what it does not</h1>\n<p class=\"dek\">Plain facts about a file search engines read.</p>"
        ),
        "{html}"
    );
}

#[test]
fn the_updated_note_shows_after_the_date_when_the_template_turns_it_on() {
    let on = pack_with(&[("updated_note", TemplateOption::Switch(true))]);
    let html = render(&renderer(&on), post());
    assert!(
        html.contains(
            "Updated <time datetime=\"2026-10-20\">20 October 2026</time></p>\n<p class=\"updated-note\">Added the part on image sitemaps.</p>"
        ),
        "{html}"
    );
}

#[test]
fn classic_shows_neither_and_a_post_without_them_shows_nothing() {
    let html = render(&renderer(&RenderPack::classic()), post());
    assert!(!html.contains("class=\"dek\""), "{html}");
    assert!(!html.contains("updated-note"), "{html}");
    let both = pack_with(&[
        ("dek", TemplateOption::Switch(true)),
        ("updated_note", TemplateOption::Switch(true)),
    ]);
    let html = render(&renderer(&both), sitemap_post());
    assert!(!html.contains("class=\"dek\""), "{html}");
    assert!(!html.contains("updated-note"), "{html}");
}
