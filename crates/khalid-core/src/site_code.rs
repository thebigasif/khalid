//! `[code]` in `site.toml`: the site's own code for every page, such as a
//! tag manager or an analytics snippet, and the consent category each
//! piece waits for (KH-SEO-12, privacy plan KTD2 and KTD12).

use std::fmt;

use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{AbsoluteUrl, ConsentCategory, HeadCode, HeadCodeError};

/// HTML for the end of every page's body: only `script` elements, and
/// comments, which are dropped.
///
/// It is checked and written again as [`HeadCode`] is, and each `script`
/// gets the page's nonce when it is drawn ([`BodyCode::to_html`]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BodyCode(HeadCode);

impl BodyCode {
    /// Reads `html` and keeps it in its checked form.
    ///
    /// # Errors
    ///
    /// Returns [`BodyCodeError::Element`] for an element other than
    /// `script`, and [`BodyCodeError::Code`] for the other faults
    /// [`HeadCode::try_new`] finds.
    pub fn try_new(html: &str) -> Result<Self, BodyCodeError> {
        HeadCode::scripts_only(html)
            .map(Self)
            .map_err(|error| match error {
                HeadCodeError::Element { name } => BodyCodeError::Element { name },
                other => BodyCodeError::Code(other),
            })
    }

    /// Whether there is nothing to write, for body code that held only
    /// comments or spaces.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The body code as HTML, with `nonce` on each `script`.
    pub fn to_html(&self, nonce: &str) -> String {
        self.0.to_html(nonce)
    }

    /// The body code as a static page holds it until a reader agrees to
    /// `category`, as [`HeadCode::to_held_html`] writes it.
    pub fn to_held_html(&self, category: &str) -> String {
        self.0.to_held_html(category)
    }
}

impl fmt::Display for BodyCode {
    /// The checked form, without nonces.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Why body code was refused, naming what to change.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BodyCodeError {
    /// An element other than `script`.
    #[error(
        "body code may hold only script, not {name}; put meta, link and style in the head code"
    )]
    Element {
        /// The element's name.
        name: String,
    },
    /// A fault head code has too, such as a script never closed.
    #[error("{}", body_words(.0))]
    Code(HeadCodeError),
}

/// The head code fault `error` as body code reads it.
fn body_words(error: &HeadCodeError) -> String {
    error.to_string().replacen("head code", "body code", 1)
}

impl TryFrom<String> for BodyCode {
    type Error = BodyCodeError;

    fn try_from(html: String) -> Result<Self, Self::Error> {
        Self::try_new(&html)
    }
}

impl From<BodyCode> for String {
    fn from(code: BodyCode) -> Self {
        code.to_string()
    }
}

/// Whether the old single `[code]` table waits until a reader agrees to
/// it. A `[[code]]` list says so per entry with a [`CodeCategory`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Consent {
    /// The code waits for the reader to allow Marketing.
    #[default]
    Ask,
    /// The code runs for every reader, with no banner, for code that sets
    /// no cookie and tracks no one.
    NotNeeded,
}

/// The consent category one entry of `[code]` waits for (privacy plan
/// KTD2). Necessary is not one of them: it is Khalid's own, and code that
/// sets no cookie and tracks no one says so with [`CodeCategory::NoTracking`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum CodeCategory {
    /// Code that remembers a reader's settings, such as a chosen theme.
    Preferences,
    /// Code that measures visits, such as an analytics tag.
    Analytics,
    /// Code for ads and tracking across sites, and code with no category.
    #[default]
    Marketing,
    /// Code that sets no cookie and tracks no one, which runs for every
    /// reader and is named on the privacy page (R24).
    NoTracking,
}

impl CodeCategory {
    /// Every category in the order the admin lists them.
    pub const ALL: [Self; 4] = [
        Self::Preferences,
        Self::Analytics,
        Self::Marketing,
        Self::NoTracking,
    ];

    /// The word `site.toml` writes, such as `no-tracking`.
    pub fn key(self) -> &'static str {
        match self {
            Self::Preferences => "preferences",
            Self::Analytics => "analytics",
            Self::Marketing => "marketing",
            Self::NoTracking => "no-tracking",
        }
    }

    /// The reader's category this code runs under: Necessary for code
    /// that tracks no one.
    pub fn consent(self) -> ConsentCategory {
        match self {
            Self::Preferences => ConsentCategory::Preferences,
            Self::Analytics => ConsentCategory::Analytics,
            Self::Marketing => ConsentCategory::Marketing,
            Self::NoTracking => ConsentCategory::Necessary,
        }
    }
}

