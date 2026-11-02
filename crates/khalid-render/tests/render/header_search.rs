//! The search in the header. On a site that answers searches the header's
//! search link opens a search dialog with results as the reader types; with
//! scripts off it opens the search page. The template's `header_search`
//! part picks the dialog, a plain link or nothing.

use khalid_render::{
    PageBody, PageKind, RenderPack, ScriptTag, TemplateOption, Tokens, reader_script,
    stylesheet_text,
};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::parts::{pack_with, renderer};
use crate::post::{part, render_on, sitemap_post};

fn not_found(pack: &RenderPack<'_>) -> String {
    let meta = meta("Not found", "/blog/missing/", PageKind::Website);
    let page = renderer(pack)
        .render(&demo_site("en"), &meta, &PageBody::NotFound)
        .unwrap();
    assert_valid_page(&page);
    page
}

fn header_search(word: &str) -> String {
    not_found(&pack_with(&[(
        "header_search",
        TemplateOption::Word(word.to_owned()),
    )]))
}

#[test]
fn classic_opens_the_dialog_from_a_link_to_the_search_page() {
    let html = not_found(&RenderPack::classic());
    assert!(
        html.contains("<a class=\"search-link\" href=\"/blog/search/\" data-dialog>Search</a>"),
        "{html}"
    );
}

#[test]
fn the_template_can_keep_a_plain_link_or_hide_the_search() {
    assert!(
        header_search("link")
            .contains("<a class=\"search-link\" href=\"/blog/search/\">Search</a>")
    );
    assert!(header_search("button").contains(" data-dialog>"));
    assert!(!header_search("none").contains("search-link"));
}

#[test]
fn the_script_tag_carries_the_dialog_words() {
    let mut site = demo_site("en");
    site.script = Some(ScriptTag::new(&reader_script().unwrap(), "/blog/assets/"));
    let page = render_on(&site, sitemap_post());
    let tag = part(&page, "<script", "</script>");
    assert!(
        tag.contains(" data-search=\"Search this blog\" data-find=\"Search\""),
        "{tag}"
    );
}

#[test]
fn the_dialog_is_modal_and_shows_results_while_typing() {
    let js = String::from_utf8(reader_script().unwrap().body().to_vec()).unwrap();
    assert!(js.contains("a.search-link[data-dialog]"));
    assert!(js.contains("searchDialog.showModal()"));
    assert!(
        js.contains("typing: \"1\""),
        "results come from the search page's own answer"
    );
    let css = stylesheet_text(&Tokens::default());
    assert!(css.contains(".search-dialog {"), "{css}");
    assert!(css.contains(".search-dialog::backdrop"));
}
