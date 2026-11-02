//! The posts a save touched, read again for [`SitePages`] to render alone.

use std::path::PathBuf;
use std::sync::PoisonError;

use khalid_content::SiteContent;

use super::SitePages;
use crate::listed_post::ListedPost;
use crate::site_outline;

/// How a wikilink starts in markdown, such as `[[link-equity]]`.
const WIKILINK: &str = "[[";

impl SitePages {
    /// The saved `files`, relative to the site folder, and their other
    /// language versions, read again as posts, with every author their
    /// pages may name, and the dated posts of the last full render, which
    /// their related posts are chosen from. `None` when a saved file is
    /// not a post the last full render read, or one cannot be read now:
    /// the full render then shows it, or says what is wrong.
    ///
    /// Also `None` when a post or an author's bio has a wikilink, which may
    /// name a page outside these, so the page could differ from the full
    /// render's.
    pub(super) fn saved_posts(&self, files: &[PathBuf]) -> Option<(SiteContent, Vec<ListedPost>)> {
        let outline = self
            .0
            .outline
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let posts = &outline.content.posts;
        let mut saved = SiteContent::default();
        for file in files {
            let mut doc = posts.iter().find(|doc| &doc.path == file)?.clone();
            site_outline::read_again(&self.0.folder, &mut doc)?;
            saved.posts.push(doc);
        }
        // A page's head links its other language versions, so they are
        // rendered with it.
        let keys: Vec<&String> = saved
            .posts
            .iter()
            .filter_map(|doc| doc.front.translation.as_ref())
            .collect();
        let versions = posts.iter().filter(|doc| {
            let key = doc.front.translation.as_ref();
            key.is_some_and(|key| keys.contains(&key)) && !files.contains(&doc.path)
        });
        let mut versions: Vec<_> = versions.cloned().collect();
        for doc in &mut versions {
            site_outline::read_again(&self.0.folder, doc)?;
        }
        saved.posts.extend(versions);
        saved.authors.clone_from(&outline.content.authors);
        let bodies = saved.posts.iter().map(|doc| &doc.body);
        let mut bodies = bodies.chain(saved.authors.iter().map(|doc| &doc.body));
        if bodies.any(|body| body.contains(WIKILINK)) {
            return None;
        }
        Some((saved, outline.listed.clone()))
    }
}
