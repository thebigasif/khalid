//! Severities a site sets in `lint.toml`.

use std::collections::{BTreeMap, BTreeSet};

use khalid_core::{Slug, UnclearWords};
use serde::Deserialize;
use toml::Spanned;

use crate::{Catalogue, LintError, RuleId, Severity, Thresholds};

/// The settings from a site's `lint.toml`.
///
/// The `[rules]` table maps a rule ID to `"block"`, `"warn"`, `"info"` or
/// `"off"`. A
/// rule the file does not name keeps its default. The `[thresholds]` table
/// tunes the numbers behind the rules that measure (see [`Thresholds`]).
/// The `[reviewers]` table names, in `required-in`, the categories whose
/// posts must say who checked them, such as health or money posts.
/// The `[words]` table adds a site's own `hedges` and `vague-openers`,
/// such as words in its own language, to the built-in [`UnclearWords`]
/// that the `plain-answer` rule and the audit both flag.
///
/// # Examples
///
/// ```
/// use khalid_lint::{LintConfig, RuleId, Severity};
///
/// let config = LintConfig::parse("[rules]\nno-author = \"warn\"\n")?;
/// assert_eq!(config.severity(RuleId::new("no-author")), Some(Severity::Warn));
/// # Ok::<(), khalid_lint::LintError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LintConfig {
    severities: BTreeMap<String, Severity>,
    thresholds: Thresholds,
    reviewers_required_in: Vec<Slug>,
    /// The line of each category in `reviewers_required_in`, in its order.
    required_in_lines: Vec<usize>,
    unclear_words: UnclearWords,
}

/// The file as written, with the position of each rule name.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    #[serde(default)]
    rules: BTreeMap<Spanned<String>, Severity>,
    #[serde(default)]
    thresholds: Thresholds,
    #[serde(default)]
    reviewers: RawReviewers,
    #[serde(default)]
    words: RawWords,
}

/// The `[words]` table as written.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RawWords {
    #[serde(default)]
    hedges: Vec<String>,
    #[serde(default)]
    vague_openers: Vec<String>,
}

/// The `[reviewers]` table as written.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RawReviewers {
    #[serde(default)]
    required_in: Vec<Spanned<Slug>>,
}

impl LintConfig {
    /// Reads the text of a `lint.toml` file.
    ///
    /// # Errors
    ///
    /// Returns [`LintError::Syntax`] for text that is not TOML or holds a key
    /// or severity the linter does not know, and [`LintError::UnknownRule`]
    /// for a rule name that is not in the catalogue. Both name the line and
    /// column.
    pub fn parse(text: &str) -> Result<Self, LintError> {
        let raw: RawConfig = toml::from_str(text).map_err(|source| {
            let (line, column) = position(text, source.span().map_or(0, |span| span.start));
            LintError::Syntax {
                line,
                column,
                source,
            }
        })?;
        let catalogue = Catalogue::builtin()?;
        let mut severities = BTreeMap::new();
        for (name, severity) in raw.rules {
            if !catalogue.contains(name.get_ref()) {
                let (line, column) = position(text, name.span().start);
                return Err(LintError::UnknownRule {
                    rule: name.into_inner(),
                    line,
                    column,
                });
            }
            severities.insert(name.into_inner(), severity);
        }
        let required_in = raw.reviewers.required_in;
        let required_in_lines = (required_in.iter())
            .map(|slug| position(text, slug.span().start).0)
            .collect();
        Ok(Self {
            severities,
            thresholds: raw.thresholds,
            reviewers_required_in: required_in.into_iter().map(Spanned::into_inner).collect(),
            required_in_lines,
            unclear_words: UnclearWords::default()
                .with_own(&raw.words.hedges, &raw.words.vague_openers),
        })
    }

    /// The severity the file sets for `rule`, or `None` to keep its default.
    pub fn severity(&self, rule: RuleId) -> Option<Severity> {
        self.severities.get(rule.as_str()).copied()
    }

    /// The limits for the rules that measure, with defaults for any the file
    /// leaves out.
    pub fn thresholds(&self) -> Thresholds {
        self.thresholds
    }

    /// The slugs of the categories whose posts must name a reviewer in
    /// `reviewed_by`, from `required-in` in the `[reviewers]` table. Empty
    /// when the file sets none, so no post needs one.
    pub fn reviewers_required_in(&self) -> &[Slug] {
        &self.reviewers_required_in
    }

    /// Checks that each category in `required-in` is one of `known`, the
    /// slugs of the site's categories, so a slip such as `heath` cannot
    /// switch the reviewer rule off without a word.
    ///
    /// # Errors
    ///
    /// Returns [`LintError::UnknownCategory`] for the first category that
    /// `known` lacks, with its line and the known slugs.
    pub fn check_categories<'k>(
        &self,
        known: impl IntoIterator<Item = &'k str>,
    ) -> Result<(), LintError> {
        let known: BTreeSet<&str> = known.into_iter().collect();
        let lines = self.required_in_lines.iter().copied();
        let unknown = (self.reviewers_required_in.iter().zip(lines))
            .find(|(slug, _)| !known.contains(slug.as_str()));
        match unknown {
            None => Ok(()),
            Some((category, line)) => Err(LintError::UnknownCategory {
                category: category.clone(),
                line,
                known: known.into_iter().map(str::to_owned).collect(),
            }),
        }
    }

    /// The hedge words and vague openers to flag: the built-in ones, then
    /// the site's own from the `[words]` table.
    pub fn unclear_words(&self) -> &UnclearWords {
        &self.unclear_words
    }
}

/// The line and column, counted from 1, of the byte at `offset` in `text`.
fn position(text: &str, offset: usize) -> (usize, usize) {
    let before = text.get(..offset).unwrap_or(text);
    let line = before.matches('\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |at| at + 1);
    let column = before.get(line_start..).unwrap_or_default().chars().count() + 1;
    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_counts_lines_and_characters_from_one() {
        let text = "[rules]\nnö-date = 1\n";
        assert_eq!(position(text, 0), (1, 1));
        assert_eq!(position(text, 8), (2, 1));
        assert_eq!(position(text, 11), (2, 3), "ö is one character");
    }
}
