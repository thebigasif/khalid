//! The breadcrumbs part: the trail from the home page to the page, as a
//! list of links above the content, when the template turns it on.

use khalid_render::{
    BreadcrumbStyle, Link, PageBody, PageKind, PageMeta, RenderPack, TemplateOption,
};

use crate::common::{demo_site, meta};
use crate::parts::{pack_with, renderer};

fn link(name: &str, href: &str) -> Link {
    Link {
        name: name.to_owned(),
        href: href.to_owned(),
    }
}

fn trail() -> PageMeta {
    let mut meta = meta(
        "What a sitemap does",
        "/blog/what-a-sitemap-does/",
        PageKind::Article,
    );
    meta.breadcrumbs = vec![
        link("Field Notes on SEO", "/blog/"),
        link("Technical SEO", "/blog/category/technical-seo/"),
        link("What a sitemap does", "/blog/what-a-sitemap-does/"),
    ];
    meta
}

#[test]
fn the_trail_shows_as_links_with_the_page_marked_current() {
    let on = pack_with(&[("breadcrumbs", TemplateOption::Switch(true))]);
    let html = renderer(&on)
        .render(&demo_site("en"), &trail(), &PageBody::NotFound)
        .unwrap();
    let want = "<nav class=\"breadcrumbs\" aria-label=\"Breadcrumbs\">\n<ol>\n\
<li><a href=\"/blog/\">Field Notes on SEO</a></li>\n\
<li><span class=\"crumb-sep\" aria-hidden=\"true\">/</span><a href=\"/blog/category/technical-seo/\">Technical SEO</a></li>\n\
<li><span class=\"crumb-sep\" aria-hidden=\"true\">/</span><a href=\"/blog/what-a-sitemap-does/\" aria-current=\"page\">What a sitemap does</a></li>\n\
</ol>\n</nav>";
    assert!(html.contains(want), "{html}");
}

#[test]
fn the_sites_separator_and_bold_last_step_reach_the_trail() {
    let on = pack_with(&[("breadcrumbs", TemplateOption::Switch(true))]);
    let mut site = demo_site("en");
    site.breadcrumbs = BreadcrumbStyle {
        separator: "»".to_owned(),
        bold_last: true,
    };
    let html = renderer(&on)
        .render(&site, &trail(), &PageBody::NotFound)
        .unwrap();
    assert!(
        html.contains("<nav class=\"breadcrumbs\" data-bold-last "),
        "{html}"
    );
    assert_eq!(html.matches(">»</span>").count(), 2, "{html}");
}

#[test]
fn classic_and_a_page_without_a_trail_show_no_breadcrumbs() {
    let classic = renderer(&RenderPack::classic())
        .render(&demo_site("en"), &trail(), &PageBody::NotFound)
        .unwrap();
    assert!(!classic.contains("breadcrumbs"), "{classic}");
    let on = pack_with(&[("breadcrumbs", TemplateOption::Switch(true))]);
    let home = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    let html = renderer(&on)
        .render(&demo_site("en"), &home, &PageBody::NotFound)
        .unwrap();
    assert!(!html.contains("class=\"breadcrumbs\""), "{html}");
}
