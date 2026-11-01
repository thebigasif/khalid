//! The template slots around a page: share links and the newsletter form,
//! each switched on by the template's options and read with scripts off
//! (templates spec 4.2).

use khalid_render::{PageBody, PageKind, RenderPack, RenderSite, Renderer, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::{first_page, home};
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
    post_on(renderer, &demo_site("en"))
}

fn post_on(renderer: &Renderer, site: &RenderSite) -> String {
    let post = sitemap_post();
    let meta = meta(&post.title, "/blog/what-a-sitemap-does/", PageKind::Article);
    let body = PageBody::Post(Box::new(post));
    renderer.render(site, &meta, &body).unwrap()
}

fn home_on(renderer: &Renderer, site: &RenderSite) -> String {
    let meta = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    let body = PageBody::Home(home(first_page()));
    renderer.render(site, &meta, &body).unwrap()
}

/// The demo site with a newsletter that takes sign-ups.
fn with_newsletter() -> RenderSite {
    let mut site = demo_site("en");
    site.subscribe = Some("/blog/subscribe".to_owned());
    site
}

#[test]
fn the_newsletter_form_posts_to_the_site_and_reads_with_scripts_off() {
    let renderer = with_options(&[("newsletter", word("post"))]);
    let page = post_on(&renderer, &with_newsletter());
    assert_valid_page(&page);
    let form = part(&page, "<section class=\"newsletter-slot\"", "</section>");
    insta::assert_snapshot!(form);
    let home = home_on(&renderer, &with_newsletter());
    assert!(!home.contains("newsletter-slot"), "post only");
}

#[test]
fn the_newsletter_shows_on_the_home_page_when_the_template_says() {
    let renderer = with_options(&[("newsletter", word("both"))]);
    let home = home_on(&renderer, &with_newsletter());
    assert_valid_page(&home);
    assert!(
        home.contains("<section class=\"newsletter-slot\""),
        "{home}"
    );
    assert!(post_on(&renderer, &with_newsletter()).contains("newsletter-slot"));
}

#[test]
fn a_site_without_a_newsletter_or_a_template_without_the_slot_shows_no_form() {
    let renderer = with_options(&[("newsletter", word("both"))]);
    assert!(
        !post_page(&renderer).contains("newsletter-slot"),
        "no target"
    );
    let classic = Renderer::new().unwrap();
    assert!(!post_on(&classic, &with_newsletter()).contains("newsletter-slot"));
    assert!(!home_on(&classic, &with_newsletter()).contains("newsletter-slot"));
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
