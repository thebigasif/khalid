//! The site's fact registry, `facts.toml` in the site folder (GEO P7).
//!
//! Each table is one fact posts repeat, such as a price, named by its key:
//!
//! ```toml
//! [pro-plan-price]
//! value = "29 USD a month"
//! old = ["19 USD a month"]
//! checked = 2026-09-01
//! source = "https://example.com/pricing/"
//! ```
//!
//! A post writes `[[fact:pro-plan-price]]` to show the current value, and
//! the linter warns when a post still gives an old value or a fact nobody
//! has checked for a long time.

use std::collections::BTreeMap;

use chrono::NaiveDate;
use khalid_core::{AbsoluteUrl, Slug};
use serde::Deserialize;
use toml::value::Datetime;

/// The file's name in the site folder.
pub const FACTS_FILE: &str = "facts.toml";

/// One fact from the registry: the value posts give, the values it held
/// before, the day someone last checked it and where they checked it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Fact {
    /// The fact's name, such as `pro-plan-price`.
    pub key: Slug,
    /// The value as posts should give it, such as "29 USD a month".
    pub value: String,
    /// Values the fact held before, without blanks, in the file's order.
    pub old: Vec<String>,
    /// The day someone last made sure the value still holds.
    pub checked: NaiveDate,
    /// The page that shows the value, when the file names one.
    pub source: Option<AbsoluteUrl>,
}

/// Why `facts.toml` could not be read as a fact registry.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum FactError {
    /// The file is not TOML, or a fact misses a part or has one it should not.
    #[error("line {line}: {detail}")]
    NotToml {
        /// The 1-based line the mistake is on.
        line: usize,
        /// What the TOML reader found, in a phrase.
        detail: String,
    },
    /// A fact's key is not lowercase words joined by hyphens.
    #[error("the fact `{0}` is not a usable name")]
    BadKey(String),
    /// A fact's value is blank.
    #[error("the fact `{0}` has an empty value")]
    NoValue(String),
    /// A fact's `checked` is not a plain date.
    #[error("the fact `{0}` has a checked date that is not a plain date")]
    NotADate(String),
    /// A fact's `source` is not a full `http` or `https` address.
    #[error("the fact `{0}` has a source that is not a web address")]
    BadSource(String),
}

/// One fact as written, under its key.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    value: String,
    #[serde(default)]
    old: Vec<String>,
    checked: Datetime,
    #[serde(default)]
    source: Option<String>,
}

/// The facts in `text`, the contents of a `facts.toml`, sorted by key.
///
/// # Errors
///
/// Returns a [`FactError`] when `text` is not TOML or holds a fact with an
/// unknown part, a key that is not lowercase words and hyphens, an empty
/// value, a `checked` that is not a plain date, or a `source` that is not
/// a web address.
pub fn parse_facts(text: &str) -> Result<Vec<Fact>, FactError> {
    let entries: BTreeMap<String, Entry> =
        toml::from_str(text).map_err(|error| FactError::NotToml {
            line: error
                .span()
                .and_then(|span| text.get(..span.start))
                .map_or(1, |before| before.matches('\n').count().saturating_add(1)),
            detail: error.message().trim_end_matches('.').to_owned(),
        })?;
    entries
        .into_iter()
        .map(|(key, entry)| fact(key, &entry))
        .collect()
}

/// The fact `key` once its parts are checked.
fn fact(key: String, entry: &Entry) -> Result<Fact, FactError> {
    let Ok(slug) = Slug::try_new(&key) else {
        return Err(FactError::BadKey(key));
    };
    let value = entry.value.trim();
    if value.is_empty() {
        return Err(FactError::NoValue(key));
    }
    let Some(checked) = plain_date(&entry.checked) else {
        return Err(FactError::NotADate(key));
    };
    let source = match entry.source.as_deref().map(str::trim) {
        None | Some("") => None,
        Some(source) => match AbsoluteUrl::parse(source) {
            Ok(url) => Some(url),
            Err(_) => return Err(FactError::BadSource(key)),
        },
    };
    let old = entry
        .old
        .iter()
        .map(|old| old.trim())
        .filter(|old| !old.is_empty());
    Ok(Fact {
        key: slug,
        value: value.to_owned(),
        old: old.map(str::to_owned).collect(),
        checked,
        source,
    })
}

