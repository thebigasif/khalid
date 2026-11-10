//! The Content API's site lists on a running site: authors, tags and
//! categories from the content index, collections from the content, and
//! settings and menus from `site.toml`.

use std::collections::BTreeMap;

use khalid_api::content_store::SiteResources;
use khalid_api::resources::{Author, ContentCollection, Menu, MenuLink, Settings, Term};
use khalid_content::SiteContent;
use khalid_core::{AbsoluteUrl, Config, Locale, Slug, UserMessage};
use khalid_db::{ContentIndex, IndexedTerm};

use crate::{build_message, build_pages, index, site_chrome};

/// The menus every page has: the header menu, then the footer menu. A menu
/// with no links shows nothing.
const MENUS: [&str; 2] = ["primary", "footer"];

/// The site's lists from its `index`, the rows a rebuild writes to
/// `data/site.db`, with each author's role, introduction and profiles from
/// `content`.
///
/// # Errors
///
/// Returns what, why and fix when an address cannot be made from the site's
/// settings.
pub(crate) fn of(
    config: &Config,
    index: &ContentIndex,
    content: &SiteContent,
) -> Result<SiteResources, UserMessage> {
    let docs: BTreeMap<String, _> = content
        .authors
        .iter()
        .filter_map(|doc| Some((index::slashed(&doc.path)?, doc)))
        .collect();
    let mut authors = Vec::new();
    for row in &index.authors {
        let url = address(config, &row.locale, &format!("authors/{}/", row.slug))?;
        let mut author = Author::new(row.slug.clone(), &row.name, row.locale.clone()).with_url(url);
        if let Some(doc) = docs.get(&row.path) {
            if let Some(role) = &doc.front.role {
                author = author.with_role(role);
            }
            let bio = doc.body.trim();
            if !bio.is_empty() {
                author = author.with_bio(bio);
            }
            author = author.with_same_as(doc.front.same_as.clone());
        }
        authors.push(author);
    }
    let terms = |rows: &[IndexedTerm]| -> Vec<Term> {
        let term = |row: &IndexedTerm| Term::new(row.slug.clone(), &row.name, row.locale.clone());
        rows.iter().map(term).collect()
    };
    let tags = terms(&index.tags);
    let categories = terms(&index.categories);
    let home = address(config, config.default_language(), "")?;
    let settings = Settings::new(
        config.name(),
        home,
        config.default_language().clone(),
        config.languages().to_vec(),
    );
    Ok(SiteResources::default()
        .with_authors(authors)
        .with_tags(tags)
        .with_categories(categories)
        .with_collections(collections(config, content))
        .with_menus(menus(config))
        .with_settings(settings))
}

/// Every collection folder of each language that holds entries, posts
/// first, then pages, glossary, case studies and changelog, with the title
/// and description of its `_index.md`.
fn collections(config: &Config, content: &SiteContent) -> Vec<ContentCollection> {
    let folders = content
        .posts
        .iter()
        .map(|doc| (&doc.locale, &doc.collection))
        .chain(
            content
                .pages
                .iter()
                .map(|doc| (&doc.locale, &doc.collection)),
        )
        .chain(
            content
                .glossary
                .iter()
                .map(|doc| (&doc.locale, &doc.collection)),
        )
        .chain(
            content
                .case_studies
                .iter()
                .map(|doc| (&doc.locale, &doc.collection)),
        )
        .chain(
            content
                .changelog
                .iter()
                .map(|doc| (&doc.locale, &doc.collection)),
        );
    let folders: Vec<(&Locale, &String)> = folders.collect();
    let mut found = Vec::new();
    for locale in config.languages() {
        let mut seen: Vec<&str> = Vec::new();
        for (_, folder) in folders.iter().filter(|(of, _)| *of == locale) {
            let Ok(slug) = Slug::try_new(folder) else {
                continue;
            };
            if seen.contains(&folder.as_str()) {
                continue;
            }
            seen.push(folder);
            let mut collection = ContentCollection::new(slug, locale.clone());
            let intro = content
                .indexes
                .iter()
                .find(|doc| &doc.locale == locale && &doc.collection == *folder);
            if let Some(front) = intro.map(|doc| &doc.front) {
                if let Some(title) = &front.title {
                    collection = collection.with_title(title);
                }
                if let Some(description) = &front.description {
                    collection = collection.with_description(description);
                }
            }
            found.push(collection);
        }
    }
    found
}

