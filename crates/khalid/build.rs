//! Packs the texts the binary hands out whole, the agents' guide, the
//! starter posts and the staff toolbar's styles, into one text compressed
//! with Brotli in cargo's output folder, with an index of where each file
//! starts and ends (`assets::texts`). Kept plain, they took 15 KB of a
//! binary with a size budget; packed they take about a third.

use std::ffi::OsString;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::PathBuf;
use std::{fs, io};

/// Every packed file, by its path from this crate's folder.
const FILES: [&str; 10] = [
    "src/agents.md",
    "src/staff_toolbar.css",
    "starters/content/en/comparison.md",
    "starters/content/en/alternatives.md",
    "starters/content/en/best-of.md",
    "starters/content/en/review.md",
    "starters/content/de/comparison.md",
    "starters/content/de/alternatives.md",
    "starters/content/de/best-of.md",
    "starters/content/de/review.md",
];

fn main() -> io::Result<()> {
    let (mut all, mut index) = (String::new(), String::new());
    for name in FILES {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(name);
        println!("cargo::rerun-if-changed={}", path.display());
        let start = all.len();
        all.push_str(&fs::read_to_string(&path)?);
        writeln!(index, "{name}\t{start}\t{}", all.len()).map_err(io::Error::other)?;
    }
    let mut out = PathBuf::from(out_dir()?);
    out.push("texts.br");
    fs::write(&out, brotli(all.as_bytes())?)?;
    out.set_file_name("texts.index");
    fs::write(&out, index)
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
