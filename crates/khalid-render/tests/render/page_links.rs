//! The numbered page links a template picks with its `pagination` part:
//! the first and last pages, the pages beside this one, and a gap for
//! each run of pages left out, as plain links that work with scripts off.

use khalid_render::{PageBody, PageKind, Pagination, RenderPack, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::{first_page, main_of};
use crate::parts::{pack_with, renderer};

fn pages(total: usize) -> Vec<String> {
    (1..=total)
        .map(|number| match number {
            1 => "/blog/archive/".to_owned(),
            _ => format!("/blog/archive/page/{number}/"),
        })
        .collect()
}

fn render(pack: &RenderPack<'_>, total: usize, current: usize) -> String {
    let mut listing = first_page();
    listing.pagination = Pagination::new(pages(total), current);
    let meta = meta("Archive", "/blog/archive/", PageKind::Website);
    let page = renderer(pack)
        .render(&demo_site("en"), &meta, &PageBody::Archive(listing))
        .unwrap();
    assert_valid_page(&page);
    main_of(&page).to_owned()
}

/// The page numbers and gaps shown, in order, as text such as `1 … 4`.
fn shown(html: &str) -> String {
    let (_, list) = html.split_once("<ol class=\"page-numbers\">").unwrap();
    let (list, _) = list.split_once("</ol>").unwrap();
    list.split("<li>")
        .skip(1)
        .map(|item| {
            let text = item.split('>').nth(1).unwrap_or_default();
            text.split('<').next().unwrap_or_default().to_owned()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn numbered() -> RenderPack<'static> {
    pack_with(&[("pagination", TemplateOption::Word("numbered".to_owned()))])
}

#[test]
fn the_pages_beside_this_one_show_with_gaps_for_the_rest() {
    assert_eq!(shown(&render(&numbered(), 9, 5)), "1 … 4 5 6 … 9");
    assert_eq!(shown(&render(&numbered(), 9, 1)), "1 2 … 9");
    assert_eq!(shown(&render(&numbered(), 9, 9)), "1 … 8 9");
}

#[test]
fn a_gap_of_one_page_shows_the_page_instead() {
    assert_eq!(shown(&render(&numbered(), 4, 1)), "1 2 3 4");
    assert_eq!(shown(&render(&numbered(), 7, 4)), "1 2 3 4 5 6 7");
}

#[test]
fn this_page_is_marked_current_and_the_others_link() {
    let html = render(&numbered(), 9, 5);
    assert!(
        html.contains("<li><a href=\"/blog/archive/page/5/\" aria-current=\"page\">5</a></li>"),
        "{html}"
    );
    assert!(
        html.contains("<li><a href=\"/blog/archive/page/4/\">4</a></li>"),
        "{html}"
    );
    assert!(html.contains("rel=\"prev\""), "{html}");
    assert!(html.contains("rel=\"next\""), "{html}");
}

#[test]
fn classic_keeps_newer_and_older_links() {
    let html = render(&RenderPack::classic(), 9, 5);
    assert!(!html.contains("page-numbers"), "{html}");
    assert!(html.contains("Page 5 of 9"), "{html}");
}
