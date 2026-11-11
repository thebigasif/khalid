//! The checks on whether the blog says who stands behind it: an About or
//! contact page a reader can reach (GEO P5), and an AI info page (GEO P3),
//! in the blog or on the main site it links to.

use khalid_core::AbsoluteUrl;
use serde_json::Value;

use crate::{Check, Crawl, Issue, Page};

/// The last part of an About or contact page's address. The linter's
/// `no-about-page` rule knows the same four.
const ABOUT_SLUGS: &[&str] = &["about", "contact", "about-us", "contact-us"];

/// The last part of an AI info page's address. Khalid writes its own at
/// `ai/` under each language.
const AI_PAGE_SLUGS: &[&str] = &["ai", "ai-info"];

/// The schema types `page_type:` gives an About or contact page.
const ABOUT_TYPES: &[&str] = &["AboutPage", "ContactPage"];

/// Warns once, on the home page, when no page of the blog that answered
/// is an About or contact page by its address or its schema, and no link
/// leads to one on the blog's own host. Adds a notice there too when no
/// such page or link is an AI info page by its address. A crawl that read
/// no page at all has bigger problems, so it gets neither.
pub(crate) fn trust_issues(crawl: &Crawl, issues: &mut Vec<Issue>) {
    let pages: Vec<(&AbsoluteUrl, &Page)> = crawl
        .visits()
        .iter()
        .filter(|visit| visit.outcome().is_ok())
        .filter_map(|visit| visit.page().map(|page| (visit.url(), page)))
        .collect();
    if pages.is_empty() {
        return;
    }
    let home = crawl.home();
    let on_main_site = |slugs: &[&str]| {
        crawl
            .outside()
            .iter()
            .any(|url| same_site(home.host(), url.host()) && ends_in(url, slugs))
    };
    let about = pages
        .iter()
        .any(|(url, page)| ends_in(url, ABOUT_SLUGS) || declares_about(page));
    if !about && !on_main_site(ABOUT_SLUGS) {
        issues.push(Issue::new(Check::AboutMissing, home, ""));
    }
    let ai_page = pages.iter().any(|(url, _)| ends_in(url, AI_PAGE_SLUGS));
    if !ai_page && !on_main_site(AI_PAGE_SLUGS) {
        issues.push(Issue::new(Check::AiPageMissing, home, ""));
    }
}

/// Whether `other` is the blog's own site: the blog's host, or with any
/// `www.` left off, the main domain of a blog on a subdomain, such as
/// `example.com` for `blog.example.com`.
fn same_site(blog: &str, other: &str) -> bool {
    let blog = blog.strip_prefix("www.").unwrap_or(blog);
    let other = other.strip_prefix("www.").unwrap_or(other);
    blog == other
        || blog
            .strip_suffix(other)
            .is_some_and(|sub| sub.ends_with('.'))
}

/// Whether the last part of `url`'s path is one of `slugs`.
fn ends_in(url: &AbsoluteUrl, slugs: &[&str]) -> bool {
    url.segments()
        .last()
        .is_some_and(|last| slugs.contains(last))
}

/// Whether one of `page`'s JSON-LD blocks names it an About or contact
/// page, at the top or inside `@graph`.
fn declares_about(page: &Page) -> bool {
    page.structured_data()
        .iter()
        .filter_map(|block| serde_json::from_str::<Value>(block).ok())
        .any(|value| has_about_type(&value))
}

/// Whether `value`, searched depth first, holds an About or contact type.
fn has_about_type(value: &Value) -> bool {
    match value {
        Value::Array(items) => items.iter().any(has_about_type),
        Value::Object(node) => {
            let typed = match node.get("@type") {
                Some(Value::String(kind)) => ABOUT_TYPES.contains(&kind.as_str()),
                Some(Value::Array(kinds)) => kinds.iter().any(|kind| {
                    kind.as_str()
                        .is_some_and(|kind| ABOUT_TYPES.contains(&kind))
                }),
                _ => false,
            };
            typed || node.get("@graph").is_some_and(has_about_type)
        }
        _ => false,
    }
}
