//! The admin's Design screen over `theme/tokens.toml`: the values the
//! site's pages use, and one token changed at a time in place, so the
//! comments, the order and every table the screen does not edit stay as
//! they were written.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use toml_edit::DocumentMut;

use crate::tokens::{HEADER, parse_table, tokens_path, write_text};
use crate::{ThemeTokens, TokensError};

/// `theme/tokens.toml` as the Design screen reads and changes it.
#[derive(Debug, Clone)]
pub struct DesignFile {
    /// The file, comments and all.
    document: DocumentMut,
    /// When the file last changed, or `None` when there is no file yet.
    changed: Option<SystemTime>,
    /// Whether the site had no file, so writing makes one.
    made: bool,
}

impl DesignFile {
    /// Reads `theme/tokens.toml` inside `site_root`. A site without the file
    /// gets an empty one, which [`DesignFile::write`] creates.
    ///
    /// # Errors
    ///
    /// Returns [`TokensError::Path`] when `theme/` leads outside the site
    /// folder, [`TokensError::Read`] when the file cannot be read, and
    /// [`TokensError::Parse`] when it is not TOML.
    pub fn read(site_root: &Path) -> Result<Self, TokensError> {
        let path = tokens_path(site_root)?;
        let read = |source| TokensError::Read {
            path: path.clone(),
            source,
        };
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(Self {
                    made: true,
                    ..Self::from_toml("")?
                });
            }
            Err(source) => return Err(read(source)),
        };
        let changed = fs::metadata(&path)
            .and_then(|meta| meta.modified())
            .map_err(read)?;
        let mut file = Self::from_toml(&text)?;
        file.changed = Some(changed);
        Ok(file)
    }

    /// The file whose text is `text`. A value is checked only when the
    /// tokens are read with [`DesignFile::effective`].
    ///
    /// # Errors
    ///
    /// Returns [`TokensError::Parse`] with the line of the first mistake
    /// when `text` is not TOML.
    pub fn from_toml(text: &str) -> Result<Self, TokensError> {
        parse_table(text)?;
        let document = text
            .parse::<DocumentMut>()
            .map_err(|error| TokensError::Parse {
                line: None,
                source: serde::de::Error::custom(error.message()),
            })?;
        Ok(Self {
            document,
            changed: None,
            made: false,
        })
    }

    /// When the file last changed, or `None` when there is no file yet.
    pub fn changed(&self) -> Option<SystemTime> {
        self.changed
    }

    /// The tokens the site's pages use: each value the file sets, then the
    /// value of `pack`, the chosen template's tokens in the same form,
    /// then the light default, as the renderer loads them.
    ///
    /// # Errors
    ///
    /// Returns [`TokensError::Parse`] when a value is one the renderer
    /// refuses, or a field is one it does not know.
    pub fn effective(&self, pack: &str) -> Result<ThemeTokens, TokensError> {
        let mut tokens = parse_table(&ThemeTokens::default().to_toml())?;
        merge(&mut tokens, parse_table(pack)?);
        merge(&mut tokens, parse_table(&self.document.to_string())?);
        toml::Value::Table(tokens)
            .try_into()
            .map_err(|source| TokensError::Parse { line: None, source })
    }

    /// The file's text: as read, with each change in place. A file this
    /// screen makes starts with the header that says what the file is.
    pub fn to_toml(&self) -> String {
        let text = self.document.to_string();
        if self.made {
            format!("{HEADER}{text}")
        } else {
            text
        }
    }

    /// Writes the file atomically to `theme/tokens.toml` inside
    /// `site_root`, as [`ThemeTokens::write`] does, and returns the path.
    ///
    /// # Errors
    ///
    /// Returns [`TokensError::Path`] and [`TokensError::Write`] as
    /// [`ThemeTokens::write`] does.
    pub fn write(&self, site_root: &Path) -> Result<PathBuf, TokensError> {
        write_text(site_root, &self.to_toml())
    }
}

/// Puts every value of `over` into `under`, table by table, as the
/// renderer layers a site's file over its pack.
fn merge(under: &mut toml::Table, over: toml::Table) {
    for (key, value) in over {
        match (under.get_mut(&key), value) {
            (Some(toml::Value::Table(below)), toml::Value::Table(above)) => merge(below, above),
            (_, value) => {
                under.insert(key, value);
            }
        }
    }
}
