//! The site's call to action from `[template.cta]`: a band after a post or
//! on the front page, and a button in the header, each switched on by the
//! template's options, and the band wherever a body places `::cta`
//! (templates spec 4.2).

use khalid_core::{Locale, SlugMode};
use khalid_render::{
    BodyHtml, PageBody, PageKind, RenderCta, RenderPack, Renderer, TemplateOption,
};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::post::{part, sitemap_post};
use crate::slots::{home_on, post_on, with_pack, word};

pub(crate) fn trial_cta() -> RenderCta {
    RenderCta {
        label: "Start a free trial".to_owned(),
        href: "https://www.example.com/signup/".to_owned(),
        note: Some("No card needed.".to_owned()),
    }
}

#[test]
fn the_band_shows_the_note_and_the_button_where_the_template_says() {
    let renderer = with_pack(&[("cta_band", word("post"))], Some(trial_cta()));
    let page = post_on(&renderer, &demo_site("en"));
    assert_valid_page(&page);
    insta::assert_snapshot!(part(&page, "<div class=\"cta-band\">", "</div>"));
    assert!(!home_on(&renderer, &demo_site("en")).contains("cta-band"));

    let both = with_pack(&[("cta_band", word("both"))], Some(trial_cta()));
    let home = home_on(&both, &demo_site("en"));
    assert_valid_page(&home);
    assert!(home.contains("<div class=\"cta-band\">"), "{home}");
}

#[test]
fn the_header_button_links_to_the_call_to_action() {
    let options = [("cta_button", TemplateOption::Switch(true))];
    let page = post_on(&with_pack(&options, Some(trial_cta())), &demo_site("en"));
    assert_valid_page(&page);
    let header = part(&page, "<header class=\"site-header\">", "</header>");
    let button = "<a class=\"button header-button\" href=\"https://www.example.com/signup/\">Start a free trial</a>";
    assert!(header.contains(button), "{header}");
}

/// A post whose body places `::cta` between two paragraphs.
fn post_with_block(renderer: &Renderer) -> String {
    let locale = Locale::try_new("en").unwrap();
    let markdown = "Read this first.\n\n::cta\n\nThen this.\n";
    let html = khalid_content::render(markdown, &locale, SlugMode::Ascii).unwrap();
    let mut post = sitemap_post();
    post.body = BodyHtml::from_sanitised(html.html);
    let meta = meta(&post.title, "/blog/what-a-sitemap-does/", PageKind::Article);
    let body = PageBody::Post(Box::new(post));
    renderer.render(&demo_site("en"), &meta, &body).unwrap()
}

#[test]
fn a_cta_block_places_the_band_where_it_stands_in_every_template() {
    let site = tempfile::tempdir().unwrap();
    for renderer in [
        with_pack(&[], Some(trial_cta())),
        Renderer::with_pack(site.path(), &classic_with(trial_cta())).unwrap(),
    ] {
        let page = post_with_block(&renderer);
        assert_valid_page(&page);
        let body = part(&page, "<p>Read this first.</p>", "<p>Then this.</p>");
        assert!(body.contains("<div class=\"cta-band\">"), "{body}");
        assert!(!page.contains("khalid-cta"), "{page}");
    }
}

#[test]
fn a_cta_block_without_the_table_shows_nothing() {
    let page = post_with_block(&Renderer::new().unwrap());
    let body = part(&page, "<p>Read this first.</p>", "<p>Then this.</p>");
    assert_eq!(body, "<p>Read this first.</p>\n<p>Then this.</p>");
}

#[test]
fn the_mark_is_the_one_the_content_crate_writes() {
    assert_eq!(khalid_content::CTA_MARK, "<!--khalid-cta-->\n");
}

/// Classic, today's look, with the site's call to action.
fn classic_with(cta: RenderCta) -> RenderPack<'static> {
    let mut pack = RenderPack::classic();
    pack.cta = Some(cta);
    pack
}

#[test]
fn nothing_shows_without_the_table_or_the_options() {
    let on = [
        ("cta_band", word("both")),
        ("cta_button", TemplateOption::Switch(true)),
    ];
    let page = post_on(&with_pack(&on, None), &demo_site("en"));
    assert!(!page.contains("cta-band") && !page.contains("header-button"));
    let classic = Renderer::new().unwrap();
    let page = post_on(&classic, &demo_site("en"));
    assert!(!page.contains("cta-band") && !page.contains("header-button"));
}
