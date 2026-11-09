//! `[seo.search_filter]`: what the site's search refuses.

use serde::Deserialize;

/// What the site's search refuses, as Yoast's crawl settings filter search
/// terms. A refused search answers 400 with an empty search page, so bots
/// cannot fill result pages with their words. Every filter is on:
///
/// ```toml
/// [seo.search_filter]
/// max_length = 80
/// symbols = false
/// spam = false
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct SearchFilterSettings {
    /// The most characters a search may have, 50 by default. 0 sets no
    /// limit.
    pub max_length: u16,
    /// Refuse a search with an emoji or another symbol that is not a
    /// letter, digit or punctuation.
    pub symbols: bool,
    /// Refuse a search with a link, markup, a spam domain or a long run of
    /// one character.
    pub spam: bool,
}

impl Default for SearchFilterSettings {
    fn default() -> Self {
        Self {
            max_length: 50,
            symbols: true,
            spam: true,
        }
    }
}
