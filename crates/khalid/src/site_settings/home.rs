//! The home page's sections as the Admin API reads and changes them:
//! `home_sections` under `[reader]` in `site.toml`, written as one list
//! of words and tables in the order the home page shows them.

use khalid_api::resources::HomeSectionSetting;
use khalid_core::{Config, HomeSection};
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value};

/// The sections `config` reads, top first.
pub(super) fn sections(config: &Config) -> Vec<HomeSectionSetting> {
    let named = |slug: &khalid_core::Slug| slug.as_str().to_owned();
    (config.reader().home_sections.iter())
        .filter_map(|section| {
            Some(match section {
                HomeSection::Latest => HomeSectionSetting::Latest,
                HomeSection::Featured => HomeSectionSetting::Featured,
                HomeSection::Newsletter => HomeSectionSetting::Newsletter,
                HomeSection::Category(slug) => HomeSectionSetting::Category { slug: named(slug) },
                HomeSection::Collection(slug) => {
                    HomeSectionSetting::Collection { slug: named(slug) }
                }
                HomeSection::Channel(slug) => HomeSectionSetting::Channel { slug: named(slug) },
                HomeSection::Series(slug) => HomeSectionSetting::Series { slug: named(slug) },
                _ => return None,
            })
        })
        .collect()
}

/// Sets `home_sections` under `[reader]` to `sections`, in place of the
/// old list in either form. An empty list removes the key, so the home
/// page is one list of the newest posts again. The rest of `[reader]`
/// stays as written.
pub(super) fn set_sections(document: &mut DocumentMut, sections: Vec<HomeSectionSetting>) {
    let reader = document
        .entry("reader")
        .or_insert_with(|| Item::Table(Table::new()));
    if let Item::Value(Value::InlineTable(inline)) = reader {
        *reader = Item::Table(inline.clone().into_table());
    }
    let Some(reader) = reader.as_table_like_mut() else {
        return;
    };
    reader.remove("home_sections");
    if sections.is_empty() {
        return;
    }
    let mut list: Array = sections.into_iter().filter_map(written).collect();
    for item in list.iter_mut() {
        item.decor_mut().set_prefix("\n  ");
    }
    list.set_trailing("\n");
    list.set_trailing_comma(true);
    reader.insert("home_sections", Item::Value(Value::Array(list)));
}

/// One section as `site.toml` writes it: a word, or a table naming it.
/// `None` for a kind this version of the API does not know yet.
fn written(section: HomeSectionSetting) -> Option<Value> {
    let (key, slug) = match section {
        HomeSectionSetting::Latest => return Some("latest".into()),
        HomeSectionSetting::Featured => return Some("featured".into()),
        HomeSectionSetting::Newsletter => return Some("newsletter".into()),
        HomeSectionSetting::Category { slug } => ("category", slug),
        HomeSectionSetting::Collection { slug } => ("collection", slug),
        HomeSectionSetting::Channel { slug } => ("channel", slug),
        HomeSectionSetting::Series { slug } => ("series", slug),
        _ => return None,
    };
    let mut table = InlineTable::new();
    table.insert(key, slug.into());
    Some(Value::InlineTable(table))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_replaces_the_old_tables_and_keeps_the_switches() {
        let before = "name = \"Blog\"\n\n# What readers see.\n[reader]\narchive = false\n\n\
                      [[reader.home_sections]]\ncategory = \"seo\"\n";
        let mut document: DocumentMut = before.parse().unwrap();
        let sections = vec![
            HomeSectionSetting::Featured,
            HomeSectionSetting::Category {
                slug: "seo".to_owned(),
            },
            HomeSectionSetting::Newsletter,
        ];
        set_sections(&mut document, sections);
        let after = "name = \"Blog\"\n\n# What readers see.\n[reader]\narchive = false\n\
                     home_sections = [\n  \"featured\",\n  { category = \"seo\" },\n  \"newsletter\",\n]\n";
        assert_eq!(document.to_string(), after);
    }

    #[test]
    fn an_empty_list_removes_the_key_and_a_new_list_makes_the_table() {
        let mut document: DocumentMut = "name = \"Blog\"\n".parse().unwrap();
        set_sections(&mut document, vec![HomeSectionSetting::Latest]);
        let made = "name = \"Blog\"\n\n[reader]\nhome_sections = [\n  \"latest\",\n]\n";
        assert_eq!(document.to_string(), made);
        set_sections(&mut document, Vec::new());
        assert_eq!(document.to_string(), "name = \"Blog\"\n\n[reader]\n");
    }
}
