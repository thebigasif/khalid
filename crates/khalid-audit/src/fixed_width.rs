//! Finds fixed pixel widths wider than a phone screen (`docs/plan.md`
//! section 6, "Fixed width").

use scraper::ElementRef;

/// The narrowest screen every page must fit, in CSS pixels (`FRONTEND.md`,
/// "Responsive on every device").
const PHONE: f64 = 320.0;

/// The elements whose `width` attribute sets how wide they lay out. An
/// image, video or frame's `width` is only its shape, which the default
/// styles scale down to fit, so those are left out.
const LAID_OUT: [&str; 7] = ["table", "td", "th", "col", "colgroup", "hr", "pre"];

/// What `element` sets wider than a phone screen, in words for the report,
/// or `None` when it fits.
///
/// It reads a `style` attribute's `width` and `min-width`, the `width`
/// attribute of a table and its parts, and every rule of a `<style>`
/// block. A `max-width` only caps a width, so it is fine.
pub(crate) fn part(element: ElementRef<'_>) -> Option<String> {
    let name = element.value().name();
    let attr = |attribute: &str| element.value().attr(attribute);
    if name == "style" {
        let css: String = element.text().collect();
        return style_block(&css)
            .map(|(selector, width)| format!("\"{selector}\" with \"{width}\" in a style block"));
    }
    if let Some(width) = attr("style").and_then(too_wide) {
        return Some(format!("a {name} with \"{width}\""));
    }
    let width = attr("width").filter(|_| LAID_OUT.contains(&name))?;
    pixels(width)
        .filter(|wide| *wide > PHONE)
        .map(|_| format!("a {name} with width=\"{}\"", width.trim()))
}

/// The first `width` or `min-width` in `declarations` wider than a phone,
/// as `width: 800px`.
fn too_wide(declarations: &str) -> Option<String> {
    declarations.split(';').find_map(|declaration| {
        let (property, value) = declaration.split_once(':')?;
        let property = property.trim().to_ascii_lowercase();
        if property != "width" && property != "min-width" {
            return None;
        }
        let value = value.trim().to_ascii_lowercase();
        let value = value.trim_end_matches("!important").trim();
        let size = value.strip_suffix("px")?;
        pixels(size)
            .filter(|wide| *wide > PHONE)
            .map(|_| format!("{property}: {value}"))
    })
}

/// A number of pixels, with or without `px`, or `None` for any other
/// value, such as `100%` or `40rem`.
fn pixels(value: &str) -> Option<f64> {
    let value = value.trim();
    value
        .strip_suffix("px")
        .unwrap_or(value)
        .trim()
        .parse()
        .ok()
}

/// The first rule of `css` that sets a width wider than a phone, as its
/// selector and the width.
///
/// Rules inside an `@media` or `@container` block for wide screens or
/// print are left out, since a phone never applies them.
fn style_block(css: &str) -> Option<(String, String)> {
    let css = without_comments(css);
    // One entry per open brace: the rule's selector, or `None` for an
    // at-rule, and whether a phone skips what is inside.
    let mut open: Vec<(Option<String>, bool)> = Vec::new();
    let mut text = String::new();
    for character in css.chars() {
        match character {
            '{' => {
                let prelude = text.trim().to_owned();
                text.clear();
                let skipped = open.last().is_some_and(|(_, skipped)| *skipped);
                if prelude.starts_with('@') {
                    open.push((None, skipped || not_for_phones(&prelude)));
                } else {
                    open.push((Some(prelude), skipped));
                }
            }
            '}' => {
                if let Some((Some(selector), false)) = open.pop()
                    && let Some(width) = too_wide(&text)
                {
                    return Some((selector, width));
                }
                text.clear();
            }
            _ => text.push(character),
        }
    }
    None
}

/// Whether the at-rule `prelude` holds rules a phone never applies: a
/// media or container query for wide screens, or for print.
fn not_for_phones(prelude: &str) -> bool {
    let prelude = prelude.to_ascii_lowercase();
    (prelude.starts_with("@media") || prelude.starts_with("@container"))
        && (prelude.contains("min-width") || prelude.contains('>') || prelude.contains("print"))
}

/// `css` with its `/* ... */` comments taken out.
fn without_comments(css: &str) -> String {
    let mut left = String::with_capacity(css.len());
    let mut rest = css;
    while let Some((before, after)) = rest.split_once("/*") {
        left.push_str(before);
        rest = after.split_once("*/").map_or("", |(_, after)| after);
    }
    left.push_str(rest);
    left
}

#[cfg(test)]
mod tests {
    use super::{not_for_phones, style_block, too_wide, without_comments};

    #[test]
    fn only_pixel_widths_past_a_phone_are_too_wide() {
        assert_eq!(too_wide("width: 321px"), Some("width: 321px".to_owned()));
        assert_eq!(
            too_wide("min-width:400.5px"),
            Some("min-width: 400.5px".to_owned())
        );
        for fits in [
            "width: 320px",
            "max-width: 900px",
            "width: 90vw",
            "width:",
            "height: 900px",
        ] {
            assert_eq!(too_wide(fits), None, "{fits}");
        }
    }

    #[test]
    fn wide_screen_and_print_queries_are_skipped() {
        assert!(not_for_phones("@media (min-width: 48rem)"));
        assert!(not_for_phones("@media (width >= 900px)"));
        assert!(not_for_phones("@MEDIA print"));
        assert!(!not_for_phones("@media (max-width: 600px)"));
        assert!(!not_for_phones("@supports (display: grid)"));
    }

    #[test]
    fn a_rule_nested_in_a_phone_query_is_read() {
        let css = "@media (max-width: 600px) { .table { min-width: 700px } }";
        assert_eq!(
            style_block(css),
            Some((".table".to_owned(), "min-width: 700px".to_owned()))
        );
    }

    #[test]
    fn comments_are_taken_out_even_when_left_open() {
        assert_eq!(without_comments("a /* b */ c /* d"), "a  c ");
    }
}
