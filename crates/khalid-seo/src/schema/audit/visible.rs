//! Values in the markup that the page's visible text never shows, such as a
//! price or a product name. Google's guidelines ask that markup describe
//! what a reader can see, and may ignore or penalise markup that does not.

use super::tree::{Json, place};
use super::{FindingLevel, SchemaFinding, SchemaReport, finding};

/// Properties whose text a reader should find on the page.
const SHOWN: [&str; 11] = [
    "headline",
    "name",
    "price",
    "ratingCount",
    "ratingValue",
    "recipeIngredient",
    "reviewCount",
    "streetAddress",
    "telephone",
    "lowPrice",
    "highPrice",
];

/// Every value of a [`SHOWN`] property in `root` that `visible` does not
/// contain, with the path to it.
pub(super) fn unseen(root: &Json, visible: &str) -> Vec<SchemaFinding> {
    let page = Page::new(visible);
    let mut found = Vec::new();
    walk(root, "", None, &page, &mut found);
    found
}

/// The visible text, kept two ways for matching.
struct Page {
    words: String,
    digits: String,
}

impl Page {
    fn new(text: &str) -> Self {
        Self {
            words: words(text),
            digits: text.chars().filter(char::is_ascii_digit).collect(),
        }
    }

    /// Whether the page shows `value` of property `name`.
    fn shows(&self, name: &str, value: &str) -> bool {
        match name {
            "telephone" => {
                let digits: String = value.chars().filter(char::is_ascii_digit).collect();
                // The page may leave out the country code the markup gives.
                let tail = digits
                    .char_indices()
                    .nth(digits.len().saturating_sub(7))
                    .map_or(digits.as_str(), |(at, _)| {
                        digits.get(at..).unwrap_or_default()
                    });
                tail.is_empty() || self.digits.contains(tail)
            }
            "price" | "lowPrice" | "highPrice" | "ratingValue" | "ratingCount" | "reviewCount" => {
                number_forms(value)
                    .iter()
                    .any(|form| self.words.contains(form.as_str()))
            }
            _ => {
                let wanted = words(value);
                wanted.trim().is_empty() || self.words.contains(&wanted)
            }
        }
    }
}

/// `text` in lower case, with every run of other characters than letters
/// and digits made one space, and a space at each end. A `.` or `,` stays
/// between digits, as in `19.99`.
fn words(text: &str) -> String {
    let mut out = String::from(" ");
    let mut chars = text.chars().flat_map(char::to_lowercase).peekable();
    while let Some(c) = chars.next() {
        let in_number = (c == '.' || c == ',')
            && out.ends_with(|before: char| before.is_ascii_digit())
            && chars.peek().is_some_and(char::is_ascii_digit);
        if c.is_alphanumeric() || in_number {
            out.push(c);
        } else if !out.ends_with(' ') {
            out.push(' ');
        }
    }
    if !out.ends_with(' ') {
        out.push(' ');
    }
    out
}

/// The ways a page may write the number `value`: `19.99` and `19,99`, and
/// `20` for `20.00`.
fn number_forms(value: &str) -> Vec<String> {
    let value = value.trim();
    let mut forms = vec![value.to_owned(), value.replace('.', ",")];
    if let Some(whole) = value
        .strip_suffix(".00")
        .or_else(|| value.strip_suffix(".0"))
    {
        forms.push(whole.to_owned());
    }
    forms
        .into_iter()
        .filter(|form| !form.is_empty())
        .map(|form| format!(" {form} "))
        .collect()
}

fn walk(
    value: &Json,
    at: &str,
    property: Option<&str>,
    page: &Page,
    found: &mut Vec<SchemaFinding>,
) {
    match value {
        Json::Object(object) => {
            for (name, inner) in object.iter().filter(|(name, _)| !name.starts_with('@')) {
                walk(inner, &place(at, name), Some(name), page, found);
            }
            if let Some(graph) = object.get("@graph") {
                walk(graph, &place(at, "@graph"), None, page, found);
            }
        }
        Json::List(items) => {
            for (index, item) in items.iter().enumerate() {
                walk(item, &format!("{at}[{index}]"), property, page, found);
            }
        }
        Json::Text(text) => check(text, at, property, page, found),
        Json::Number(number) => check(&number.to_string(), at, property, page, found),
        Json::Null | Json::Bool(_) => {}
    }
}

fn check(
    value: &str,
    at: &str,
    property: Option<&str>,
    page: &Page,
    found: &mut Vec<SchemaFinding>,
) {
    let Some(name) = property.filter(|name| SHOWN.contains(name)) else {
        return;
    };
    if page.shows(name, value) {
        return;
    }
    found.push(finding(
        (SchemaReport::Google, FindingLevel::Warning),
        at,
        (
            format!("The page does not show the {name} \"{value}\" that the markup gives."),
            "Google asks that markup only describe what readers can see, and may ignore it otherwise.",
            format!("Show \"{value}\" in the post, or change the markup to match what the post says."),
        ),
    ));
}

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn prices_phones_and_names_match_as_readers_see_them() {
        let page = Page::new("The Lentil Soup kit costs $19.99. Call +44 20 7946 0958.");
        assert!(page.shows("price", "19.99"));
        assert!(!page.shows("price", "29.99"));
        assert!(page.shows("telephone", "020 7946 0958"));
        assert!(page.shows("name", "lentil soup kit"));
        assert!(!page.shows("name", "Tomato soup"));
        let euro = Page::new("Preis: 19,99 €. Nur 20 Euro.");
        assert!(euro.shows("price", "19.99"));
        assert!(euro.shows("price", "20.00"));
    }
}
