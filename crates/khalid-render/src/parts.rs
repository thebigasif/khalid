//! The parts the shared templates build once and a template pack switches
//! on with its options (templates spec 4.2), such as the logo in the
//! header. Classic sets no option, so each part's value here is the one
//! that keeps today's look.

use std::collections::BTreeMap;

use crate::TemplateOption;

/// One part's value when the pack does not set its key.
#[derive(Debug, Clone, Copy)]
enum Off {
    /// An on or off part.
    Switch(bool),
    /// One word, such as a card shape.
    Word(&'static str),
    /// A whole number, such as how many picks.
    Number(i64),
}

/// Every part by its option key, with the value that keeps classic's look.
/// A pack offers a part by listing an option with the same key.
const PARTS: &[(&str, Off)] = &[
    // The site's logo in the header, from `[brand] logo` in the tokens, in
    // place of its name as text.
    ("logo", Off::Switch(false)),
    // The trail from the home page to the page, above the content.
    ("breadcrumbs", Off::Switch(false)),
    // A post's subtitle, its description, under its title.
    ("dek", Off::Switch(false)),
    // Why a post was updated, after its dates.
    ("updated_note", Off::Switch(false)),
    // The shape of the cards in every list: image, text, row, compact or
    // ranked.
    ("cards", Off::Word("image")),
    // A card's reading time after its date.
    ("card_reading_time", Off::Switch(false)),
    // A card author's role after their name.
    ("card_author_role", Off::Switch(false)),
    // A row of tabs on the home page: all posts, then each category.
    ("category_tabs", Off::Switch(false)),
    // The newest featured post above the home page and each list.
    ("featured", Off::Switch(false)),
    // How many editor's picks the home page shows; none at 0.
    ("picks", Off::Number(0)),
    // The page links under a list: pages (newer and older) or numbered.
    ("pagination", Off::Word("pages")),
    // How the related posts under a post are laid out: grid (the list's
    // cards), list, row, scroll or links.
    ("related", Off::Word("grid")),
    // How many related posts show, from 3 to 10.
    ("related_count", Off::Number(5)),
    // Where a post's contents go: side-end (classic), side-start, inline,
    // inline-folded or none.
    ("contents", Off::Word("side-end")),
    // How the contents are numbered: numbered, nested or none.
    ("contents_numbers", Off::Word("numbered")),
];

/// The option key of every part the shared templates read.
pub fn part_keys() -> Vec<&'static str> {
    PARTS.iter().map(|(key, _)| *key).collect()
}

/// `options` with each part the pack leaves out set to classic's value, so
/// the shared templates can read every part's key.
pub(crate) fn with_parts(
    mut options: BTreeMap<String, TemplateOption>,
) -> BTreeMap<String, TemplateOption> {
    for &(key, off) in PARTS {
        options.entry(key.to_owned()).or_insert(match off {
            Off::Switch(on) => TemplateOption::Switch(on),
            Off::Word(word) => TemplateOption::Word(word.to_owned()),
            Off::Number(number) => TemplateOption::Number(number),
        });
    }
    options
}