/// The header and footer menu of each language, with the links its pages
/// show: those of `[[nav.primary]]` and `[[nav.footer]]` for every language
/// and for that one, and the AI info page at the end of the footer when it
/// is on.
fn menus(config: &Config) -> Vec<Menu> {
    let nav = config.nav();
    let mut menus = Vec::new();
    for locale in config.languages() {
        let primary = site_chrome::menu(&nav.primary, locale);
        let footer = site_chrome::footer(config, &nav.footer, locale);
        for (name, links) in MENUS.into_iter().zip([primary, footer]) {
            let Ok(slug) = Slug::try_new(name) else {
                continue;
            };
            let links = links
                .iter()
                .map(|link| MenuLink::new(&link.label, &link.href))
                .collect();
            menus.push(Menu::new(slug, locale.clone()).with_links(links));
        }
    }
    menus
}

/// The full address of `rest` in `locale`, such as the German author page.
fn address(config: &Config, locale: &Locale, rest: &str) -> Result<AbsoluteUrl, UserMessage> {
    let path = build_pages::path_of(config, locale, rest);
    let address = format!("{}{path}", config.address().as_str().trim_end_matches('/'));
    AbsoluteUrl::parse(&address).map_err(|_| build_message::address(&address))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use khalid_content::{ContentTree, read_content};
    use khalid_core::Environment;

    use super::*;

    #[test]
    fn the_lists_come_from_the_index_and_the_content_without_the_database() {
        let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        demo.push("../../fixtures/demo-site");
        let config = Config::load(&demo, &Environment::empty()).unwrap();
        let content = read_content(&ContentTree::walk(&demo).unwrap()).unwrap();
        let indexed = index::of_content(&demo, &config, &content).unwrap();
        let lists = of(&config, &indexed, &content).unwrap();

        let authors = serde_json::to_value(lists.authors()).unwrap();
        assert_eq!(authors.as_array().unwrap().len(), 4, "{authors}");
        assert_eq!(authors[2]["id"], "en.asif", "{authors}");
        assert_eq!(authors[2]["role"], "Founder of SerpDad", "{authors}");
        assert!(
            lists.categories().len() > 1,
            "{} categories",
            lists.categories().len()
        );
        assert!(
            lists.collections().len() > 1,
            "{} collections",
            lists.collections().len()
        );
    }

    #[test]
    fn each_menu_holds_the_links_its_language_shows_in_the_order_written() {
        let text = "name = \"a\"\naddress = \"https://www.example.com/blog/\"\n\
                    default_language = \"en\"\nlanguages = [\"en\", \"de\"]\n\
                    [[nav.primary]]\nlabel = \"Guides\"\nhref = \"/blog/guides/\"\n\
                    [[nav.primary]]\nlabel = \"Anleitungen\"\nhref = \"/blog/de/guides/\"\n\
                    language = \"de\"\n\
                    [[nav.footer]]\nlabel = \"Pricing\"\nhref = \"https://www.example.com/pricing/\"\n";
        let config = Config::from_toml(
            std::path::Path::new("site.toml"),
            text,
            &Environment::empty(),
        )
        .unwrap();
        let menus = serde_json::to_value(menus(&config)).unwrap();
        let links = |at: usize| menus[at]["links"].clone();

        assert_eq!(menus[0]["id"], "en.primary", "{menus}");
        assert_eq!(
            links(0),
            serde_json::json!([{ "label": "Guides", "url": "/blog/guides/" }])
        );
        assert_eq!(menus[2]["id"], "de.primary", "{menus}");
        assert_eq!(links(2)[1]["label"], "Anleitungen", "{menus}");
        let footer = serde_json::json!([
            { "label": "Pricing", "url": "https://www.example.com/pricing/" },
            { "label": "AI info", "url": "/ai/" },
        ]);
        assert_eq!(links(1), footer, "the footer ends with the AI info page");
        assert_eq!(links(3), footer, "every language links the one AI page");
    }
}
