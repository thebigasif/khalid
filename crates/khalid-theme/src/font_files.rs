//! The font files `theme sync` downloads and saves under `theme/fonts/`,
//! with the metrics of each face's tuned local fallback (G20). Kept apart
//! from `fonts.rs`, which finds the fonts, so each file stays under 600
//! lines (BL-SIZE-1).

use khalid_core::AbsoluteUrl;
use khalid_outbound::Outbound;
use read_fonts::tables::{head::Head, hhea::Hhea, os2::Os2};
use read_fonts::{FontData, FontRead};

use crate::fonts::FontSource;
use crate::woff2;

/// The most bytes one downloaded font file may hold before it is skipped.
/// A web font subset is tens of kilobytes; four megabytes leaves room for
/// whole families while keeping a hostile file from being read far.
pub const MAX_FONT_BYTES: usize = 4 * 1024 * 1024;

/// Who a sync says it is when it asks for a font stylesheet: a current
/// browser. Google Fonts answers each browser with the files it can read.
/// A modern one gets WOFF2 files split by subset, each with its
/// `unicode-range`; a client that sends no user agent gets one whole
/// TrueType file per weight, several times larger.
const BROWSER: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                       (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36";

/// The subset a sync keeps when the stylesheet splits a font by subset:
/// the Latin letters, digits and punctuation. Text in other scripts is
/// drawn in the stack's next font.
const KEPT_SUBSET: &str = "latin";

/// A self-hosted font file `theme sync` saved, as the renderer's tokens
/// carry it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedFont {
    /// The family the file gives a face, such as `Inter`.
    pub family: String,
    /// `normal` or `italic`.
    pub style: String,
    /// The weight the file carries, such as `400`.
    pub weight: String,
    /// The file name under `theme/fonts/`.
    pub path: String,
    /// The Unicode range the file covers, when the CSS named one.
    pub range: Option<String>,
    /// The face's metric overrides for its tuned local fallback, as CSS
    /// percentages (G20). `None` when the file could not be measured.
    pub metrics: Option<FontMetrics>,
}

/// A face's metrics as its fallback's `@font-face` overrides read them.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FontMetrics {
    /// `ascent-override`, such as `90.44%`.
    pub ascent: String,
    /// `descent-override`, such as `22.52%`.
    pub descent: String,
    /// `line-gap-override`, such as `0%`.
    pub line_gap: String,
    /// `size-adjust`, such as `107.12%`.
    pub size_adjust: String,
}

/// Downloads each rehostable font in `sources` and saves its files under
/// `into`, the site's `theme/fonts/` folder. A font that cannot be read is
/// skipped, never fatal: the blog keeps working with the next font in the
/// stack (G20). Returns what was saved, in the CSS's order.
///
/// # Errors
///
/// Returns an error only when the folder cannot be made; every network
/// miss is a skip.
pub async fn download(
    outbound: &Outbound,
    sources: &[FontSource],
    into: &std::path::Path,
) -> Result<Vec<SavedFont>, crate::FetchError> {
    let (saved, _) = download_and_report(outbound, sources, into).await?;
    Ok(saved)
}

/// As [`download`], also saying in plain words why each font was skipped.
/// A font comes from someone else's website, so nothing about it is
/// trusted: files over [`MAX_FONT_BYTES`] are not read further, and a file
/// that is not a WOFF2 web font is dropped, never a failure of the sync
/// (G20, maintainer review). When the stylesheet splits a font by subset,
/// only the Latin files are kept, and a variable file Google names once
/// per weight is saved once, for all of them.
///
/// # Errors
///
/// Returns an error only when the folder cannot be made; every network
/// miss is a skip with its sentence.
pub async fn download_and_report(
    outbound: &Outbound,
    sources: &[FontSource],
    into: &std::path::Path,
) -> Result<(Vec<SavedFont>, Vec<String>), crate::FetchError> {
    let mut saved = Vec::new();
    let mut skipped = Vec::new();
    for source in sources.iter().filter(|source| source.host().can_rehost()) {
        let Ok(url) = AbsoluteUrl::parse(source.url()) else {
            continue;
        };
        let Some(css) = stylesheet(outbound, &url).await else {
            skipped.push(format!(
                "The stylesheet for {} could not be read, so its fonts stay where they are.",
                source.family()
            ));
            continue;
        };
        if !css.status().is_success() {
            let status = css.status().as_u16();
            skipped.push(format!(
                "The stylesheet for {} answered {status}, so its fonts stay where they are.",
                source.family()
            ));
            continue;
        }
        let Ok(css) = std::str::from_utf8(css.body()) else {
            continue;
        };
        for face in kept(faces(css)) {
            match save(outbound, &face, into).await {
                Ok(metrics) => saved.push(SavedFont {
                    weight: face.weight(),
                    path: face.file(),
                    family: face.family,
                    style: face.style,
                    range: face.range,
                    metrics,
                }),
                Err(why) => skipped.push(why),
            }
        }
    }
    Ok((saved, skipped))
}

