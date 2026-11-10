//! Finds the parts of a page that only work with scripts on
//! (`docs/plan.md` section 6, "Needs JavaScript").

use scraper::ElementRef;

use crate::page::link_text;

/// What `element` does that needs scripts, in words for the report, or
/// `None` when it works without them.
///
/// A form needs a real `action`, and a link a real address: `#` alone or
/// `javascript:` only does something when a script catches the click. A
/// `<noscript>` notice that names JavaScript asks the reader to turn it on.
/// A link to `#main` or `#top` is a real place on the page, and a
/// `<noscript>` notice that does not name JavaScript, such as the privacy
/// choices' "your browser runs no scripts", is fine.
pub(crate) fn part(element: ElementRef<'_>) -> Option<String> {
    let attr = |name: &str| element.value().attr(name);
    match element.value().name() {
        "form" => match attr("action") {
            None => Some("a form with no action".to_owned()),
            Some(action) if goes_nowhere(action) || action.trim().is_empty() => {
                Some(format!("a form whose action is \"{}\"", action.trim()))
            }
            Some(_) => None,
        },
        "a" => attr("href").filter(|href| goes_nowhere(href)).map(|href| {
            format!(
                "the link \"{}\" goes to \"{}\"",
                link_text(element),
                href.trim()
            )
        }),
        "noscript" => element
            .text()
            .any(|text| text.to_ascii_lowercase().contains("javascript"))
            .then(|| "a notice asks readers to turn on JavaScript".to_owned()),
        _ => None,
    }
}

/// Whether `address` leads nowhere without a script: `#` alone, or a
/// `javascript:` address in any case.
fn goes_nowhere(address: &str) -> bool {
    let address = address.trim();
    address == "#"
        || address
            .get(..11)
            .is_some_and(|scheme| scheme.eq_ignore_ascii_case("javascript:"))
}

#[cfg(test)]
mod tests {
    use super::goes_nowhere;

    #[test]
    fn only_a_bare_hash_and_script_addresses_go_nowhere() {
        for address in ["#", " # ", "javascript:void(0)", "JavaScript:open()"] {
            assert!(goes_nowhere(address), "{address}");
        }
        for address in ["#main", "/blog/", "", "javascript", "java"] {
            assert!(!goes_nowhere(address), "{address}");
        }
    }
}
