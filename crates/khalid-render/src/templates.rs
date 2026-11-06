//! The default templates built into the binary, and a theme's replacements for them.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::io::Read as _;
use std::ops::Range;
use std::path::Path;
use std::sync::OnceLock;

use khalid_core::{SitePath, TemplateName};

use crate::RenderError;

/// Where a site keeps the templates that replace the defaults.
const THEME_TEMPLATES: &str = "theme/templates";

/// Every default template in one text, compressed with Brotli by the build
/// script, with each comment emptied. Debug builds carry the same archive,
/// so development renders exactly what production renders.
static ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/templates.br"));

/// Where each template starts and ends in the archive's text: one line per
/// file, its name, its start and its end, split by tabs.
static INDEX: &str = include_str!(concat!(env!("OUT_DIR"), "/templates.index"));

/// The default templates, read out of the archive the first time a renderer
/// asks for them and kept.
#[derive(Debug, Default)]
struct Defaults {
    /// Every template's text, one after the other.
    text: String,
    /// Each template's place in `text`, by name, such as `post.html` or
    /// `sections/hero.html`.
    files: BTreeMap<&'static str, Range<usize>>,
}

impl Defaults {
    /// The archive the build script wrote. Were its bytes ever damaged, no
    /// default template would exist, which every test of a page catches.
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

    /// The template `name`, when there is a default of that name.
    fn get(&self, name: &str) -> Option<&str> {
        self.files
            .get(name)
            .and_then(|place| self.text.get(place.clone()))
    }
}

/// The default templates, once read out.
fn built_in() -> &'static Defaults {
    static DEFAULTS: OnceLock<Defaults> = OnceLock::new();
    DEFAULTS.get_or_init(Defaults::read)
}

/// One template: its name and its source.
pub(crate) struct Source {
    pub(crate) name: String,
    pub(crate) text: String,
}

/// Every default template, in name order: the page templates, and the
/// shared section parts under `sections/`, such as `sections/hero.html`,
/// which pack templates import. Only `.html` files are templates, so the
/// parts' README stays out.
pub(crate) fn defaults() -> Vec<Source> {
    let defaults = built_in();
    let mut sources: Vec<Source> = (defaults.files.keys())
        .filter_map(|&name| {
            Some(Source {
                text: defaults.get(name)?.to_owned(),
                name: name.to_owned(),
            })
        })
        .collect();
    sources.sort_by(|a, b| a.name.cmp(&b.name));
    sources
}

/// The templates in `site_root/theme/templates/`, in name order: files that
/// replace a default, and post templates such as `post-wide.html` that a
/// post asks for with `template: wide`. A missing
/// folder means no replacements.
///
/// # Errors
///
/// Returns [`RenderError::UnknownTemplate`] for a file or folder whose name is
/// neither a post template nor one that
/// a default template has, [`RenderError::ThemePath`] when a symlink leads out
/// of the site folder, and [`RenderError::ReadTemplate`] when a file cannot
/// be read.
pub(crate) fn overrides(site_root: &Path) -> Result<Vec<Source>, RenderError> {
    let folder = SitePath::within(site_root, THEME_TEMPLATES)?.resolve()?;
    let entries = match fs::read_dir(&folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => {
            return Err(RenderError::ReadTemplate {
                path: folder,
                source,
            });
        }
    };
    let mut sources = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| RenderError::ReadTemplate {
            path: folder.clone(),
            source,
        })?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if built_in().get(&name).is_none() && !is_post_template(&name) {
            return Err(RenderError::UnknownTemplate { name });
        }
        let path = SitePath::within(site_root, &format!("{THEME_TEMPLATES}/{name}"))?.resolve()?;
        let text = fs::read_to_string(&path)
            .map_err(|source| RenderError::ReadTemplate { path, source })?;
        sources.push(Source { name, text });
    }
    sources.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(sources)
}

/// Whether `name` is one of the default templates, such as `post.html`.
pub(crate) fn is_default(name: &str) -> bool {
    built_in().get(name).is_some()
}

/// Whether `name` is a pack's own part, such as `parts-locale.html`: a file
/// of macros the pack's other templates import. Only a built-in pack adds
/// one; a site's `theme/templates/` cannot.
pub(crate) fn is_pack_part(name: &str) -> bool {
    name.strip_prefix("parts-")
        .and_then(|rest| rest.strip_suffix(".html"))
        .is_some_and(|inner| {
            !inner.is_empty() && inner.bytes().all(|b| b.is_ascii_lowercase() || b == b'-')
        })
}

/// Whether `name` is a post template a post may ask for, such as
/// `post-wide.html` for `template: wide`.
pub(crate) fn is_post_template(name: &str) -> bool {
    let inner = name
        .strip_prefix("post-")
        .and_then(|n| n.strip_suffix(".html"));
    inner.is_some_and(|inner| TemplateName::try_new(inner).is_ok())
}
