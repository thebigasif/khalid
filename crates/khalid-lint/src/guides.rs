//! Where each rule's guide lives: one section of the rules page in Khalid's
//! docs, `docs/lint-rules.md`, which the docs pack serves on serpdad.com.
//! The CLI, the API and the admin all link a finding to its section.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::catalogue::{Catalogue, Message};

/// The address of the rules page, where each guide is one section.
pub const GUIDE_PAGE: &str = "https://serpdad.com/khalid/docs/lint-rules/";

/// The top of the rules page: its front matter for the docs pack, its one
/// heading and what the page is for.
const HEAD: &str = "---
title: \"What the linter checks\"
order: 65
---

# What the linter checks

Every message from `khalid lint`, the editor and the publish check links one
section of this page. Each section lists the rules it explains, with the
three sentences the writer sees: what is wrong, why it matters and what to
do. A site turns a rule up, down or off in `lint.toml`.

This page is made from `crates/khalid-lint/catalogue.toml`. Change a message
there, and the lint tests rewrite this page.
";

impl Message {
    /// The address of the guide that explains the rule, such as
    /// `https://serpdad.com/khalid/docs/lint-rules/#internal-links`.
    pub fn guide_url(&self) -> String {
        format!("{GUIDE_PAGE}#{}", self.guide())
    }
}

/// A guide's heading: its slug in words with a capital first letter, such
/// as "Internal links", whose anchor on the page is the slug again.
fn heading(slug: &str) -> String {
    let words = slug.replace('-', " ");
    let mut chars = words.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

/// The rules page: each guide as a section, in guide order, listing every
/// rule that links to it with its three sentences.
pub fn rules_page(catalogue: &Catalogue) -> String {
    let mut by_guide: BTreeMap<&str, Vec<(&str, &Message)>> = BTreeMap::new();
    for (rule, message) in catalogue {
        let rules = by_guide.entry(message.guide().as_str()).or_default();
        rules.push((rule.as_str(), message));
    }
    let mut page = String::from(HEAD);
    for (guide, rules) in by_guide {
        let _ = write!(page, "\n## {}\n\n", heading(guide));
        for (rule, message) in rules {
            let (what, why, fix) = (message.what(), message.why(), message.fix());
            let _ = writeln!(page, "- `{rule}`: {what} {why} {fix}");
        }
    }
    page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_heading_is_the_slug_in_words() {
        assert_eq!(heading("internal-links"), "Internal links");
        assert_eq!(heading("forms"), "Forms");
        assert_eq!(heading(""), "");
    }
}
