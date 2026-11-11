//! A sync of the main site over the file the Design screen edits: each
//! value that changed is set in place, so a file that sets only a few
//! tokens stays short and keeps its comments.

use serde::Serialize;
use toml_edit::{DocumentMut, Item, Table};

use super::DesignFile;
use crate::{FontFileEntry, ThemeDiff, ThemeTokens, TokenChange, TokenName, TokensError};

impl DesignFile {
    /// Takes the values in `site`, a fresh sync of the main site, for every
    /// token the file does not lock, and returns the tokens the pages use
    /// after it and what changed.
    ///
    /// The file is compared as the pages see it: its own values over
    /// `pack`, the chosen template's tokens, over the light default. Only a
    /// token whose value changes is written into the file, so a value the
    /// template already shares with the site stays the template's. When
    /// `site` lists font files, they replace the file's list.
    ///
    /// # Errors
    ///
    /// Returns [`TokensError::Parse`] when the file or `pack` holds a value
    /// the renderer refuses. Nothing changes then.
    pub fn resync(
        &mut self,
        pack: &str,
        site: &ThemeTokens,
    ) -> Result<(ThemeTokens, ThemeDiff), TokensError> {
        let mut tokens = self.effective(pack)?;
        let diff = tokens.resync(site);
        for change in diff.changes() {
            if let TokenChange::Updated { name, .. } = change {
                self.set(*name, &written(site, *name))?;
            }
        }
        if !site.fonts.files.is_empty() {
            self.set_font_files(&site.fonts.files);
        }
        Ok((self.effective(pack)?, diff))
    }

    /// Replaces the `files` list of the `fonts` table with `files`.
    fn set_font_files(&mut self, files: &[FontFileEntry]) {
        #[derive(Serialize)]
        struct Fonts<'a> {
            files: &'a [FontFileEntry],
        }
        let text = toml::to_string(&Fonts { files }).unwrap_or_default();
        let Some(list) = text
            .parse::<DocumentMut>()
            .ok()
            .and_then(|mut document| document.remove("files"))
        else {
            return;
        };
        let fonts = (self.document)
            .entry("fonts")
            .or_insert_with(|| Item::Table(Table::new()));
        if let Item::Table(table) = fonts {
            table.insert("files", list);
        } else if let (Some(table), Ok(list)) = (fonts.as_table_like_mut(), list.into_value()) {
            table.insert("files", Item::Value(list));
        }
    }
}

/// The value of `name` in `tokens` as [`DesignFile::set`] takes it: an
/// empty text for a logo or icon that is not set.
fn written(tokens: &ThemeTokens, name: TokenName) -> String {
    let address = |url: Option<&khalid_core::AbsoluteUrl>| url.map(ToString::to_string);
    match name {
        TokenName::Logo => address(tokens.brand.logo.as_ref()).unwrap_or_default(),
        TokenName::Icon => address(tokens.brand.icon.as_ref()).unwrap_or_default(),
        name => tokens.value(name),
    }
}
