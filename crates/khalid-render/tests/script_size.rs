//! The public site's scripts stay small: each one under 5 KB compressed
//! (`docs/standards/typescript.md`), counted as the gzip copy a reader's
//! browser downloads when it does not take Brotli, the larger of the two.

use std::fmt::Write;

use khalid_render::{Asset, consent_script, reader_script};

/// The most one public script may weigh compressed: 5 KB of 1,024 bytes.
const LIMIT_BYTES: usize = 5 * 1024;

/// What to tell the person who made `script` too big, or `None` when it
/// fits.
#[cfg(test)]
fn over_limit(script: &Asset) -> Option<String> {
    let gzip = script.gzip().len();
    (gzip >= LIMIT_BYTES).then(|| {
        format!(
            "{} is {gzip} bytes gzipped ({} with Brotli), over the 5 KB limit for a public \
             script. Move the new behaviour into its own script that only the pages using it \
             load, or make the code shorter.",
            script.name(),
            script.brotli().len(),
        )
    })
}

#[test]
fn the_reader_script_is_under_5_kb_compressed() {
    let script = reader_script().unwrap();
    assert_eq!(over_limit(&script), None);
}

#[test]
fn the_consent_script_is_under_5_kb_compressed() {
    let script = consent_script().unwrap();
    assert_eq!(over_limit(&script), None);
}

#[test]
fn a_script_over_the_limit_is_refused_with_a_fix() {
    let big = (0..4000_u32).fold(String::new(), |mut out, n| {
        let _ = writeln!(out, "const v{n} = {};", n.wrapping_mul(2_654_435_761));
        out
    });
    let script = Asset::build(
        "big",
        "js",
        "text/javascript; charset=utf-8",
        big.into_bytes(),
    )
    .unwrap();
    let message = over_limit(&script).unwrap();
    assert!(message.starts_with(script.name()), "{message}");
    assert!(message.contains("over the 5 KB limit"), "{message}");
    assert!(message.contains("own script"), "{message}");
}
