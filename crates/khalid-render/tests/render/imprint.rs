//! The imprint page, the Impressum German law asks for (backlog row 36):
//! who runs the site, how to reach them, and the lines the owner set.

use khalid_render::{PageBody, PageKind, RenderImprint, Renderer};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;

/// An imprint with only what `[privacy.contact]` gives.
fn bare() -> RenderImprint {
    RenderImprint {
        title: "Imprint".to_owned(),
        name: "Notes GmbH".to_owned(),
        address: "Hauptstraße 1, 10115 Berlin".to_owned(),
        email: "hello@example.com".to_owned(),
        phone: None,
        register: None,
        vat_id: None,
        responsible: None,
        privacy: Some("/blog/privacy/".to_owned()),
    }
}

/// An imprint with every line set.
fn full() -> RenderImprint {
    RenderImprint {
        phone: Some("+49 30 1234567".to_owned()),
        register: Some("Amtsgericht Berlin, HRB 12345".to_owned()),
        vat_id: Some("DE123456789".to_owned()),
        responsible: Some("Erika Muster, Hauptstraße 1, 10115 Berlin".to_owned()),
        ..bare()
    }
}

/// `page` drawn as the imprint of the English or German demo site.
fn drawn(tag: &str, page: RenderImprint) -> String {
    let path = if tag == "en" {
        "/blog/impressum/"
    } else {
        "/blog/de/impressum/"
    };
    let meta = meta(&page.title, path, PageKind::Website);
    Renderer::new()
        .unwrap()
        .render(&demo_site(tag), &meta, &PageBody::Imprint(Box::new(page)))
        .unwrap()
}

/// The `main` element of `page`.
fn main_of(page: &str) -> &str {
    let start = page.find("<main").unwrap();
    let end = page.find("</main>").unwrap();
    page.get(start..end).unwrap()
}

#[test]
fn the_imprint_shows_every_line_the_owner_set() {
    let page = drawn("en", full());
    assert_valid_page(&page);
    let main = main_of(&page);
    assert_eq!(main.matches("<h1").count(), 1, "{main}");
    for line in [
        "<h1>Imprint</h1>",
        "<address>Notes GmbH<br>Hauptstraße 1, 10115 Berlin</address>",
        "<a href=\"mailto:hello@example.com\">hello@example.com</a>",
        "<a href=\"tel:+49301234567\">+49 30 1234567</a>",
        "Amtsgericht Berlin, HRB 12345",
        "DE123456789",
        "Erika Muster, Hauptstraße 1, 10115 Berlin",
        "§ 5",
        "<a href=\"/blog/privacy/\">",
    ] {
        assert!(main.contains(line), "{line} in {main}");
    }
}

#[test]
fn lines_the_owner_left_out_leave_no_empty_heading() {
    let main = main_of(&drawn("en", bare())).to_owned();
    assert!(main.contains("hello@example.com"), "{main}");
    for gone in [
        "tel:",
        "imprint-register",
        "imprint-responsible",
        "<dd></dd>",
    ] {
        assert!(!main.contains(gone), "{gone} in {main}");
    }
}

#[test]
fn the_german_imprint_uses_the_german_words() {
    let page = drawn("de", full());
    assert_valid_page(&page);
    let main = main_of(&page);
    for words in ["Telefon", "Registereintrag", "§ 27a", "§ 18 Abs. 2"] {
        assert!(main.contains(words), "{words} in {main}");
    }
}

#[test]
fn the_owners_words_are_escaped() {
    let mut page = full();
    page.name = "<script>alert(1)</script>".to_owned();
    let main = main_of(&drawn("en", page)).to_owned();
    assert!(!main.contains("<script>alert"), "{main}");
    assert!(main.contains("&lt;script&gt;"), "{main}");
}
