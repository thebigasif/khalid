//! Every directive's name, which are leaves, and the nearest name to a
//! typing slip.

/// Every block name, for messages.
pub(super) const BLOCKS: [&str; 27] = [
    "audio",
    "bookmark",
    "business",
    "button",
    "callout",
    "columns",
    "embed",
    "faq",
    "figure",
    "file",
    "form",
    "gallery",
    "gated",
    "header",
    "howto",
    "html",
    "maths",
    "newsletter",
    "product",
    "proscons",
    "pullquote",
    "recipe",
    "review",
    "snippet",
    "toc",
    "toggle",
    "video",
];

/// Blocks with no body, written on one line as `::name{...}`. Every other
/// block opens with `:::name{...}` and closes with `:::`.
pub(super) const LEAVES: [&str; 10] = [
    "audio",
    "bookmark",
    "business",
    "button",
    "embed",
    "file",
    "newsletter",
    "snippet",
    "toc",
    "video",
];

/// What to say after "there is no block called `name`": the nearest block
/// name when one is a typing slip away, or where the names come from.
pub(crate) fn unknown_hint(name: &str) -> String {
    BLOCKS
        .iter()
        .map(|block| (distance(name, block), block))
        .filter(|(steps, _)| *steps <= 2)
        .min()
        .map_or_else(
            || "a block's name is one word, such as callout, figure or toggle".to_owned(),
            |(_, block)| format!("the closest name is {block}"),
        )
}

/// How many letters must be added, removed or changed to turn `a` into `b`.
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut previous: Vec<usize> = (0..=b.len()).collect();
    for (i, left) in a.chars().enumerate() {
        let mut current = vec![i.saturating_add(1)];
        for (j, right) in b.iter().enumerate() {
            let replace =
                previous.get(j).copied().unwrap_or_default() + usize::from(left != *right);
            let remove = previous
                .get(j.saturating_add(1))
                .copied()
                .unwrap_or_default()
                + 1;
            let add = current.last().copied().unwrap_or_default() + 1;
            current.push(replace.min(remove).min(add));
        }
        previous = current;
    }
    previous.last().copied().unwrap_or_default()
}
