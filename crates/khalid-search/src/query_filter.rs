//! What the site's search refuses before it looks anything up, as Yoast's
//! crawl settings filter search terms: bots fill search pages with long,
//! symbol-laden or spam queries to make a site show their words.

use khalid_core::SearchFilterSettings;

/// Punctuation outside ASCII that ordinary searches use: quotes, dashes,
/// the ellipsis and the middle dot.
const TYPOGRAPHIC: [char; 12] = [
    '‘', '’', '‚', '“', '”', '„', '«', '»', '–', '\u{2014}', '…', '·',
];

/// Text that only spam searches hold: links and markup.
const SPAM_TEXT: [&str; 8] = ["http:", "https:", "www.", "://", "<", ">", "[url", "t.me/"];

/// Domain endings spam searches name. A query such as `node.js` is not
/// one of them.
const SPAM_ENDINGS: [&str; 17] = [
    "com", "net", "org", "info", "biz", "xyz", "top", "ru", "cn", "club", "online", "site", "shop",
    "vip", "win", "bet", "casino",
];

/// The same character this many times in a row is spam, not a word.
const MOST_REPEATS: usize = 6;

/// Why a search was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Junk {
    /// It is longer than the site allows.
    TooLong,
    /// It holds an emoji or another symbol that is not a letter, digit or
    /// punctuation.
    Symbols,
    /// It holds a link, markup, a spam domain or a long run of one
    /// character.
    Spam,
}

/// What the site's search refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryFilter {
    most_chars: usize,
    symbols: bool,
    spam: bool,
}

impl From<&SearchFilterSettings> for QueryFilter {
    /// The filter `[seo.search_filter]` asks for: queries over
    /// `max_length` characters (none when 0), with symbols when `symbols`,
    /// and spam when `spam` are refused.
    fn from(settings: &SearchFilterSettings) -> Self {
        Self {
            most_chars: usize::from(settings.max_length),
            symbols: settings.symbols,
            spam: settings.spam,
        }
    }
}

impl QueryFilter {
    /// Whether the search `text`, as typed, may run.
    ///
    /// # Errors
    ///
    /// The [`Junk`] it is, the first found of too long, symbols and spam.
    pub fn check(&self, text: &str) -> Result<(), Junk> {
        if self.most_chars > 0 && text.chars().count() > self.most_chars {
            return Err(Junk::TooLong);
        }
        if self.symbols && text.chars().any(is_symbol) {
            return Err(Junk::Symbols);
        }
        if self.spam && is_spam(text) {
            return Err(Junk::Spam);
        }
        Ok(())
    }
}

fn is_symbol(c: char) -> bool {
    !(c.is_alphanumeric()
        || c.is_whitespace()
        || c.is_ascii_punctuation()
        || TYPOGRAPHIC.contains(&c))
}

fn is_spam(text: &str) -> bool {
    let lower = text.to_lowercase();
    if SPAM_TEXT.iter().any(|spam| lower.contains(spam)) {
        return true;
    }
    let domain = lower.split_whitespace().any(|word| {
        word.rsplit_once('.').is_some_and(|(name, ending)| {
            let ending = ending.trim_end_matches(|c: char| c.is_ascii_punctuation());
            !name.is_empty() && SPAM_ENDINGS.contains(&ending)
        })
    });
    domain || repeats(&lower)
}

/// Whether some character comes [`MOST_REPEATS`] times in a row.
fn repeats(text: &str) -> bool {
    let mut run = 0;
    let mut previous = None;
    for c in text.chars() {
        run = if previous == Some(c) { run + 1 } else { 1 };
        if run >= MOST_REPEATS && !c.is_whitespace() {
            return true;
        }
        previous = Some(c);
    }
    false
}
