//! What a reader of a tag, series or sitemap page needs to know where
//! they are: the tag's label, each part's number, and categories and tags
//! told apart.

use khalid_render::{
    Link, PageBody, PageKind, RenderSitemap, Renderer, SitemapGroup, SitemapSection,
};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::first_page;

fn render(tag: &str, body: &PageBody) -> String {
    let meta = meta(
        "Internal Links",
        "/blog/tag/internal-links/",
        PageKind::Website,
    );
    Renderer::new()
        .unwrap()
        .render(&demo_site(tag), &meta, body)
        .unwrap()
}

fn link(name: &str, href: &str) -> Link {
    Link {
        name: name.to_owned(),
        href: href.to_owned(),
    }
}

#[test]
fn a_tag_page_says_it_is_a_tag_above_its_name() {
    let mut listing = first_page();
    "Internal Links".clone_into(&mut listing.title);
    let page = render("en", &PageBody::Tag(listing.clone()));
    assert_valid_page(&page);
    assert!(
        page.contains("<p class=\"list-kind\">Tag</p>\n<h1>Internal Links</h1>"),
        "{page}"
    );
    let german = render("de", &PageBody::Tag(listing.clone()));
    assert!(
        german.contains("<p class=\"list-kind\">Schlagwort</p>"),
        "{german}"
    );
    let category = render("en", &PageBody::Category(listing));
    assert!(!category.contains("list-kind"), "{category}");
}

#[test]
fn a_series_page_numbers_its_parts_in_order() {
    let mut listing = first_page();
    listing.numbered = true;
    listing.pagination = None;
    let page = render("en", &PageBody::Collection(listing.clone()));
    assert_valid_page(&page);
    let first = page
        .find("<h2><span class=\"card-part\">Part 1</span> <a")
        .unwrap();
    let second = page
        .find("<h2><span class=\"card-part\">Part 2</span> <a")
        .unwrap();
    assert!(first < second, "{page}");
    let german = render("de", &PageBody::Collection(listing.clone()));
    assert!(
        german.contains("<span class=\"card-part\">Teil 2</span>"),
        "{german}"
    );
    listing.numbered = false;
    assert!(!render("en", &PageBody::Collection(listing)).contains("card-part"));
}

#[test]
fn the_sitemap_puts_categories_and_tags_under_their_own_headings() {
    let sitemap = RenderSitemap {
        title: "Sitemap".to_owned(),
        groups: vec![
            SitemapGroup {
                section: SitemapSection::Categories,
                links: vec![link("Links", "/blog/category/links/")],
            },
            SitemapGroup {
                section: SitemapSection::Tags,
                links: vec![link("Links", "/blog/tag/links/")],
            },
        ],
    };
    let page = render("en", &PageBody::Sitemap(sitemap));
    assert_valid_page(&page);
    let categories = page.find("<h2>Categories</h2>").unwrap();
    let tags = page.find("<h2>Tags</h2>").unwrap();
    assert!(categories < tags, "{page}");
}
