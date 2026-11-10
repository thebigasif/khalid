//! A draft's preview page, rendered among the site's posts as they are now.

use std::fs;
use std::path::PathBuf;

use khalid_content::{ContentTree, parse_post, read_content};
use khalid_core::{Locale, Slug, UserMessage};

use super::SitePages;
use crate::post_files::Assets;
use crate::post_photos::Shown;
use crate::site_html::Which;
use crate::{build_pages, content_message, serve_message};

impl SitePages {
    /// The page the draft of post `slug` in `locale` would have if it went
    /// live today, rendered among the site's posts as they are now, for
    /// its preview link. `None` when there is no such draft, the site has
    /// no theme, or it has no post to shape the draft's entry on. Reads
    /// files and renders, so it runs off the async runtime (BL-RUN-7).
    pub(crate) fn preview(
        &self,
        locale: &Locale,
        slug: &Slug,
    ) -> Result<Option<String>, UserMessage> {
        let (folder, settings) = (&self.0.folder, self.settings());
        let mut file = folder.clone();
        file.push(format!("drafts/{locale}/{slug}.md"));
        let (Some(_), Ok(text)) = (&settings.theme, fs::read_to_string(&file)) else {
            return Ok(None);
        };
        let unreadable = |error| content_message::of(folder, &error);
        let tree = ContentTree::walk(folder).map_err(unreadable)?;
        let mut content = read_content(&tree).map_err(unreadable)?;
        let today = self.0.clock.now().date_naive();
        let path = PathBuf::from(format!("content/{locale}/posts/{today}-{slug}.md"));
        let post = parse_post(&path, &text).map_err(unreadable)?;
        // A Document is made only by khalid-content, so the draft's entry
        // is another post's with every field that names a file replaced.
        let Some(mut doc) = content.posts.first().cloned() else {
            return Ok(None);
        };
        doc.locale = locale.clone();
        "posts".clone_into(&mut doc.collection);
        doc.path = path;
        doc.front = post.front;
        doc.front.date.get_or_insert(today);
        let front_lines = text
            .lines()
            .count()
            .saturating_sub(post.body.lines().count());
        doc.body_line = front_lines.saturating_add(1);
        doc.body = post.body;
        content
            .posts
            .retain(|other| &other.locale != locale || other.front.slug.as_ref() != Some(slug));
        content.posts.push(doc);
        let assets = Assets::of(&tree);
        let mut shown = Shown::default();
        let which = Which::Posts;
        let shown = (which, &mut shown);
        let (served, _) = self.render_of(&settings, &assets, (&content, &[]), shown)?;
        let address = build_pages::path_of(&settings.config, locale, &format!("{slug}/"));
        let page = served
            .pages
            .iter()
            .rev()
            .find(|page| page.path().as_str() == address);
        let Some(page) = page else {
            return Ok(None);
        };
        let body = page.body().map_err(|_| serve_message::internal())?;
        Ok(Some(String::from_utf8_lossy(&body).into_owned()))
    }
}
