//! The few tables of a WOFF2 file that a face's fallback is tuned from:
//! `head`, `hhea` and `OS/2`. WOFF2 packs a font's tables into one Brotli
//! stream after a directory; these three are never transformed, so they
//! are read straight from the unpacked stream (W3C WOFF2, sections 4 to 5).

use std::io::Read as _;

/// The most bytes the unpacked tables may hold. A Latin subset is well
/// under a megabyte; the cap stops a hostile file from unpacking far.
const MAX_TABLES_BYTES: u64 = 16 * 1024 * 1024;

/// The length of the fixed WOFF2 header, before the table directory.
const HEADER_BYTES: usize = 48;

/// The tag of the glyph location table, which WOFF2 may transform like
/// `glyf`. Written as bytes, since its four letters read as a misspelling.
const GLYPH_LOCATIONS: &[u8; 4] = &[0x6c, 0x6f, 0x63, 0x61];

/// The table tags WOFF2 names by their place in its known-tags list, up to
/// the glyph locations, the last this reader needs to tell apart.
const KNOWN_TAGS: [&[u8; 4]; 12] = [
    b"cmap",
    b"head",
    b"hhea",
    b"hmtx",
    b"maxp",
    b"name",
    b"OS/2",
    b"post",
    b"cvt ",
    b"fpgm",
    b"glyf",
    GLYPH_LOCATIONS,
];

/// The `head`, `hhea` and `OS/2` tables of a font.
#[derive(Debug, Default)]
pub(crate) struct Tables {
    pub(crate) head: Vec<u8>,
    pub(crate) hhea: Vec<u8>,
    pub(crate) os2: Vec<u8>,
}

/// The `head`, `hhea` and `OS/2` tables of the WOFF2 file `bytes`, or
/// `None` when it is not a single-font WOFF2 file these can be read from.
pub(crate) fn tables(bytes: &[u8]) -> Option<Tables> {
    let header = bytes.get(..HEADER_BYTES)?;
    if header.get(..4)? != b"wOF2" || header.get(4..8)? == b"ttcf" {
        return None;
    }
    let count = u16::from_be_bytes(header.get(12..14)?.try_into().ok()?);
    let packed = usize::try_from(u32::from_be_bytes(header.get(20..24)?.try_into().ok()?)).ok()?;
    let mut at = HEADER_BYTES;
    let mut entries = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let flags = *bytes.get(at)?;
        at = at.checked_add(1)?;
        let tag: [u8; 4] = match usize::from(flags & 0x3f) {
            0x3f => {
                let tag = bytes.get(at..at.checked_add(4)?)?.try_into().ok()?;
                at = at.checked_add(4)?;
                tag
            }
            known => **KNOWN_TAGS.get(known).unwrap_or(&b"    "),
        };
        let mut length = base128(bytes, &mut at)?;
        let version = flags >> 6;
        let shape_table = &tag == b"glyf" || &tag == GLYPH_LOCATIONS;
        if (shape_table && version == 0) || (!shape_table && version != 0) {
            length = base128(bytes, &mut at)?;
        }
        entries.push((tag, usize::try_from(length).ok()?));
    }
    let mut stream = Vec::new();
    brotli::Decompressor::new(bytes.get(at..at.checked_add(packed)?)?, 4096)
        .take(MAX_TABLES_BYTES)
        .read_to_end(&mut stream)
        .ok()?;
    let mut found = Tables::default();
    let mut offset = 0_usize;
    for (tag, length) in entries {
        let end = offset.checked_add(length)?;
        let table = match &tag {
            b"head" => &mut found.head,
            b"hhea" => &mut found.hhea,
            b"OS/2" => &mut found.os2,
            _ => {
                offset = end;
                continue;
            }
        };
        *table = stream.get(offset..end)?.to_vec();
        offset = end;
    }
    Some(found)
}

/// Reads one `UIntBase128` at `at`, moving `at` past it: up to five bytes of
/// seven bits each, high bits first, with no leading zero byte.
fn base128(bytes: &[u8], at: &mut usize) -> Option<u32> {
    let mut value: u32 = 0;
    for read in 0..5 {
        let byte = *bytes.get(*at)?;
        *at = at.checked_add(1)?;
        if (read == 0 && byte == 0x80) || value & 0xfe00_0000 != 0 {
            return None;
        }
        value = (value << 7) | u32::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}
