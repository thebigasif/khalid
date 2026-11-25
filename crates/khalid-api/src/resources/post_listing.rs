//! How search engines list and describe a post beyond its title and
//! keywords: the sitemaps, the breadcrumb, the schema types, the keyword's
//! other words and who checked it (plan 5.2 and 5.4).

use khalid_core::{ArticleSchemaType, PageSchemaType, Slug};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::ToSchema;
use utoipa::openapi::schema::{ObjectBuilder, Schema, SchemaType, Type};

use super::NonBlank;
use super::settings_edit::Change;

/// The kinds of web page a post may choose, as schema.org names them.
pub const PAGE_TYPES: [&str; 7] = [
    "WebPage",
    "FAQPage",
    "ContactPage",
    "AboutPage",
    "QAPage",
    "ItemPage",
    "CollectionPage",
];

/// The kinds of article a post may choose, as schema.org names them.
pub const ARTICLE_TYPES: [&str; 5] = [
    "BlogPosting",
    "Article",
    "NewsArticle",
    "TechArticle",
    "Report",
];

/// How search engines list and describe a post. Every part has a value:
/// `true` for the sitemap, and `null` or an empty list otherwise, when
/// never set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct PostListing {
    /// Whether the post is listed in the XML sitemap. `false` keeps it
    /// out, as for a thank-you post no one searches for.
    pub sitemap: bool,
    /// `false` keeps a news post out of the news sitemap; `null` lists it
    /// as its article type says.
    pub news: Option<bool>,
    /// A shorter name for the post in breadcrumb trails, or `null` for
    /// its title.
    #[schema(value_type = Option<String>, example = "Anchor text")]
    pub breadcrumb_title: Option<NonBlank>,
    /// The kind of web page, such as `FAQPage`, or `null` for `WebPage`.
    #[serde(serialize_with = "page_name")]
    #[schema(schema_with = page_type_schema)]
    pub page_type: Option<PageSchemaType>,
    /// The kind of article, such as `Report`, or `null` for the
    /// collection's.
    #[serde(serialize_with = "article_name")]
    #[schema(schema_with = article_type_schema)]
    pub article_type: Option<ArticleSchemaType>,
    /// Other words for the focus keyword that count as using it, such as
    /// `site links` for `internal links`.
    #[schema(value_type = Vec<String>, example = json!(["site links"]))]
    pub keyword_synonyms: Vec<NonBlank>,
    /// The slugs of the authors who checked the post, shown as "Reviewed
    /// by".
    #[serde(serialize_with = "slugs")]
    #[schema(value_type = Vec<String>, example = json!(["mira"]))]
    pub reviewed_by: Vec<Slug>,
}

impl Default for PostListing {
    fn default() -> Self {
        Self {
            sitemap: true,
            news: None,
            breadcrumb_title: None,
            page_type: None,
            article_type: None,
            keyword_synonyms: Vec::new(),
            reviewed_by: Vec::new(),
        }
    }
}

/// New values for some of how a post is listed. A part left out keeps its
/// value, and `null` clears one that may be empty.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListingEdit {
    /// Whether the post is listed in the XML sitemap.
    sitemap: Option<bool>,
    /// `false` to keep a news post out of the news sitemap, or `null` to
    /// follow its article type.
    #[serde(default)]
    #[schema(value_type = Option<bool>)]
    news: Change<bool>,
    /// A new breadcrumb name, or `null` for the post's title.
    #[serde(default)]
    #[schema(value_type = Option<String>, example = "Anchor text")]
    breadcrumb_title: Change<NonBlank>,
    /// A new kind of web page, or `null` for `WebPage`.
    #[serde(default)]
    #[schema(schema_with = page_type_schema)]
    page_type: Change<PageSchemaType>,
    /// A new kind of article, or `null` for the collection's.
    #[serde(default)]
    #[schema(schema_with = article_type_schema)]
    article_type: Change<ArticleSchemaType>,
    /// Every other word for the focus keyword; an empty list for none.
    #[serde(default, deserialize_with = "distinct_words")]
    #[schema(value_type = Option<Vec<String>>, example = json!(["site links"]))]
    keyword_synonyms: Option<Vec<NonBlank>>,
    /// Every reviewer's author slug; an empty list for none.
    #[schema(value_type = Option<Vec<String>>, example = json!(["mira"]))]
    reviewed_by: Option<Vec<Slug>>,
}

impl ListingEdit {
    /// `listing` with the parts this edit names put in their place.
    #[must_use]
    pub fn applied_to(self, listing: PostListing) -> PostListing {
        PostListing {
            sitemap: self.sitemap.unwrap_or(listing.sitemap),
            news: self.news.applied_to(listing.news),
            breadcrumb_title: self.breadcrumb_title.applied_to(listing.breadcrumb_title),
            page_type: self.page_type.applied_to(listing.page_type),
            article_type: self.article_type.applied_to(listing.article_type),
            keyword_synonyms: self.keyword_synonyms.unwrap_or(listing.keyword_synonyms),
            reviewed_by: self.reviewed_by.unwrap_or(listing.reviewed_by),
        }
    }
}

/// Reads a list of keyword synonyms, refusing one listed twice, whatever
/// its capitals. Serde calls it only for a field that is present.
fn distinct_words<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<NonBlank>>, D::Error> {
    let words = Vec::<NonBlank>::deserialize(deserializer)?;
    let key = |word: &NonBlank| word.as_str().trim().to_lowercase();
    for (at, word) in words.iter().enumerate() {
        if words.iter().take(at).any(|seen| key(seen) == key(word)) {
            return Err(serde::de::Error::custom(format!(
                "the synonym {:?} is listed twice; keep one",
                word.as_str()
            )));
        }
    }
    Ok(Some(words))
}

/// The schema for a page type: one of [`PAGE_TYPES`], or `null`.
fn page_type_schema() -> Schema {
    names_or_null(
        &PAGE_TYPES,
        "The kind of web page, as schema.org names it, or `null` for `WebPage`.",
    )
}

/// The schema for an article type: one of [`ARTICLE_TYPES`], or `null`.
fn article_type_schema() -> Schema {
    names_or_null(
        &ARTICLE_TYPES,
        "The kind of article, as schema.org names it, or `null` for the collection's.",
    )
}

/// A string schema that takes one of `names`, or `null`.
fn names_or_null(names: &[&str], description: &str) -> Schema {
    let values = names.iter().map(|name| Some(*name)).chain([None]);
    let schema = ObjectBuilder::new()
        .schema_type(SchemaType::from_iter([Type::String, Type::Null]))
        .enum_values(Some(values))
        .description(Some(description));
    schema.into()
}

/// Writes the page type's schema.org name, or `null`.
#[expect(
    clippy::ref_option,
    clippy::trivially_copy_pass_by_ref,
    reason = "serde's serialize_with passes the field by reference"
)]
fn page_name<S: Serializer>(
    kind: &Option<PageSchemaType>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    kind.map(PageSchemaType::as_str).serialize(serializer)
}

/// Writes the article type's schema.org name, or `null`.
#[expect(
    clippy::ref_option,
    clippy::trivially_copy_pass_by_ref,
    reason = "serde's serialize_with passes the field by reference"
)]
fn article_name<S: Serializer>(
    kind: &Option<ArticleSchemaType>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    kind.map(ArticleSchemaType::as_str).serialize(serializer)
}

/// Writes each slug as text.
fn slugs<S: Serializer>(slugs: &[Slug], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(slugs.iter().map(Slug::as_str))
}
