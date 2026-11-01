//! The template pack a site chose, as the renderer takes it: the files it
//! replaces and the options its templates read (templates spec 4.1).
//!
//! The packs themselves live in `khalid-theme`; the binary hands the
//! chosen one over in this shape, so this crate needs no theme code.

use std::collections::BTreeMap;

use serde::Serialize;

/// The chosen pack: its name, the template files it replaces, and its
/// options, every one set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderPack<'a> {
    /// The pack's name, such as `classic`.
    pub name: &'a str,
    /// The files it replaces, as the shared file's name and the pack's
    /// source for it.
    pub templates: Vec<(&'a str, &'a str)>,
    /// Its options by key, which templates read as `template.options`.
    pub options: BTreeMap<String, TemplateOption>,
}

/// One option's value as templates read it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum TemplateOption {
    /// On or off.
    Switch(bool),
    /// One word.
    Word(String),
    /// A whole number.
    Number(i64),
    /// Lines of text.
    Lines(Vec<String>),
}

impl RenderPack<'_> {
    /// Classic, today's look: no file replaced and no options.
    pub fn classic() -> Self {
        Self {
            name: "classic",
            templates: Vec::new(),
            options: BTreeMap::new(),
        }
    }
}

/// What templates read as `template`: the pack's name and options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct PackContext {
    pub(crate) name: String,
    pub(crate) options: BTreeMap<String, TemplateOption>,
}

impl From<&RenderPack<'_>> for PackContext {
    fn from(pack: &RenderPack<'_>) -> Self {
        Self {
            name: pack.name.to_owned(),
            options: crate::parts::with_parts(pack.options.clone()),
        }
    }
}
