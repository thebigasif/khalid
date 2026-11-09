//! A post.

use chrono::{DateTime, SecondsFormat, Utc};
use khalid_core::{AbsoluteUrl, Locale, Slug};
use serde::Serialize;
use utoipa::ToSchema;

use super::{HeadParts, Listable, PostHead, display};
use crate::query::{FieldValues, Format, Literal};

/// A published post in API shape. Bodies are present only when asked for
/// with `formats`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct Post {
    /// Stable across edits: the content file's identity.
    id: String,
    /// The last part of its address.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "why-internal-links-matter")]
    slug: Slug,
    /// The title.
    title: String,
    /// The full public address.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "https://example.com/blog/why-internal-links-matter/")]
    url: AbsoluteUrl,
    /// The language, such as `en` or `de`.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "en")]
    language: Locale,
    /// A short summary, if the writer gave one.
    excerpt: Option<String>,
    /// Whether the post is marked as featured.
    featured: bool,
    /// Tag slugs, in the order the writer gave them.
    tags: Vec<String>,
    /// Author slugs, in the order the writer gave them.
    authors: Vec<String>,
    /// When it went live.
    published_at: DateTime<Utc>,
    /// When it last changed.
    updated_at: DateTime<Utc>,
    /// What its page carries in `<head>` for search engines and share
    /// cards, so a front end of its own keeps the meta tags and schema.
    /// Absent until the site's pages have been made once.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<HeadParts>)]
    head: Option<PostHead>,
    /// The series the post is one part of, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    series: Option<PostSeries>,
    /// The body, in the formats asked for.
    #[serde(flatten)]
    bodies: Bodies,
}

/// The series a post is one part of: posts written to be read in order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct PostSeries {
    /// The series' name, the same on every part.
    #[schema(example = "Topic clusters from scratch")]
    name: String,
    /// The end of the series page's address.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "topic-clusters-from-scratch")]
    slug: Slug,
    /// The post's place in the series, counted from 1, as the writer set
    /// it. Absent when the writer left the order to the dates.
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<u32>,
}

impl PostSeries {
    /// The series called `name`, whose page ends in `slug`, with the post
    /// at `order`.
    pub fn new(name: &str, slug: Slug, order: Option<u32>) -> Self {
        Self {
            name: name.to_owned(),
            slug,
            order,
        }
    }
}

/// A post's body in each format. A format left out of `formats` is absent.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, ToSchema)]
pub struct Bodies {
    /// Rendered HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    /// The markdown source.
    #[serde(skip_serializing_if = "Option::is_none")]
    markdown: Option<String>,
    /// Text with the markup removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    plaintext: Option<String>,
}

impl Bodies {
    /// All three formats.
    pub fn new(html: &str, markdown: &str, plaintext: &str) -> Self {
        Self {
            html: Some(html.to_owned()),
            markdown: Some(markdown.to_owned()),
            plaintext: Some(plaintext.to_owned()),
        }
    }
}

impl Post {
    /// The fields a `filter` and `order` may name on posts. `tag` and
    /// `author` are Ghost's names for one tag or author slug.
    pub const FILTER_FIELDS: [&'static str; 11] = [
        "id",
        "slug",
        "title",
        "language",
        "featured",
        "tag",
        "tags",
        "author",
        "authors",
        "published_at",
        "updated_at",
    ];

