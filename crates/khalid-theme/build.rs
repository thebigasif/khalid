//! Writes each template's stylesheet without its comments, indentation and
//! blank lines into cargo's output folder, where the packs embed it from.
//! The renderer strips the same things when it builds a site's stylesheet,
//! and stripping twice gives what stripping once gives, so every site's
//! stylesheet is unchanged; the binary just stops carrying the comments.

use std::ffi::OsString;
use std::path::PathBuf;
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut packs = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    packs.push("packs");
    println!("cargo::rerun-if-changed={}", packs.display());
    let mut out = PathBuf::from(out_dir()?);
    out.push("packs");
    for pack in fs::read_dir(&packs)? {
        let pack = pack?;
        if !pack.file_type()?.is_dir() {
            continue;
        }
        let mut into = out.clone();
        into.push(pack.file_name());
        fs::create_dir_all(&into)?;
        for file in fs::read_dir(pack.path())? {
            let file = file?;
            let path = file.path();
            if path.extension().is_some_and(|extension| extension == "css") {
                let mut target = into.clone();
                target.push(file.file_name());
                fs::write(target, minify(&fs::read_to_string(&path)?))?;
            }
        }
    }
    Ok(())
}

/// Where cargo wants this script's output. Cargo hands it over only as an
/// environment variable, and a build script cannot read `Config`, so this
/// is the one setting read here (BL-RUN-2 is about the running program).
fn out_dir() -> io::Result<OsString> {
    std::env::vars_os()
        .find_map(|(name, value)| (name == "OUT_DIR").then_some(value))
        .ok_or_else(|| io::Error::other("cargo did not set OUT_DIR"))
}

/// Removes comments, indentation and blank lines, as
/// `khalid_render::stylesheet` does, so the two always agree.
fn minify(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        push_lines(&mut out, rest.get(..start).unwrap_or_default());
        rest = rest
            .get(start..)
            .and_then(|from| {
                from.find("*/")
                    .and_then(|end| from.get(end.saturating_add(2)..))
            })
            .unwrap_or_default();
    }
    push_lines(&mut out, rest);
    out
}

fn push_lines(out: &mut String, text: &str) {
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        out.push_str(line);
        out.push('\n');
    }
}
