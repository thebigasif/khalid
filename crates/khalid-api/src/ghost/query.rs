//! The list parameters Ghost's client sends, read into the shared query
//! grammar (BL-API-5), with `include` and `fields` kept here.

use khalid_core::UserMessage;
use serde::Deserialize;
use serde_json::{Map, Value};

use super::error::GhostError;
use super::filter::{Names, translate};
use super::people::Include;
use crate::query::{Condition, ListQuery, RawListQuery};

/// The parameters Ghost's client sends. Others, such as `key` and the
/// `slug` the client repeats on a read, are left alone.
#[derive(Debug, Default, Deserialize)]
pub(super) struct GhostQuery {
    filter: Option<String>,
    include: Option<String>,
    fields: Option<String>,
    order: Option<String>,
    formats: Option<String>,
    limit: Option<String>,
    page: Option<String>,
}

impl GhostQuery {
    /// The checked list query for a resource whose filter may name the
    /// Ghost fields in `names` and whose order may name `fields`. Every
    /// condition in `scope` must hold as well, such as the site's language.
    pub(super) fn list_query(
        &self,
        names: Names,
        fields: &[&str],
        scope: Vec<Condition>,
    ) -> Result<ListQuery, GhostError> {
        let raw = RawListQuery {
            filter: translate(self.filter.as_deref(), names, scope)?,
            include: None,
            fields: None,
            order: self.order.clone(),
            formats: self.formats.clone(),
            limit: self.limit.clone(),
            page: self.page.clone(),
        };
        let query = ListQuery::try_from(&raw)?;
        match query
            .order()
            .iter()
            .find(|key| !fields.contains(&key.field()))
        {
            Some(key) => Err(GhostError::bad_request(UserMessage::new(
                format!(
                    "The order uses {}, which this API cannot sort by.",
                    key.field()
                ),
                "The list cannot be sent until it is fixed.",
                format!("Use one of these fields: {}.", fields.join(", ")),
            ))),
            None => Ok(query),
        }
    }

    /// What `include` asks for, out of `offered`: `tags` and `authors` on
    /// posts and pages, nothing elsewhere.
    pub(super) fn include(&self, offered: Include) -> Result<Include, GhostError> {
        let mut asked = Include::default();
        for name in names(self.include.as_deref()) {
            match name {
                "tags" if offered.tags => asked.tags = true,
                "authors" if offered.authors => asked.authors = true,
                _ => return Err(cannot_include(name, offered)),
            }
        }
        Ok(asked)
    }

    /// `entry` with only the fields asked for, and what `include` added.
    /// Without `fields`, the whole entry.
    pub(super) fn keep_fields(&self, entry: Map<String, Value>, include: Include) -> Value {
        let fields: Vec<&str> = names(self.fields.as_deref()).collect();
        if fields.is_empty() {
            return Value::Object(entry);
        }
        let added = |key: &str| match key {
            "tags" | "primary_tag" => include.tags,
            "authors" | "primary_author" => include.authors,
            _ => false,
        };
        let kept = entry
            .into_iter()
            .filter(|(key, _)| fields.contains(&key.as_str()) || added(key));
        Value::Object(kept.collect())
    }
}

/// The names in a comma list, trimmed, without empty ones.
fn names(text: Option<&str>) -> impl Iterator<Item = &str> {
    let names = text.unwrap_or_default().split(',').map(str::trim);
    names.filter(|name| !name.is_empty())
}

fn cannot_include(name: &str, offered: Include) -> GhostError {
    let fix = if offered.any() {
        "Ask for include=tags,authors, or leave include out."
    } else {
        "Leave include out here; tags and authors are included only on posts and pages."
    };
    GhostError::bad_request(UserMessage::new(
        format!("The include parameter asks for {name}, which this API cannot add."),
        "Khalid answers Ghost's include for tags and authors only.",
        fix,
    ))
}
