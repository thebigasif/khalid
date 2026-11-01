//! A site's chosen look: its pack, and each option checked against the
//! pack's list with the pack's default for the ones left out.

use std::collections::BTreeMap;

use khalid_core::{OptionValue, TemplateChoice};
use serde::Serialize;

use crate::{OptionKind, OptionSpec, Pack, packs};

/// The pack a site chose, with every one of its options set.
#[derive(Debug, Clone)]
pub struct Look {
    pack: &'static Pack,
    options: BTreeMap<&'static str, CheckedOption>,
}

/// One option's value, checked against the pack. Templates read it as
/// `template.options.<key>`. A new kind of option adds a variant here and
/// in the renderer's `TemplateOption`, so the enum is exhaustive on
/// purpose: the binary's match stops the build until it is handled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum CheckedOption {
    /// An on or off option.
    Switch(bool),
    /// One word from the option's list.
    Word(String),
    /// A whole number in the option's range.
    Number(i64),
    /// Lines of text, no more than the option allows.
    Lines(Vec<String>),
}

/// Why a `[template]` table was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum LookError {
    /// No pack has the name.
    #[error("there is no template named {name}; use one of: {known}")]
    UnknownTemplate {
        /// The name written.
        name: String,
        /// Every pack's name, joined with commas.
        known: String,
    },
    /// The pack has no option with the key.
    #[error("the {template} template has no option {key}; {fix}")]
    UnknownOption {
        /// The pack.
        template: &'static str,
        /// The key written.
        key: String,
        /// What to write instead.
        fix: String,
    },
    /// The option holds a value it does not take.
    #[error("the option {key} of the {template} template takes {wants}")]
    WrongValue {
        /// The pack.
        template: &'static str,
        /// The option.
        key: &'static str,
        /// What it takes, such as `true or false`.
        wants: String,
    },
}

impl LookError {
    /// The key in `site.toml` the mistake is at, such as `name`.
    pub fn key(&self) -> &str {
        match self {
            Self::UnknownTemplate { .. } => "name",
            Self::UnknownOption { key, .. } => key,
            Self::WrongValue { key, .. } => key,
        }
    }

    /// The table of `site.toml` the key is in, such as `[template]`.
    pub fn table(&self) -> &'static str {
        match self {
            Self::UnknownTemplate { .. } => "[template]",
            Self::UnknownOption { .. } | Self::WrongValue { .. } => "[template.options]",
        }
    }
}

impl Look {
    /// Classic with no options: the look of a site without `[template]`.
    pub fn classic() -> Self {
        Self {
            pack: crate::classic(),
            options: BTreeMap::new(),
        }
    }

    /// The pack `choice` names, with its options checked.
    ///
    /// # Errors
    ///
    /// Returns [`LookError::UnknownTemplate`] for a name no pack has,
    /// [`LookError::UnknownOption`] for a key the pack does not list, and
    /// [`LookError::WrongValue`] for a value the option does not take.
    pub fn choose(choice: &TemplateChoice) -> Result<Self, LookError> {
        let name = choice.name().as_str();
        let pack = packs()
            .iter()
            .find(|pack| pack.name == name)
            .ok_or_else(|| LookError::UnknownTemplate {
                name: name.to_owned(),
                known: names(packs().iter().map(|pack| pack.name)),
            })?;
        let written = choice.options();
        if let Some(key) = written
            .keys()
            .find(|key| !pack.options.iter().any(|option| option.key == key.as_str()))
        {
            let fix = if pack.options.is_empty() {
                "it takes no options, so remove the line".to_owned()
            } else {
                let keys = names(pack.options.iter().map(|option| option.key));
                format!("it takes {keys}")
            };
            return Err(LookError::UnknownOption {
                template: pack.name,
                key: key.clone(),
                fix,
            });
        }
        let mut options = BTreeMap::new();
        for spec in pack.options {
            let value =
                check(spec, written.get(spec.key)).ok_or_else(|| LookError::WrongValue {
                    template: pack.name,
                    key: spec.key,
                    wants: wants(spec.kind),
                })?;
            options.insert(spec.key, value);
        }
        Ok(Self { pack, options })
    }

    /// The chosen pack.
    pub fn pack(&self) -> &'static Pack {
        self.pack
    }

    /// Every option of the pack, set as written or to its default.
    pub fn options(&self) -> &BTreeMap<&'static str, CheckedOption> {
        &self.options
    }
}

/// The value of `spec` from `written`, its default when not written, or
/// `None` when the written value is not one it takes.
fn check(spec: &OptionSpec, written: Option<&OptionValue>) -> Option<CheckedOption> {
    Some(match (spec.kind, written) {
        (OptionKind::Switch { default }, None) => CheckedOption::Switch(default),
        (OptionKind::Switch { .. }, Some(OptionValue::Switch(on))) => CheckedOption::Switch(*on),
        (OptionKind::Choice { default, .. }, None) => CheckedOption::Word(default.to_owned()),
        (OptionKind::Choice { allowed, .. }, Some(OptionValue::Word(word)))
            if allowed.contains(&word.as_str()) =>
        {
            CheckedOption::Word(word.clone())
        }
        (OptionKind::Count { default, .. }, None) => CheckedOption::Number(default),
        (OptionKind::Count { least, most, .. }, Some(OptionValue::Number(number)))
            if (least..=most).contains(number) =>
        {
            CheckedOption::Number(*number)
        }
        (OptionKind::Lines { .. }, None) => CheckedOption::Lines(Vec::new()),
        (OptionKind::Lines { most }, Some(OptionValue::Words(lines))) if lines.len() <= most => {
            CheckedOption::Lines(lines.clone())
        }
        _ => return None,
    })
}

/// What an option of `kind` takes, in the words of a refusal.
fn wants(kind: OptionKind) -> String {
    match kind {
        OptionKind::Switch { .. } => "true or false".to_owned(),
        OptionKind::Choice { allowed, .. } => format!("one of {}", allowed.join(", ")),
        OptionKind::Count { least, most, .. } => {
            format!("a whole number from {least} to {most}")
        }
        OptionKind::Lines { most } => format!("a list of up to {most} lines of text"),
    }
}

/// `names` joined with commas, such as `classic, ledger`.
fn names<'a>(names: impl Iterator<Item = &'a str>) -> String {
    names.collect::<Vec<_>>().join(", ")
}
