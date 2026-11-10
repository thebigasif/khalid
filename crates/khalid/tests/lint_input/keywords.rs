//! The focus keyword in each place it belongs, how often the body uses it,
//! the secondary keywords and the synonyms that count for the focus one.

use super::{found, lint, post, site};

#[test]
fn a_focus_keyword_missing_from_each_place_is_a_warning_per_place() {
    let body = "Readers come back for clear writing.\n\n## How it starts\n\n\
                ![A tidy desk](/media/desk.jpg)";
    let extra = "focus_keyword: internal links\n";
    let site = site(
        None,
        &[("posts/writing-well.md", &post("Writing well", extra, body))],
    );
    let report = lint(site.path());
    for rule in [
        "focus-not-in-title",
        "focus-not-in-description",
        "focus-not-in-first-paragraph",
        "focus-not-in-subheading",
        "focus-not-in-address",
        "focus-not-in-image-alt",
    ] {
        assert_eq!(
            found(&report, rule, "writing-well"),
            ["internal links"],
            "{rule}"
        );
    }
}

/// `count` sentences of filler, ten words each, the first `uses` of them
/// holding "internal links".
#[cfg(test)]
fn filler(count: usize, uses: usize) -> String {
    (0..count)
        .map(|n| {
            if n < uses {
                "Good internal links help readers find the next useful page here."
            } else {
                "A clear sentence helps every reader keep going through the page."
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn a_focus_keyword_used_too_rarely_is_a_warning() {
    let extra = "focus_keyword: internal links\n";
    let body = filler(40, 0);
    let site = site(
        None,
        &[(
            "posts/internal-links.md",
            &post("Internal links", extra, &body),
        )],
    );
    let report = lint(site.path());
    assert_eq!(
        found(&report, "focus-too-rare", "internal-links"),
        ["internal links"]
    );
}

#[test]
fn a_focus_keyword_used_too_often_is_a_warning() {
    let extra = "focus_keyword: internal links\n";
    let body = filler(40, 40);
    let site = site(
        None,
        &[(
            "posts/internal-links.md",
            &post("Internal links", extra, &body),
        )],
    );
    let report = lint(site.path());
    assert_eq!(
        found(&report, "focus-too-often", "internal-links"),
        ["internal links"]
    );
}

#[test]
fn a_secondary_keyword_the_body_never_uses_is_a_warning() {
    let extra =
        "focus_keyword: internal links\nsecondary_keywords:\n  - link text\n  - site structure\n";
    let body = "Internal links need good link text.";
    let site = site(
        None,
        &[(
            "posts/internal-links.md",
            &post("Internal links", extra, body),
        )],
    );
    let report = lint(site.path());
    assert_eq!(
        found(&report, "secondary-keyword-unused", "internal-links"),
        ["site structure"]
    );
}

#[test]
fn a_keyword_synonym_in_the_header_counts_as_the_focus_keyword() {
    let extra = "focus_keyword: internal links\nsecondary_keywords: [site links]\n";
    let body = "Site links help readers.";
    let lone = site(
        None,
        &[("posts/site-links.md", &post("Site links", extra, body))],
    );
    let report = lint(lone.path());
    assert_eq!(
        found(&report, "focus-not-in-title", "site-links"),
        ["internal links"]
    );
    let synonym = format!("{extra}keyword_synonyms: site links\n");
    let with = site(
        None,
        &[("posts/site-links.md", &post("Site links", &synonym, body))],
    );
    let report = lint(with.path());
    assert_eq!(
        found(&report, "focus-not-in-title", "site-links"),
        Vec::<String>::new()
    );
}
