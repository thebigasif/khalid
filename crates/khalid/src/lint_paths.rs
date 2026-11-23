//! The addresses a site answers and the moves it keeps, for the lint rules
//! that judge where a post's links lead (backlog 2026-11-12 row 18):
//! `broken-internal-link` and `redirecting-link`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use khalid_content::SiteContent;
use khalid_core::{Config, UserMessage};
use khalid_lint::{KnownPaths, LintRedirect, LintSite};
use khalid_routes::{PostAddress, Routes};

use crate::build_lists::collection_rest;
use crate::build_pages::{path_of, routed};
use crate::lint_input::entry_files;
use crate::{build_redirects, db};

/// The pages every language of a site may have besides its posts, pages
/// and collections, each after the mount and the language prefix: the
/// lists and fixed pages `khalid build` writes, and the routes `khalid
/// serve` answers there. A link to one of them is never a link to nothing.
const FIXED: [&str; 16] = [
    "admin/",
    "ai/",
    "api/",
    "archive/",
    "authors/",
    "forms/",
    "glossary/",
    "impressum/",
    "newsletter/",
    "privacy/",
    "privacy-choices/",
    "search/",
    "series/",
    "sitemap/",
    "subscribe/",
    "unsubscribe/",
];

/// Every address `site`, the lint model of `content` on the site `config`
/// describes, answers: its posts and pages, its case studies and
/// changelog entries, the home page, author and collection pages of each
/// language and the [`FIXED`] pages, and the folders that hold its posts
/// and pages.
pub(crate) fn known(
    site: &LintSite,
    content: &SiteContent,
    (config, routes): (&Config, &Routes),
) -> KnownPaths {
    let posts = site.posts.iter().filter_map(|post| post.path.clone());
    let pages =
        (site.pages.iter()).map(|page| path_of(config, &page.locale, &format!("{}/", page.slug)));
    let held: BTreeSet<String> = posts.chain(pages).collect();
    let folders = held
        .iter()
        .filter_map(|path| path.trim_end_matches('/').rsplit_once('/'))
        .map(|(folder, _)| folder.to_owned())
        .collect();
    let mut paths = held;
    for locale in config.languages() {
        let rests = FIXED.iter().map(|rest| (*rest).to_owned());
        let collections =
            (site.collections.iter()).filter_map(|c| collection_rest(routes, &c.name));
        let authors = (site.authors.iter().flatten())
            .filter(|author| &author.locale == locale)
            .map(|author| format!("authors/{}/", author.slug));
        let all = std::iter::once(String::new())
            .chain(rests)
            .chain(collections)
            .chain(authors);
        paths.extend(all.map(|rest| path_of(config, locale, &rest)));
    }
    // Each entry's page sits where `khalid build` routes it, under its
    // collection's page.
    let collections: BTreeMap<&Path, &str> = (content.case_studies.iter())
        .map(|d| (d.path.as_path(), d.collection.as_str()))
        .chain(
            content
                .changelog
                .iter()
                .map(|d| (d.path.as_path(), d.collection.as_str())),
        )
        .collect();
    for (entry, file) in entry_files(content, config) {
        let Some(collection) = collections.get(file) else {
            continue;
        };
        let locale = &entry.locale;
        paths.insert(path_of(config, locale, &format!("{collection}/")));
        let address = PostAddress::new(collection, locale, &entry.slug, entry.date);
        if let Ok(rest) = routed(config, routes, locale, &address, file) {
            paths.insert(path_of(config, locale, &rest));
        }
    }
    KnownPaths::new(paths, folders)
}

/// The moves stored in the `site` folder's database, each from its old
/// address to where it now leads, or none when the site has no database.
///
/// # Errors
///
/// What, why and fix when the database cannot be read or its moves loop.
pub(crate) fn stored(site: &Path) -> Result<Vec<LintRedirect>, UserMessage> {
    // Checked here, before `stored_moves` makes `data/`, so a lint never
    // leaves a folder behind.
    let mut file = site.to_path_buf();
    file.push(db::DATA);
    file.push(db::FILE);
    if !file.exists() {
        return Ok(Vec::new());
    }
    let moves = build_redirects::stored_moves(site)?;
    let moves = moves.iter().filter_map(|rule| {
        let to = rule.redirect.location()?;
        Some(LintRedirect::new(rule.from.to_string(), to.to_string()))
    });
    Ok(moves.collect())
}
