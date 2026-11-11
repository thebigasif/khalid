//! A glossary term as the Admin API reads and writes it.

use chrono::{DateTime, Utc};
use khalid_core::{Locale, Slug};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::display;
use crate::entry_store::Entry;

/// A glossary term in one language, such as "topical authority", in Admin
/// API shape. A post links to it with a wikilink to its slug, and the term
/// gets its own page under the glossary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct AdminTerm {
    /// The language and the slug, such as `en.topical-authority`.
    id: String,
    /// The slug a wikilink names the term by, such as `topical-authority`.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "topical-authority")]
    slug: Slug,
    /// The language, such as `en`.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "en")]
    language: Locale,
    /// The term as it is written.
    term: String,
    /// The key shared by the term in other languages, if any.
    translation: Option<String>,
    /// The definition, in markdown.
    markdown: String,
    /// When the term last changed. An edit or delete sends it back.
    updated_at: DateTime<Utc>,
}

impl AdminTerm {
    /// A term as its file holds it, last changed at `updated_at`.
    pub fn new(
        (language, slug): (Locale, Slug),
        term: &str,
        markdown: &str,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: format!("{language}.{slug}"),
            slug,
            language,
            term: term.to_owned(),
            translation: None,
            markdown: markdown.to_owned(),
            updated_at,
        }
    }

    /// The same term with the translation key `translation`.
    #[must_use]
    pub fn with_translation(self, translation: Option<&str>) -> Self {
        Self {
            translation: translation.map(str::to_owned),
            ..self
        }
    }
}

/// A new glossary term, as a create request sends it.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct NewTerm {
    /// The language, such as `de`; the site's default when left out.
    #[schema(value_type = Option<String>, example = "en")]
    language: Option<Locale>,
    /// The slug a wikilink will name it by, such as `topical-authority`.
    #[schema(value_type = String, example = "topical-authority")]
    slug: Slug,
    /// The term as it is written.
    term: String,
    /// The key shared by the term in other languages, if any.
    translation: Option<String>,
    /// The definition, in markdown.
    markdown: String,
}

impl NewTerm {
    /// The slug.
    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    /// The term as it is written.
    pub fn term(&self) -> &str {
        &self.term
    }

    /// The translation key, if sent.
    pub fn translation(&self) -> Option<&str> {
        self.translation.as_deref()
    }

    /// The definition.
    pub fn markdown(&self) -> &str {
        &self.markdown
    }
}

/// A change to a glossary term: only the fields sent change. An empty
/// `translation` removes it.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TermEdit {
    /// The term as it is now written.
    term: Option<String>,
    /// A new translation key, or `""` to remove it.
    translation: Option<String>,
    /// A new definition, in markdown.
    markdown: Option<String>,
    /// The `updated_at` of the copy this edit was made on.
    updated_at: DateTime<Utc>,
}

impl TermEdit {
    /// The new term, if sent.
    pub fn term(&self) -> Option<&str> {
        self.term.as_deref()
    }

    /// The new translation key, if sent; `""` removes it.
    pub fn translation(&self) -> Option<&str> {
        self.translation.as_deref()
    }

    /// The new definition, if sent.
    pub fn markdown(&self) -> Option<&str> {
        self.markdown.as_deref()
    }
}

impl Entry for AdminTerm {
    type New = NewTerm;
    type Edit = TermEdit;

    fn id(&self) -> &str {
        &self.id
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn seen(edit: &TermEdit) -> DateTime<Utc> {
        edit.updated_at
    }

    fn language(new: &NewTerm) -> Option<&Locale> {
        new.language.as_ref()
    }

    fn made(new: NewTerm, language: Locale, at: DateTime<Utc>) -> Self {
        Self::new((language, new.slug), &new.term, &new.markdown, at)
            .with_translation(new.translation.as_deref())
    }

    fn edited(self, edit: TermEdit, at: DateTime<Utc>) -> Self {
        let translation = match edit.translation {
            Some(sent) if sent.is_empty() => None,
            Some(sent) => Some(sent),
            None => self.translation,
        };
        Self {
            term: edit.term.unwrap_or(self.term),
            translation,
            markdown: edit.markdown.unwrap_or(self.markdown),
            updated_at: at,
            ..self
        }
    }
}
