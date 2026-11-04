//! The common part of Ghost's filter language (KTD13): Ghost's field names
//! read as Khalid's, joined with `+` and `,`, with `-` for not, `[a,b]`
//! lists, and `>` or `<` on dates. Anything else is Ghost's 400, naming
//! the part this API does not answer.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, SecondsFormat, Utc};
use khalid_core::UserMessage;

use super::error::GhostError;
use crate::query::{Condition, Filter, Literal, ParseFilterError, Predicate};

/// Each Ghost field a filter may name, and the field it is read as.
pub(super) type Names = &'static [(&'static str, &'static str)];

/// The fields of posts and pages.
pub(super) const POSTS: Names = &[
    ("id", "id"),
    ("slug", "slug"),
    ("tag", "tag"),
    ("tags", "tag"),
    ("tags.slug", "tag"),
    ("author", "author"),
    ("authors", "author"),
    ("authors.slug", "author"),
    ("primary_tag", "primary_tag"),
    ("primary_tag.slug", "primary_tag"),
    ("primary_author", "primary_author"),
    ("primary_author.slug", "primary_author"),
    ("featured", "featured"),
    ("status", "status"),
    ("visibility", "visibility"),
    ("published_at", "published_at"),
];

/// The fields of tags.
pub(super) const TAGS: Names = &[
    ("id", "id"),
    ("slug", "slug"),
    ("name", "name"),
    ("visibility", "visibility"),
];

/// The fields of authors.
pub(super) const AUTHORS: Names = &[("id", "id"), ("slug", "slug"), ("name", "name")];

/// The one field `>` and `<` compare.
const DATE: &str = "published_at";

/// Ghost's filter `text` written with Khalid's field names, for a resource
/// whose fields are `names`.
///
/// # Errors
///
/// Returns Ghost's 400 when the filter cannot be read or uses a part
/// outside the common one.
pub(super) fn translate(text: &str, names: Names) -> Result<String, GhostError> {
    let filter: Filter = text.parse().map_err(|error| cannot_read(&error))?;
    Ok(rename(filter, names)?.to_string())
}

fn rename(filter: Filter, names: Names) -> Result<Filter, GhostError> {
    let all = |parts: Vec<Filter>| -> Result<Vec<Filter>, GhostError> {
        parts.into_iter().map(|part| rename(part, names)).collect()
    };
    match filter {
        Filter::And(parts) => all(parts).map(Filter::And),
        Filter::Or(parts) => all(parts).map(Filter::Or),
        Filter::Condition(condition) => read_as(&condition, names).map(Filter::Condition),
    }
}

/// `condition` with Khalid's field name, its dates written as Khalid
/// stores them.
fn read_as(condition: &Condition, names: Names) -> Result<Condition, GhostError> {
    let ghost = condition.field();
    let Some((_, field)) = names.iter().find(|(name, _)| *name == ghost) else {
        return Err(outside(&format!("the field {ghost}"), names));
    };
    let dated = *field == DATE;
    let value = |literal: &Literal| -> Result<Literal, GhostError> {
        if !dated {
            return Ok(literal.clone());
        }
        match literal {
            Literal::Text(text) => {
                date(text).ok_or_else(|| outside(&format!("the date '{text}'"), names))
            }
            _ => Err(outside(&format!("{literal} as a date"), names)),
        }
    };
    let values = |list: &[Literal]| list.iter().map(value).collect::<Result<Vec<_>, _>>();
    let predicate = match condition.predicate() {
        Predicate::Is(literal) => Predicate::Is(value(literal)?),
        Predicate::IsNot(literal) => Predicate::IsNot(value(literal)?),
        Predicate::In(list) => Predicate::In(values(list)?),
        Predicate::NotIn(list) => Predicate::NotIn(values(list)?),
        Predicate::Greater(literal) if dated => Predicate::Greater(value(literal)?),
        Predicate::GreaterOrEqual(literal) if dated => Predicate::GreaterOrEqual(value(literal)?),
        Predicate::Less(literal) if dated => Predicate::Less(value(literal)?),
        Predicate::LessOrEqual(literal) if dated => Predicate::LessOrEqual(value(literal)?),
        Predicate::Greater(_)
        | Predicate::GreaterOrEqual(_)
        | Predicate::Less(_)
        | Predicate::LessOrEqual(_) => {
            return Err(outside(&format!("{ghost} with > or <"), names));
        }
        _ => return Err(outside(&format!("~ text matching on {ghost}"), names)),
    };
    Ok(Condition::new(field, predicate))
}

/// `text`, a date in one of Ghost's forms, as Khalid stores times: UTC to
/// the second. A day alone is its first second.
fn date(text: &str) -> Option<Literal> {
    let at = DateTime::parse_from_rfc3339(text)
        .map(|at| at.with_timezone(&Utc))
        .or_else(|_| {
            NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S").map(|at| at.and_utc())
        })
        .or_else(|_| {
            NaiveDate::parse_from_str(text, "%Y-%m-%d")
                .map(|day| day.and_time(NaiveTime::MIN).and_utc())
        })
        .ok()?;
    Some(Literal::Text(at.to_rfc3339_opts(SecondsFormat::Secs, true)))
}

fn cannot_read(error: &ParseFilterError) -> GhostError {
    GhostError::bad_request(UserMessage::new(
        "The filter could not be read.",
        format!("In this request, {error}."),
        "Write each condition as field:value, for example tag:news, and join them with + or a comma.",
    ))
}

/// Ghost's 400 for `part` of a filter on a resource with `names`.
fn outside(part: &str, names: Names) -> GhostError {
    let fields: Vec<&str> = names.iter().map(|(name, _)| *name).collect();
    let dates = if fields.contains(&DATE) {
        ", and > or < on published_at only"
    } else {
        ""
    };
    GhostError::bad_request(UserMessage::new(
        format!("The filter uses {part}, which this API does not answer."),
        "Khalid answers the common part of Ghost's filter language, not all of it.",
        format!(
            "Use {}, with - for not and [a,b] lists{dates}.",
            fields.join(", ")
        ),
    ))
}
