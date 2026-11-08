//! The text files built into the binary: the default templates, the shared
//! stylesheets, the reader script and the strings files, in one archive.

use std::collections::BTreeMap;
use std::io::Read as _;
use std::ops::Range;
use std::sync::OnceLock;

/// Every built-in file in one text, compressed with Brotli by the build
/// script, with each template comment emptied. Debug builds carry the same
/// archive, so development renders exactly what production renders.
static ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/archive.br"));

/// Where each file starts and ends in the archive's text: one line per
/// file, its name, its start and its end, split by tabs.
static INDEX: &str = include_str!(concat!(env!("OUT_DIR"), "/archive.index"));

/// The built-in files, read out of the archive the first time they are
/// needed and kept.
#[derive(Debug, Default)]
pub(crate) struct Archive {
    /// Every file's text, one after the other.
    text: String,
    /// Each file's place in `text`, by name: a template by its path from
    /// `templates/`, such as `post.html` or `sections/hero.html`, and the
    /// other files by their path from the crate, such as `assets/site.css`
    /// or `strings/en.toml`.
    files: BTreeMap<&'static str, Range<usize>>,
}

impl Archive {
    /// The archive the build script wrote. Were its bytes ever damaged, no
    /// built-in file would exist, which every test of a page catches.
    fn read() -> Self {
        let mut text = String::new();
        if brotli::Decompressor::new(ARCHIVE, 4096)
            .read_to_string(&mut text)
            .is_err()
        {
            return Self::default();
        }
        let files = INDEX
            .lines()
            .filter_map(|line| {
                let mut parts = line.split('\t');
                let name = parts.next()?;
                let start = parts.next()?.parse().ok()?;
                let end = parts.next()?.parse().ok()?;
                Some((name, start..end))
            })
            .collect();
        Self { text, files }
    }

    /// The file `name`, when the archive has one of that name.
    pub(crate) fn get(&self, name: &str) -> Option<&str> {
        self.files
            .get(name)
            .and_then(|place| self.text.get(place.clone()))
    }

    /// Every file's name, in name order.
    pub(crate) fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.files.keys().copied()
    }
}

/// The built-in files, once read out.
pub(crate) fn built_in() -> &'static Archive {
    static BUILT_IN: OnceLock<Archive> = OnceLock::new();
    BUILT_IN.get_or_init(Archive::read)
}

/// The built-in file `name`, such as `assets/site.css`, or nothing when
/// there is none, which this module's tests rule out for every file the
/// crate reads.
pub(crate) fn text(name: &str) -> &'static str {
    built_in().get(name).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    /// Every file under `folder` of this crate and its folders, as the
    /// archive names it.
    fn files_in(folder: &str) -> Vec<(String, String)> {
        let path = format!("{}/{folder}", env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        for entry in fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            let name = format!("{folder}/{}", entry.file_name().to_string_lossy());
            if entry.file_type().unwrap().is_dir() {
                files.extend(files_in(&name));
            } else {
                files.push((name, fs::read_to_string(entry.path()).unwrap()));
            }
        }
        files.sort();
        files
    }

    #[test]
    fn every_asset_and_strings_file_is_kept_byte_for_byte() {
        let mut files = files_in("assets");
        files.extend(files_in("strings"));
        assert!(files.len() >= 8, "{files:?}");
        for (name, text) in files {
            assert_eq!(built_in().get(&name), Some(text.as_str()), "{name}");
        }
    }

    #[test]
    fn a_name_the_archive_lacks_reads_as_nothing() {
        assert_eq!(built_in().get("assets/missing.css"), None);
        assert_eq!(text("assets/missing.css"), "");
    }
}
