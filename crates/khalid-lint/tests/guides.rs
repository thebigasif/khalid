//! Every lint message links a guide a writer can open (plan 6, D-12): a
//! section of the rules page, `docs/lint-rules.md`, which the docs pack
//! serves at [`GUIDE_PAGE`] and this test keeps in step with the catalogue.

use std::fs;

use khalid_lint::{Catalogue, GUIDE_PAGE, rules_page};

/// The rules page in the repository.
const PAGE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/lint-rules.md");

/// The anchor a docs page gives a `## ` heading: its words in lower case,
/// joined by hyphens, as GitHub and the docs site make it.
fn anchor(heading: &str) -> String {
    heading
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

#[test]
fn every_guide_link_opens_a_section_of_the_rules_page() {
    let catalogue = Catalogue::builtin().unwrap();
    let page = rules_page(&catalogue);
    let anchors: Vec<String> = page
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(anchor)
        .collect();
    for (rule, message) in &catalogue {
        let url = message.guide_url();
        let section = url
            .strip_prefix(GUIDE_PAGE)
            .and_then(|rest| rest.strip_prefix('#'));
        let section = section.unwrap_or_else(|| panic!("{rule}: {url}"));
        assert_eq!(section, message.guide().as_str());
        assert!(
            anchors.iter().any(|a| a == section),
            "{rule}: no section {section}"
        );
        assert!(
            page.contains(&format!("- `{rule}`: {}", message.what())),
            "{rule}"
        );
    }
}

#[test]
fn the_rules_page_in_the_docs_matches_the_catalogue() {
    let page = rules_page(&Catalogue::builtin().unwrap());
    let written = fs::read_to_string(PAGE).unwrap_or_default();
    if written != page {
        fs::write(PAGE, &page).unwrap();
        panic!("docs/lint-rules.md was out of date and is now rewritten; commit it");
    }
}
