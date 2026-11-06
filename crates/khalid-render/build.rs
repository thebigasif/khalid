//! Writes every default template (`templates/`, with the shared section
//! parts in `templates/sections/`) into one text compressed with Brotli in
//! cargo's output folder, with an index of where each file starts and ends.
//! The binary embeds both. Each comment is emptied first: a comment renders
//! nothing either way, so every page is the same, and the binary carries
//! neither the comments' words nor the text uncompressed.

use std::ffi::OsString;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("templates");
    println!("cargo::rerun-if-changed={}", root.display());
    let mut files = Vec::new();
    collect(&root, "", &mut files)?;
    // The order is fixed, so every build writes the same archive.
    files.sort();
    let (mut all, mut index) = (String::new(), String::new());
    for (name, text) in &files {
        let start = all.len();
        all.push_str(text);
        writeln!(index, "{name}\t{start}\t{}", all.len()).map_err(io::Error::other)?;
    }
    let mut out = PathBuf::from(out_dir()?);
    out.push("templates.br");
    fs::write(&out, brotli(all.as_bytes())?)?;
    out.set_file_name("templates.index");
    fs::write(&out, index)
}

/// Adds every `.html` file under `folder` to `files`, named by its path
/// from `templates/` with `/` between folders, such as `sections/hero.html`.
fn collect(folder: &Path, prefix: &str, files: &mut Vec<(String, String)>) -> io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        if entry.file_type()?.is_dir() {
            collect(&entry.path(), &format!("{name}/"), files)?;
        } else if name.ends_with(".html") {
            files.push((name, strip_comments(&fs::read_to_string(entry.path())?)));
        }
    }
    Ok(())
}

/// Empties every `{# … #}` comment of a template and keeps its `-` marks,
/// which trim the space around it.
fn strip_comments(template: &str) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{#") {
        out.push_str(rest.get(..start).unwrap_or_default());
        let comment = rest.get(start..).unwrap_or_default();
        let Some(end) = comment.find("#}") else {
            out.push_str(comment);
            return out;
        };
        let inner = comment.get(2..end).unwrap_or_default();
        out.push_str("{#");
        if inner.starts_with('-') {
            out.push('-');
        }
        out.push(' ');
        if inner.len() > 1 && inner.ends_with('-') {
            out.push('-');
        }
        out.push_str("#}");
        rest = comment.get(end.saturating_add(2)..).unwrap_or_default();
    }
    out.push_str(rest);
    out
}

/// Where cargo wants this script's output. Cargo hands it over only as an
/// environment variable, and a build script cannot read `Config`, so this
/// is the one setting read here (BL-RUN-2 is about the running program).
fn out_dir() -> io::Result<OsString> {
    std::env::vars_os()
        .find_map(|(name, value)| (name == "OUT_DIR").then_some(value))
        .ok_or_else(|| io::Error::other("cargo did not set OUT_DIR"))
}

/// `text` compressed with Brotli at its highest quality: the build pays
/// once, and every binary is smaller.
fn brotli(text: &[u8]) -> io::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(text.len() / 4);
    {
        let mut writer = brotli::CompressorWriter::new(&mut out, 4096, 11, 24);
        writer.write_all(text)?;
        writer.flush()?;
    }
    Ok(out)
}
