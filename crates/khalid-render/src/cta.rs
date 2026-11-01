//! The site's call to action wherever a body places `::cta`: the content
//! crate leaves a mark, and the page swaps it for the band its template
//! draws from `[template.cta]` (templates spec 4.4).

use minijinja::value::Value;
use minijinja::{Error, ErrorKind};

/// The mark `khalid-content` writes for each `::cta` block. A test checks
/// it against that crate's own.
const CTA_MARK: &str = "<!--khalid-cta-->\n";

/// The `place_cta(band)` filter: `body` with each `::cta` mark swapped for
/// `band`, the HTML a template drew, or for nothing when `band` is empty
/// because the site has no call to action.
///
/// # Errors
///
/// An [`ErrorKind::InvalidOperation`] error when `body` is not text, or
/// when `band` is text a template did not draw, which would print
/// unescaped.
pub(crate) fn place_cta(body: &Value, band: &Value) -> Result<Value, Error> {
    let fail = |what| Error::new(ErrorKind::InvalidOperation, what);
    let html = body
        .as_str()
        .ok_or_else(|| fail("place_cta needs the body's HTML"))?;
    if !html.contains(CTA_MARK) {
        return Ok(body.clone());
    }
    let band = match band.as_str() {
        Some("") => String::new(),
        Some(drawn) if band.is_safe() => format!("{drawn}\n"),
        _ => return Err(fail("place_cta needs the band's HTML, drawn by a template")),
    };
    Ok(Value::from_safe_string(html.replace(CTA_MARK, &band)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_a_band_that_is_plain_text() {
        let body = Value::from_safe_string(format!("<p>a</p>\n{CTA_MARK}"));
        let error = place_cta(&body, &Value::from("<b>x</b>")).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidOperation);
        let empty = place_cta(&body, &Value::from("")).unwrap();
        assert_eq!(empty.as_str(), Some("<p>a</p>\n"));
    }
}
