//! Whether a plain value, such as text, a number or `true`, is what its
//! property expects: a real date, a number, a web address, an allowed
//! enumeration value, a length of time or a currency code.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

use super::tree::{Json, either};
use super::{FindingLevel, FixTarget, SchemaFinding, SchemaFix, SchemaReport, finding};
use crate::schema::vocabulary::{Names, Vocabulary, slip_for};

/// Types whose values are any text.
const ANY_TEXT: [&str; 4] = ["Text", "PronounceableText", "CssSelectorType", "XPathType"];

/// schema.org's data types. schema.org marks them as instances of
/// `DataType`, not as types that extend it, so the vocabulary cannot tell.
const DATA_TYPES: [&str; 12] = [
    "Boolean",
    "CssSelectorType",
    "Date",
    "DateTime",
    "Float",
    "Integer",
    "Number",
    "PronounceableText",
    "Text",
    "Time",
    "URL",
    "XPathType",
];

/// The schema.org prefixes an enumeration value may carry.
const PREFIXES: [&str; 2] = ["https://schema.org/", "http://schema.org/"];

/// A finding when `value`, at `at` for property `name`, is not one of
/// `ranges`. A property schema.org gives no range takes anything.
pub(super) fn check_value(
    value: &Json,
    at: &str,
    name: &str,
    ranges: &[&str],
) -> Option<SchemaFinding> {
    if ranges.is_empty() {
        return None;
    }
    match value {
        Json::Null => Some(wrong(
            at,
            format!("{name} is empty."),
            "Leave the property out instead.".to_owned(),
        )),
        Json::Bool(_) => (!takes(ranges, "Boolean")).then(|| expects(at, name, ranges)),
        Json::Number(number) => (!number_fits(number, ranges)).then(|| expects(at, name, ranges)),
        Json::Text(text) => text_finding(text.trim(), at, name, ranges),
        Json::List(_) | Json::Object(_) => None,
    }
}

/// Whether one of `ranges` is `kind` or extends it.
fn takes(ranges: &[&str], kind: &str) -> bool {
    let vocabulary = Vocabulary::get();
    ranges.iter().any(|range| vocabulary.is_a(range, kind))
}

fn number_fits(number: &serde_json::Number, ranges: &[&str]) -> bool {
    let whole = number.is_i64() || number.is_u64();
    ranges
        .iter()
        .any(|range| matches!(*range, "Number" | "Float") || (*range == "Integer" && whole))
        || ranges.iter().any(|range| ANY_TEXT.contains(range))
        || ranges.iter().any(|range| takes_a_bare_number(range))
}

/// Whether a plain number is a fair value of `range`: a quantity such as a
/// `Distance`, as image `width` and `height` are written, but not a
/// `Duration`, which has its own format.
fn takes_a_bare_number(range: &str) -> bool {
    let vocabulary = Vocabulary::get();
    range == "QuantitativeValue" || (vocabulary.is_a(range, "Quantity") && range != "Duration")
}

/// The finding for `text`, or `None` when one of `ranges` takes it.
fn text_finding(text: &str, at: &str, name: &str, ranges: &[&str]) -> Option<SchemaFinding> {
    let vocabulary = Vocabulary::get();
    if ranges.iter().any(|range| ANY_TEXT.contains(range)) {
        return currency(text, at, name);
    }
    let enumerations: Vec<&str> = ranges
        .iter()
        .copied()
        .filter(|range| vocabulary.is_a(range, "Enumeration"))
        .collect();
    let bare = PREFIXES
        .iter()
        .find_map(|prefix| text.strip_prefix(prefix))
        .unwrap_or(text);
    let member = |kind: &str| {
        vocabulary
            .enumeration_of(bare)
            .is_some_and(|of| vocabulary.is_a(of, kind))
    };
    let accepted = enumerations.iter().any(|kind| member(kind))
        || ranges.iter().any(|range| text_fits(text, range));
    if accepted {
        return duration(text, at, name, ranges);
    }
    if enumerations.is_empty() {
        return Some(expects(at, name, ranges));
    }
    Some(not_a_member(text, bare, at, name, &enumerations))
}

/// Whether `text` is a value of the data or thing type `range`. Text for a
/// thing, such as a person's name for `author`, is allowed: schema.org
/// takes text where it expects a thing.
fn text_fits(text: &str, range: &str) -> bool {
    match range {
        "URL" => is_web_address(text),
        "Date" => is_date(text),
        "DateTime" => is_date_time(text),
        "Time" => is_time(text),
        "Number" | "Float" => text.parse::<f64>().is_ok_and(f64::is_finite),
        "Integer" => text.parse::<i64>().is_ok(),
        "Boolean" => matches!(bare_boolean(text), "True" | "False" | "true" | "false"),
        _ => {
            let vocabulary = Vocabulary::get();
            !DATA_TYPES.contains(&range) && !vocabulary.is_a(range, "Enumeration")
        }
    }
}

fn bare_boolean(text: &str) -> &str {
    PREFIXES
        .iter()
        .find_map(|prefix| text.strip_prefix(prefix))
        .unwrap_or(text)
}

fn is_web_address(text: &str) -> bool {
    text.starts_with("https://") || text.starts_with("http://") || text.starts_with("//")
}

/// A day in ISO 8601: `2026-11-12`, or a month or year alone.
fn is_date(text: &str) -> bool {
    NaiveDate::parse_from_str(text, "%Y-%m-%d").is_ok()
        || NaiveDate::parse_from_str(&format!("{text}-01"), "%Y-%m-%d").is_ok()
        || (text.len() == 4 && text.bytes().all(|b| b.is_ascii_digit()))
}