/// The day `datetime` names, when it is a date with no time of day.
fn plain_date(datetime: &Datetime) -> Option<NaiveDate> {
    let date = datetime.date.filter(|_| datetime.time.is_none())?;
    let year = i32::from(date.year);
    NaiveDate::from_ymd_opt(year, u32::from(date.month), u32::from(date.day))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn facts_are_read_sorted_by_key_with_old_values_trimmed() {
        let text = "[pro-plan-price]\nvalue = \" 29 USD a month \"\n\
            old = [\"19 USD a month \", \"\"]\nchecked = 2026-09-01\n\
            source = \"https://example.com/pricing/\"\n\n\
            [founded]\nvalue = \"2019\"\nchecked = 2026-06-01\n";
        let facts = parse_facts(text).unwrap();
        let keys: Vec<&str> = facts.iter().map(|fact| fact.key.as_str()).collect();
        assert_eq!(keys, ["founded", "pro-plan-price"]);
        let price = facts.get(1).unwrap();
        assert_eq!(price.value, "29 USD a month");
        assert_eq!(price.old, ["19 USD a month"]);
        assert_eq!(price.checked, day(2026, 9, 1));
        let source = price.source.as_ref().map(AbsoluteUrl::as_str);
        assert_eq!(source, Some("https://example.com/pricing/"));
        assert_eq!(facts.first().unwrap().source, None);
    }

    #[test]
    fn an_empty_file_has_no_facts() {
        assert_eq!(parse_facts("").unwrap(), Vec::new());
    }

    #[test]
    fn a_fact_without_a_checked_date_is_refused_with_its_line() {
        let error = parse_facts("\n[price]\nvalue = \"29 USD\"\n").unwrap_err();
        assert!(
            matches!(error, FactError::NotToml { line: 2, .. }),
            "{error}"
        );
    }

    #[test]
    fn a_key_that_is_not_a_usable_name_is_refused() {
        let error = parse_facts("[\"Pro Price\"]\nvalue = \"29\"\nchecked = 2026-09-01\n");
        assert_eq!(error, Err(FactError::BadKey("Pro Price".to_owned())));
    }

    #[test]
    fn an_empty_value_is_refused() {
        let error = parse_facts("[price]\nvalue = \"  \"\nchecked = 2026-09-01\n");
        assert_eq!(error, Err(FactError::NoValue("price".to_owned())));
    }

    #[test]
    fn a_checked_time_or_text_is_refused() {
        let with_time = "[price]\nvalue = \"29\"\nchecked = 2026-09-01T10:00:00\n";
        let error = parse_facts(with_time);
        assert_eq!(error, Err(FactError::NotADate("price".to_owned())));
        let as_text = "[price]\nvalue = \"29\"\nchecked = \"2026-09-01\"\n";
        assert!(
            parse_facts(as_text).is_err(),
            "a quoted date is text, not a date"
        );
    }

    #[test]
    fn a_source_that_is_not_a_web_address_is_refused() {
        let text = "[price]\nvalue = \"29\"\nchecked = 2026-09-01\nsource = \"our wiki\"\n";
        let error = parse_facts(text);
        assert_eq!(error, Err(FactError::BadSource("price".to_owned())));
    }

    #[test]
    fn an_unknown_part_in_a_fact_is_refused() {
        let text = "[price]\nvalue = \"29\"\nchecked = 2026-09-01\nsources = \"x\"\n";
        let error = parse_facts(text).unwrap_err();
        assert!(
            matches!(error, FactError::NotToml { line: 4, .. }),
            "{error}"
        );
    }
}
