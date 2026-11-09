//! Home sections of every kind in the owner's order: a newsletter section
//! puts the sign-up form between the others, and the template's own place
//! for the form on the home page stays empty, so it shows once.

use std::collections::BTreeMap;

use khalid_render::{
    HomeSectionKind, HomeSectionPosts, PageBody, PageKind, RenderHome, RenderPack, RenderSite,
    Renderer, TemplateOption,
};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::{first_page, home, main_of};

/// The demo home page with a Technical SEO section, the newsletter, then
/// the newest posts.
fn ordered() -> RenderHome {
    let listing = first_page();
    let mut front = home(listing.clone());
    let posts = |title: &str, href: &str| HomeSectionPosts {
        title: title.to_owned(),
        href: href.to_owned(),
        posts: listing.posts.clone(),
        kind: HomeSectionKind::Posts,
    };
    front.sections = vec![
        posts("Technical SEO", "/blog/category/technical-seo/"),
        HomeSectionPosts::newsletter(),
        posts("Latest posts", "/blog/archive/"),
    ];
    front
}

fn subscribed() -> RenderSite {
    let mut site = demo_site("en");
    site.subscribe = Some("/blog/subscribe".to_owned());
    site
}

/// The home page rendered by a pack whose `newsletter` option is `place`.
fn render(site: &RenderSite, front: RenderHome, place: &str) -> String {
    let options = BTreeMap::from([(
        "newsletter".to_owned(),
        TemplateOption::Word(place.to_owned()),
    )]);
    let pack = RenderPack {
        name: "trial",
        templates: Vec::new(),
        options,
        cta: None,
    };
    let folder = tempfile::tempdir().unwrap();
    let renderer = Renderer::with_pack(folder.path(), &pack).unwrap();
    let meta = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    renderer
        .render(site, &meta, &PageBody::Home(front))
        .unwrap()
}

#[test]
fn the_newsletter_shows_once_where_its_section_stands() {
    let page = render(&subscribed(), ordered(), "both");
    assert_valid_page(&page);
    let main = main_of(&page);
    assert_eq!(
        main.matches("<form class=\"subscribe\"").count(),
        1,
        "{main}"
    );
    let at = |mark: &str| {
        main.find(mark)
            .unwrap_or_else(|| panic!("{mark} in {main}"))
    };
    let form = at("id=\"home-newsletter-title\"");
    assert!(at("<h2>Technical SEO</h2>") < form, "{main}");
    assert!(form < at("<h2>Latest posts</h2>"), "{main}");
}

#[test]
fn without_a_newsletter_section_the_template_keeps_its_own_place() {
    let mut front = ordered();
    front
        .sections
        .retain(|section| section.kind == HomeSectionKind::Posts);
    let page = render(&subscribed(), front, "home");
    let main = main_of(&page);
    assert_eq!(
        main.matches("<form class=\"subscribe\"").count(),
        1,
        "{main}"
    );
    assert!(!main.contains("home-newsletter"), "{main}");
}

#[test]
fn a_site_without_a_newsletter_shows_no_form_in_the_section() {
    let page = render(&demo_site("en"), ordered(), "home");
    assert_valid_page(&page);
    assert!(!page.contains("<form class=\"subscribe\""));
    assert!(main_of(&page).contains("<h2>Latest posts</h2>"));
}
