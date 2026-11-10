//! [`AdminFolder`]: the folder under the blog where the admin answers.

use std::fmt;

/// The most characters a folder name may have.
const MAX_LEN: usize = 40;

/// Folders Khalid answers itself under every blog, so the admin cannot
/// take them.
const TAKEN: [&str; 2] = ["api", "ghost"];

/// The folder under the blog's mount where the admin answers, such as
/// `admin` for `/blog/admin/` (plan section 4.6). It is one folder name:
/// 1 to 40 lowercase letters, digits and hyphens, with no hyphen at either
/// end, and not `api` or `ghost`, which the APIs use.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdminFolder(String);

impl AdminFolder {
    /// Checks `text` against the folder name rules.
    ///
    /// # Errors
    ///
    /// Returns [`ParseAdminFolderError::Shape`] when `text` is empty, too
    /// long, or holds anything but lowercase letters, digits and inner
    /// hyphens, and [`ParseAdminFolderError::Taken`] when the APIs use it.
    pub fn try_new(text: &str) -> Result<Self, ParseAdminFolderError> {
        let allowed = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-';
        let shaped = (1..=MAX_LEN).contains(&text.len())
            && text.chars().all(allowed)
            && !text.starts_with('-')
            && !text.ends_with('-');
        if !shaped {
            return Err(ParseAdminFolderError::Shape);
        }
        if TAKEN.contains(&text) {
            return Err(ParseAdminFolderError::Taken);
        }
        Ok(Self(text.to_owned()))
    }

    /// The folder name, with no slashes.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for AdminFolder {
    /// `admin`, where the admin answers unless `site.toml` says otherwise.
    fn default() -> Self {
        Self("admin".to_owned())
    }
}

impl fmt::Display for AdminFolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for AdminFolder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::try_new(&text).map_err(serde::de::Error::custom)
    }
}

/// Why a text is not an [`AdminFolder`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ParseAdminFolderError {
    /// The name is empty, too long, or holds a character not allowed.
    #[error(
        "the admin folder must be one folder name of 1 to 40 lowercase letters, digits and hyphens, like studio"
    )]
    Shape,
    /// The APIs answer under this folder.
    #[error(
        "the admin folder cannot be api or ghost, since the APIs answer there; pick another name, like studio"
    )]
    Taken,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_folder_name_is_kept() {
        let folder = AdminFolder::try_new("studio-2").map(|folder| folder.to_string());
        assert_eq!(folder, Ok("studio-2".to_owned()));
        assert_eq!(AdminFolder::default().as_str(), "admin");
    }

    #[test]
    fn slashes_capitals_and_edge_hyphens_are_refused() {
        for text in ["", "/studio/", "a/b", "Studio", "-studio", "studio-", "stü"] {
            assert_eq!(
                AdminFolder::try_new(text),
                Err(ParseAdminFolderError::Shape),
                "{text}"
            );
        }
        let long = "a".repeat(MAX_LEN + 1);
        assert_eq!(
            AdminFolder::try_new(&long),
            Err(ParseAdminFolderError::Shape)
        );
    }

    #[test]
    fn the_api_folders_are_refused() {
        for text in TAKEN {
            assert_eq!(
                AdminFolder::try_new(text),
                Err(ParseAdminFolderError::Taken)
            );
        }
    }
}
