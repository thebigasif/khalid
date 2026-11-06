//! The featured slot, the newest featured post above a list and out of the
//! list below it, and the editor's picks on the home page, both switched
//! on by a template's parts.

use khalid_render::{Loading, PageBody, PageKind, RenderPack, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::image::sitemap_image;
use crate::listing::{
    assert_only_the_first_picture_loads_at_once, card, first_page, home, main_of, with_pictures,
};
use crate::parts::{pack_with, renderer};

fn render(pack: &RenderPack<'_>, body: &PageBody) -> String {
    let meta = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    let page = renderer(pack)
        .render(&demo_site("en"), &meta, body)
        .unwrap();
    assert_valid_page(&page);
    main_of(&page).to_owned()
}

fn featured_home() -> PageBody {
    let mut listing = first_page();
    let mut star = listing.posts[1].clone();
    star.featured = true;
    listing.posts[1] = star.clone();
    listing.featured = Some(star.clone());
    let mut picked = home(listing);
    let mut older = card(
        "Planning a topic cluster",
        "planning-a-topic-cluster",
        7,
        "",
    );
    older.featured = true;
    picked.picks = vec![star, older];
    PageBody::Home(picked)
}

#[test]
fn the_featured_post_leads_the_list_and_leaves_it() {
    let on = pack_with(&[("featured", TemplateOption::Switch(true))]);
    let html = render(&on, &featured_home());
    let slot = "<section class=\"featured\" aria-labelledby=\"featured-title\">\n\
<h2 id=\"featured-title\">Featured</h2>\n<article class=\"card card-featured\">";
    assert!(html.contains(slot), "{html}");
    assert_eq!(
        html.matches("href=\"/blog/how-to-write-a-title-tag/\"")
            .count(),
        1,
        "{html}"
    );
}

#[test]
fn the_picks_follow_the_featured_post_and_skip_it() {
    let on = pack_with(&[
        ("featured", TemplateOption::Switch(true)),
        ("picks", TemplateOption::Number(4)),
    ]);
    let html = render(&on, &featured_home());
    assert!(
        html.contains("<section class=\"picks\">\n<h2>Editor&#39;s picks</h2>"),
        "{html}"
    );
    assert!(html.contains("Planning a topic cluster"), "{html}");
    assert_eq!(
        html.matches("href=\"/blog/how-to-write-a-title-tag/\"")
            .count(),
        1,
        "{html}"
    );
}

#[test]
fn the_picks_alone_list_every_featured_post_up_to_their_count() {
    let on = pack_with(&[("picks", TemplateOption::Number(1))]);
    let html = render(&on, &featured_home());
    assert!(html.contains("<section class=\"picks\">"), "{html}");
    assert!(!html.contains("Planning a topic cluster"), "{html}");
    assert!(!html.contains("class=\"featured\""), "{html}");
}

#[test]
fn classic_shows_neither() {
    let html = render(&RenderPack::classic(), &featured_home());
    assert!(!html.contains("class=\"featured\""), "{html}");
    assert!(!html.contains("class=\"picks\""), "{html}");
    assert_eq!(
        html.matches("href=\"/blog/how-to-write-a-title-tag/\"")
            .count(),
        1,
        "{html}"
    );
}

#[test]
fn the_featured_post_above_the_list_is_the_picture_that_loads_at_once() {
    let on = pack_with(&[("featured", TemplateOption::Switch(true))]);
    let mut listing = with_pictures(first_page());
    let mut star = card(
        "Core Web Vitals in plain words",
        "core-web-vitals",
        30,
        "Three numbers.",
    );
    star.image = Some(sitemap_image(Loading::Lazy));
    listing.featured = Some(star);
    let html = render(&on, &PageBody::Collection(listing));
    assert!(html.contains("card card-featured"), "{html}");
    let (featured, list) = html.split_once("<ul class=\"cards\"").unwrap();
    assert!(featured.contains(r#"fetchpriority="high""#), "{featured}");
    assert!(!list.contains("fetchpriority"), "{list}");
    assert_only_the_first_picture_loads_at_once(&html, 3);
}
