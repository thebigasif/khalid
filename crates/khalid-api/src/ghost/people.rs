//! Tags and authors in Ghost's shape, and what `include=tags,authors` adds
//! to a post.

use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::content_store::SiteResources;
use crate::resources::{Author, Listable, Post, Term};

/// A tag as Ghost writes it. Khalid's tags have a name and a slug in each
/// language; the rest carries Ghost's neutral values.
#[derive(Debug, Serialize)]
pub(crate) struct GhostTag {
    id: String,
    name: String,
    slug: String,
    description: Option<String>,
    feature_image: Option<String>,
    visibility: &'static str,
    meta_title: Option<String>,
    meta_description: Option<String>,
    og_image: Option<String>,
    twitter_image: Option<String>,
    codeinjection_head: Option<String>,
    codeinjection_foot: Option<String>,
    canonical_url: Option<String>,
    accent_color: Option<String>,
    url: Option<String>,
}

impl GhostTag {
    /// `term` in Ghost's shape.
    pub(crate) fn new(term: &Term) -> Self {
        Self {
            id: term.id().to_owned(),
            name: term.name().to_owned(),
            slug: term.slug().to_string(),
            description: None,
            feature_image: None,
            visibility: "public",
            meta_title: None,
            meta_description: None,
            og_image: None,
            twitter_image: None,
            codeinjection_head: None,
            codeinjection_foot: None,
            canonical_url: None,
            accent_color: None,
            url: None,
        }
    }
}

/// An author as Ghost writes it: Khalid's name, introduction and page,
/// with Ghost's neutral values for the rest.
#[derive(Debug, Serialize)]
pub(crate) struct GhostAuthor {
    id: String,
    name: String,
    slug: String,
    profile_image: Option<String>,
    cover_image: Option<String>,
    bio: Option<String>,
    website: Option<String>,
    location: Option<String>,
    facebook: Option<String>,
    twitter: Option<String>,
    meta_title: Option<String>,
    meta_description: Option<String>,
    url: Option<String>,
}

impl GhostAuthor {
    /// `author` in Ghost's shape.
    pub(crate) fn new(author: &Author) -> Self {
        Self {
            id: author.id().to_owned(),
            name: author.name().to_owned(),
            slug: author.slug().to_string(),
            profile_image: None,
            cover_image: None,
            bio: author.bio().map(str::to_owned),
            website: None,
            location: None,
            facebook: None,
            twitter: None,
            meta_title: None,
            meta_description: None,
            url: author.url().map(ToString::to_string),
        }
    }
}

/// What `include` asks to add to each post.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Include {
    pub(crate) tags: bool,
    pub(crate) authors: bool,
}

impl Include {
    /// Whether anything is asked for.
    pub(crate) fn any(self) -> bool {
        self.tags || self.authors
    }
}

/// Adds to `entry`, the post `post` in Ghost's shape, the tags and authors
/// `include` asks for, each found in `site` in the post's language, with
/// the first of each as `primary_tag` and `primary_author`.
pub(crate) fn add_included(
    entry: &mut Map<String, Value>,
    post: &Post,
    site: &SiteResources,
    include: Include,
) {
    let language = post.language();
    if include.tags {
        let tags: Vec<GhostTag> = post
            .tags()
            .iter()
            .filter_map(|slug| {
                site.tags()
                    .iter()
                    .find(|tag| tag.slug().as_str() == slug && tag.language() == language)
            })
            .map(GhostTag::new)
            .collect();
        entry.insert("primary_tag".to_owned(), json!(tags.first()));
        entry.insert("tags".to_owned(), json!(tags));
    }
    if include.authors {
        let authors: Vec<GhostAuthor> = post
            .authors()
            .iter()
            .filter_map(|slug| {
                site.authors()
                    .iter()
                    .find(|author| author.slug().as_str() == slug && author.language() == language)
            })
            .map(GhostAuthor::new)
            .collect();
        entry.insert("primary_author".to_owned(), json!(authors.first()));
        entry.insert("authors".to_owned(), json!(authors));
    }
}
