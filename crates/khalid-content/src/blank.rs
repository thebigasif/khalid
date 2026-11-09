//! A blank field in front matter, such as `tags:` with nothing after it or
//! `tags: ~`, means an empty list.
//!
//! The YAML reader, serde-saphyr, refuses null for a list since 2.1 (its
//! #233), so a site whose files say `tags:` would stop building. Every list
//! field read from front matter goes through [`or_empty`].

use serde::{Deserialize, Deserializer};

/// Reads a value, or its empty default when the field is blank or `~`.
///
/// Use it as `#[serde(default, deserialize_with = "crate::blank::or_empty")]`
/// on a list field, or inside a field's own reader before its checks.
pub(crate) fn or_empty<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}
