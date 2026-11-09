//! The texts the binary hands out whole, packed with Brotli by the build
//! script: the agents' guide, the starter posts and the staff toolbar's
//! styles. Read out once, on first use, and kept.

use std::collections::BTreeMap;
use std::io::Read as _;
use std::ops::Range;
use std::sync::OnceLock;

/// Every packed file's text, one after the other.
static PACKED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/texts.br"));

/// Where each file starts and ends in the unpacked text: one line per
/// file, its path from the crate's folder, its start and its end, split by
/// tabs.
static INDEX: &str = include_str!(concat!(env!("OUT_DIR"), "/texts.index"));

/// The unpacked text and each file's place in it.
#[derive(Debug, Default)]
struct Texts {
    text: String,
    files: BTreeMap<&'static str, Range<usize>>,
}

/// The texts, read out once.
fn texts() -> &'static Texts {
    static TEXTS: OnceLock<Texts> = OnceLock::new();
    TEXTS.get_or_init(|| {
        let mut text = String::new();
        // Were the bytes ever damaged, every text would read empty, which
        // the tests of each file would catch first.
        if brotli::Decompressor::new(PACKED, 4096)
            .read_to_string(&mut text)
            .is_err()
        {
            return Texts::default();
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
        Texts { text, files }
    })
}

/// The text of the packed file `name`, such as `src/agents.md`, or nothing
/// when no such file was packed.
pub(crate) fn text(name: &str) -> &'static str {
    let texts = texts();
    texts
        .files
        .get(name)
        .and_then(|place| texts.text.get(place.clone()))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_packed_file_reads_as_written() {
        let files = [
            ("src/agents.md", include_str!("agents.md")),
            ("src/staff_toolbar.css", include_str!("staff_toolbar.css")),
            (
                "starters/content/en/comparison.md",
                include_str!("../starters/content/en/comparison.md"),
            ),
            (
                "starters/content/de/review.md",
                include_str!("../starters/content/de/review.md"),
            ),
        ];
        for (name, want) in files {
            assert_eq!(text(name), want, "{name}");
        }
        assert_eq!(texts().files.len(), 10);
        assert_eq!(text("src/missing.md"), "");
    }
}