/// The font stylesheet at `url`, asked for as a [`BROWSER`], or `None`
/// when it cannot be read.
async fn stylesheet(outbound: &Outbound, url: &AbsoluteUrl) -> Option<http::Response<Vec<u8>>> {
    let request = http::Request::get(url.as_str())
        .header(http::header::USER_AGENT, BROWSER)
        .body(Vec::new())
        .ok()?;
    outbound.exchange(request).await.ok()
}

/// Downloads and saves one face, and measures it. The error is why the
/// face was left out, in plain words.
async fn save(
    outbound: &Outbound,
    face: &Face,
    into: &std::path::Path,
) -> Result<Option<FontMetrics>, String> {
    let named = format!("{} {}", face.family, face.weight());
    let url = AbsoluteUrl::parse(&face.url)
        .map_err(|_| format!("The address of {named} is not valid, so that face is left out."))?;
    let bytes = outbound.get(&url).await.map_err(|_| {
        format!("The file for {named} could not be read, so that face stays where it is.")
    })?;
    let body = bytes.body();
    if !(200..300).contains(&bytes.status()) || body.is_empty() {
        let status = bytes.status();
        return Err(format!(
            "The file for {named} answered {status}, so that face stays where it is."
        ));
    }
    if body.len() > MAX_FONT_BYTES {
        return Err(format!(
            "The file for {named} is {} KB, over the {} KB a font may be, so it stays where it is.",
            body.len() / 1024,
            MAX_FONT_BYTES / 1024
        ));
    }
    if !body.starts_with(b"wOF2") {
        return Err(format!(
            "The file for {named} is not a WOFF2 web font, so it stays where it is."
        ));
    }
    write_face(into, &face.file(), body).map_err(|_| {
        format!(
            "The file for {named} could not be saved beside the theme, so that face is left out."
        )
    })?;
    Ok(measure(body))
}

/// The face's metric overrides for a local Arial fallback, as CSS
/// percentages, from the WOFF2 file `bytes`. `None` when the file is not a
/// readable face. The formula is the one Capsize popularised: each
/// override is the face's own value over its size, divided by Arial's, as
/// a percentage of Arial's.
fn measure(bytes: &[u8]) -> Option<FontMetrics> {
    // Arial's metrics, in font units of 2048 per em: ascent 1854,
    // descent 434, line gap 67, average character width 1934.
    const ARIAL_ASCENT: f64 = 1854.0;
    const ARIAL_DESCENT: f64 = 434.0;
    const ARIAL_LINE_GAP: f64 = 67.0;
    const ARIAL_WIDTH: f64 = 1934.0;
    // The tables are read through read-fonts' typed API, so a malformed
    // file is an Err, never a panic or a wrong offset (maintainer review).
    let tables = woff2::tables(bytes)?;
    let head = Head::read(FontData::new(&tables.head)).ok()?;
    let hhea = Hhea::read(FontData::new(&tables.hhea)).ok()?;
    let os2 = Os2::read(FontData::new(&tables.os2)).ok()?;
    let per_em = f64::from(head.units_per_em());
    let ascent = f64::from(hhea.ascender().to_i16()) / per_em;
    let descent = f64::from(hhea.descender().to_i16()).abs() / per_em;
    let line_gap = f64::from(hhea.line_gap().to_i16()) / per_em;
    // The average character width is OS/2's `xAvgCharWidth`. Faces without
    // the field cannot be tuned.
    let width = f64::from(os2.x_avg_char_width()) / per_em;
    let percent = |value: f64, arial: f64| format!("{:.2}%", value / arial * 100.0);
    Some(FontMetrics {
        ascent: percent(ascent, ARIAL_ASCENT / 2048.0),
        descent: percent(descent, ARIAL_DESCENT / 2048.0),
        line_gap: percent(line_gap, ARIAL_LINE_GAP / 2048.0),
        size_adjust: percent(width, ARIAL_WIDTH / 2048.0),
    })
}

/// One `@font-face` rule's parts. Every part that reaches the blog's
/// stylesheet is checked here, since the CSS is someone else's.
#[derive(Debug)]
struct Face {
    /// Letters, digits, spaces and hyphens only.
    family: String,
    /// `normal` or `italic`.
    style: String,
    /// The lightest and boldest weight the file carries.
    weights: (u16, u16),
    url: String,
    /// The subset the comment above the rule names, such as `latin`.
    subset: Option<String>,
    /// Only `U+` code points, ranges and commas.
    range: Option<String>,
}

impl Face {
    /// The weight as CSS writes it: `400`, or `400 700` for a variable file.
    fn weight(&self) -> String {
        match self.weights {
            (low, high) if low == high => low.to_string(),
            (low, high) => format!("{low} {high}"),
        }
    }

