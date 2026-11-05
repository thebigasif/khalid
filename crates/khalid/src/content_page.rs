//! The plain pages, such as About, as the Content API lists them on a
//! running site: each at the address `khalid build` gives its page.

use std::fs;
use std::path::Path;

use chrono::{DateTime, SubsecRound, Utc};
use khalid_api::resources::Post;
use khalid_content::SiteContent;
use khalid_core::{AbsoluteUrl, Config, Locale, SitePath, UserMessage};

use crate::build_pages::{named, path_of};
use crate::{build_message, index};

/// One plain page, in API shape, and the markdown its body is rendered
/// from. Pages are few, so the store keeps their markdown, where a post's
/// body is read from its file when asked for.
#[derive(Debug, Clone)]
pub(crate) struct ContentPage {
    /// The page without its body.
    pub(crate) post: Post,
    /// The language it is written in.
    pub(crate) locale: Locale,
    /// The markdown after the front matter, exactly as written.
    pub(crate) markdown: String,
}

impl ContentPage {
    /// Every page of `content`, read from the site `folder`, in each
    /// language of `config`, in path order. A page has no date, so it went
    /// live, and last changed, when its file was last saved, as the Admin
    /// API says. A page whose file is gone since it was read is left out.
    ///
    /// # Errors
    ///
    /// Returns what, why and fix when a page has no slug or an address
    /// cannot be made from the site's settings.
    pub(crate) fn all(
        folder: &Path,
        config: &Config,
        content: &SiteContent,
    ) -> Result<Vec<Self>, UserMessage> {
        let mut pages = Vec::new();
        for locale in config.languages() {
            for doc in content.pages.iter().filter(|doc| &doc.locale == locale) {
                let file =
                    index::slashed(&doc.path).and_then(|path| SitePath::within(folder, &path).ok());
                let Some(saved) = file.and_then(|file| saved_at(file.as_path())) else {
                    continue;
                };
                let front = &doc.front;
                let (title, slug) =
                    named(config, doc, front.slug.as_ref(), front.title.as_deref())?;
                let path = path_of(config, locale, &format!("{slug}/"));
                let address = format!("{}{path}", config.address().as_str().trim_end_matches('/'));
                let url =
                    AbsoluteUrl::parse(&address).map_err(|_| build_message::address(&address))?;
                let id = format!("{}.{slug}", locale.as_str());
                let mut post = Post::new(&id, slug, &title, url, locale.clone(), saved);
                if let Some(excerpt) = &front.description {
                    post = post.with_excerpt(excerpt);
                }
                pages.push(Self {
                    post,
                    locale: locale.clone(),
                    markdown: doc.body.clone(),
                });
            }
        }
        Ok(pages)
    }
}

/// When the file at `path` was last saved, to the second as a post's
/// times are, or `None` when it cannot be read.
fn saved_at(path: &Path) -> Option<DateTime<Utc>> {
    let modified = fs::metadata(path).and_then(|meta| meta.modified()).ok()?;
    Some(DateTime::<Utc>::from(modified).trunc_subsecs(0))
}
