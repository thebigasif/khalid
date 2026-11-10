//! The check behind the editor's SEO panel (G54): a post as it would go
//! live now, with the publish gate's findings and score, the search
//! result its head makes, and the schema graph it carries. Nothing is
//! written.

use std::path::PathBuf;

use khalid_api::content_store::BoxFuture;
use khalid_api::link_map::LinkMap;
use khalid_api::link_suggestions::{LinkSuggestion, Topics, suggest};
use khalid_api::post_check::{PostCheck, PostChecks, Snippet};
use khalid_api::post_store::PostStoreError;
use khalid_api::resources::PostStatus;
use khalid_api::site_issues::{SiteIssue, top_issues};
use khalid_content::parse_post;
use khalid_core::Slug;
use khalid_seo::{HeadTag, SeoPage, SeoSite, head_tags, page_graph};
use serde_json::Value;

use super::{Found, Site, SitePosts, blocking, failure};
use crate::lint_input::{PostParts, lint_site, seo_page};
use crate::post_text::PostText;

impl Site {
    /// `found`'s text as it would go live now, and its path in the site
    /// folder: a draft dated today in `content/`, a published post as it is.
    pub(super) fn as_now(&self, found: &Found) -> Result<(PostText, PathBuf), PostStoreError> {
        let mut text = found.text.clone();
        let relative = match found.status {
            PostStatus::Draft => self.dated(found, &mut text).1,
            _ => found
                .path
                .strip_prefix(&self.folder)
                .map_err(PostStoreError::new)?
                .to_path_buf(),
        };
        Ok((text, relative))
    }

    /// The check of post `id`, or `None` when no post has that identity.
    fn check_now(&self, id: &str) -> Result<Option<PostCheck>, PostStoreError> {
        let Some(found) = self.find(id)? else {
            return Ok(None);
        };
        let judge = self.judge(&self.content()?)?;
        let report = self.report(&judge, &found)?;
        let words = self.candidate_now(&judge, &found)?.word_count;
        let (text, relative) = self.as_now(&found)?;
        let post = parse_post(&relative, &text.to_text()).map_err(PostStoreError::new)?;
        let collection = relative.iter().nth(2).and_then(|part| part.to_str());
        let parts = PostParts {
            locale: &found.locale,
            path: &relative,
            collection: collection.unwrap_or("posts"),
            front: &post.front,
            body: &post.body,
        };
        let (site, page) = seo_page(&parts, &judge.config, &judge.input)
            .ok_or_else(|| failure("the post's page cannot be made for its search preview"))?;
        let graph = page_graph(site, &page).map_err(PostStoreError::new)?;
        let schema: Value = serde_json::from_str(&graph.to_json()).map_err(PostStoreError::new)?;
        let check = PostCheck::new(id, report, snippet(site, &page)?, schema);
        Ok(Some(check.with_words(words)))
    }
}

impl Site {
    /// The posts post `id` should link to, or `None` when no post has that
    /// identity. The site's posts are read with the document each came
    /// from, for their categories and tags.
    fn suggestions_now(&self, id: &str) -> Result<Option<Vec<LinkSuggestion>>, PostStoreError> {
        let Some(found) = self.find(id)? else {
            return Ok(None);
        };
        let content = self.content()?;
        let judge = self.judge(&content)?;
        let (site, documents) = lint_site(&content, &judge.config, &judge.input);
        let topics: Vec<Topics<'_>> = (documents.iter())
            .map(|doc| {
                Topics::new(
                    doc.front.category.as_ref().map(Slug::as_str),
                    &doc.front.tags,
                )
            })
            .collect();
        let (text, relative) = self.as_now(&found)?;
        let front = parse_post(&relative, &text.to_text())
            .map_err(PostStoreError::new)?
            .front;
        let post = Self::candidate(&judge, &found, &text, &relative)?;
        let own = Topics::new(front.category.as_ref().map(Slug::as_str), &front.tags);
        Ok(Some(suggest(
            &post,
            own,
            &LinkMap::new(&site.posts),
            &topics,
        )))
    }
}

/// The search result `page`'s head makes: its title, canonical address
/// and description.
fn snippet(site: &SeoSite, page: &SeoPage) -> Result<Snippet, PostStoreError> {
    let mut snippet = Snippet {
        title: String::new(),
        url: String::new(),
        description: String::new(),
    };
    let tags = head_tags(site, page).map_err(PostStoreError::new)?;
    for tag in tags.as_slice() {
        match tag {
            HeadTag::Title(title) => snippet.title.clone_from(title),
            HeadTag::Meta {
                name: "description",
                content,
            } => snippet.description.clone_from(content),
            HeadTag::Link {
                rel: "canonical",
                href,
            } => href.as_str().clone_into(&mut snippet.url),
            _ => {}
        }
    }
    Ok(snippet)
}

impl PostChecks for SitePosts {
    fn check<'a>(
        &'a self,
        id: &'a str,
    ) -> BoxFuture<'a, Result<Option<PostCheck>, PostStoreError>> {
        let id = id.to_owned();
        Box::pin(blocking(&self.0, move |site| site.check_now(&id)))
    }

    fn link_suggestions<'a>(
        &'a self,
        id: &'a str,
    ) -> BoxFuture<'a, Result<Option<Vec<LinkSuggestion>>, PostStoreError>> {
        let id = id.to_owned();
        Box::pin(blocking(&self.0, move |site| site.suggestions_now(&id)))
    }

    fn site_issues(&self) -> BoxFuture<'_, Result<Vec<SiteIssue>, PostStoreError>> {
        Box::pin(blocking(&self.0, |site| {
            let judge = site.judge(&site.content()?)?;
            Ok(top_issues(&judge.linter.lint_site(&judge.site)))
        }))
    }
}
