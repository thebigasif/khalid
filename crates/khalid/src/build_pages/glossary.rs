//! The glossary's A to Z page of each language, at `/glossary/` (plan
//! section 5.13): a row of letters to jump to, then every term under its
//! letter with its one-sentence definition, and the `DefinedTermSet` that
//! names each term in the page's JSON-LD.

use std::collections::BTreeMap;

use khalid_content::{SiteContent, Wikilinks};
use khalid_core::{Config, Locale, Slug, SlugMode, UserMessage};
use khalid_render::{
    GlossaryGroup, GlossaryLinks, GlossaryTerm, Link, PageBody, PageKind, RenderGlossary, Renderer,
};
use khalid_routes::Routes;
use khalid_seo::{ListEntry, PageKind as SeoKind};

use crate::build_pages::{self, LanguagePages, Page};
use crate::{build_lists, build_seo};

/// The end of the glossary page's address, after the mount and the
/// language, where `khalid_seo::SeoSite` expects the `DefinedTermSet`.
pub(crate) const REST: &str = "glossary/";

/// The folder of the glossary's terms, whose `_index.md` introduces it.
const COLLECTION: &str = "glossary";

/// The group of terms that start with a digit.
const DIGITS: (&str, &str) = ("0-9", "digits");

/// Adds the glossary page to each of `languages` that has a term, made
/// from its term pages, titled in its language by `renderer`'s strings or
/// by the glossary's own `_index.md` in `content`, whose body is the text
/// above the letters. A language with no term gets no page, so no empty
/// page reaches search.
///
/// # Errors
///
/// Returns what, why and fix when the page's address cannot be made from
/// the site's settings, or the `_index.md` body cannot be rendered.
pub(crate) fn add(
    (config, routes): (&Config, &Routes),
    renderer: &Renderer,
    content: &SiteContent,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let terms: Vec<&Page> = (language.pages.iter())
            .filter(|page| page.seo.kind == SeoKind::GlossaryTerm)
            .collect();
        if terms.is_empty() {
            continue;
        }
        let entries = terms
            .iter()
            .map(|page| ListEntry::new(&page.meta.title, page.meta.canonical.clone()))
            .collect();
        let groups = groups(&locale, &terms);
        let index = (content.indexes.iter())
            .find(|doc| doc.locale == locale && doc.collection == COLLECTION);
        let front = index.map(|doc| &doc.front);
        let title = match front.and_then(|front| front.title.clone()) {
            Some(title) => title,
            None => build_lists::text(renderer, &locale, "glossary_title")?,
        };
        let description = (front.and_then(|front| front.description.clone())).unwrap_or_default();
        let intro = match index.filter(|doc| !doc.body.trim().is_empty()) {
            Some(doc) => {
                let links = Wikilinks::for_language(content, config, routes, &locale);
                Some(build_pages::body_html(config, &links, doc)?)
            }
            None => None,
        };
        let path = build_pages::path_of(config, &locale, REST);
        let back = path.clone();
        let meta = build_pages::meta(
            config,
            title.clone(),
            description.clone(),
            path,
            PageKind::Website,
        )?;
        let mut seo = build_seo::page(SeoKind::GlossaryIndex, &locale, REST, &title)?;
        seo.excerpt = description;
        seo.translation = build_lists::list_key("glossary")?;
        seo.item_count = u32::try_from(terms.len()).ok();
        seo.terms = entries;
        let body = PageBody::Glossary(RenderGlossary {
            title,
            intro,
            groups,
        });
        language
            .pages
            .push(build_pages::page(&locale, config, REST, meta, body, seo));
        let origin = config.address().as_str().trim_end_matches('/');
        neighbours(language, &back, origin);
    }
    Ok(())
}

/// How many related terms a term's page lists.
const MOST_RELATED: usize = 5;

/// How many of the posts that use a term its page lists.
const MOST_POSTS: usize = 10;

/// Gives each term page of `language` the way back to the glossary at
/// `glossary`, its related terms and the newest posts that link to it.
/// Links are read from the rendered bodies, by path or by `origin` and
/// path, so a wikilink and a pasted address count the same. Each body is
/// read once, however many terms the glossary has.
fn neighbours(language: &mut LanguagePages, glossary: &str, origin: &str) {
    let terms: Vec<Link> = (language.pages.iter())
        .filter(|page| page.seo.kind == SeoKind::GlossaryTerm)
        .map(|page| Link {
            name: page.meta.title.clone(),
            href: page.meta.path.clone(),
        })
        .collect();
    let index: BTreeMap<String, usize> = (terms.iter().enumerate())
        .map(|(at, term)| (term.href.clone(), at))
        .collect();
    let related = related(&language.pages, origin, &index);
    let mut posts = posts(language, origin, &index);
    for page in &mut language.pages {
        let Some(&at) = index.get(&page.meta.path) else {
            continue;
        };
        let PageBody::Page(own) = &mut page.body else {
            continue;
        };
        let shown = related.get(at).map(Vec::as_slice).unwrap_or_default();
        own.term = Some(GlossaryLinks {
            glossary: glossary.to_owned(),
            related: (shown.iter().take(MOST_RELATED))
                .filter_map(|other| terms.get(*other).cloned())
                .collect(),
            posts: posts.get_mut(at).map(std::mem::take).unwrap_or_default(),
        });
    }
}

