//! The `[template]` table of `site.toml`: which built-in look the site
//! uses, and that look's options (templates spec 4.1).
//!
//! This crate reads the table as written. Which names exist and which
//! options each look takes belong to `khalid-theme`, which checks them
//! when the site starts.

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

use crate::{OneLine, SiteHref};

/// The longest a template's name may be.
const MAX_NAME: usize = 32;

/// The look a site chose and its options. Without the table, the site uses
/// `classic`, today's look, with no options:
///
/// ```toml
/// [template]
/// name = "classic"
/// [template.options]
/// lead_image = true
/// [template.cta]
/// label = "Start a free trial"
/// href = "https://example.com/signup"
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TemplateChoice {
    #[serde(default)]
    name: PackName,
    #[serde(default)]
    options: BTreeMap<String, OptionValue>,
    #[serde(default)]
    cta: Option<CallToAction>,
}

impl TemplateChoice {
    /// The look's name, such as `classic`.
    pub fn name(&self) -> &PackName {
        &self.name
    }

    /// The options as written, by key.
    pub fn options(&self) -> &BTreeMap<String, OptionValue> {
        &self.options
    }

    /// The site's one call to action, if `[template.cta]` sets it.
    pub fn cta(&self) -> Option<&CallToAction> {
        self.cta.as_ref()
    }
}

/// The one thing the site asks a reader to do, such as start a trial. The
/// call-to-action band, the header button and the `::cta` block show it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CallToAction {
    /// The words on the button.
    pub label: OneLine,
    /// Where the button goes.
    pub href: SiteHref,
    /// One line beside the button, such as "No card needed."
    #[serde(default)]
    pub note: Option<OneLine>,
}

/// A look's name: lowercase letters, digits and hyphens, starting with a
/// letter, at most 32 characters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct PackName(String);

impl PackName {
    /// The name every site has unless it chooses another: today's look.
    pub const CLASSIC: &str = "classic";

    /// The name as written.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PackName {
    fn default() -> Self {
        Self(Self::CLASSIC.to_owned())
    }
}

impl fmt::Display for PackName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for PackName {
    type Error = &'static str;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        let starts_with_letter = text.starts_with(|c: char| c.is_ascii_lowercase());
        let plain = text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        if starts_with_letter && plain && text.len() <= MAX_NAME {
            Ok(Self(text))
        } else {
            Err(
                "a template name is one word of lowercase letters, digits and hyphens, such as classic",
            )
        }
    }
}

/// One option's value as written. The look decides which kind each of its
/// options takes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "toml::Value")]
#[non_exhaustive]
pub enum OptionValue {
    /// `true` or `false`.
    Switch(bool),
    /// A whole number, such as `5`.
    Number(i64),
    /// A word or a line of text, such as `"side-end"`.
    Word(String),
    /// A list of lines of text, such as `["12,000 readers"]`.
    Words(Vec<String>),
    /// A small table of options, such as `{ layout = "list", count = 5 }`.
    Table(BTreeMap<String, OptionValue>),
}

impl OptionValue {
    /// What an option may hold, for the refusal of any other value.
    const KINDS: &str =
        "an option is on or off, a whole number, a word, a list of words, or a table of these";
}

impl TryFrom<toml::Value> for OptionValue {
    type Error = &'static str;

    fn try_from(value: toml::Value) -> Result<Self, Self::Error> {
        Ok(match value {
            toml::Value::Boolean(on) => Self::Switch(on),
            toml::Value::Integer(number) => Self::Number(number),
            toml::Value::String(word) => Self::Word(word),
            toml::Value::Array(items) => Self::Words(
                items
                    .into_iter()
                    .map(|item| match item {
                        toml::Value::String(word) => Ok(word),
                        _ => Err(Self::KINDS),
                    })
                    .collect::<Result<_, _>>()?,
            ),
            toml::Value::Table(table) => Self::Table(
                table
                    .into_iter()
                    .map(|(key, value)| Self::try_from(value).map(|value| (key, value)))
                    .collect::<Result<_, _>>()?,
            ),
            toml::Value::Float(_) | toml::Value::Datetime(_) => return Err(Self::KINDS),
        })
    }
}
