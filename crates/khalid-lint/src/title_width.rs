//! How wide a title is in search results, in pixels.
//!
//! Google cuts a desktop result's title at about 600 pixels of Arial at
//! 20 pixels, not at a number of characters: "Wild" is wider than "lilt".
//! The table holds each character's width in thousandths of the font
//! size, so a title's width is the sum times 20 over 1000. A character
//! the table lacks, such as one from a script Arial draws differently,
//! makes the title unmeasured, and the character limit of `long-title`
//! judges it instead.

use crate::focus::search_title;
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// The size search results set titles in, in pixels.
const FONT_PIXELS: usize = 20;

/// The width of `c` in Arial, in thousandths of the font size, or `None`
/// for a character the table lacks.
fn width(c: char) -> Option<usize> {
    let units = match plain(c) {
        'i' | 'j' | 'l' | '\'' | '\u{2018}' | '\u{2019}' => 222,
        '|' => 260,
        ' ' | '!' | ',' | '.' | '/' | ':' | ';' | 'I' | '[' | '\\' | ']' | 'f' | 't' => 278,
        '(' | ')' | '-' | '`' | 'r' | '\u{201c}' | '\u{201d}' => 333,
        '{' | '}' => 334,
        '"' => 355,
        '*' => 389,
        '^' => 469,
        'J' | 'c' | 'k' | 's' | 'v' | 'x' | 'y' | 'z' => 500,
        '0'..='9'
        | '#'
        | '$'
        | '?'
        | '_'
        | 'L'
        | 'a'
        | 'b'
        | 'd'
        | 'e'
        | 'g'
        | 'h'
        | 'n'
        | 'o'
        | 'p'
        | 'q'
        | 'u'
        | '\u{2013}'
        | '\u{20ac}' => 556,
        '+' | '<' | '=' | '>' | '~' | '\u{d7}' => 584,
        'F' | 'T' | 'Z' | '\u{df}' => 611,
        '&' | 'A' | 'B' | 'E' | 'K' | 'P' | 'S' | 'V' | 'X' | 'Y' => 667,
        'C' | 'D' | 'H' | 'N' | 'R' | 'U' | 'w' => 722,
        'G' | 'O' | 'Q' => 778,
        'M' | 'm' => 833,
        '%' => 889,
        'W' => 944,
        '@' => 1015,
        '\u{2014}' | '\u{2026}' => 1000,
        _ => return None,
    };
    Some(units)
}

/// The letter without its accent, which Arial draws about as wide, such
/// as `e` for `é`.
fn plain(c: char) -> char {
    match c {
        'à'..='å' => 'a',
        'À'..='Å' => 'A',
        'è'..='ë' => 'e',
        'È'..='Ë' => 'E',
        'ì'..='ï' => 'i',
        'Ì'..='Ï' => 'I',
        'ò'..='ö' => 'o',
        'Ò'..='Ö' => 'O',
        'ù'..='ü' => 'u',
        'Ù'..='Ü' => 'U',
        'ñ' => 'n',
        'Ñ' => 'N',
        'ç' => 'c',
        'Ç' => 'C',
        'ý' | 'ÿ' => 'y',
        'Ý' => 'Y',
        other => other,
    }
}

/// The width of `title` in search results, in pixels, or `None` when it
/// holds a character the table lacks. `{sep}` counts as " - ", and
/// `{site}`, the site's name, is left out, because the linter does not
/// know it.
pub(crate) fn pixels(title: &str) -> Option<usize> {
    let shown = title.replace("{sep}", " - ").replace("{site}", "");
    let units = shown.trim().chars().map(width).sum::<Option<usize>>()?;
    Some(units * FONT_PIXELS / 1000)
}

/// The title search results show is wider than the limit, so Google cuts
/// it short.
#[derive(Debug)]
pub(crate) struct TitleTooWide {
    /// From `Thresholds::title_max_pixels`.
    pub(crate) max: usize,
}

impl Rule for TitleTooWide {
    fn id(&self) -> RuleId {
        RuleId::new("title-too-wide")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        search_title(post)
            .or(post.title.as_deref())
            .and_then(pixels)
            .filter(|wide| *wide > self.max)
            .map(|wide| Hit::about(format!("{wide} of {} pixels", self.max)))
            .into_iter()
            .collect()
    }
}

/// Whether the pixel rule judges `post`'s search title, so the character
/// rule stays quiet.
pub(crate) fn measured(post: &LintPost) -> bool {
    search_title(post)
        .or(post.title.as_deref())
        .and_then(pixels)
        .is_some()
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::LintSite;

    #[test]
    fn wide_letters_cost_more_than_narrow_ones() {
        assert_eq!(pixels("lilt"), Some(18));
        assert_eq!(pixels("WWWW"), Some(75));
        assert_eq!(pixels("Über Größe"), pixels("Uber Große"));
        assert_eq!(pixels("Why links{sep}{site}"), pixels("Why links -"));
        assert_eq!(pixels("内部リンク"), None);
    }

    #[test]
    fn a_title_over_600_pixels_is_too_wide_in_english_and_german() {
        let site = LintSite::default();
        let rule = TitleTooWide { max: 600 };
        let en = "Why internal links matter for readers, search engines and your own archive";
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.title = Some(en.to_owned());
        let width = pixels(en).unwrap();
        assert!(width > 600, "{width}");
        assert_eq!(
            rule.check(&post, &site),
            [Hit::about(format!("{width} of 600 pixels"))]
        );
        post.seo_title = Some("Interne Links: Warum Leser profitieren".to_owned());
        assert_eq!(rule.check(&post, &site), []);
        post.seo_title = Some(
            "Interne Verlinkung für Blogs: Wie Leser und Suchmaschinen den Weg finden".to_owned(),
        );
        assert_eq!(rule.check(&post, &site).len(), 1);
        assert!(measured(&post));
        post.seo_title = Some("内部リンクの使い方".to_owned());
        assert!(!measured(&post));
    }
}
