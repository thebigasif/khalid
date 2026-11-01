//! The parts a template switches on with its options (templates spec 4.2):
//! each is built once in the shared templates and stays off in classic.

use std::collections::BTreeMap;

use khalid_core::AbsoluteUrl;
use khalid_render::{Brand, PageBody, PageKind, RenderPack, RenderSite, Renderer, TemplateOption};

use crate::common::{demo_site, meta};

/// A pack that sets each of `options` and replaces no file.
pub(crate) fn pack_with(options: &[(&str, TemplateOption)]) -> RenderPack<'static> {
    let options: BTreeMap<String, TemplateOption> = options
        .iter()
        .map(|(key, value)| ((*key).to_owned(), value.clone()))
        .collect();
    RenderPack {
        name: "trial",
        templates: Vec::new(),
        options,
        cta: None,
    }
}

/// A renderer with the shared templates and `pack`.
pub(crate) fn renderer(pack: &RenderPack<'_>) -> Renderer {
    let site = tempfile::tempdir().unwrap();
    Renderer::with_pack(site.path(), pack).unwrap()
}

fn brand() -> Brand {
    Brand {
        logo: Some(AbsoluteUrl::parse("https://www.example.com/logo.svg").unwrap()),
        icon: Some(AbsoluteUrl::parse("https://www.example.com/icon.png").unwrap()),
    }
}

fn not_found(renderer: &Renderer, site: &RenderSite) -> String {
    let meta = meta("Not found", "/blog/missing/", PageKind::Website);
    renderer.render(site, &meta, &PageBody::NotFound).unwrap()
}

#[test]
fn the_logo_shows_in_the_header_when_the_template_turns_it_on() {
    let on = pack_with(&[("logo", TemplateOption::Switch(true))]);
    let html = not_found(&renderer(&on).with_brand(&brand()), &demo_site("en"));
    assert!(
        html.contains(
            "<a class=\"site-name site-logo\" href=\"/blog/\"><img src=\"https://www.example.com/logo.svg\" alt=\"Field Notes on SEO\" decoding=\"async\"></a>"
        ),
        "{html}"
    );
}

#[test]
fn classic_keeps_the_name_as_text_even_with_a_logo() {
    let html = not_found(
        &renderer(&RenderPack::classic()).with_brand(&brand()),
        &demo_site("en"),
    );
    assert!(
        html.contains("<a class=\"site-name\" href=\"/blog/\">Field Notes on SEO</a>"),
        "{html}"
    );
    assert!(!html.contains("logo.svg"), "{html}");
}

#[test]
fn a_template_with_the_logo_on_shows_the_name_when_there_is_no_logo() {
    let on = pack_with(&[("logo", TemplateOption::Switch(true))]);
    let html = not_found(&renderer(&on), &demo_site("en"));
    assert!(
        html.contains("<a class=\"site-name\" href=\"/blog/\">Field Notes on SEO</a>"),
        "{html}"
    );
}

#[test]
fn the_icon_shows_in_the_browser_tab_in_every_template() {
    let html = not_found(
        &renderer(&RenderPack::classic()).with_brand(&brand()),
        &demo_site("en"),
    );
    assert!(
        html.contains("<link rel=\"icon\" href=\"https://www.example.com/icon.png\">"),
        "{html}"
    );
    let plain = not_found(&renderer(&RenderPack::classic()), &demo_site("en"));
    assert!(!plain.contains("rel=\"icon\""), "{plain}");
}

#[test]
fn every_part_is_off_in_classic_and_named_once() {
    let keys = khalid_render::part_keys();
    assert!(keys.contains(&"logo"), "{keys:?}");
    let mut sorted = keys.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), keys.len(), "a part is named twice: {keys:?}");
}
