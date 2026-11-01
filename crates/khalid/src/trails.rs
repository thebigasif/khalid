//! The breadcrumb trail of each page: the home page, a post's category,
//! then the page. One trail feeds both the page's `BreadcrumbList` and the
//! breadcrumbs a template shows (templates spec 4.2).

use std::collections::BTreeMap;
use std::path::Path;

use khalid_core::{Config, UserMessage};
use khalid_render::{Link, PageBody};
use khalid_routes::Routes;
use khalid_seo::{Crumb, UrlPath};

use crate::build_pages::{LanguagePages, path_of};
use crate::{build_message, categories};

/// Gives each page of `languages` but the home page and the not-found page
/// its trail: the home page, the category of a post that has one, named as
/// `categories.toml` in the `site` folder names it, then the page itself.
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
    for language in languages {
        let locale = &language.site.locale;
        let home = Link {
            name: config.name().to_owned(),
            href: language.site.home.clone(),
        };
        let category_of: BTreeMap<&str, &str> = (language.listed.iter())
            .filter_map(|post| Some((post.card.href.as_str(), post.topics.category.as_deref()?)))
            .collect();
        for page in &mut language.pages {
            if page.meta.path == home.href || matches!(page.body, PageBody::NotFound) {
                continue;
            }
            let mut trail = vec![home.clone()];
            let category = (matches!(page.body, PageBody::Post(_)))
                .then(|| category_of.get(page.meta.path.as_str()))
                .flatten();
            if let Some(&slug) = category {
                let rest = format!("{}/{slug}/", routes.category_prefix());
                let name = (categories.iter())
                    .find(|term| &term.locale == locale && term.slug.as_str() == slug)
                    .map_or(slug, |term| term.name.as_str());
                let path = UrlPath::try_new(&format!("/{rest}"))
                    .map_err(|error| build_message::seo(&error))?;
                page.seo.breadcrumbs = vec![Crumb::new(name, path)];
                trail.push(Link {
                    name: name.to_owned(),
                    href: path_of(config, locale, &rest),
                });
            }
            trail.push(Link {
                name: page.seo.title.clone(),
                href: page.meta.path.clone(),
            });
            page.meta.breadcrumbs = trail;
        }
    }
    Ok(())
}
