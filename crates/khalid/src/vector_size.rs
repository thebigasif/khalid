//! The size an SVG file draws at, read from its root element, so a lead
//! image or photo in SVG reserves its space like any other picture and the
//! page never shifts as it loads (RULES.md BL-WEB-5).

/// The width and height `svg` declares on its root element: its `width`
/// and `height` in whole pixels, or else the last two numbers of its
/// `viewBox`. `None` when it declares neither as whole numbers above zero.
pub(crate) fn of(svg: &str) -> Option<(u32, u32)> {
    let start = svg.find("<svg")?;
    let tag = svg.get(start..)?.split_once('>')?.0;
    let pixels = |name| attribute(tag, name).and_then(whole);
    if let (Some(width), Some(height)) = (pixels("width"), pixels("height")) {
        return Some((width, height));
    }
    let view = attribute(tag, "viewBox")?;
    let mut numbers = view.split([' ', ',']).filter(|part| !part.is_empty());
    let (width, height) = (numbers.nth(2)?, numbers.next()?);
    Some((whole(width)?, whole(height)?))
}

/// The value of the attribute `name` in `tag`, when `name` stands alone
/// rather than ending a longer name such as `stroke-width`.
fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let wanted = format!("{name}=\"");
    let mut rest = tag;
    while let Some(at) = rest.find(&wanted) {
        let (before, after) = rest.split_at(at);
        let after = after.get(wanted.len()..)?;
        if before.ends_with(char::is_whitespace) {
            return after.split_once('"').map(|(value, _)| value);
        }
        rest = after;
    }
    None
}

/// `text` as a whole number above zero, allowing a `px` unit.
fn whole(text: &str) -> Option<u32> {
    let number: u32 = text.trim().trim_end_matches("px").parse().ok()?;
    (number > 0).then_some(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_width_and_height_win_over_the_view_box() {
        let svg = "<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\"\n width=\"96px\" height=\"96\" viewBox=\"0 0 48 48\">";
        assert_eq!(of(svg), Some((96, 96)));
    }

    #[test]
    fn the_view_box_gives_the_size_when_there_is_no_width() {
        let svg = "<svg stroke-width=\"2\" viewBox=\"0,0 1600 900\"><rect width=\"5\"/></svg>";
        assert_eq!(of(svg), Some((1600, 900)));
    }

    #[test]
    fn a_size_in_percent_or_fractions_or_none_at_all_is_no_size() {
        assert_eq!(of("<svg width=\"100%\" height=\"50%\">"), None);
        assert_eq!(of("<svg viewBox=\"0 0 10.5 4\">"), None);
        assert_eq!(of("<svg viewBox=\"0 0 0 4\">"), None);
        assert_eq!(of("<svg>"), None);
        assert_eq!(of("<p>not a picture</p>"), None);
    }
}