    /// A post with no excerpt, tags, authors or body, not featured, last
    /// changed when it was published.
    pub fn new(
        id: &str,
        slug: Slug,
        title: &str,
        url: AbsoluteUrl,
        language: Locale,
        published_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.to_owned(),
            slug,
            title: title.to_owned(),
            url,
            language,
            excerpt: None,
            featured: false,
            tags: Vec::new(),
            authors: Vec::new(),
            published_at,
            updated_at: published_at,
            head: None,
            series: None,
            bodies: Bodies::default(),
        }
    }

    /// The same post with this excerpt.
    #[must_use]
    pub fn with_excerpt(self, excerpt: &str) -> Self {
        let excerpt = Some(excerpt.to_owned());
        Self { excerpt, ..self }
    }

    /// The same post with these tag slugs.
    #[must_use]
    pub fn with_tags(self, tags: Vec<String>) -> Self {
        Self { tags, ..self }
    }

    /// The same post with these author slugs.
    #[must_use]
    pub fn with_authors(self, authors: Vec<String>) -> Self {
        Self { authors, ..self }
    }

    /// The same post, marked as featured.
    #[must_use]
    pub fn featured(self) -> Self {
        Self {
            featured: true,
            ..self
        }
    }

    /// The same post, last changed at `updated_at`.
    #[must_use]
    pub fn with_updated_at(self, updated_at: DateTime<Utc>) -> Self {
        Self { updated_at, ..self }
    }

    /// The same post with the head of its page, or none, held packed: a
    /// store keeps a head for every post and sends few of them.
    #[must_use]
    pub fn with_head(self, head: Option<&PostHead>) -> Self {
        let head = head.map(PostHead::packed);
        Self { head, ..self }
    }

    /// The same post as a part of `series`, or of none.
    #[must_use]
    pub fn with_series(self, series: Option<PostSeries>) -> Self {
        Self { series, ..self }
    }

    /// The same post with these bodies.
    #[must_use]
    pub fn with_bodies(self, bodies: Bodies) -> Self {
        Self { bodies, ..self }
    }

    /// The same post with only the body formats in `formats`.
    #[must_use]
    pub fn with_formats(self, formats: &[Format]) -> Self {
        let keep = |format, body: Option<String>| body.filter(|_| formats.contains(&format));
        let bodies = Bodies {
            html: keep(Format::Html, self.bodies.html),
            markdown: keep(Format::Markdown, self.bodies.markdown),
            plaintext: keep(Format::Plaintext, self.bodies.plaintext),
        };
        Self { bodies, ..self }
    }

    /// The identity.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The slug.
    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    /// The full public address.
    pub fn url(&self) -> &AbsoluteUrl {
        &self.url
    }

    /// The language.
    pub fn language(&self) -> &Locale {
        &self.language
    }

    /// The title.
    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    /// The writer's summary, if any.
    pub(crate) fn excerpt(&self) -> Option<&str> {
        self.excerpt.as_deref()
    }

    /// Whether it is marked as featured.
    pub(crate) fn is_featured(&self) -> bool {
        self.featured
    }

    /// Tag slugs, in the writer's order.
    pub(crate) fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Author slugs, in the writer's order.
    pub(crate) fn authors(&self) -> &[String] {
        &self.authors
    }

    /// When it went live and when it last changed.
    pub(crate) fn times(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        (self.published_at, self.updated_at)
    }

    /// The head of its page, when the pages have been made.
    pub(crate) fn head(&self) -> Option<&PostHead> {
        self.head.as_ref()
    }

    /// The rendered HTML and the plain text, when the store holds them.
    pub(crate) fn html_and_text(&self) -> (Option<&str>, Option<&str>) {
        let bodies = &self.bodies;
        (bodies.html.as_deref(), bodies.plaintext.as_deref())
    }
}

impl Listable for Post {
    const FIELDS: &'static [&'static str] = &Self::FILTER_FIELDS;

    fn id(&self) -> &str {
        &self.id
    }

    fn slug(&self) -> &Slug {
        &self.slug
    }
}

impl FieldValues for Post {
    fn values(&self, field: &str) -> Vec<Literal> {
        let text = |value: &str| vec![Literal::Text(value.to_owned())];
        let time = |at: &DateTime<Utc>| text(&at.to_rfc3339_opts(SecondsFormat::Secs, true));
        let all = |values: &[String]| values.iter().map(|v| Literal::Text(v.clone())).collect();
        match field {
            "id" => text(&self.id),
            "slug" => text(self.slug.as_str()),
            "title" => text(&self.title),
            "language" => text(self.language.as_str()),
            "featured" => vec![Literal::Bool(self.featured)],
            "tag" | "tags" => all(&self.tags),
            "author" | "authors" => all(&self.authors),
            "published_at" => time(&self.published_at),
            "updated_at" => time(&self.updated_at),
            // Ghost's names, which only its compatible API lets a filter
            // use: every post served is published and public.
            "primary_tag" => all(self.tags.get(..1).unwrap_or_default()),
            "primary_author" => all(self.authors.get(..1).unwrap_or_default()),
            "status" => text("published"),
            "visibility" => text("public"),
            _ => Vec::new(),
        }
    }
}
