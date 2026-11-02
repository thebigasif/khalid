//! A post in a series: "Part 2 of 3", every part in order, and the parts
//! on either side.

use khalid_render::{Link, RenderSeries};

use crate::html_check::assert_valid_page;
use crate::post::{part, render, sitemap_post};

fn link(name: &str, slug: &str) -> Link {
    Link {
        name: name.to_owned(),
        href: format!("/blog/{slug}/"),
    }
}

fn series(part: usize) -> RenderSeries {
    let parts = vec![
        link("Pick the subject", "pick-the-subject"),
        link("Plan the posts", "plan-the-posts"),
        link("Link the cluster", "link-the-cluster"),
    ];
    RenderSeries {
        name: "Topic clusters from scratch".to_owned(),
        href: "/blog/series/topic-clusters-from-scratch/".to_owned(),
        part,
        total: parts.len(),
        previous: part.checked_sub(2).and_then(|at| parts.get(at)).cloned(),
        next: parts.get(part).cloned(),
        parts,
    }
}

#[test]
fn a_post_in_a_series_shows_its_part_the_list_and_the_steps() {
    let mut post = sitemap_post();
    post.series = Some(series(2));
    let page = render("en", post);
    assert_valid_page(&page);
    insta::assert_snapshot!(part(&page, r#"<nav class="series-box""#, "</nav>"));
    insta::assert_snapshot!(part(&page, r#"<nav class="series-steps""#, "</nav>"));
}

#[test]
fn the_first_part_links_only_forward_and_german_says_teil() {
    let mut post = sitemap_post();
    post.series = Some(series(1));
    let page = render("de", post);
    assert!(page.contains("Teil 1 von 3"), "{page}");
    assert!(page.contains(r#"<li><span aria-current="page">Pick the subject</span></li>"#));
    assert!(!page.contains("series-previous"), "{page}");
    assert!(page.contains("<span>Nächster Teil</span> Plan the posts</a>"));
}

#[test]
fn a_post_in_no_series_shows_neither() {
    let page = render("en", sitemap_post());
    assert!(!page.contains("series-"), "{page}");
}
