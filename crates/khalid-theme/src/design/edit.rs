//! One token of `theme/tokens.toml` changed in place, checked with the
//! renderer's rules first.

use khalid_core::AbsoluteUrl;
use toml_edit::{Array, Item, Table, TableLike, Value};

use super::DesignFile;
use crate::token_values::hex;
use crate::{FontStack, Length, TokenName, TokensError};

/// What a logo or icon must be.
const ADDRESS: &str = "a logo or icon must be a full http or https address, or left empty";

impl DesignFile {
    /// Sets `token` to `value`, as the file writes it, such as `#0b57d0`
    /// for a colour. An empty logo or icon takes it out of the file.
    ///
    /// # Errors
    ///
    /// Returns [`TokensError::Value`] when `value` is not one the renderer
    /// takes for `token`. Nothing changes then.
    pub fn set(&mut self, token: TokenName, value: &str) -> Result<(), TokensError> {
        let refused = |problem| TokensError::Value {
            token,
            value: value.to_owned(),
            problem,
        };
        let checked = match token {
            TokenName::Background
            | TokenName::Surface
            | TokenName::Text
            | TokenName::Muted
            | TokenName::Link
            | TokenName::Accent
            | TokenName::Border => hex::parse(value)
                .map(|_| Some(value))
                .ok_or(refused(hex::PROBLEM))?,
            TokenName::Body | TokenName::Heading | TokenName::Mono => {
                FontStack::try_from(value.to_owned()).map_err(refused)?;
                Some(value)
            }
            TokenName::Radius | TokenName::Measure => {
                Length::try_from(value.to_owned()).map_err(refused)?;
                Some(value)
            }
            TokenName::Logo | TokenName::Icon if value.is_empty() => None,
            TokenName::Logo | TokenName::Icon => {
                AbsoluteUrl::parse(value).map_err(|_| refused(ADDRESS))?;
                Some(value)
            }
        };
        let (table, key) = token.as_str().split_once('.').unwrap_or_default();
        if let Some(value) = checked {
            self.set_in(table, key, value.into());
        } else if let Some(table) = self
            .document
            .get_mut(table)
            .and_then(Item::as_table_like_mut)
        {
            table.remove(key);
        }
        Ok(())
    }

    /// Replaces the `locked` list: the tokens a sync keeps as they are,
    /// written in the order of the file.
    pub fn set_locked(&mut self, names: &[TokenName]) {
        let mut sorted = names.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        if sorted.is_empty() {
            self.document.remove("locked");
            return;
        }
        let list: Array = sorted.iter().map(|name| name.as_str()).collect();
        set_value(self.document.as_table_mut(), "locked", Value::Array(list));
    }

    /// Sets `key` of the table `name` to `new`, making the table when it is
    /// missing.
    fn set_in(&mut self, name: &str, key: &str, new: Value) {
        let item = self
            .document
            .entry(name)
            .or_insert_with(|| Item::Table(Table::new()));
        if item.as_table_like().is_none() {
            *item = Item::Table(Table::new());
        }
        if let Some(table) = item.as_table_like_mut() {
            set_value(table, key, new);
        }
    }
}

/// Sets `key` in `table` to `new`, keeping the comment and spacing around
/// the old value, such as `# brand red` after it.
fn set_value(table: &mut dyn TableLike, key: &str, new: Value) {
    match table.get_mut(key).and_then(Item::as_value_mut) {
        Some(old) => {
            let decor = old.decor().clone();
            *old = new;
            *old.decor_mut() = decor;
        }
        None => {
            table.insert(key, Item::Value(new));
        }
    }
}
