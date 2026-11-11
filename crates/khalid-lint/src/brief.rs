//! A post's brief: the questions, terms and links the writer planned, each
//! checked against the post as written, and each planned link against the
//! site (GEO P6).

use std::collections::BTreeSet;

use crate::focus::{holds, words};
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// The entries of `list` that hold a word, without their outer spaces, so
/// a blank entry asks for nothing.
fn entries(list: &[String]) -> impl Iterator<Item = &str> {
    list.iter()
        .map(|entry| entry.trim())
        .filter(|entry| !words(entry).is_empty())
}

/// Whether `heading` asks `question`: every word of the question is in the
/// heading, in any order, so "What internal links are" asks "What are
/// internal links?".
fn asks(heading: &[String], question: &[String]) -> bool {
    question.iter().all(|word| heading.contains(word))
}

/// A question in the brief that no heading of the post asks, one hit per
/// question in the brief's order.
#[derive(Debug)]
pub(crate) struct QuestionUnanswered;

impl Rule for QuestionUnanswered {
    fn id(&self) -> RuleId {
        RuleId::new("brief-question-unanswered")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let Some(brief) = &post.brief else {
            return Vec::new();
        };
        let headings: Vec<Vec<String>> = post.headings.iter().map(|h| words(&h.text)).collect();
        entries(&brief.questions)
            .filter(|question| {
                let question = words(question);
                !headings.iter().any(|heading| asks(heading, &question))
            })
            .map(Hit::about)
            .collect()
    }
}

/// A term in the brief that the post's body and headings never use, one hit
/// per term in the brief's order.
#[derive(Debug)]
pub(crate) struct TermMissing;

impl Rule for TermMissing {
    fn id(&self) -> RuleId {
        RuleId::new("brief-term-missing")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let Some(brief) = &post.brief else {
            return Vec::new();
        };
        let body = words(&post.body_text);
        let headings: Vec<Vec<String>> = post.headings.iter().map(|h| words(&h.text)).collect();
        entries(&brief.terms)
            .filter(|term| {
                let term = words(term);
                !holds(&body, &term) && !headings.iter().any(|heading| holds(heading, &term))
            })
            .map(Hit::about)
            .collect()
    }
}

/// `target` without its `#part` or `?query` and its last slash, so
/// `/blog/a/#why` and `/blog/a` name the same page. The home page `/` keeps
/// its slash.
fn page(target: &str) -> &str {
    let end = target.find(['#', '?']).unwrap_or(target.len());
    let path = target.get(..end).unwrap_or(target);
    match path.strip_suffix('/') {
        Some(rest) if !rest.is_empty() => rest,
        _ => path,
    }
}

/// A page in the brief that the post does not link to, one hit per page in
/// the brief's order.
#[derive(Debug)]
pub(crate) struct LinkMissing;

impl Rule for LinkMissing {
    fn id(&self) -> RuleId {
        RuleId::new("brief-link-missing")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let Some(brief) = &post.brief else {
            return Vec::new();
        };
        brief
            .links
            .iter()
            .map(|link| link.trim())
            .filter(|link| !page(link).is_empty())
            .filter(|link| !post.links.iter().any(|l| page(&l.target) == page(link)))
            .map(Hit::about)
            .collect()
    }
}

/// The folder a page sits in: `page` without its last part, so `/blog/a`
/// sits in `/blog` and `/a` in the site root, `""`.
fn folder(page: &str) -> &str {
    page.rsplit_once('/').map_or("", |(folder, _)| folder)
}

/// `folder` with each part of only digits, such as a year or a month, in
/// zeros of the same length, so the dated folders `/news/2025/03` and
/// `/news/2026/11` have one shape, `/news/0000/00`.
fn shape(folder: &str) -> String {
    let parts = folder.split('/').map(|part| {
        if !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()) {
            "0".repeat(part.len())
        } else {
            part.to_owned()
        }
    });
    parts.collect::<Vec<_>>().join("/")
}

/// A page in the brief at an address where the site's posts live, such as
/// `/blog/a-post/`, that no post, page or glossary term has, one hit per
/// page in the brief's order. When the address is an old one of a post,
/// the hit suggests the post's current address.
///
/// Only addresses in a folder that holds posts are judged, so tag, list
/// and home pages, which the linter does not know, never count as missing.
/// A dated folder counts for every date, so `/news/2025/03/a-post/` is
/// judged on a site whose posts sit in `/news/2026/11/`.
#[derive(Debug)]
pub(crate) struct LinkUnknown;

impl LinkUnknown {
    /// Whether the site has a page at `wanted`, a page key from [`page`].
    fn known(wanted: &str, site: &dyn SiteView) -> bool {
        let path_is = |path: &Option<String>| path.as_deref().is_some_and(|p| page(p) == wanted);
        let last = wanted.rsplit('/').next().unwrap_or_default();
        site.posts().iter().any(|post| path_is(&post.path))
            || site.pages().iter().any(|p| p.slug.as_str() == last)
            || site
                .glossary()
                .iter()
                .any(|term| term.slug.as_str() == last)
    }

    /// The current address of the post that lived at `wanted` before.
    fn moved(wanted: &str, site: &dyn SiteView) -> Vec<String> {
        let had = |post: &&LintPost| post.aliases.iter().any(|alias| page(alias) == wanted);
        let post = site.posts().iter().find(had);
        post.and_then(|post| post.path.clone())
            .into_iter()
            .collect()
    }
}

impl Rule for LinkUnknown {
    fn id(&self) -> RuleId {
        RuleId::new("brief-link-unknown")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        let Some(brief) = &post.brief else {
            return Vec::new();
        };
        let paths = site.posts().iter().filter_map(|post| post.path.as_deref());
        let folders: BTreeSet<String> = paths.map(|path| shape(folder(page(path)))).collect();
        brief
            .links
            .iter()
            .map(|link| link.trim())
            .filter(|link| {
                let wanted = page(link);
                wanted.starts_with('/')
                    && wanted != "/"
                    && folders.contains(&shape(folder(wanted)))
                    && !Self::known(wanted, site)
            })
            .map(|link| Hit::about(link).suggesting(Self::moved(page(link), site)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_sits_in_the_folder_before_its_last_part() {
        assert_eq!(folder("/blog/a"), "/blog");
        assert_eq!(folder("/news/2026/a"), "/news/2026");
        assert_eq!(folder("/a"), "");
    }

    #[test]
    fn dated_folders_share_one_shape_and_others_keep_theirs() {
        assert_eq!(shape("/news/2025/03"), shape("/news/2026/11"));
        assert_eq!(shape("/news/2026/11"), "/news/0000/00");
        assert_ne!(shape("/news/2026"), shape("/news/2026/11"));
        assert_eq!(shape("/blog/v2"), "/blog/v2");
        assert_eq!(shape(""), "");
    }

    #[test]
    fn a_page_is_the_same_with_or_without_its_slash_part_or_query() {
        assert_eq!(page("/blog/a/#why"), "/blog/a");
        assert_eq!(page("/blog/a?ref=x"), "/blog/a");
        assert_eq!(page("/blog/a"), "/blog/a");
        assert_eq!(page("/"), "/");
        assert_eq!(page("#top"), "");
    }

    #[test]
    fn a_heading_asks_a_question_in_any_word_order() {
        let heading = words("What internal links are");
        assert!(asks(&heading, &words("What are internal links?")));
        assert!(!asks(&heading, &words("How many internal links?")));
    }
}
