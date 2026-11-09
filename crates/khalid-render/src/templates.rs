//! The default templates built into the binary, and a theme's replacements for them.

use std::fs;
use std::io;
use std::path::Path;

use khalid_core::{SitePath, TemplateName};

use crate::RenderError;
use crate::archive;

/// Where a site keeps the templates that replace the defaults.
const THEME_TEMPLATES: &str = "theme/templates";

/// The default template `name`, such as `post.html`, from the built-in
/// archive. Only `.html` files there are templates; the stylesheets and
/// strings files beside them are not.
fn built_in(name: &str) -> Option<&'static str> {
    let html = Path::new(name)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("html"));
    if html {
        archive::built_in().get(name)
    } else {
        None
    }
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
    let mut sources: Vec<Source> = (archive::built_in().names())
        .filter_map(|name| {
            Some(Source {
                text: built_in(name)?.to_owned(),
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
        if built_in(&name).is_none() && !is_post_template(&name) {
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
    built_in(name).is_some()
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
