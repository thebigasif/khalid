//! The site search index, written at publish time: each published post's
//! title, headings and body as terms, in its language (ADR "Site search is
//! our own inverted index in SQLite").

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use khalid_content::{parse_post, render};
use khalid_core::{Config, SitePath};
use khalid_db::{Database, DbError, IndexedPost, SearchPage, SiteId};
use khalid_search::{Analyzer, Language};

use crate::build_pages;
use crate::post_lock::Lock;
use crate::site_content::Catalog;

/// How many pages one transaction writes, so a save waiting for the writer
/// never waits behind the whole site.
const PAGES_PER_WRITE: usize = 100;

/// Which posts of a catalog go into the index.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Which<'a> {
    /// Every published post, after a rebuild emptied the index.
    All,
    /// Only these files, after a save: new or edited ones are written again,
    /// and `removed` ones leave the index.
    Changed {
        written: &'a BTreeSet<String>,
        removed: &'a BTreeSet<String>,
    },
}

/// Writes the search pages of `which` posts of `catalog` into `db`. The
/// files are read and rendered off the async runtime (RULES.md BL-RUN-7).
///
/// # Errors
///
/// Returns the database's error when a page cannot be written. A post whose
/// file cannot be read now is left out, and the full render says why.
pub(crate) async fn write(
    (db, site): (&Database, SiteId),
    folder: &Path,
    config: &Config,
    catalog: &Catalog,
    which: Which<'_>,
) -> Result<(), DbError> {
    let rows: Vec<IndexedPost> = catalog
        .index
        .posts
        .iter()
        .filter(|row| match which {
            Which::All => true,
            Which::Changed { written, .. } => written.contains(&row.path),
        })
        .cloned()
        .collect();
    let (folder, config) = (folder.to_path_buf(), config.clone());
    let made = tokio::task::spawn_blocking(move || pages(&folder, &config, &rows));
    let pages = made.await.unwrap_or_else(|error| {
        tracing::warn!(?error, "the search pages could not be made");
        Vec::new()
    });
    let none = BTreeSet::new();
    let removed = match which {
        Which::All => &none,
        Which::Changed { removed, .. } => removed,
    };
    db.index_search(site, &[], removed).await?;
    for chunk in pages.chunks(PAGES_PER_WRITE) {
        db.index_search(site, chunk, &none).await?;
    }
    Ok(())
}

/// The search page of each published post among `rows`, read from its file
/// in `folder`. A post without a date is a draft, which search never shows.
pub(crate) fn pages(folder: &Path, config: &Config, rows: &[IndexedPost]) -> Vec<SearchPage> {
    rows.iter()
        .filter(|row| row.date.is_some())
        .filter_map(|row| match page(folder, config, row) {
            Ok(page) => Some(page),
            Err(why) => {
                tracing::warn!(path = %row.path, %why, "a post was left out of search");
                None
            }
        })
        .collect()
}

/// Why one post could not be read for search: its path, its file, its
/// front matter or a block in its body.
type Unindexed = Box<dyn std::error::Error + Send + Sync>;

/// One post's search page: its file read and rendered now.
fn page(folder: &Path, config: &Config, row: &IndexedPost) -> Result<SearchPage, Unindexed> {
    let file = SitePath::within(folder, &row.path)?;
    let text = fs::read_to_string(file.as_path())?;
    let post = parse_post(file.relative(), &text)?;
    // A locked post is found by its title alone, so no result shows its body.
    let markdown = match Lock::of(&post.front) {
        Some(_) => "",
        None => post.body.as_str(),
    };
    let rendered = render(markdown, &row.locale, config.slugs())?;
    let title = row.title.clone().unwrap_or_else(|| row.slug.to_string());
    let headings: Vec<&str> = rendered
        .headings
        .iter()
        .map(|heading| heading.text.as_str())
        .collect();
    let headings = headings.join("\n");
    let body = text_without_headings(&rendered.html);
    let analyzer = Analyzer::new(Language::of(&row.locale));
    let indexed = analyzer.index(&title, &headings, &body);
    let href = build_pages::path_of(config, &row.locale, &format!("{}/", row.slug));
    Ok(SearchPage::new(
        &row.path,
        row.locale.clone(),
        &href,
        &title,
        &body,
        indexed,
    ))
}

/// Marks where one block of a body ends, until [`joined`] reads it.
const BREAK: &str = " \u{1} ";

/// What stands between two blocks whose first does not end a sentence, so
/// a bookmark's title and its description never read as one sentence.
const SEPARATOR: &str = " · ";

