//! The words templates show, one file per language (RULES.md BL-TXT-3).

use std::collections::BTreeMap;
use std::path::Path;
use std::{fs, io};

use khalid_core::{Locale, SitePath};
use serde::Deserialize;

use crate::RenderError;
use crate::archive;

/// The language whose strings fill in for a language with no file.
const FALLBACK_LANGUAGE: &str = "en";

/// Where a site keeps its own strings files, one per language, such as
/// `theme/strings/fr.toml`.
const SITE_FOLDER: &str = "theme/strings";

/// The site's file for the fallback language.
const ENGLISH_FILE: &str = "en.toml";

/// Where the strings files sit in the built-in archive, such as
/// `strings/en.toml`. A language's words for one part of the site may sit
/// in their own folder, such as `strings/privacy/en.toml`, so no file grows
/// too long; they join the language's table.
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

/// The words readers see on one site, by language: the built-in strings
/// with the site's own `theme/strings/<lang>.toml` files laid over them, as
/// the templates read them, for words written outside a template, such as
/// the month names in a title or the hours on a location card.
#[derive(Debug, Clone)]
pub struct SiteWords(Strings);

impl SiteWords {
    /// The built-in words alone, for a site that has no folder to read.
    ///
    /// # Errors
    ///
    /// [`RenderError::Strings`] when a built-in strings file cannot be read.
    pub fn builtin() -> Result<Self, RenderError> {
        Strings::load().map(Self)
    }

    /// The built-in words with the files in `site_root/theme/strings/`
    /// laid over them.
    ///
    /// # Errors
    ///
    /// The errors of [`crate::Renderer::with_theme`] for the strings files:
    /// a file not named for a language, a key the templates never read, a
    /// TOML mistake, or a file that cannot be read.
    pub fn of_site(site_root: &Path) -> Result<Self, RenderError> {
        Strings::with_site(site_root).map(Self)
    }

    /// The words of `locale`'s language, or the English ones when it has
    /// none of its own.
    pub fn table(&self, locale: &Locale) -> &BTreeMap<String, String> {
        static NONE: BTreeMap<String, String> = BTreeMap::new();
        self.0.for_locale(locale).unwrap_or(&NONE)
    }

    /// The word `key` of `locale`'s language, such as `imprint_title`.
    pub fn get(&self, locale: &Locale, key: &str) -> Option<&str> {
        self.table(locale).get(key).map(String::as_str)
    }

    /// The site's words for another crate's pages and emails, by language:
    /// the `[newsletter]`, `[comments]` or `[notes]` table of each strings
    /// file that has one. That crate checks the keys, since only it knows
    /// them.
    pub fn part(&self, part: &str) -> BTreeMap<String, toml::Table> {
        self.0.parts.get(part).cloned().unwrap_or_default()
    }
}

/// Every built-in strings table, by language, and the site's tables for
/// other crates, by part and language.
#[derive(Debug, Clone)]
pub(crate) struct Strings {
    tables: BTreeMap<String, BTreeMap<String, String>>,
    parts: BTreeMap<String, BTreeMap<String, toml::Table>>,
}

impl Strings {
    /// Reads every built-in strings file, joining each part file into its
    /// language's table.
    pub(crate) fn load() -> Result<Self, RenderError> {
        let mut tables: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        let files = archive::built_in();
        for name in files.names() {
            let Some(language) = name
                .strip_prefix(FOLDER)
                .and_then(|file| file.strip_suffix(".toml"))
                .map(|file| file.rsplit('/').next().unwrap_or(file))
            else {
                continue;
            };
            let Some(text) = files.get(name) else {
                continue;
            };
            let table: BTreeMap<String, String> =
                toml::from_str(text).map_err(|source| RenderError::Strings {
                    language: language.to_owned(),
                    source,
                })?;
            tables.entry(language.to_owned()).or_default().extend(table);
        }
        Ok(Self {
            tables,
            parts: BTreeMap::new(),
        })
    }

