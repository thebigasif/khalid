//! The template slots around a post: share links, each switched on by the
//! template's options and read with scripts off (templates spec 4.2).

use khalid_render::{PageBody, PageKind, RenderPack, Renderer, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::post::{part, sitemap_post};

fn word(text: &str) -> TemplateOption {
    TemplateOption::Word(text.to_owned())
}

fn lines(items: &[&str]) -> TemplateOption {
    TemplateOption::Lines(items.iter().map(|item| (*item).to_owned()).collect())
}

/// A renderer for a pack that replaces no file and sets `options`.
fn with_options(options: &[(&str, TemplateOption)]) -> Renderer {
    let site = tempfile::tempdir().unwrap();
    let pack = RenderPack {
        name: "trial",
        templates: Vec::new(),
        options: (options.iter())
            .map(|(key, value)| ((*key).to_owned(), value.clone()))
            .collect(),
    };
    Renderer::with_pack(site.path(), &pack).unwrap()
}

fn post_page(renderer: &Renderer) -> String {
    let post = sitemap_post();
    let meta = meta(&post.title, "/blog/what-a-sitemap-does/", PageKind::Article);
    let body = PageBody::Post(Box::new(post));
    renderer.render(&demo_site("en"), &meta, &body).unwrap()
}

#[test]
fn classic_shows_no_share_links() {
    let page = post_page(&Renderer::new().unwrap());
    assert!(!page.contains("class=\"share\""), "{page}");
}

#[test]
fn share_links_are_plain_links_at_the_top_and_the_end() {
    let renderer = with_options(&[
        ("share", word("both")),
        ("share_networks", lines(&["linkedin", "x", "email"])),
    ]);
    let page = post_page(&renderer);
    assert_valid_page(&page);
    assert_eq!(page.matches("<div class=\"share\">").count(), 2, "{page}");
    let row = part(&page, "<div class=\"share\">", "</div>");
    assert!(!row.contains("<script") && !row.contains("<img"), "{row}");
    insta::assert_snapshot!(row);
}

#[test]
fn share_at_the_end_sits_after_the_body() {
    let renderer = with_options(&[
        ("share", word("end")),
        ("share_networks", lines(&["email"])),
    ]);
    let page = post_page(&renderer);
    assert_eq!(page.matches("<div class=\"share\">").count(), 1, "{page}");
    let body = page.find("<div class=\"post-body\">").unwrap();
    let share = page.find("<div class=\"share\">").unwrap();
    assert!(body < share, "{page}");
}

#[test]
fn share_off_shows_nothing() {
    let renderer = with_options(&[("share", word("none"))]);
    assert!(!post_page(&renderer).contains("class=\"share\""));
}
