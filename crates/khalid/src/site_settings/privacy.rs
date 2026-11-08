//! The privacy settings of `site.toml` for the Admin API: the `[privacy]`
//! table, changed one part at a time as `[seo]` is, and the site's own
//! code in `[code]`, read as one object per entry and written back whole as
//! a `[[code]]` list (privacy plan U8, KTD12).

use std::collections::BTreeMap;

use khalid_api::entry_store::EntryStoreError;
use khalid_api::settings_store::SettingsError;
use serde_json::{Map, Value};
use toml_edit::{DocumentMut, Item};

use super::parts;

/// The table the consent settings sit in.
const PRIVACY: &str = "privacy";

/// The table, or list of tables, the site's own code sits in.
const CODE: &str = "code";

/// The `[privacy]` table in `text`, as JSON, without defaults filled in.
///
/// # Errors
///
/// Returns [`EntryStoreError`] when `text` is not TOML.
pub(crate) fn privacy_table(text: &str) -> Result<Map<String, Value>, EntryStoreError> {
    parts::table(text, PRIVACY)
}

/// Puts each part of `parts` into `[privacy]`, removing each sent as `None`.
///
/// # Errors
///
/// Returns [`SettingsError::Refused`] naming a part TOML cannot hold.
pub(crate) fn set_privacy(
    document: &mut DocumentMut,
    parts: BTreeMap<String, Option<Value>>,
) -> Result<(), SettingsError> {
    parts::set_parts(document, PRIVACY, parts)
}

/// Every entry of `[code]` in `text`, as written. The old single table
/// reads as one entry in the words of the list: `consent = "not-needed"`
/// is the `no-tracking` category, and `consent = "ask"` has no category,
/// so it counts as Marketing.
///
/// # Errors
///
/// Returns [`EntryStoreError`] when `text` is not TOML.
pub(crate) fn code_entries(text: &str) -> Result<Vec<Map<String, Value>>, EntryStoreError> {
    let file: toml::Table = text.parse().map_err(EntryStoreError::new)?;
    let Some(code) = file.get(CODE) else {
        return Ok(Vec::new());
    };
    let entries = match serde_json::to_value(code).map_err(EntryStoreError::new)? {
        Value::Array(entries) => entries,
        Value::Object(mut one) => {
            if one.remove("consent").as_ref().and_then(Value::as_str) == Some("not-needed") {
                one.insert("category".to_owned(), "no-tracking".into());
            }
            let has_code = one.contains_key("head") || one.contains_key("body");
            if has_code {
                vec![Value::Object(one)]
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    };
    let tables = entries.into_iter().filter_map(|entry| match entry {
        Value::Object(entry) => Some(entry),
        _ => None,
    });
    Ok(tables.collect())
}

/// Writes `entries` as the `[[code]]` list in place of the old `[code]`,
/// keeping the comment above it; no entries removes `[code]`.
///
/// # Errors
///
/// Returns [`SettingsError::Refused`] naming `code` when TOML cannot hold
/// an entry.
pub(crate) fn set_code(
    document: &mut DocumentMut,
    entries: Vec<Map<String, Value>>,
) -> Result<(), SettingsError> {
    let old = document.remove(CODE);
    if entries.is_empty() {
        return Ok(());
    }
    let list = Value::Array(entries.into_iter().map(Value::Object).collect());
    let mut item = parts::item(CODE, CODE, list)?;
    let decor = match &old {
        Some(Item::Table(table)) => Some(table.decor().clone()),
        Some(Item::ArrayOfTables(tables)) => tables.get(0).map(|first| first.decor().clone()),
        _ => None,
    };
    if let (Some(decor), Item::ArrayOfTables(tables)) = (decor, &mut item)
        && let Some(first) = tables.get_mut(0)
    {
        *first.decor_mut() = decor;
    }
    document.insert(CODE, item);
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[cfg(test)]
    fn entry(value: Value) -> Map<String, Value> {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn a_privacy_part_is_written_inside_privacy_and_reads_back() {
        let mut document: DocumentMut = "name = \"Blog\"\n".parse().unwrap();
        let sent = BTreeMap::from([
            ("lifetime_months".to_owned(), Some(json!(12))),
            (
                "contact".to_owned(),
                Some(json!({"email": "privacy@example.com"})),
            ),
        ]);
        set_privacy(&mut document, sent).unwrap();
        let text = document.to_string();
        assert!(text.contains("[privacy]\nlifetime_months = 12\n"), "{text}");
        assert!(
            text.contains("[privacy.contact]\nemail = \"privacy@example.com\"\n"),
            "{text}"
        );
        let read = privacy_table(&text).unwrap();
        assert_eq!(read["lifetime_months"], 12);
    }

    #[test]
    fn the_old_single_table_reads_as_one_entry_in_the_words_of_the_list() {
        let ask = "[code]\nhead = \"<script></script>\"\n";
        assert_eq!(
            code_entries(ask).unwrap(),
            vec![entry(json!({"head": "<script></script>"}))]
        );
        let quiet = "[code]\nbody = \"<script></script>\"\nconsent = \"not-needed\"\n";
        let read = code_entries(quiet).unwrap();
        assert_eq!(
            read,
            vec![entry(
                json!({"body": "<script></script>", "category": "no-tracking"})
            )]
        );
        let none: Vec<Map<String, Value>> = Vec::new();
        assert_eq!(code_entries("[code]\nconsent = \"ask\"\n").unwrap(), none);
        assert_eq!(code_entries("name = \"Blog\"\n").unwrap(), none);
    }

    #[test]
    fn entries_replace_the_old_table_as_a_list_keeping_its_comment() {
        let before = "name = \"Blog\"\n\n# Our stats.\n[code]\nhead = \"<script></script>\"\n";
        let mut document: DocumentMut = before.parse().unwrap();
        let tagged = entry(
            json!({"head": "<script></script>", "category": "no-tracking",
            "vendor": "Example Stats"}),
        );
        set_code(&mut document, vec![tagged.clone()]).unwrap();
        let text = document.to_string();
        assert!(text.contains("# Our stats.\n[[code]]\n"), "{text}");
        assert_eq!(code_entries(&text).unwrap(), vec![tagged]);
        set_code(&mut document, Vec::new()).unwrap();
        assert!(!document.to_string().contains("code"), "{document}");
    }
}
