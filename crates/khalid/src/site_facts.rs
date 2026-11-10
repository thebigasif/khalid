//! The site's fact registry, from `facts.toml` in the site folder (P7), as
//! the linter reads it.
//!
//! The content reader reads the file with the posts, so every page that
//! shows a `[[fact:key]]` and every lint run agree on one registry. The
//! linter warns when a post still gives an old value, and when a post gives
//! a fact nobody has checked for six months.

use khalid_content::{Fact, FactError};
use khalid_core::UserMessage;
use khalid_lint::LintFact;

/// Every message about `facts.toml` shares this reason.
const WHY: &str = "facts.toml holds the values every post must agree on, so Khalid reads it first.";

/// Each of `facts` as the linter reads it, in the same order.
pub(crate) fn for_lint(facts: &[Fact]) -> Vec<LintFact> {
    facts
        .iter()
        .map(|fact| {
            LintFact::new(fact.key.as_str(), fact.value.as_str(), fact.checked)
                .with_old(fact.old.clone())
                .with_source(fact.source.as_ref().map(|url| url.as_str().to_owned()))
        })
        .collect()
}

/// What a person reads when `facts.toml` is not a fact registry.
pub(crate) fn message(error: &FactError) -> UserMessage {
    match error {
        FactError::NotToml { line, detail } => UserMessage::new(
            format!("facts.toml has a mistake on line {line}: {detail}."),
            WHY,
            "Write each fact as a table with a value and a checked date, such as [pro-plan-price].",
        ),
        FactError::BadKey(key) => UserMessage::new(
            format!("facts.toml names a fact `{key}`, which is not a usable name."),
            WHY,
            "Rename it with lowercase letters, digits and hyphens, such as [pro-plan-price].",
        ),
        FactError::NoValue(key) => UserMessage::new(
            format!("The fact `{key}` in facts.toml has an empty value."),
            WHY,
            format!(
                "Write the value posts should give in [{key}], such as value = \"29 USD a month\"."
            ),
        ),
        FactError::NotADate(key) => UserMessage::new(
            format!("The fact `{key}` in facts.toml has a checked date that is not a plain date."),
            WHY,
            format!("Write the day you last checked it in [{key}], such as checked = 2026-09-01."),
        ),
        FactError::BadSource(key) => UserMessage::new(
            format!("The fact `{key}` in facts.toml has a source that is not a web address."),
            WHY,
            format!(
                "Write the full address of the page that shows the value in [{key}], such as source = \"https://example.com/pricing/\", or remove it."
            ),
        ),
        _ => UserMessage::new(
            "Khalid could not read facts.toml in the site folder.",
            WHY,
            "Check that facts.toml is a file you can open, or remove it.",
        ),
    }
}
