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
}

/// Every part by its option key, with the value that keeps classic's look.
/// A pack offers a part by listing an option with the same key.
const PARTS: [(&str, Off); 2] = [
    // The site's logo in the header, from `[brand] logo` in the tokens, in
    // place of its name as text.
    ("logo", Off::Switch(false)),
    // The trail from the home page to the page, above the content.
    ("breadcrumbs", Off::Switch(false)),
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
    for (key, off) in PARTS {
        options.entry(key.to_owned()).or_insert(match off {
            Off::Switch(on) => TemplateOption::Switch(on),
        });
    }
    options
}
