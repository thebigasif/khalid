//! A Khalid post or page in Ghost's post shape.

use std::fmt::Write as _;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::query::Format;
use crate::resources::{HeadElement, Post, PostHead};

/// Words a minute, as Ghost counts reading time.
const WORDS_A_MINUTE: usize = 275;

/// The longest excerpt Ghost makes from the text, in characters.
const EXCERPT_CHARACTERS: usize = 500;

/// A post as Ghost's Content API writes it. Fields Khalid has no meaning
/// for carry Ghost's neutral values in [`Neutral`].
#[derive(Debug, Serialize)]
pub(crate) struct GhostPost {
    id: String,
    uuid: String,
    title: String,
    slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plaintext: Option<String>,
    comment_id: String,
    feature_image: Option<String>,
    featured: bool,
    created_at: String,
    updated_at: String,
    published_at: String,
    custom_excerpt: Option<String>,
    url: String,
    excerpt: String,
    reading_time: usize,
    meta_title: Option<String>,
    meta_description: Option<String>,
    #[serde(flatten)]
    neutral: Neutral,
}

/// Ghost's fields with no meaning in Khalid, or with one value: no code
/// injection, members' email or custom template. `visibility` and `access`
/// say whether the post is open to everyone or for members only.
#[derive(Debug, Default, Serialize)]
struct Neutral {
    visibility: &'static str,
    access: bool,
    comments: bool,
    feature_image_alt: Option<String>,
    feature_image_caption: Option<String>,
    codeinjection_head: Option<String>,
    codeinjection_foot: Option<String>,
    custom_template: Option<String>,
    canonical_url: Option<String>,
    og_image: Option<String>,
    og_title: Option<String>,
    og_description: Option<String>,
    twitter_image: Option<String>,
    twitter_title: Option<String>,
    twitter_description: Option<String>,
    email_subject: Option<String>,
    frontmatter: Option<String>,
}

impl GhostPost {
    /// `post` in Ghost's shape, with the bodies `formats` asks for.
    pub(crate) fn new(post: &Post, formats: &[Format]) -> Self {
        let (published, updated) = post.times();
        let (html, text) = post.html_and_text();
        // Khalid hands no body for a members-only post. Ghost answers a key
        // with no member the same way: members visibility, no access and
        // empty bodies.
        let for_members = html.is_none();
        let html = html.or(for_members.then_some(""));
        let text = text.unwrap_or_default();
        let head = Head::of(post);
        let excerpt = match post.excerpt() {
            Some(excerpt) => excerpt.to_owned(),
            None => text.chars().take(EXCERPT_CHARACTERS).collect(),
        };
        let asked = |format, body: Option<&str>| {
            body.filter(|_| formats.contains(&format))
                .map(str::to_owned)
        };
        Self {
            id: post.id().to_owned(),
            uuid: uuid_of(post.id()),
            title: post.title().to_owned(),
            slug: post.slug().to_string(),
            html: asked(Format::Html, html),
            plaintext: asked(Format::Plaintext, Some(text)),
            comment_id: post.id().to_owned(),
            feature_image: head.image,
            featured: post.is_featured(),
            created_at: stamp(published),
            updated_at: stamp(updated),
            published_at: stamp(published),
            custom_excerpt: post.excerpt().map(str::to_owned),
            url: post.url().to_string(),
            excerpt,
            reading_time: reading_time(text),
            meta_title: head.title,
            meta_description: head.description,
            neutral: Neutral {
                visibility: if for_members { "members" } else { "public" },
                access: !for_members,
                ..Neutral::default()
            },
        }
    }
}

/// What a post's head says, when its page has been made.
#[derive(Default)]
struct Head {
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
}

impl Head {
    fn of(post: &Post) -> Self {
        let mut found = Self::default();
        let Some(parts) = post.head().and_then(PostHead::parts) else {
            return found;
        };
        for element in parts.tags() {
            match element {
                HeadElement::Title { text } => found.title = Some(text.clone()),
                HeadElement::Meta {
                    name,
                    property,
                    content,
                } => match (name.as_deref(), property.as_deref()) {
                    (Some("description"), _) => found.description = Some(content.clone()),
                    (_, Some("og:image")) => found.image = Some(content.clone()),
                    _ => {}
                },
                _ => {}
            }
        }
        found
    }
}

/// `at` as Ghost writes times: UTC with milliseconds.
fn stamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// Minutes to read `text` as Ghost counts them: words over 275, rounded,
/// and at least 1 for any text at all.
fn reading_time(text: &str) -> usize {
    let words = text.split_whitespace().count();
    if words == 0 {
        return 0;
    }
    let rounded = (words * 2 + WORDS_A_MINUTE) / (WORDS_A_MINUTE * 2);
    rounded.max(1)
}

/// A UUID made from `id`, the same every time, in the shape Ghost's
/// `uuid` has, so a front end keying on it keeps a stable key.
fn uuid_of(id: &str) -> String {
    let digest = Sha256::digest(id.as_bytes());
    let mut hex = String::with_capacity(32);
    for byte in digest.iter().take(16) {
        let _ = write!(hex, "{byte:02x}");
    }
    let part = |range: std::ops::Range<usize>| hex.get(range).unwrap_or_default().to_owned();
    format!(
        "{}-{}-{}-{}-{}",
        part(0..8),
        part(8..12),
        part(12..16),
        part(16..20),
        part(20..32)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_time_rounds_as_ghost_does() {
        let words = |count: usize| vec!["word"; count].join(" ");
        let minutes: Vec<usize> = [0, 1, 400, 413, 600, 2750]
            .map(|count| reading_time(&words(count)))
            .into();
        assert_eq!(minutes, [0, 1, 1, 2, 2, 10]);
    }

    #[test]
    fn a_post_without_a_body_is_for_members_as_ghost_shows_it() {
        let en = khalid_core::Locale::try_new("en").unwrap();
        let url = khalid_core::AbsoluteUrl::parse("https://example.com/blog/brief/").unwrap();
        let slug = khalid_core::Slug::try_new("brief").unwrap();
        let at = DateTime::<Utc>::from_timestamp(1_790_000_000, 0).unwrap();
        let post = Post::new("en.brief", slug, "Brief", url, en, at);
        let ghost = serde_json::to_value(GhostPost::new(&post, &[Format::Html])).unwrap();
        assert_eq!(ghost["visibility"], "members");
        assert_eq!(ghost["access"], false);
        assert_eq!(ghost["html"], "");
    }

    #[test]
    fn the_uuid_is_stable_and_shaped_like_ghosts() {
        let uuid = uuid_of("en.anchor-text");
        assert_eq!(uuid, uuid_of("en.anchor-text"));
        let lengths: Vec<usize> = uuid.split('-').map(str::len).collect();
        assert_eq!(lengths, [8, 4, 4, 4, 12]);
        assert_ne!(uuid, uuid_of("en.title-tags"));
    }
}
