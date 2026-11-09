//! The author box at the end of a post: each author's photo, name, role,
//! bio and profiles, then who reviewed it.

use khalid_render::{BodyHtml, Feature, RenderPost};

use crate::html_check::assert_valid_page;
use crate::image::avatar_image;
use crate::post::{part, person, profile, render, render_on, site_without, sitemap_post};

/// The demo post by Lena Vogt with her photo, bio and profiles, reviewed by
/// Mara Lind.
fn post_with_profile() -> RenderPost {
    let mut post = sitemap_post();
    let lena = &mut post.authors[0];
    lena.image = Some(avatar_image());
    lena.bio = Some(BodyHtml::from_sanitised(
        "<p>Lena has fixed crawl problems on sites with a million pages.</p>".to_owned(),
    ));
    lena.credentials = Some("Ten years running crawls for news sites".to_owned());
    lena.profiles = vec![
        profile("https://www.linkedin.com/in/lena-vogt-example"),
        profile("https://speakerdeck.com/lena-vogt-example"),
    ];
    post.reviewers = vec![person("Mara Lind", "mara-lind", None)];
    post
}

#[test]
fn the_post_ends_with_the_author_and_the_reviewer() {
    let page = render("en", post_with_profile());
    assert_valid_page(&page);
    insta::assert_snapshot!(part(&page, "<section class=\"author-box\">", "</section>"));
}

#[test]
fn the_box_shows_credentials_and_every_profile_by_icon_and_name() {
    let page = render("en", post_with_profile());
    let parts = part(&page, "<section class=\"author-box\">", "</section>");
    assert!(parts.contains("<strong>Experience</strong> Ten years running crawls"));
    let linkedin = "data-icon=\"linkedin\"><svg";
    assert!(parts.contains(linkedin), "{parts}");
    assert!(parts.contains("</svg><span>LinkedIn</span>"), "{parts}");
    // A page on a site with no icon of its own gets the globe, and its
    // host, so no link is a bare icon beside a named one.
    assert!(parts.contains("data-icon=\"website\"><svg"), "{parts}");
    assert!(
        parts.contains("</svg><span>speakerdeck.com</span>"),
        "{parts}"
    );
    assert!(!parts.contains("visually-hidden"), "{parts}");
    // Only the author's own page marks profiles rel="me".
    assert!(!parts.contains("rel=\"me\""));
}

#[test]
fn two_authors_get_the_plural_heading() {
    let mut post = post_with_profile();
    post.authors.push(person("Asif", "asif", None));
    let page = render("en", post);
    assert_valid_page(&page);
    assert!(page.contains("<h2>About the authors</h2>"));
}

#[test]
fn the_heading_reads_in_the_page_language() {
    let page = render("de", post_with_profile());
    assert!(page.contains("<section class=\"author-box\">\n<h2>Über den "));
}

#[test]
fn switched_off_there_is_no_box() {
    let page = render_on(&site_without(Feature::AuthorBox), post_with_profile());
    assert_valid_page(&page);
    assert!(!page.contains("author-box"));
}

#[test]
fn with_reviewed_by_off_the_box_names_no_reviewer() {
    let page = render_on(&site_without(Feature::ReviewedBy), post_with_profile());
    assert!(page.contains("author-box"));
    assert!(!page.contains("Mara Lind"));
}

#[test]
fn a_post_without_authors_has_no_box() {
    let mut post = sitemap_post();
    post.authors.clear();
    assert!(!render("en", post).contains("author-box"));
}
