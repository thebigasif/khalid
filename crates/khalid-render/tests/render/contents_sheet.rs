//! Smarter contents: the side column stays in view and marks the section
//! being read, and on a narrow screen the reader script adds a floating
//! button that opens the contents as a sheet. With scripts off the folded
//! box above the post still lists every section.

use khalid_render::{ScriptTag, Tokens, reader_script, stylesheet_text};

use crate::common::demo_site;
use crate::contents::long_post;
use crate::html_check::assert_valid_page;
use crate::post::{part, render_on};

fn script() -> String {
    String::from_utf8(reader_script().unwrap().body().to_vec()).unwrap()
}

#[test]
fn the_script_tag_carries_the_word_the_sheet_and_its_button_show() {
    let mut site = demo_site("en");
    site.script = Some(ScriptTag::new(&reader_script().unwrap(), "/blog/assets/"));
    let page = render_on(&site, long_post());
    assert_valid_page(&page);
    let tag = part(&page, "<script", "</script>");
    assert!(tag.contains(" data-contents=\"Contents\""), "{tag}");
    assert!(
        page.contains("<details class=\"toc-inline\">"),
        "scripts off still list the sections"
    );
}

#[test]
fn the_sheet_is_a_modal_dialog_with_a_close_button() {
    let js = script();
    assert!(js.contains("toc-button"));
    assert!(js.contains("toc-sheet"));
    assert!(
        js.contains("sheet.showModal()"),
        "a modal dialog keeps focus inside and Escape closes it"
    );
    assert!(js.contains("sheet.close()"));
    assert!(js.contains("aria-haspopup"));
}

#[test]
fn every_contents_list_marks_the_section_being_read() {
    let js = script();
    assert!(js.contains("\"aria-current\", \"location\""));
    assert!(
        js.contains("innerHeight / 3"),
        "the section is the last one past the top third"
    );
    assert!(js.contains(".toc-sheet a"));
}

#[test]
fn the_button_floats_on_narrow_screens_and_the_column_stays_in_view() {
    let css = stylesheet_text(&Tokens::default());
    assert!(css.contains(".toc-button { position: fixed;"), "{css}");
    assert!(
        css.contains(
            "@media (min-width: 70rem) { body:has(.post-with-toc) .toc-button { display: none; } }"
        ),
        "{css}"
    );
    assert!(css.contains("@media print { .toc-button, .toc-sheet { display: none; } }"));
    assert!(css.contains(".toc-sheet a[aria-current]"));
    assert!(
        css.contains("position: sticky;"),
        "the side column stays in view"
    );
}
