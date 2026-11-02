//! The copy link button on each H2 and H3: with scripts on, the reader
//! script turns each heading's `#` link into a button that copies the
//! section's address and says "Link copied" in a live region. With scripts
//! off, or where the page cannot use the clipboard, the plain `#` link stays.

use khalid_render::{ScriptTag, Tokens, reader_script, stylesheet_text};

use crate::common::demo_site;
use crate::contents::long_post;
use crate::html_check::assert_valid_page;
use crate::post::{part, render_on};

fn script() -> String {
    String::from_utf8(reader_script().unwrap().body().to_vec()).unwrap()
}

#[test]
fn the_page_keeps_the_plain_link_and_the_script_tag_carries_both_words() {
    let mut site = demo_site("en");
    site.script = Some(ScriptTag::new(&reader_script().unwrap(), "/blog/assets/"));
    let page = render_on(&site, long_post());
    assert_valid_page(&page);
    assert!(page.contains(
        "<a class=\"heading-link\" href=\"#what-it-does\" aria-label=\"Link to this section\">#</a>"
    ));
    let tag = part(&page, "<script", "</script>");
    assert!(tag.contains(" data-copy=\"Copy link\""), "{tag}");
    assert!(tag.contains(" data-copied=\"Link copied\""), "{tag}");
}

#[test]
fn the_button_replaces_the_link_only_where_the_clipboard_works() {
    let js = script();
    assert!(js.contains("if (navigator.clipboard)"));
    assert!(js.contains("button.className = \"heading-link copy-link\""));
    assert!(js.contains("link.replaceWith(button)"));
    assert!(
        js.contains("say(words.copied"),
        "the live region says the link is copied"
    );
}

#[test]
fn the_button_shows_its_words_on_pointing_focus_and_copy() {
    let css = stylesheet_text(&Tokens::default());
    assert!(
        css.contains(".copy-link::after { content: attr(data-tip);"),
        "{css}"
    );
    assert!(css.contains(
        ".copy-link:hover::after, .copy-link:focus-visible::after, .copy-link[data-done]::after { display: block; }"
    ));
}
