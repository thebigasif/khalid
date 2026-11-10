//! Case studies and changelog entries (plan section 5.13), and the rules
//! on the fields each one needs.

use chrono::NaiveDate;
use khalid_core::{Locale, Slug};
use serde::Deserialize;

use crate::{EntryRule, Hit, RuleId, Severity};

/// A case study or a changelog entry, with the fields its rules read.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LintEntry {
    /// The entry's language.
    pub locale: Locale,
    /// The last part of the entry's address.
    pub slug: Slug,
    /// The day the entry is dated, when it has one.
    #[serde(default)]
    pub date: Option<NaiveDate>,
    /// Which collection the entry is in, with that collection's fields.
    pub kind: EntryKind,
}

impl LintEntry {
    /// The entry `slug` in `locale`, dated `date`, of `kind`.
    pub fn new(locale: Locale, slug: Slug, date: Option<NaiveDate>, kind: EntryKind) -> Self {
        Self {
            locale,
            slug,
            date,
            kind,
        }
    }
}

/// The collection an entry is in, with the fields only that collection has.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
#[non_exhaustive]
pub enum EntryKind {
    /// A case study, in `case-studies/`.
    CaseStudy {
        /// The client the study is about.
        #[serde(default)]
        client: Option<String>,
        /// The result, in one line.
        #[serde(default)]
        outcome: Option<String>,
    },
    /// A changelog entry, in `changelog/`.
    Changelog {
        /// The version it shipped in, such as `2.4.0`.
        #[serde(default)]
        version: Option<String>,
    },
}

/// A case study with no client, no outcome or no date, one hit per field
/// it leaves out. Without them it is a story, not proof.
#[derive(Debug)]
pub(crate) struct CaseStudyIncomplete;

impl EntryRule for CaseStudyIncomplete {
    fn id(&self) -> RuleId {
        RuleId::new("case-study-incomplete")
    }

    fn default_severity(&self) -> Severity {
        Severity::Block
    }

    fn check(&self, entry: &LintEntry) -> Vec<Hit> {
        let EntryKind::CaseStudy { client, outcome } = &entry.kind else {
            return Vec::new();
        };
        let fields = [
            ("client", given(client.as_deref())),
            ("outcome", given(outcome.as_deref())),
            ("date", entry.date.is_some()),
        ];
        (fields.into_iter())
            .filter(|(_, present)| !present)
            .map(|(field, _)| Hit::about(field))
            .collect()
    }
}

/// A changelog entry with no date. The date also gives the month the list
/// groups it under, so with a date an entry needs no version.
#[derive(Debug)]
pub(crate) struct ChangelogEntryUndated;

impl EntryRule for ChangelogEntryUndated {
    fn id(&self) -> RuleId {
        RuleId::new("changelog-entry-undated")
    }

    fn default_severity(&self) -> Severity {
        Severity::Block
    }

    fn check(&self, entry: &LintEntry) -> Vec<Hit> {
        match entry.kind {
            EntryKind::Changelog { .. } if entry.date.is_none() => vec![Hit::post()],
            _ => Vec::new(),
        }
    }
}

/// Whether `text` holds more than spaces.
fn given(text: Option<&str>) -> bool {
    text.is_some_and(|text| !text.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(date: Option<NaiveDate>, kind: EntryKind) -> LintEntry {
        let locale = Locale::try_new("en").unwrap();
        LintEntry::new(locale, Slug::try_new("a").unwrap(), date, kind)
    }

    fn subjects(hits: &[Hit]) -> Vec<&str> {
        hits.iter().filter_map(Hit::subject).collect()
    }

    #[test]
    fn a_case_study_names_each_field_it_leaves_out() {
        let study = |client: Option<&str>, outcome: Option<&str>, date| {
            let kind = EntryKind::CaseStudy {
                client: client.map(str::to_owned),
                outcome: outcome.map(str::to_owned),
            };
            CaseStudyIncomplete.check(&entry(date, kind))
        };
        let day = NaiveDate::from_ymd_opt(2026, 10, 9);
        assert_eq!(study(Some("Harbour Tea"), Some("Doubled."), day), []);
        let hits = study(None, Some(" "), None);
        assert_eq!(subjects(&hits), ["client", "outcome", "date"]);
        assert_eq!(subjects(&study(Some("A"), None, day)), ["outcome"]);
    }

    #[test]
    fn a_changelog_entry_needs_a_date_but_not_a_version() {
        let change = |version: Option<&str>, date| {
            let version = version.map(str::to_owned);
            ChangelogEntryUndated.check(&entry(date, EntryKind::Changelog { version }))
        };
        let day = NaiveDate::from_ymd_opt(2026, 11, 1);
        assert_eq!(change(None, day), []);
        assert_eq!(change(Some("2.4"), day), []);
        assert_eq!(change(Some("2.4"), None), vec![Hit::post()]);
    }

    #[test]
    fn each_rule_leaves_the_other_collection_alone() {
        let study = EntryKind::CaseStudy {
            client: None,
            outcome: None,
        };
        let change = EntryKind::Changelog { version: None };
        assert_eq!(ChangelogEntryUndated.check(&entry(None, study)), []);
        assert_eq!(CaseStudyIncomplete.check(&entry(None, change)), []);
    }
}
