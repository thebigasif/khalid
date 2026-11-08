//! The words templates show, one file per language (RULES.md BL-TXT-3).

use std::collections::BTreeMap;

use khalid_core::Locale;

use crate::RenderError;
use crate::archive;

/// The language whose strings fill in for a language with no file.
const FALLBACK_LANGUAGE: &str = "en";

/// Where the strings files sit in the built-in archive, such as
/// `strings/en.toml`.
const FOLDER: &str = "strings/";

/// The built-in strings of `locale`'s language, as templates read them under
/// `t`, or the English ones when the language has no file, for words
/// written outside a template, such as the month names in a title.
///
/// # Errors
///
/// [`RenderError::Strings`] when a built-in strings file cannot be read.
pub fn builtin_strings(locale: &Locale) -> Result<BTreeMap<String, String>, RenderError> {
    let strings = Strings::load()?;
    Ok(strings.for_locale(locale).cloned().unwrap_or_default())
}

/// Every built-in strings table, by language.
#[derive(Debug, Clone)]
pub(crate) struct Strings {
    tables: BTreeMap<String, BTreeMap<String, String>>,
}

impl Strings {
    /// Reads every built-in strings file.
    pub(crate) fn load() -> Result<Self, RenderError> {
        let mut tables = BTreeMap::new();
        let files = archive::built_in();
        for name in files.names() {
            let Some(language) = name
                .strip_prefix(FOLDER)
                .and_then(|file| file.strip_suffix(".toml"))
            else {
                continue;
            };
            let Some(text) = files.get(name) else {
                continue;
            };
            let table = toml::from_str(text).map_err(|source| RenderError::Strings {
                language: language.to_owned(),
                source,
            })?;
            tables.insert(language.to_owned(), table);
        }
        Ok(Self { tables })
    }

    /// The table for `locale`'s language, or the English one when the
    /// language has no file yet. `None` only if the English file is missing,
    /// which this crate's tests rule out.
    pub(crate) fn for_locale(&self, locale: &Locale) -> Option<&BTreeMap<String, String>> {
        self.tables
            .get(locale.language())
            .or_else(|| self.tables.get(FALLBACK_LANGUAGE))
    }

    /// The languages that have a strings file.
    #[cfg(test)]
    pub(crate) fn languages(&self) -> impl Iterator<Item = &str> {
        self.tables.keys().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_has_exactly_the_english_keys() {
        let strings = Strings::load().unwrap();
        let english: Vec<&String> = strings.tables["en"].keys().collect();
        for language in strings.languages() {
            let keys: Vec<&String> = strings.tables[language].keys().collect();
            assert_eq!(keys, english, "strings/{language}.toml");
        }
    }

    #[test]
    fn every_key_reads_as_its_file_says() {
        let strings = Strings::load().unwrap();
        for (language, file) in [
            ("en", include_str!("../strings/en.toml")),
            ("de", include_str!("../strings/de.toml")),
        ] {
            let want: BTreeMap<String, String> = toml::from_str(file).unwrap();
            assert_eq!(strings.tables[language], want, "strings/{language}.toml");
        }
        assert_eq!(strings.languages().collect::<Vec<_>>(), ["de", "en"]);
    }

    #[test]
    fn a_language_without_a_file_falls_back_to_english() {
        let strings = Strings::load().unwrap();
        let french = strings.for_locale(&Locale::try_new("fr").unwrap()).unwrap();
        assert_eq!(french, &strings.tables["en"]);
    }

    #[test]
    fn a_regional_locale_uses_its_language_file() {
        let strings = Strings::load().unwrap();
        let swiss = strings
            .for_locale(&Locale::try_new("de-CH").unwrap())
            .unwrap();
        assert_eq!(swiss, &strings.tables["de"]);
    }
}