/// A moment in ISO 8601, with or without its offset and seconds.
fn is_date_time(text: &str) -> bool {
    DateTime::parse_from_rfc3339(text).is_ok()
        || NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M:%S").is_ok()
        || NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").is_ok()
        || DateTime::parse_from_str(text, "%Y-%m-%dT%H:%M%:z").is_ok()
}

/// A time of day, such as `09:00` or `09:00:00+01:00`.
fn is_time(text: &str) -> bool {
    let clock = text.split_once(['+', 'Z']).map_or(text, |(clock, _)| clock);
    let clock = match clock.rsplit_once('-') {
        Some((clock, _)) if clock.contains(':') => clock,
        _ => clock,
    };
    NaiveTime::parse_from_str(clock, "%H:%M:%S").is_ok()
        || NaiveTime::parse_from_str(clock, "%H:%M").is_ok()
}

/// A length of time not written as ISO 8601, such as `20 minutes` for
/// `cookTime`, which takes a `Duration`.
fn duration(text: &str, at: &str, name: &str, ranges: &[&str]) -> Option<SchemaFinding> {
    let only_duration = ranges.iter().all(|range| *range == "Duration");
    let iso = text.starts_with('P') && text.len() > 1;
    (only_duration && !iso).then(|| {
        wrong(
            at,
            format!("{name} is \"{text}\", not a length of time in ISO 8601."),
            format!("Write {name} as ISO 8601, such as PT20M for 20 minutes or PT1H30M for an hour and a half."),
        )
    })
}

/// A currency that is not a three-letter ISO 4217 code, such as `$` or
/// `usd`.
fn currency(text: &str, at: &str, name: &str) -> Option<SchemaFinding> {
    if name != "priceCurrency" && name != "currency" {
        return None;
    }
    let code = text.len() == 3 && text.bytes().all(|b| b.is_ascii_uppercase());
    if code {
        return None;
    }
    let upper = text.to_ascii_uppercase();
    let mut found = wrong(
        at,
        format!("{name} is \"{text}\", not a three-letter currency code."),
        format!("Write {name} as an ISO 4217 code, such as USD, EUR or GBP."),
    );
    let fixable = upper.len() == 3 && upper.bytes().all(|b| b.is_ascii_uppercase());
    found.fix = fixable.then(|| SchemaFix {
        target: FixTarget::Value,
        path: at.to_owned(),
        from: text.to_owned(),
        to: upper,
    });
    Some(found)
}

/// `text` is not a member of any of `enumerations`; the fix names the
/// member it is a slip for, if any.
fn not_a_member(
    text: &str,
    bare: &str,
    at: &str,
    name: &str,
    enumerations: &[&str],
) -> SchemaFinding {
    let vocabulary = Vocabulary::get();
    let meant = slip_for(bare, Names::Types)
        .or_else(|| near_member(bare, enumerations))
        .filter(|meant| {
            enumerations.iter().any(|kind| {
                vocabulary
                    .enumeration_of(meant)
                    .is_some_and(|of| vocabulary.is_a(of, kind))
            })
        });
    let examples: Vec<&str> = enumerations
        .iter()
        .flat_map(|kind| vocabulary.members(kind))
        .take(3)
        .collect();
    let mut found = wrong(
        at,
        format!(
            "{name} is \"{text}\", which is not one of the values {} allows.",
            either(enumerations)
        ),
        format!(
            "Use one of the values schema.org lists, such as https://schema.org/{}.",
            examples.join(" or https://schema.org/")
        ),
    );
    found.fix = meant.map(|meant| SchemaFix {
        target: FixTarget::Value,
        path: at.to_owned(),
        from: text.to_owned(),
        to: format!("https://schema.org/{meant}"),
    });
    found
}

/// The member of `enumerations` that `text` names in other capitals or
/// with spaces, such as `in stock` for `InStock`.
fn near_member(text: &str, enumerations: &[&str]) -> Option<&'static str> {
    let squeezed: String = text
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_ascii_lowercase();
    enumerations
        .iter()
        .flat_map(|kind| Vocabulary::get().members(kind))
        .find(|member| member.to_ascii_lowercase() == squeezed)
}

fn expects(at: &str, name: &str, ranges: &[&str]) -> SchemaFinding {
    wrong(
        at,
        format!("{name} is not written as {}.", either(ranges)),
        format!("Write {name} as {}, as schema.org expects.", either(ranges)),
    )
}

fn wrong(at: &str, what: String, fix: String) -> SchemaFinding {
    finding(
        (SchemaReport::SchemaOrg, FindingLevel::Error),
        at,
        (
            what,
            "Search engines skip a value that is not what the property expects.",
            fix,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{is_date, is_date_time, is_time};

    #[test]
    fn dates_times_and_moments_follow_iso_8601() {
        assert!(is_date("2026-11-12") && is_date("2026-11") && is_date("2026"));
        assert!(!is_date("12/11/2026") && !is_date("2026-13-01"));
        assert!(is_date_time("2026-11-12T09:30:00Z") && is_date_time("2026-11-12T09:30"));
        assert!(is_date_time("2026-11-12T09:30:00+01:00"));
        assert!(!is_date_time("next Tuesday"));
        assert!(is_time("09:00") && is_time("09:00:00+01:00") && is_time("17:30:00-05:00"));
        assert!(!is_time("9am"));
    }
}
