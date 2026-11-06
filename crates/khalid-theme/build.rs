//! Writes every template's stylesheet, without its comments, indentation
//! and blank lines, and every template file into one text, compressed with
//! Brotli, in cargo's output folder, with an index of where each file
//! starts and ends. The binary embeds both (`PackText`). One archive is
//! about 40% smaller than one per file, because the templates share many
//! patterns. The renderer strips the same things when it
//! builds a site's stylesheet, and stripping twice gives what stripping
//! once gives, so every site's stylesheet is unchanged; the binary just
//! stops carrying the comments, and carries the rest compressed.
//!
//! The shared section parts (`khalid-render/templates/sections/`) keep
//! their rules in `packs/_sections/<part>.css`. A pack ships a part's
//! rules only when one of its template files imports the part, such as
//! `{% from "sections/hero.html" import hero %}`: the script puts the
//! part's rules before the pack's own, so the pack can still change them.
//! A folder whose name starts with `_` is not a pack.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut packs = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    packs.push("packs");
    println!("cargo::rerun-if-changed={}", packs.display());
    let mut files = Vec::new();
    for pack in fs::read_dir(&packs)? {
        let pack = pack?;
        let shared = pack.file_name().to_string_lossy().starts_with('_');
        if !pack.file_type()?.is_dir() || shared {
            continue;
        }
        let parts = section_rules(&packs, &pack.path())?;
        for file in fs::read_dir(pack.path())? {
            let path = file?.path();
            let kind = path.extension().and_then(|extension| extension.to_str());
            let text = match kind {
                Some("css") => minify(&format!("{parts}{}", fs::read_to_string(&path)?)),
                Some("html") => fs::read_to_string(&path)?,
                _ => continue,
            };
            let name = format!(
                "{}/{}",
                pack.file_name().to_string_lossy(),
                path.file_name().unwrap_or_default().to_string_lossy()
            );
            files.push((name, text));
        }
    }
    // The order is fixed, so every build writes the same archive.
    files.sort();
    let (mut all, mut index) = (String::new(), String::new());
    for (name, text) in &files {
        let start = all.len();
        all.push_str(text);
        writeln!(index, "{name}\t{start}\t{}", all.len()).map_err(io::Error::other)?;
    }
    let mut out = PathBuf::from(out_dir()?);
    out.push("packs.br");
    fs::write(&out, brotli(all.as_bytes())?)?;
    out.set_file_name("packs.index");
    fs::write(&out, index)?;
    Ok(())
}

/// The rules of every section part the pack's template files import, in
/// name order, read from `packs/_sections/<part>.css`. A part with no
/// rules file stops the build, so no pack ships a part unstyled.
fn section_rules(packs: &Path, pack: &Path) -> io::Result<String> {
    let mut parts = BTreeSet::new();
    for file in fs::read_dir(pack)? {
        let path = file?.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("html") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        for (at, _) in text.match_indices("\"sections/") {
            let rest = text.get(at.saturating_add(10)..).unwrap_or_default();
            if let Some((name, _)) = rest.split_once(".html\"") {
                parts.insert(name.to_owned());
            }
        }
    }
    let mut rules = String::new();
    for name in parts {
        let mut path = packs.to_path_buf();
        path.push("_sections");
        path.push(format!("{name}.css"));
        let text = fs::read_to_string(&path)
            .map_err(|error| io::Error::other(format!("{}: {error}", path.display())))?;
        rules.push_str(&text);
        rules.push('\n');
    }
    Ok(rules)
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
