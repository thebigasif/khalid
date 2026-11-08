//! The public site's one script: polish on top of pages that read
//! completely without it.

use std::sync::LazyLock;

use serde::Serialize;

use crate::archive;
use crate::asset::{hashed_name, integrity_of};
use crate::{Asset, RenderError};

/// What the script does, in plain JavaScript with no dependencies: copy a
/// heading's address, open an enlarged image in a dialog, open search with
/// `/` or Ctrl+K, and mark the section being read. Its name in the
/// built-in archive.
const READER_JS: &str = "assets/reader.js";

/// The script as served: [`READER_JS`] without its comment lines, which
/// explain the code to the people who change it but not to browsers. Every
/// comment in the file is a whole line, so no code is touched.
static SERVED_JS: LazyLock<String> = LazyLock::new(|| {
    let script = archive::text(READER_JS);
    script
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .fold(String::with_capacity(script.len()), |mut out, line| {
            out.push_str(line);
            out.push('\n');
            out
        })
});

/// The script's name before its hash, and its extension.
const STEM: &str = "reader";
const EXTENSION: &str = "js";

/// Builds the reader script (plan section 4.12) as a hashed asset with its
/// compressed copies. Pages load it deferred, so it never holds up reading.
///
/// # Errors
///
/// Returns [`RenderError::Compress`] if a compressor fails.
pub fn reader_script() -> Result<Asset, RenderError> {
    Asset::build(
        STEM,
        EXTENSION,
        "text/javascript; charset=utf-8",
        SERVED_JS.as_bytes().to_vec(),
    )
}

/// The reader script without its compressed copies: the name and integrity
/// value [`reader_script`] gives it, and its bytes. Making them costs one
/// hash, so a server that compresses as it answers can start at once.
pub fn reader_script_file() -> ScriptFile {
    let body = SERVED_JS.as_bytes();
    ScriptFile {
        name: hashed_name(STEM, EXTENSION, body),
        integrity: integrity_of(body),
        body,
    }
}

/// A script's hashed name, integrity value and bytes, from
/// [`reader_script_file`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptFile {
    /// The hashed file name, such as `reader.3f2a9c1e.js`.
    pub name: String,
    /// Its SHA-256 as a Subresource Integrity value.
    pub integrity: String,
    /// The file as built.
    pub body: &'static [u8],
}

/// The `script` tag every page carries when the site serves the reader
/// script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScriptTag {
    /// The script's address, such as `/blog/assets/reader.3f2a9c1e.js`.
    pub src: String,
    /// Its [`Asset::integrity`], so the browser runs only these bytes.
    pub integrity: String,
}

impl ScriptTag {
    /// The tag for `script`, served from the folder `assets`, such as
    /// `/blog/assets/`.
    pub fn new(script: &Asset, assets: &str) -> Self {
        Self {
            src: format!("{assets}{}", script.name()),
            integrity: script.integrity(),
        }
    }

    /// The tag for `script`, served from the folder `assets`, without
    /// building its compressed copies.
    pub fn of_file(script: &ScriptFile, assets: &str) -> Self {
        Self {
            src: format!("{assets}{}", script.name),
            integrity: script.integrity.clone(),
        }
    }
}
