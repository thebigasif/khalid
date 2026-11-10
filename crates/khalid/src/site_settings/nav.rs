//! The header and footer menus as the Admin API reads and changes them:
//! `[[nav.primary]]` and `[[nav.footer]]` in `site.toml`, one table per
//! link in the order the menu shows them. The footer's columns,
//! `[[nav.footer_groups]]`, stay as written.

use khalid_api::resources::{MenuLinkSetting, NavChange, NavSetting};
use khalid_core::{Config, MenuLink};
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, Value};

/// The menus `config` reads.
pub(super) fn menus(config: &Config) -> NavSetting {
    let links = |menu: &[MenuLink]| menu.iter().map(link).collect();
    NavSetting {
        primary: links(&config.nav().primary),
        footer: links(&config.nav().footer),
    }
}

fn link(link: &MenuLink) -> MenuLinkSetting {
    MenuLinkSetting {
        label: link.label.as_str().to_owned(),
        href: link.href.as_str().to_owned(),
        language: link.language.as_ref().map(|l| l.as_str().to_owned()),
    }
}

/// Sets each menu `change` sends, in place of the old one in any form. An
/// empty menu removes its key. The rest of `[nav]` stays as written.
pub(super) fn set_menus(document: &mut DocumentMut, change: NavChange) {
    let menus = [("primary", change.primary), ("footer", change.footer)];
    if menus.iter().all(|(_, links)| links.is_none()) {
        return;
    }
    let nav = document.entry("nav").or_insert_with(|| {
        let mut table = Table::new();
        table.set_implicit(true);
        Item::Table(table)
    });
    if let Item::Value(Value::InlineTable(inline)) = nav {
        *nav = Item::Table(inline.clone().into_table());
    }
    let Some(nav) = nav.as_table_mut() else {
        return;
    };
    let nav_at = nav.position();
    for (key, links) in menus {
        let Some(links) = links else { continue };
        // The new links take the old ones' place in the file, or else sit
        // right under `[nav]`, ahead of the footer's columns.
        let old_at = match nav.remove(key) {
            Some(Item::ArrayOfTables(old)) => old.iter().find_map(Table::position),
            _ => None,
        };
        if !links.is_empty() {
            let at = old_at.or(nav_at);
            let tables: ArrayOfTables = links.into_iter().map(|l| written(l, at)).collect();
            nav.insert(key, Item::ArrayOfTables(tables));
        }
    }
}

/// One link as `site.toml` writes it, placed at `at` in the file.
fn written(link: MenuLinkSetting, at: Option<isize>) -> Table {
    let mut table = Table::new();
    table.set_position(at);
    table.insert("label", Item::Value(link.label.into()));
    table.insert("href", Item::Value(link.href.into()));
    if let Some(language) = link.language {
        table.insert("language", Item::Value(language.into()));
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    fn guides() -> MenuLinkSetting {
        MenuLinkSetting {
            label: "Guides".to_owned(),
            href: "/blog/guides/".to_owned(),
            language: None,
        }
    }

    #[test]
    fn a_new_menu_replaces_the_old_one_and_keeps_the_footer_columns() {
        let before = "name = \"Blog\"\n\n# The menus.\n[nav]\nprimary = [{ label = \"Old\", href = \"/old/\" }]\n\n\
                      [[nav.footer_groups]]\nheading = \"Product\"\n\n\
                      [[nav.footer_groups.links]]\nlabel = \"Pricing\"\nhref = \"/pricing/\"\n";
        let mut document: DocumentMut = before.parse().unwrap();
        let impressum = MenuLinkSetting {
            label: "Impressum".to_owned(),
            href: "/blog/de/impressum/".to_owned(),
            language: Some("de".to_owned()),
        };
        let change = NavChange {
            primary: Some(vec![guides(), impressum]),
            footer: None,
        };
        set_menus(&mut document, change);
        let after = "name = \"Blog\"\n\n# The menus.\n[nav]\n\n\
                     [[nav.primary]]\nlabel = \"Guides\"\nhref = \"/blog/guides/\"\n\n\
                     [[nav.primary]]\nlabel = \"Impressum\"\nhref = \"/blog/de/impressum/\"\nlanguage = \"de\"\n\n\
                     [[nav.footer_groups]]\nheading = \"Product\"\n\n\
                     [[nav.footer_groups.links]]\nlabel = \"Pricing\"\nhref = \"/pricing/\"\n";
        assert_eq!(document.to_string(), after);
        let mut again: DocumentMut = after.parse().unwrap();
        let change = NavChange {
            primary: Some(vec![guides()]),
            footer: None,
        };
        set_menus(&mut again, change);
        let text = again.to_string();
        let (first, _) = text.split_once("[[nav.footer_groups]]").unwrap();
        assert!(first.ends_with("href = \"/blog/guides/\"\n\n"), "{text}");
    }

    #[test]
    fn an_empty_menu_removes_it_and_a_file_without_nav_gains_one() {
        let mut document: DocumentMut = "name = \"Blog\"\n".parse().unwrap();
        let change = NavChange {
            primary: None,
            footer: Some(vec![guides()]),
        };
        set_menus(&mut document, change);
        let made =
            "name = \"Blog\"\n\n[[nav.footer]]\nlabel = \"Guides\"\nhref = \"/blog/guides/\"\n";
        assert_eq!(document.to_string(), made);
        let change = NavChange {
            primary: None,
            footer: Some(Vec::new()),
        };
        set_menus(&mut document, change);
        assert_eq!(document.to_string(), "name = \"Blog\"\n");
    }
}
