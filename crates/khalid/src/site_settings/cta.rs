//! The call to action as the Admin API reads and changes it:
//! `[template.cta]` in `site.toml`. The rest of `[template]` stays as
//! written, and the checks at start judge the label, the address and the
//! note before anything is saved.

use khalid_api::resources::CtaSetting;
use khalid_core::Config;
use toml_edit::{DocumentMut, Item, Table, Value};

/// The call to action `config` reads, if it sets one.
pub(super) fn cta(config: &Config) -> Option<CtaSetting> {
    let cta = config.template().cta()?;
    Some(CtaSetting {
        label: cta.label.as_str().to_owned(),
        href: cta.href.as_str().to_owned(),
        note: cta.note.as_ref().map(|note| note.as_str().to_owned()),
    })
}

/// Sets `[template.cta]` to `sent`, or removes it for `None`. An old table
/// keeps its place and the comment above it; a new one goes at the end of
/// `[template]`, which is made when the file has none.
pub(super) fn set_cta(document: &mut DocumentMut, sent: Option<CtaSetting>) {
    let template = document.entry("template").or_insert_with(|| {
        let mut table = Table::new();
        table.set_implicit(true);
        Item::Table(table)
    });
    if let Item::Value(Value::InlineTable(inline)) = template {
        *template = Item::Table(inline.clone().into_table());
    }
    let Some(template) = template.as_table_mut() else {
        return;
    };
    let old = template.remove("cta");
    let Some(sent) = sent else {
        if template.is_empty() && template.is_implicit() {
            document.remove("template");
        }
        return;
    };
    let mut table = match old {
        Some(Item::Table(mut old)) => {
            old.clear();
            old
        }
        _ => Table::new(),
    };
    table.insert("label", Item::Value(sent.label.into()));
    table.insert("href", Item::Value(sent.href.into()));
    if let Some(note) = sent.note {
        table.insert("note", Item::Value(note.into()));
    }
    template.insert("cta", Item::Table(table));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trial(note: Option<&str>) -> CtaSetting {
        CtaSetting {
            label: "Start a free trial".to_owned(),
            href: "/blog/start/".to_owned(),
            note: note.map(str::to_owned),
        }
    }

    #[test]
    fn a_new_call_to_action_takes_the_old_ones_place_and_keeps_its_comment() {
        let before = "name = \"Blog\"\n\n[template]\nname = \"trial\"\n\n\
                      # The button.\n[template.cta]\nlabel = \"Try it\"\nhref = \"/start/\"\n\
                      note = \"Free.\"\n\n[nav]\nmain = []\n";
        let mut document: DocumentMut = before.parse().unwrap();
        set_cta(&mut document, Some(trial(None)));
        let after = "name = \"Blog\"\n\n[template]\nname = \"trial\"\n\n\
                     # The button.\n[template.cta]\nlabel = \"Start a free trial\"\n\
                     href = \"/blog/start/\"\n\n[nav]\nmain = []\n";
        assert_eq!(document.to_string(), after);
    }

    #[test]
    fn a_file_without_a_template_gains_only_the_call_to_action_and_loses_it_again() {
        let mut document: DocumentMut = "name = \"Blog\"\n".parse().unwrap();
        set_cta(&mut document, Some(trial(Some("No card needed."))));
        let made = "name = \"Blog\"\n\n[template.cta]\nlabel = \"Start a free trial\"\n\
                    href = \"/blog/start/\"\nnote = \"No card needed.\"\n";
        assert_eq!(document.to_string(), made);
        set_cta(&mut document, None);
        assert_eq!(document.to_string(), "name = \"Blog\"\n");
    }

    #[test]
    fn removing_the_call_to_action_keeps_the_rest_of_the_template() {
        let before =
            "[template]\nname = \"trial\"\n\n[template.cta]\nlabel = \"Go\"\nhref = \"/\"\n";
        let mut document: DocumentMut = before.parse().unwrap();
        set_cta(&mut document, None);
        assert_eq!(document.to_string(), "[template]\nname = \"trial\"\n");
    }
}