impl<'de> Deserialize<'de> for CodeCategory {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let word = String::deserialize(deserializer)?;
        Self::ALL
            .into_iter()
            .find(|category| category.key() == word)
            .ok_or_else(|| {
                de::Error::custom(format!(
                    "{word} is not a code category; use preferences, analytics, marketing or no-tracking"
                ))
            })
    }
}

/// One piece of the site's own code: what it adds to the head and the
/// body, the consent category it waits for, and who runs it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CodeEntry {
    /// Code for the end of every page's head.
    pub head: Option<HeadCode>,
    /// Code for the end of every page's body.
    pub body: Option<BodyCode>,
    /// The category it waits for, Marketing when the entry names none.
    #[serde(default)]
    pub category: CodeCategory,
    /// Who runs it, such as `Google Analytics`, for the banner and the
    /// privacy page.
    pub vendor: Option<String>,
    /// The vendor's own privacy policy.
    pub policy: Option<AbsoluteUrl>,
}

impl CodeEntry {
    /// Whether the entry has no code, or only comments.
    pub fn is_empty(&self) -> bool {
        self.head.as_ref().is_none_or(HeadCode::is_empty)
            && self.body.as_ref().is_none_or(BodyCode::is_empty)
    }
}

/// The old single `[code]` table, read into one entry: `consent = "ask"`,
/// the default, is Marketing, and `consent = "not-needed"` is
/// [`CodeCategory::NoTracking`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OneTable {
    head: Option<HeadCode>,
    body: Option<BodyCode>,
    #[serde(default)]
    consent: Consent,
}

/// The site's own code from `[code]`, as a list of `[[code]]` entries or
/// the old single table, which reads as one entry:
///
/// ```toml
/// [[code]]
/// head = "<script src='https://stats.example.net/a.js'></script>"
/// category = "analytics"
/// vendor = "Example Stats"
/// policy = "https://stats.example.net/privacy"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct SiteCode {
    /// Every entry, in the order written.
    pub entries: Vec<CodeEntry>,
}

impl SiteCode {
    /// Whether the site has no code of its own, or only comments.
    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(CodeEntry::is_empty)
    }

    /// Whether some code waits for a reader's consent: an entry with code
    /// in a category other than no-tracking.
    pub fn waits_for_consent(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.category != CodeCategory::NoTracking && !entry.is_empty())
    }

    /// Every entry's head code that is not empty, in the order written.
    pub fn heads(&self) -> impl Iterator<Item = &HeadCode> {
        let heads = self.entries.iter().filter_map(|entry| entry.head.as_ref());
        heads.filter(|head| !head.is_empty())
    }

    /// Every entry's body code that is not empty, in the order written.
    pub fn bodies(&self) -> impl Iterator<Item = &BodyCode> {
        let bodies = self.entries.iter().filter_map(|entry| entry.body.as_ref());
        bodies.filter(|body| !body.is_empty())
    }
}

impl<'de> Deserialize<'de> for SiteCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(CodeVisitor)
    }
}

/// Reads `[code]` as written: a list of entries, or the old single table.
struct CodeVisitor;

impl<'de> Visitor<'de> for CodeVisitor {
    type Value = SiteCode;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a [[code]] list or a [code] table")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<SiteCode, A::Error> {
        let entries = Vec::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(SiteCode { entries })
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<SiteCode, A::Error> {
        let table = OneTable::deserialize(MapAccessDeserializer::new(map))?;
        let category = match table.consent {
            Consent::Ask => CodeCategory::Marketing,
            Consent::NotNeeded => CodeCategory::NoTracking,
        };
        let entry = CodeEntry {
            head: table.head,
            body: table.body,
            category,
            vendor: None,
            policy: None,
        };
        let entries = if entry.head.is_none() && entry.body.is_none() {
            Vec::new()
        } else {
            vec![entry]
        };
        Ok(SiteCode { entries })
    }
}