/// The text of a body's `html` without its tags or its headings, which
/// search counts on their own, with the common entities read back, runs
/// of white space made one space and a [`SEPARATOR`] between blocks.
fn text_without_headings(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut rest = html;
    let mut in_heading = false;
    while let Some(open) = rest.find('<') {
        if !in_heading {
            text.push_str(rest.get(..open).unwrap_or_default());
        }
        let after = rest.get(open..).unwrap_or_default();
        let Some(close) = after.find('>') else {
            rest = "";
            break;
        };
        let tag = after.get(1..close).unwrap_or_default();
        let heading = is_heading(tag.trim_start_matches('/'));
        if heading {
            in_heading = !tag.starts_with('/');
        }
        text.push_str(if heading || ends_block(tag) {
            BREAK
        } else {
            " "
        });
        rest = after.get(close + 1..).unwrap_or_default();
    }
    if !in_heading {
        text.push_str(rest);
    }
    let text = text
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");
    joined(&text)
}

/// The words of `text` with one space between them, and a [`SEPARATOR`]
/// where a [`BREAK`] follows a word that ends no sentence.
fn joined(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut broken = false;
    for word in text.split_whitespace() {
        if word == BREAK.trim() {
            broken = true;
            continue;
        }
        if !out.is_empty() {
            let ended = out.ends_with(['.', '!', '?', ':', ';', '\u{2026}']);
            out.push_str(if broken && !ended { SEPARATOR } else { " " });
        }
        out.push_str(word);
        broken = false;
    }
    out
}

/// Whether the tag `tag`, its name and attributes, ends a block of text:
/// a paragraph, list item, cell, caption, quote or line break, or starts
/// the next line of a bookmark card.
fn ends_block(tag: &str) -> bool {
    const CLOSING: [&str; 10] = [
        "/p",
        "/li",
        "/dt",
        "/dd",
        "/td",
        "/th",
        "/figcaption",
        "/blockquote",
        "/pre",
        "/div",
    ];
    let name = tag.split([' ', '/']).find(|part| !part.is_empty());
    CLOSING.iter().any(|closing| tag.trim_end() == *closing)
        || name == Some("br")
        || tag.starts_with("span class=\"bookmark-description\"")
        || tag.starts_with("span class=\"bookmark-host\"")
}

/// Whether the tag `tag`, its name and attributes, opens a heading.
fn is_heading(tag: &str) -> bool {
    let mut name = tag.chars();
    matches!(
        (name.next(), name.next(), name.next()),
        (Some('h'), Some('1'..='6'), None | Some(' ' | '>'))
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use khalid_content::{ContentTree, read_content};
    use khalid_core::{Environment, Locale, Slug, SlugMode};
    use khalid_search::{Completion, Query};

    use super::*;

    #[test]
    fn a_body_keeps_its_text_and_leaves_its_headings_out() {
        let html = "<h2 id=\"why\">Why links</h2>\n<p>Links &amp; <a href=\"/x/\">anchors</a></p>\
                    <h3>More</h3><p>&lt;3</p>";
        assert_eq!(text_without_headings(html), "Links & anchors · <3");
    }

    #[test]
    fn a_bookmark_card_and_the_blocks_around_it_never_run_together() {
        let markdown = "Read this first.\n\n\
                        ::bookmark{href=\"https://developers.google.com/search/docs\" \
                        title=\"SEO Starter Guide\" description=\"Google's own introduction\"}\n\n\
                        ## Questions\n\n- One\n- Two\n";
        let english = Locale::try_new("en").unwrap();
        let html = render(markdown, &english, SlugMode::Ascii).unwrap().html;
        assert_eq!(
            text_without_headings(&html),
            "Read this first. SEO Starter Guide · Google's own introduction · \
             developers.google.com · One · Two"
        );
    }

    #[tokio::test]
    async fn every_published_demo_post_is_written_and_found_in_its_language() {
        let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        folder.push("../../fixtures/demo-site");
        let config = Config::load(&folder, &Environment::empty()).unwrap();
        let content = read_content(&ContentTree::walk(&folder).unwrap()).unwrap();
        let catalog = Catalog::of(&folder, &config, &content).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let mut file = dir.path().to_path_buf();
        file.push("site.db");
        let db = Database::open(&file).await.unwrap();
        db.migrate().await.unwrap();
        let site = db.site(&Slug::try_new("main").unwrap()).await.unwrap();

        write((&db, site), &folder, &config, &catalog, Which::All)
            .await
            .unwrap();

        let english = Locale::try_new("en").unwrap();
        let analyzer = Analyzer::new(Language::English);
        let query = Query::parse(&analyzer, "sitemaps", Completion::WholeWords).unwrap();
        let found = db.search(site, &english, &query, 10).await.unwrap();
        let first = found.hits.first().map(|hit| hit.href.as_str());
        assert_eq!(first, Some("/blog/what-a-sitemap-does/"), "{found:?}");
        let german = Locale::try_new("de").unwrap();
        let analyzer = Analyzer::new(Language::German);
        let query = Query::parse(&analyzer, "Sitemap", Completion::WholeWords).unwrap();
        let found = db.search(site, &german, &query, 10).await.unwrap();
        assert!(
            found
                .hits
                .iter()
                .all(|hit| hit.href.starts_with("/blog/de/")),
            "{found:?}"
        );
        db.close().await;
    }
}
