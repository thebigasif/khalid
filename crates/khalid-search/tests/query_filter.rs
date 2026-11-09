//! What the site's search refuses before it looks anything up, as Yoast's
//! search filters do: queries that are too long, that hold emoji or other
//! symbols, or that look like spam (Yoast comparison, 2 November 2026).

use khalid_core::SearchFilterSettings;
use khalid_search::{Junk, QueryFilter};

/// The defaults, as Yoast has them: at most 50 characters, no symbols and
/// no spam.
fn yoast_defaults() -> QueryFilter {
    QueryFilter::from(&SearchFilterSettings::default())
}

#[test]
fn ordinary_searches_in_any_language_pass() {
    let filter = yoast_defaults();
    for query in [
        "internal links",
        "Häuser für Familien",
        "what's a 301?",
        "c++ & rust: \"borrow checker\"",
        "node.js",
        "café – menu",
        "日本語のサイトマップ",
    ] {
        assert_eq!(filter.check(query), Ok(()), "{query}");
    }
}

#[test]
fn a_query_longer_than_the_limit_is_refused() {
    let filter = yoast_defaults();
    assert_eq!(filter.check(&"äbcdefghij".repeat(5)), Ok(()));
    assert_eq!(filter.check(&"äbcdefghij".repeat(6)), Err(Junk::TooLong));
}

#[test]
fn emoji_and_other_symbols_are_refused() {
    let filter = yoast_defaults();
    assert_eq!(filter.check("best pizza 🍕"), Err(Junk::Symbols));
    assert_eq!(filter.check("★★★ deals"), Err(Junk::Symbols));
}

#[test]
fn links_markup_spam_domains_and_long_repeats_are_refused() {
    let filter = yoast_defaults();
    for query in [
        "buy cheap https://spam.example",
        "visit www.cheap-pills",
        "<b>unicorn",
        "[url=x]pills[/url]",
        "best-casino.xyz bonus",
        "contact t.me/seller",
        "aaaaaaaa",
    ] {
        assert_eq!(filter.check(query), Err(Junk::Spam), "{query}");
    }
}

#[test]
fn each_filter_can_be_switched_off() {
    let mut settings = SearchFilterSettings::default();
    settings.max_length = 0;
    settings.symbols = false;
    settings.spam = false;
    let open = QueryFilter::from(&settings);
    assert_eq!(open.check(&"pizza 🍕 www.example.com ".repeat(20)), Ok(()));
}
