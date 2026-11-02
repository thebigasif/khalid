//! Case studies and the changelog: a list page and a page per entry.

use chrono::NaiveDate;
use khalid_render::{
    BodyHtml, EntryCard, EntryGroup, EntryKind, Link, PageBody, PageKind, RenderEntries,
    RenderEntry, Renderer,
};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::main_of;

fn render(body: &PageBody) -> String {
    let meta = meta("Case studies", "/blog/case-studies/", PageKind::Website);
    Renderer::new()
        .unwrap()
        .render(&demo_site("en"), &meta, body)
        .unwrap()
}

fn day(text: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d").ok()
}

fn study() -> EntryCard {
    EntryCard {
        title: "How a bakery doubled its search traffic".to_owned(),
        href: "/blog/case-studies/northfield-bakery/".to_owned(),
        date: day("2026-10-20"),
        summary: "One topic cluster on sourdough, every post linked to it.".to_owned(),
        client: Some("Northfield Bakery".to_owned()),
        outcome: Some("Search visits doubled in four months.".to_owned()),
        version: None,
        body: None,
    }
}

fn change() -> EntryCard {
    EntryCard {
        title: "Glossary pages from A to Z".to_owned(),
        href: "/blog/changelog/glossary-pages/".to_owned(),
        date: day("2026-10-28"),
        summary: String::new(),
        client: None,
        outcome: None,
        version: Some("1.3.0".to_owned()),
        body: Some(BodyHtml::from_sanitised(
            "<p>Every glossary gets a page.</p>".to_owned(),
        )),
    }
}

#[test]
fn case_studies_list_their_client_and_outcome() {
    let page = render(&PageBody::Entries(RenderEntries {
        title: "Case studies".to_owned(),
        intro: None,
        kind: EntryKind::CaseStudies,
        groups: vec![EntryGroup {
            year: None,
            month: None,
            entries: vec![study()],
        }],
    }));
    assert_valid_page(&page);
    insta::assert_snapshot!(main_of(&page));
}

#[test]
fn the_changelog_groups_its_entries_by_month() {
    let page = render(&PageBody::Entries(RenderEntries {
        title: "Changelog".to_owned(),
        intro: None,
        kind: EntryKind::Changelog,
        groups: vec![EntryGroup {
            year: Some(2026),
            month: Some(10),
            entries: vec![change()],
        }],
    }));
    assert_valid_page(&page);
    insta::assert_snapshot!(main_of(&page));
}

#[test]
fn an_empty_list_says_so() {
    let page = render(&PageBody::Entries(RenderEntries {
        title: "Changelog".to_owned(),
        intro: None,
        kind: EntryKind::Changelog,
        groups: Vec::new(),
    }));
    assert!(page.contains("<p>Nothing here yet.</p>"), "{page}");
}

#[test]
fn a_case_study_shows_its_client_outcome_and_way_back() {
    let entry = study();
    let page = render(&PageBody::Entry(Box::new(RenderEntry {
        title: entry.title,
        kind: EntryKind::CaseStudies,
        date: entry.date,
        client: entry.client,
        outcome: entry.outcome,
        version: None,
        back: Link {
            name: "Case studies".to_owned(),
            href: "/blog/case-studies/".to_owned(),
        },
        body: BodyHtml::from_sanitised("<p>The story.</p>".to_owned()),
    })));
    assert_valid_page(&page);
    insta::assert_snapshot!(main_of(&page));
}
