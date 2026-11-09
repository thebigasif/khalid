//! Packs `catalogue.toml` with Brotli into cargo's output folder, so the
//! binary carries about 8 KB of messages instead of 33 KB of text. The
//! crate unpacks it once, on first use.

use std::ffi::OsString;
use std::io::Write as _;
use std::path::PathBuf;
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut source = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    source.push("catalogue.toml");
    println!("cargo::rerun-if-changed={}", source.display());
    let text = fs::read(&source)?;
    let mut out = PathBuf::from(out_dir()?);
    out.push("catalogue.toml.br");
    fs::write(&out, brotli(&text)?)
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
