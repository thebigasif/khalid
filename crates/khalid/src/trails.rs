//! The breadcrumb trail of each page: the home page, a post's category,
//! then the page. One trail feeds both the page's `BreadcrumbList` and the
//! breadcrumbs a template shows (templates spec 4.2). The same pass gives
//! the home page its list of categories, for a template's category tabs.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use khalid_core::{BreadcrumbSettings, BreadcrumbTaxonomy, Config, Locale, UserMessage};
use khalid_db::IndexedTerm;
use khalid_render::{Link, PageBody};
use khalid_routes::Routes;
use khalid_seo::{Crumb, PageKind, SeoPage, UrlPath};

use crate::build_pages::{LanguagePages, path_of};
use crate::{build_message, categories};

/// Gives each page of `languages` but the home page and the not-found page
/// its trail: the home page, the category of a post that has one, named as
/// `categories.toml` in the `site` folder names it, then the page itself.
/// Each home page gets a link to every category its language's posts use.
///
/// # Errors
///
/// The message for a `categories.toml` that cannot be read, or a category
/// address that cannot be made from the site's settings.
pub(crate) fn fill(
    site: &Path,
    (config, routes): (&Config, &Routes),
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    let categories = categories::of(site)?;
    let settings = &config.seo().breadcrumbs;
    for language in languages {
        let locale = &language.site.locale;
        let home = Link {
            name: (settings.home.clone()).unwrap_or_else(|| config.name().to_owned()),
            href: language.site.home.clone(),
        };
        let category_of: BTreeMap<&str, &str> = (language.listed.iter())
            .filter_map(|post| Some((post.card.href.as_str(), post.topics.category.as_deref()?)))
            .collect();
        let tag_of: BTreeMap<&str, (&str, &str)> = (language.listed.iter())
            .filter_map(|post| {
                let tag = post.tags.first()?;
                Some((
                    post.card.href.as_str(),
                    (tag.slug.as_str(), tag.name.as_str()),
                ))
            })
            .collect();
        let tabs = tabs(
            (config, routes),
            locale,
            &categories,
            category_of.values().copied().collect(),
        );
        for page in &mut language.pages {
            if let PageBody::Home(front) = &mut page.body {
                front.categories.clone_from(&tabs);
            }
            if page.meta.path == home.href || matches!(page.body, PageBody::NotFound) {
                continue;
            }
            let mut trail = vec![home.clone()];
            let is_post = matches!(page.body, PageBody::Post(_));
            let here = page.meta.path.as_str();
            let term = match settings.taxonomy {
                _ if !is_post => None,
                BreadcrumbTaxonomy::Category => category_of.get(here).map(|&slug| {
                    let name = (categories.iter())
                        .find(|term| &term.locale == locale && term.slug.as_str() == slug)
                        .map_or(slug, |term| term.name.as_str());
                    (format!("{}/{slug}/", routes.category_prefix()), name)
                }),
                BreadcrumbTaxonomy::Tag => (tag_of.get(here))
                    .map(|&(slug, name)| (format!("{}/{slug}/", routes.tag_prefix()), name)),
                BreadcrumbTaxonomy::None => None,
            };
            if let Some((rest, name)) = term {
                let path = UrlPath::try_new(&format!("/{rest}"))
                    .map_err(|error| build_message::seo(&error))?;
                page.seo.breadcrumbs = vec![Crumb::new(name, path)];
                trail.push(Link {
                    name: name.to_owned(),
                    href: path_of(config, locale, &rest),
                });
            }
            trail.push(Link {
                name: last_name(settings, &page.seo),
                href: page.meta.path.clone(),
            });
            page.meta.breadcrumbs = trail;
        }
    }
    Ok(())
}

/// The trail's last step for `page`: its breadcrumb title or its title,
/// after the archive or search prefix `settings` give its kind.
fn last_name(settings: &BreadcrumbSettings, page: &SeoPage) -> String {
    let name = page.breadcrumb_title.as_ref().unwrap_or(&page.title);
    let prefix = match page.kind {
        PageKind::Tag | PageKind::Category | PageKind::Author | PageKind::Date => {
            &settings.archive_prefix
        }
        PageKind::Search => &settings.search_prefix,
        _ => return name.clone(),
    };
    if prefix.is_empty() {
        name.clone()
    } else {
        format!("{prefix} {name}")
    }
}

/// A link to each category of `used` (the slugs posts name), in the order
/// `categories.toml` lists them and named as it names them in `locale`,
/// then any category it does not name, by slug, for the home page's tabs.
fn tabs(
    (config, routes): (&Config, &Routes),
    locale: &Locale,
    categories: &[IndexedTerm],
    mut used: BTreeSet<&str>,
) -> Vec<Link> {
    let named = (categories.iter())
        .filter(|term| &term.locale == locale)
        .filter_map(|term| {
            used.take(term.slug.as_str())
                .map(|_| (term.slug.as_str(), term.name.as_str()))
        });
    let named: Vec<(&str, &str)> = named.collect();
    let rest = used.into_iter().map(|slug| (slug, slug));
    named
        .into_iter()
        .chain(rest)
        .map(|(slug, name)| Link {
            name: name.to_owned(),
            href: path_of(
                config,
                locale,
                &format!("{}/{slug}/", routes.category_prefix()),
            ),
        })
        .collect()
}
