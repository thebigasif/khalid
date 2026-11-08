//! The consent banner, the privacy choices page and the footer link to it
//! (privacy plan U4, R2, R6, R7, R8, R9).

use khalid_render::{
    ConsentBanner, ConsentPart, ConsentRow, GPC_END, GPC_START, PageBody, PageKind,
    RenderPrivacyChoices, Renderer,
};

use crate::common::{demo_site, locale, meta};
use crate::html_check::assert_valid_page;
use crate::post::{render, sitemap_post};

/// The banner of the English or German demo site on a post.
fn banner(tag: &str, part: ConsentPart) -> String {
    let prefix = if tag == "en" { "/blog/" } else { "/blog/de/" };
    let banner = ConsentBanner {
        action: "/blog/_consent/".to_owned(),
        back: format!("{prefix}what-a-sitemap-does/"),
        choices: format!("{prefix}privacy-choices/"),
        privacy: Some(format!("{prefix}privacy/")),
    };
    Renderer::new()
        .unwrap()
        .consent(&locale(tag), &banner, part)
        .unwrap()
}

/// The five rows of the choices page, as the wiring fills them.
fn rows() -> Vec<ConsentRow> {
    [
        "necessary",
        "preferences",
        "analytics",
        "marketing",
        "external-media",
    ]
    .into_iter()
    .map(|key| ConsentRow {
        key: key.to_owned(),
        name: format!("Name of {key}"),
        note: format!("What {key} covers."),
        vendors: if key == "external-media" {
            vec!["YouTube".to_owned(), "Vimeo".to_owned()]
        } else {
            Vec::new()
        },
        optional: key != "necessary",
    })
    .collect()
}

/// The choices page as a running server fills it.
fn live_page() -> RenderPrivacyChoices {
    RenderPrivacyChoices {
        title: "Your privacy choices".to_owned(),
        action: "/blog/_consent/".to_owned(),
        privacy: Some("/blog/privacy/".to_owned()),
        live: true,
        categories: rows(),
    }
}

/// `page` as the choices page of the English or German demo site.
fn drawn(tag: &str, page: RenderPrivacyChoices) -> String {
    let path = if tag == "en" {
        "/blog/privacy-choices/"
    } else {
        "/blog/de/privacy-choices/"
    };
    let meta = meta("Your privacy choices", path, PageKind::Website);
    Renderer::new()
        .unwrap()
        .render(&demo_site(tag), &meta, &PageBody::PrivacyChoices(page))
        .unwrap()
}

/// The choices page of a running server in `tag`.
fn choices(tag: &str) -> String {
    drawn(tag, live_page())
}

/// The `main` element of `page`.
fn main_of(page: &str) -> &str {
    let start = page.find("<main").unwrap();
    let end = page.find("</main>").unwrap();
    page.get(start..end).unwrap()
}

/// `html` with its words and the addresses that hold a language left out:
/// the tags alone, so a page in German can be held to the English one.
/// The spelling check reads English only, so German stays out of
/// snapshots.
fn shape(html: &str) -> String {
    let tags = html
        .split('<')
        .skip(1)
        .filter_map(|part| part.split_once('>'));
    let tags = tags.map(|(tag, _)| tag.replace("/blog/de/", "/blog/"));
    tags.collect::<Vec<_>>().join("><")
}

#[test]
fn the_banner_asks_in_english_and_german() {
    let english = banner("en", ConsentPart::Ask);
    insta::assert_snapshot!("banner_en", english);
    let german = banner("de", ConsentPart::Ask);
    assert_eq!(shape(&german), shape(&english));
    for words in [
        "Dein Datenschutz",
        ">Auswählen</a>",
        "Global Privacy Control",
    ] {
        assert!(german.contains(words), "{words}: {german}");
    }
    let saved = banner("en", ConsentPart::Saved);
    assert!(saved.contains("role=\"status\""), "{saved}");
    assert!(saved.contains("href=\"/blog/privacy-choices/\""), "{saved}");
    assert!(!saved.contains("<form"), "{saved}");
}

#[test]
fn accept_all_and_reject_all_differ_only_in_their_value_and_words() {
    let html = banner("en", ConsentPart::Ask);
    let button = |value: &str| {
        let at = html.find(&format!("value=\"{value}\"")).unwrap();
        let start = html.get(..at).unwrap().rfind("<button").unwrap();
        html.get(start..at).unwrap().to_owned()
    };
    assert_eq!(button("accept-all"), button("reject-all"));
    assert!(button("accept-all").contains("class=\"consent-button\""));
    let choose = html.split_once("class=\"consent-choose\"").unwrap().0;
    assert!(choose.ends_with("<a "), "Choose is a link: {html}");
}

#[test]
fn the_banner_keeps_its_gpc_note_inert_until_the_server_shows_it() {
    let html = banner("en", ConsentPart::Ask);
    let (_, rest) = html.split_once(GPC_START).unwrap();
    let (note, _) = rest.split_once(GPC_END).unwrap();
    assert!(note.contains("Global Privacy Control"), "{note}");
}

#[test]
fn the_choices_page_has_a_fieldset_per_category_and_every_switch_off() {
    let english = choices("en");
    assert_valid_page(&english);
    insta::assert_snapshot!("choices_en", main_of(&english));
    let german = choices("de");
    assert_eq!(shape(main_of(&german)), shape(main_of(&english)));
    assert!(
        german.contains(">Meine Auswahl speichern</button>"),
        "{german}"
    );
    assert_eq!(english.matches("<fieldset>").count(), 5);
    assert_eq!(english.matches("type=\"checkbox\"").count(), 4);
    assert!(
        !english.contains("checked"),
        "every optional switch starts off"
    );
    assert!(
        !english.contains("name=\"necessary\""),
        "Necessary is no switch"
    );
    assert!(english.contains("value=\"save\""));
    assert!(english.contains("name=\"back\" value=\"/blog/privacy-choices/\""));
    assert!(english.contains("YouTube, Vimeo"), "{english}");
}

#[test]
fn a_static_choices_page_says_what_happens_with_scripts_off() {
    let page = drawn(
        "en",
        RenderPrivacyChoices {
            live: false,
            ..live_page()
        },
    );
    assert_valid_page(&page);
    assert!(page.contains("<noscript>"), "{page}");
    assert!(!choices("en").contains("<noscript>"));
}

#[test]
fn every_page_links_to_the_privacy_choices_from_its_footer() {
    let english = render("en", sitemap_post());
    let footer = english.split_once("<footer").unwrap().1;
    assert!(
        footer.contains("<li><a href=\"/blog/privacy-choices/\">Your privacy choices</a></li>"),
        "{footer}"
    );
    let german = render("de", sitemap_post());
    assert!(
        german.contains("href=\"/blog/de/privacy-choices/\">")
            && german.contains("Datenschutz-Einstellungen</a>")
    );
    let own = choices("en");
    assert!(own.contains(
        "<a href=\"/blog/privacy-choices/\" aria-current=\"page\">Your privacy choices</a>"
    ));
}
