//! `fields` on the Content API's lists: each entry keeps only the fields
//! asked for (RULES.md BL-API-5). The names a resource offers are the
//! properties of its own `OpenAPI` schema, so the check and the document
//! cannot drift apart.

use std::collections::BTreeSet;
use std::sync::Arc;

use khalid_core::UserMessage;
use serde::ser::Error as _;
use serde::{Serialize, Serializer};
use serde_json::Value;
use utoipa::ToSchema;
use utoipa::openapi::{RefOr, Schema};

use crate::problem::{Problem, ProblemType};

/// One entry as sent: whole, or cut down to the fields asked for.
#[derive(Debug)]
pub(crate) struct Sparse<T> {
    entry: T,
    /// The only top-level fields to send; empty sends them all.
    fields: Arc<[String]>,
}

impl<T> Sparse<T> {
    /// `entry` with every field.
    pub(crate) fn whole(entry: T) -> Self {
        Self {
            entry,
            fields: Arc::from([]),
        }
    }
}

impl<T: Serialize> Serialize for Sparse<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.fields.is_empty() {
            return self.entry.serialize(serializer);
        }
        match serde_json::to_value(&self.entry).map_err(S::Error::custom)? {
            Value::Object(entry) => {
                let kept = entry
                    .into_iter()
                    .filter(|(key, _)| self.fields.contains(key));
                serializer.collect_map(kept)
            }
            other => other.serialize(serializer),
        }
    }
}

/// `entries`, each cut down to `fields` once they are checked against
/// what `T` offers.
///
/// # Errors
///
/// A 400 problem naming the first field `T` does not have, with the
/// fields it does have as the fix.
pub(crate) fn keep_fields<T: ToSchema>(
    entries: Vec<T>,
    fields: &[String],
) -> Result<Vec<Sparse<T>>, Problem> {
    if !fields.is_empty() {
        let offered = offered::<T>();
        if let Some(unknown) = fields.iter().find(|field| !offered.contains(*field)) {
            let offered: Vec<&str> = offered.iter().map(String::as_str).collect();
            let message = UserMessage::new(
                format!("The fields parameter names {unknown}, which is not a field here."),
                "The list cannot be sent until it is fixed.",
                format!("Use one of these fields: {}.", offered.join(", ")),
            );
            return Err(Problem::new(ProblemType::InvalidQuery, &message).with_rule("fields"));
        }
    }
    let fields: Arc<[String]> = Arc::from(fields);
    let sparse = entries.into_iter().map(|entry| Sparse {
        entry,
        fields: Arc::clone(&fields),
    });
    Ok(sparse.collect())
}

/// The top-level fields `T` can send, from its schema and the parts it
/// flattens in, in name order.
fn offered<T: ToSchema>() -> BTreeSet<String> {
    let mut parts = Vec::new();
    T::schemas(&mut parts);
    let mut names = BTreeSet::new();
    collect(&T::schema(), &parts, &mut names);
    names
}

/// Adds the properties of `schema` to `names`, following an `allOf` and a
/// reference to one of `parts`, but not into the properties themselves.
fn collect(
    schema: &RefOr<Schema>,
    parts: &[(String, RefOr<Schema>)],
    names: &mut BTreeSet<String>,
) {
    match schema {
        RefOr::T(Schema::Object(object)) => names.extend(object.properties.keys().cloned()),
        RefOr::T(Schema::AllOf(all)) => {
            for item in &all.items {
                collect(item, parts, names);
            }
        }
        RefOr::Ref(reference) => {
            let name = reference.ref_location.rsplit('/').next();
            let part = parts.iter().find(|(part, _)| Some(part.as_str()) == name);
            if let Some((_, schema)) = part {
                collect(schema, parts, names);
            }
        }
        RefOr::T(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{Post, Term};

    #[test]
    fn a_post_offers_its_own_fields_and_its_flattened_bodies() {
        let offered = offered::<Post>();
        for name in [
            "id",
            "slug",
            "title",
            "head",
            "series",
            "html",
            "markdown",
            "plaintext",
        ] {
            assert!(offered.contains(name), "{name}: {offered:?}");
        }
        assert!(!offered.contains("bodies"), "{offered:?}");
    }

    #[test]
    fn a_term_offers_only_its_own_fields() {
        let offered = offered::<Term>();
        assert!(
            offered.contains("name") && !offered.contains("html"),
            "{offered:?}"
        );
    }
}