    /// The built-in strings with each file in `site_root/theme/strings/`
    /// laid over its language: `fr.toml` gives a French site its words, and
    /// `de.toml` changes some German ones. A key the file leaves out keeps
    /// the built-in word of that language, or the English one when the
    /// language has no built-in file.
    pub(crate) fn with_site(site_root: &Path) -> Result<Self, RenderError> {
        let mut strings = Self::load()?;
        let folder = SitePath::within(site_root, SITE_FOLDER)?.resolve()?;
        let entries = match fs::read_dir(&folder) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(strings),
            Err(source) => {
                return Err(RenderError::ReadStrings {
                    path: folder,
                    source,
                });
            }
        };
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|source| RenderError::ReadStrings {
                path: folder.clone(),
                source,
            })?;
            let name = entry.file_name().to_string_lossy().into_owned();
            // A hidden file, such as the `.DS_Store` a Mac leaves, holds no words.
            if !name.starts_with('.') {
                names.push(name);
            }
        }
        // English first, so a language with no built-in file starts from
        // the site's English words.
        names.sort_by(|a, b| (a != ENGLISH_FILE, a).cmp(&(b != ENGLISH_FILE, b)));
        for name in names {
            let language = site_language(&name)?;
            let path = SitePath::within(site_root, &format!("{SITE_FOLDER}/{name}"))?.resolve()?;
            let text = fs::read_to_string(&path)
                .map_err(|source| RenderError::ReadStrings { path, source })?;
            let own = site_table(&name, &text)?;
            strings.lay_over(language, &name, own.words)?;
            for (part, table) in [
                ("newsletter", own.newsletter),
                ("comments", own.comments),
                ("notes", own.notes),
            ] {
                if !table.is_empty() {
                    let languages = strings.parts.entry(part.to_owned()).or_default();
                    languages.insert(language.to_owned(), table);
                }
            }
        }
        Ok(strings)
    }

    /// Lays the site's table `own`, read from `file`, over `language`'s.
    fn lay_over(
        &mut self,
        language: &str,
        file: &str,
        own: BTreeMap<String, String>,
    ) -> Result<(), RenderError> {
        let english = self.tables.get(FALLBACK_LANGUAGE);
        if let Some(key) = own
            .keys()
            .find(|key| english.is_none_or(|english| !english.contains_key(*key)))
        {
            return Err(RenderError::UnknownStringsKey {
                file: file.to_owned(),
                key: key.clone(),
            });
        }
        let base = self.tables.get(language).or(english).cloned();
        let table = self
            .tables
            .entry(language.to_owned())
            .or_insert(base.unwrap_or_default());
        table.extend(own);
        Ok(())
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

/// The language a site's strings file `name` is for, such as `fr` for
/// `fr.toml`. A regional file such as `de-CH.toml` is refused, because
/// strings are kept per language.
fn site_language(name: &str) -> Result<&str, RenderError> {
    name.strip_suffix(".toml")
        .filter(|stem| {
            Locale::try_new(stem)
                .is_ok_and(|locale| locale.as_str() == *stem && !stem.contains('-'))
        })
        .ok_or_else(|| RenderError::StringsFileName {
            name: name.to_owned(),
        })
}

/// One site strings file: the words the templates read, and the words of
/// the pages and emails other crates write, each in its own table, such
/// as `[comments]` for the page a refused comment shows.
#[derive(Debug, Default, Deserialize)]
struct SiteFile {
    /// The newsletter's emails and pages, as `khalid-newsletter` reads them.
    #[serde(default)]
    newsletter: toml::Table,
    /// The page a refused comment shows, as `khalid-comments` reads it.
    #[serde(default)]
    comments: toml::Table,
    /// The notes a static site shows where a form would be.
    #[serde(default)]
    notes: toml::Table,
    #[serde(flatten)]
    words: BTreeMap<String, String>,
}

/// The tables in the site's strings file `file`, whose words are `text`.
fn site_table(file: &str, text: &str) -> Result<SiteFile, RenderError> {
    toml::from_str(text).map_err(|source: toml::de::Error| {
        let line = source.span().map(|span| {
            text.get(..span.start)
                .map_or(0, |before| before.matches('\n').count())
                .saturating_add(1)
        });
        RenderError::SiteStrings {
            file: file.to_owned(),
            line,
            source: Box::new(source),
        }
    })
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
    fn every_key_reads_as_its_files_say() {
        let strings = Strings::load().unwrap();
        for (language, files) in [
            (
                "en",
                [
                    include_str!("../strings/en.toml"),
                    include_str!("../strings/privacy/en.toml"),
                ],
            ),
            ("de", [include_str!("../strings/de.toml"), ""]),
        ] {
            let mut want = BTreeMap::new();
            for file in files {
                let part: BTreeMap<String, String> = toml::from_str(file).unwrap();
                want.extend(part);
            }
            assert_eq!(strings.tables[language], want, "strings/{language}.toml");
        }
        assert_eq!(strings.languages().collect::<Vec<_>>(), ["de", "en"]);
    }

    #[test]
    fn a_part_file_fills_in_its_language() {
        let strings = Strings::load().unwrap();
        assert_eq!(strings.tables["en"]["privacy_title"], "Privacy");
        assert_eq!(strings.tables["de"]["privacy_title"], "Datenschutz");
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
