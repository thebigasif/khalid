//! Tables of parts in `site.toml`, such as `[seo]` and `[privacy]`, for
//! the Admin API's settings: read as JSON, and changed one part at a time
//! in place, so every other table and comment in the file stays as
//! written. `SiteSettings` checks the whole file as `khalid serve` does at
//! start before anything is saved, so a part with an unknown key or a
//! wrong value is refused there.

use std::collections::BTreeMap;

use khalid_api::entry_store::EntryStoreError;
use khalid_api::settings_store::SettingsError;
use serde_json::{Map, Value};
use toml_edit::{DocumentMut, Item, Table};

/// The table `name` in `text`, as JSON, without defaults filled in. Empty
/// when the file has none.
///
/// # Errors
///
/// Returns [`EntryStoreError`] when `text` is not TOML.
pub(crate) fn table(text: &str, name: &str) -> Result<Map<String, Value>, EntryStoreError> {
    let file: toml::Table = text.parse().map_err(EntryStoreError::new)?;
    let Some(table) = file.get(name) else {
        return Ok(Map::new());
    };
    match serde_json::to_value(table).map_err(EntryStoreError::new)? {
        Value::Object(parts) => Ok(parts),
        _ => Ok(Map::new()),
    }
}

/// Puts each part of `parts` into the table `name` of `document` in place
/// of the part of that name, and removes each part sent as `None`.
///
/// # Errors
///
/// Returns [`SettingsError::Refused`] naming the part when it cannot be
/// written as TOML, such as a list holding `null`.
pub(crate) fn set_parts(
    document: &mut DocumentMut,
    name: &str,
    parts: BTreeMap<String, Option<Value>>,
) -> Result<(), SettingsError> {
    let whole = document.entry(name).or_insert_with(|| {
        let mut table = Table::new();
        table.set_implicit(true);
        Item::Table(table)
    });
    if !whole.is_table_like() {
        *whole = Item::Table(Table::new());
    }
    for (part, sent) in parts {
        let Some(table) = whole.as_table_like_mut() else {
            break;
        };
        match sent {
            None => {
                table.remove(&part);
            }
            Some(value) => {
                let mut item = item(&format!("{name}.{part}"), &part, value)?;
                // A part sent again keeps the comment above its old header.
                let old = table.get(&part).and_then(Item::as_table);
                if let (Some(old), Item::Table(new)) = (old, &mut item) {
                    *new.decor_mut() = old.decor().clone();
                }
                table.insert(&part, item);
            }
        }
    }
    Ok(())
}

/// `value` as the TOML item at `key`, refused as `field` when TOML cannot
/// hold it.
pub(crate) fn item(field: &str, key: &str, value: Value) -> Result<Item, SettingsError> {
    let refuse = |problem: String| SettingsError::Refused {
        field: field.to_owned(),
        problem,
    };
    let value = toml::Value::try_from(without_nulls(value)).map_err(|e| refuse(e.to_string()))?;
    let mut wrapper = toml::Table::new();
    wrapper.insert(key.to_owned(), value);
    let text = toml::to_string(&wrapper).map_err(|e| refuse(e.to_string()))?;
    let mut parsed: DocumentMut = text.parse().map_err(|e| refuse(format!("{e}")))?;
    let mut item = parsed
        .remove(key)
        .ok_or_else(|| refuse("it holds nothing".to_owned()))?;
    unplace(&mut item);
    // The first header of the parsed text had nothing above it; in the
    // file it follows other settings, after a blank line.
    let first = match &mut item {
        Item::Table(table) => Some(table),
        Item::ArrayOfTables(tables) => tables.get_mut(0),
        _ => None,
    };
    if let Some(table) = first {
        table.decor_mut().set_prefix("\n");
    }
    Ok(item)
}

/// `value` with every `null` in an object left out, since TOML has no
/// `null` and a setting left out keeps its default.
fn without_nulls(value: Value) -> Value {
    match value {
        Value::Object(fields) => Value::Object(
            fields
                .into_iter()
                .filter(|(_, field)| !field.is_null())
                .map(|(key, field)| (key, without_nulls(field)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(without_nulls).collect()),
        value => value,
    }
}

/// Clears the place each table had in the text it was parsed from, so it
/// is written right after the table it now sits in.
fn unplace(item: &mut Item) {
    let unplace_table = |table: &mut Table| {
        table.set_position(None);
        for (_, child) in table.iter_mut() {
            unplace(child);
        }
    };
    match item {
        Item::Table(table) => unplace_table(table),
        Item::ArrayOfTables(tables) => tables.iter_mut().for_each(unplace_table),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[cfg(test)]
    fn parts(value: Value) -> BTreeMap<String, Option<Value>> {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn a_part_is_written_inside_seo_and_the_rest_of_the_file_stays() {
        let before = "# The site.\nname = \"Blog\"\n\n[seo.feeds]\nlimit = 10\n\n\
                      [nav]\nmain = []\n";
        let mut document: DocumentMut = before.parse().unwrap();
        let sent = json!({"titles": {"separator": "·", "post": {"title": "{title} {sep} {site}"}},
            "feeds": null});
        set_parts(&mut document, "seo", parts(sent)).unwrap();
        let after = "# The site.\nname = \"Blog\"\n\n[seo.titles]\nseparator = \"·\"\n\n\
                     [seo.titles.post]\ntitle = \"{title} {sep} {site}\"\n\n\
                     [nav]\nmain = []\n";
        assert_eq!(document.to_string(), after);
        let read = table(&document.to_string(), "seo").unwrap();
        assert_eq!(read["titles"]["post"]["title"], "{title} {sep} {site}");
        assert!(!read.contains_key("feeds"), "{read:?}");
    }

    #[test]
    fn nulls_inside_a_part_are_left_out_and_lists_of_tables_stay_lists() {
        let mut document: DocumentMut = "name = \"Blog\"\n".parse().unwrap();
        let sent = json!({"publisher": {"type": "person", "name": null},
            "locations": [{"name": "Shop", "street": "1 Road", "locality": "Leeds",
                "postal_code": "LS1", "country": "GB"}]});
        set_parts(&mut document, "seo", parts(sent)).unwrap();
        let text = document.to_string();
        assert!(
            text.contains("[seo.publisher]\ntype = \"person\"\n"),
            "{text}"
        );
        assert!(text.contains("[[seo.locations]]\n"), "{text}");
        assert!(!text.contains("name = null"), "{text}");
    }

    #[test]
    fn a_list_holding_null_is_refused_naming_its_part() {
        let mut document = DocumentMut::new();
        let refused = set_parts(
            &mut document,
            "seo",
            parts(json!({"crawlers": {"block": [null]}})),
        );
        assert!(
            matches!(&refused, Err(SettingsError::Refused { field, .. }) if field == "seo.crawlers"),
            "{refused:?}"
        );
    }
}
