//! The messages every rule shows, read from `catalogue.toml`.

use std::collections::BTreeMap;
use std::collections::btree_map;
use std::io::Read as _;
use std::sync::OnceLock;

use khalid_core::{Slug, UserMessage};
use serde::Deserialize;

use crate::LintError;

/// The catalogue file, built into the crate so every user shows the same
/// words, packed with Brotli by the build script.
const PACKED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/catalogue.toml.br"));

/// The catalogue's text, unpacked on first use and kept.
static BUILTIN: OnceLock<String> = OnceLock::new();

/// The catalogue's text.
///
/// # Errors
///
/// Returns [`LintError::CatalogueUnpacked`] if the packed file cannot be
/// read, which the crate's own tests rule out.
fn builtin_text() -> Result<&'static str, LintError> {
    if let Some(text) = BUILTIN.get() {
        return Ok(text);
    }
    let mut text = String::with_capacity(PACKED.len() * 4);
    brotli::Decompressor::new(PACKED, 4096)
        .read_to_string(&mut text)
        .map_err(|source| LintError::CatalogueUnpacked { source })?;
    Ok(BUILTIN.get_or_init(|| text))
}

/// A lint message: what is wrong, why it matters, what to do, and a guide.
///
/// Each part is one plain sentence for a writer who is new to SEO
/// (RULES.md BL-TXT-2). `guide` names the `SerpDad` guide that explains the
/// rule at length; the docs map it to an address later.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    what: String,
    why: String,
    fix: String,
    #[serde(default)]
    fix_alone: Option<String>,
    guide: Slug,
    /// The words a rule looks for, when its catalogue entry lists them,
    /// such as the terms `non-inclusive-term` suggests other words for.
    #[serde(default)]
    terms: Vec<Swap>,
}

/// A word or phrase a rule looks for in one language, with the words to
/// use instead.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Swap {
    /// The language, such as `en`.
    pub(crate) lang: String,
    /// The word or phrase, as a reader would find it.
    pub(crate) term: String,
    /// The words to use instead, as one suggestion.
    pub(crate) instead: String,
}

impl Message {
    /// The fix for a post that is the only one in its language, when the
    /// rule has one, such as linking the home page while there is no other
    /// post to link to.
    pub fn fix_alone(&self) -> Option<&str> {
        self.fix_alone.as_deref()
    }

    /// This message as a post that is the only one in its language reads
    /// it: with [`Message::fix_alone`] as the fix, when the rule has one.
    pub(crate) fn for_only_post(&self) -> Self {
        let mut message = self.clone();
        if let Some(alone) = message.fix_alone.take() {
            message.fix = alone;
        }
        message
    }

    /// What is wrong.
    pub fn what(&self) -> &str {
        &self.what
    }

    /// Why it matters to the writer.
    pub fn why(&self) -> &str {
        &self.why
    }

    /// What the writer does to fix it.
    pub fn fix(&self) -> &str {
        &self.fix
    }

    /// The guide that explains the rule at length.
    pub fn guide(&self) -> &Slug {
        &self.guide
    }

    /// The words the rule looks for, when its entry lists them.
    pub(crate) fn terms(&self) -> &[Swap] {
        &self.terms
    }

    /// The three sentences as the message every Khalid surface shows.
    pub fn to_user_message(&self) -> UserMessage {
        UserMessage::new(self.what.clone(), self.why.clone(), self.fix.clone())
    }
}

/// Every message, keyed by rule ID, in rule ID order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Catalogue(BTreeMap<String, Message>);

impl Catalogue {
    /// Reads the catalogue built into this crate.
    ///
    /// # Errors
    ///
    /// Returns [`LintError::Catalogue`] if the built-in file is malformed,
    /// or [`LintError::CatalogueUnpacked`] if it cannot be unpacked, which
    /// the crate's own tests rule out.
    pub fn builtin() -> Result<Self, LintError> {
        toml::from_str(builtin_text()?)
            .map(Self)
            .map_err(|source| LintError::Catalogue { source })
    }

    /// The message for `rule`, if the catalogue has one.
    pub fn get(&self, rule: &str) -> Option<&Message> {
        self.0.get(rule)
    }

    /// Whether the catalogue has a message for `rule`.
    pub fn contains(&self, rule: &str) -> bool {
        self.0.contains_key(rule)
    }

    /// Every rule ID with its message, in rule ID order.
    pub fn iter(&self) -> btree_map::Iter<'_, String, Message> {
        self.0.iter()
    }

    /// How many messages the catalogue holds.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the catalogue holds no messages.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> IntoIterator for &'a Catalogue {
    type Item = (&'a String, &'a Message);
    type IntoIter = btree_map::Iter<'a, String, Message>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_packed_catalogue_unpacks_to_its_file_byte_for_byte() {
        let file = include_str!("../catalogue.toml");
        assert_eq!(builtin_text().unwrap(), file);
        assert!(PACKED.len() < file.len() / 2);
    }
}