    /// The name the face saves under, built from its parts and never taken
    /// from the CSS, such as `inter-latin-400-700.woff2`, so no face
    /// overwrites another.
    fn file(&self) -> String {
        let mut parts = vec![self.family.clone()];
        parts.extend(self.subset.clone());
        if self.style == "italic" {
            parts.push("italic".to_owned());
        }
        parts.push(self.weight().replace(' ', "-"));
        let name = parts.join("-").to_ascii_lowercase().replace(' ', "-");
        format!("{name}.woff2")
    }
}

/// The `@font-face` rules in a Google Fonts stylesheet, in order. The
/// comment before each rule, such as `/* latin */`, names its subset. A
/// rule with a part that is not what a font stylesheet holds is left out.
fn faces(css: &str) -> Vec<Face> {
    let mut found = Vec::new();
    let mut before = "";
    for part in css.split("@font-face") {
        let head = before;
        before = part;
        let Some(rule) = part.find('}').and_then(|end| part.get(..end)) else {
            continue;
        };
        let family = value(rule, "font-family").unwrap_or_default();
        let plain = |c: char| c.is_ascii_alphanumeric() || c == ' ' || c == '-';
        if family.is_empty() || !family.chars().all(plain) {
            continue;
        }
        let style = value(rule, "font-style").unwrap_or_else(|| "normal".to_owned());
        let weight = value(rule, "font-weight").unwrap_or_else(|| "400".to_owned());
        let (Some(span), true) = (weights(&weight), ["normal", "italic"].contains(&&*style)) else {
            continue;
        };
        let Some(url) = value(rule, "src").and_then(|src| url_of(&src)) else {
            continue;
        };
        let range = value(rule, "unicode-range").filter(|range| {
            range
                .chars()
                .all(|c| c.is_ascii_hexdigit() || "U+?-, ".contains(c))
        });
        found.push(Face {
            subset: subset(head).filter(|name| name.chars().all(plain)),
            family,
            style,
            weights: span,
            url,
            range,
        });
    }
    found
}

/// The faces a sync keeps of `faces`: only the Latin ones when the
/// stylesheet names subsets, and each file once. Google repeats a variable
/// file once per weight asked for, so those become one face covering
/// every weight.
fn kept(faces: Vec<Face>) -> Vec<Face> {
    let split = faces.iter().any(|face| face.subset.is_some());
    let mut kept: Vec<Face> = Vec::new();
    for face in faces {
        if split && face.subset.as_deref() != Some(KEPT_SUBSET) {
            continue;
        }
        let same = kept.iter_mut().find(|other| {
            (
                other.url.as_str(),
                other.family.as_str(),
                other.style.as_str(),
            ) == (face.url.as_str(), face.family.as_str(), face.style.as_str())
        });
        if let Some(same) = same {
            same.weights = (
                same.weights.0.min(face.weights.0),
                same.weights.1.max(face.weights.1),
            );
        } else if !kept.iter().any(|other| other.file() == face.file()) {
            kept.push(face);
        }
    }
    kept
}

/// The weights `400` or `100 900` name, each a number from 1 to 1000.
fn weights(text: &str) -> Option<(u16, u16)> {
    let mut numbers = text.split_whitespace().map(|n| n.parse::<u16>().ok());
    let low = numbers.next()??;
    let high = numbers.next().unwrap_or(Some(low))?;
    let fits = (1..=1000).contains(&low) && (low..=1000).contains(&high);
    (fits && numbers.next().is_none()).then_some((low, high))
}

/// The subset the comment closing `before` names, such as `latin`.
fn subset(before: &str) -> Option<String> {
    let line = before.lines().rev().find(|line| !line.trim().is_empty())?;
    let name = line.trim().strip_prefix("/*")?.strip_suffix("*/")?.trim();
    (!name.is_empty() && name.len() < 24).then(|| name.to_owned())
}

/// The value of `property` in `rule`, quotes stripped.
fn value(rule: &str, property: &str) -> Option<String> {
    let start = rule.find(property)? + property.len();
    let rest = rule.get(start..)?;
    let colon = rest.find(':')?;
    let end = rest.get(colon..).and_then(|tail| tail.find(';'))? + colon;
    let value = rest
        .get(colon + 1..end)?
        .trim()
        .trim_matches('\'')
        .trim_matches('"');
    Some(value.to_owned())
}

/// The first `url(...)` in `src`.
fn url_of(src: &str) -> Option<String> {
    let start = src.find("url(")? + 4;
    let rest = src.get(start..)?;
    let end = rest.find(')')?;
    Some(
        rest.get(..end)?
            .trim_matches('\'')
            .trim_matches('"')
            .to_owned(),
    )
}

/// Writes the bytes of one face as `name`, which [`Face::file`] built,
/// making the folder first.
fn write_face(into: &std::path::Path, name: &str, bytes: &[u8]) -> std::io::Result<()> {
    std::fs::create_dir_all(into)?;
    let mut file = into.to_path_buf();
    file.push(name);
    std::fs::write(file, bytes)
}