/// For each term of `index`, the other terms its definition among `pages`
/// links to, in the order it names them, then those whose definitions
/// link to it.
fn related(pages: &[Page], origin: &str, index: &BTreeMap<String, usize>) -> Vec<Vec<usize>> {
    let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); index.len()];
    for page in pages {
        let (PageBody::Page(own), Some(&from)) = (&page.body, index.get(&page.meta.path)) else {
            continue;
        };
        if let Some(out) = outgoing.get_mut(from) {
            for to in linked(own.body.as_str(), origin, index) {
                if to != from && !out.contains(&to) {
                    out.push(to);
                }
            }
        }
    }
    let mut related = outgoing.clone();
    for (from, out) in outgoing.iter().enumerate() {
        for &to in out {
            if let Some(theirs) = related.get_mut(to)
                && !theirs.contains(&from)
            {
                theirs.push(from);
            }
        }
    }
    related
}

/// For each term of `index`, up to [`MOST_POSTS`] of the listed posts of
/// `language` whose bodies link to it, newest first.
fn posts(
    language: &LanguagePages,
    origin: &str,
    index: &BTreeMap<String, usize>,
) -> Vec<Vec<Link>> {
    let bodies: BTreeMap<&str, &str> = (language.pages.iter())
        .filter_map(|page| match &page.body {
            PageBody::Post(post) => Some((page.meta.path.as_str(), post.body.as_str())),
            _ => None,
        })
        .collect();
    let mut posts: Vec<Vec<Link>> = vec![Vec::new(); index.len()];
    for card in language.listed.iter().map(|post| &post.card) {
        let Some(body) = bodies.get(card.href.as_str()) else {
            continue;
        };
        for to in linked(body, origin, index) {
            if let Some(theirs) = posts.get_mut(to)
                && theirs.len() < MOST_POSTS
                && theirs.iter().all(|post| post.href != card.href)
            {
                theirs.push(Link {
                    name: card.title.clone(),
                    href: card.href.clone(),
                });
            }
        }
    }
    posts
}

/// The terms of `index` that `html` links to, in order, each time it does:
/// by path, such as `/blog/glossary/canonical/`, or by `origin` and path,
/// with any `#part` left off.
fn linked<'a>(
    html: &'a str,
    origin: &'a str,
    index: &'a BTreeMap<String, usize>,
) -> impl Iterator<Item = usize> + 'a {
    html.split("href=\"").skip(1).filter_map(move |rest| {
        let href = rest.split('"').next().unwrap_or_default();
        let path = href.strip_prefix(origin).unwrap_or(href);
        index
            .get(path.split('#').next().unwrap_or_default())
            .copied()
    })
}

/// The term pages `terms` grouped under their first letter: the digits
/// first, then each letter in order, then any other script by its own
/// first letter. A letter with an accent sorts with its base letter, as the
/// language spells it in an address, so German `Ähnlichkeit` is under A.
fn groups(locale: &Locale, terms: &[&Page]) -> Vec<GlossaryGroup> {
    let mut keyed: Vec<(String, GlossaryTerm)> = terms
        .iter()
        .map(|page| {
            let term = GlossaryTerm {
                term: page.meta.title.clone(),
                href: page.meta.path.clone(),
                summary: page.meta.description.clone(),
            };
            (sort_key(locale, &term.term), term)
        })
        .collect();
    keyed.sort_by(|(a, x), (b, y)| a.cmp(b).then_with(|| x.term.cmp(&y.term)));
    let mut groups: Vec<GlossaryGroup> = Vec::new();
    for (key, term) in keyed {
        let (letter, anchor) = letter(&key);
        match groups.last_mut() {
            Some(group) if group.anchor == anchor => group.terms.push(term),
            _ => groups.push(GlossaryGroup {
                letter,
                anchor,
                terms: vec![term],
            }),
        }
    }
    groups
}

/// The text a term sorts by: its address form in ASCII, such as
/// `aehnlichkeit`, or the term in lowercase when its script has no ASCII
/// form.
fn sort_key(locale: &Locale, term: &str) -> String {
    Slug::from_title(term, locale, SlugMode::Ascii)
        .map_or_else(|_| term.to_lowercase(), |slug| slug.to_string())
}

/// The letter a reader sees for a sort key, and its ASCII anchor.
fn letter(key: &str) -> (String, String) {
    let first = key.chars().next().unwrap_or('0');
    if first.is_ascii_digit() {
        (DIGITS.0.to_owned(), DIGITS.1.to_owned())
    } else if first.is_ascii_alphabetic() {
        let anchor = first.to_ascii_lowercase().to_string();
        (anchor.to_ascii_uppercase(), anchor)
    } else {
        let anchor = format!("u{:x}", u32::from(first));
        (first.to_uppercase().to_string(), anchor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_sort_with_their_base_letter_and_other_scripts_keep_theirs() {
        let german = Locale::try_new("de").unwrap();
        assert_eq!(letter(&sort_key(&german, "Ähnlichkeit")).0, "A");
        assert_eq!(letter(&sort_key(&german, "301 redirect")).1, "digits");
        let shown = letter(&sort_key(&german, "ссылка"));
        assert_eq!(shown, ("С".to_owned(), "u441".to_owned()));
    }
}
