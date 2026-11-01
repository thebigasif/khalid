//! The shared block styles: every class a body block writes has a rule in
//! the one stylesheet, so every template starts from styled blocks (templates
//! spec section 4.4).

use khalid_content::render;
use khalid_core::{Locale, SlugMode};
use khalid_render::{Tokens, stylesheet};

/// A post that uses every block Khalid has.
const EVERY_BLOCK: &str = include_str!("fixtures/every-block.md");

/// Classes the code highlighter writes for each grammar's scopes. The
/// stylesheet colours a few of them; the rest read as plain code.
const HIGHLIGHTER_PREFIXES: [&str; 2] = ["hl-", "language-"];

#[cfg(test)]
fn css() -> String {
    String::from_utf8(stylesheet(&Tokens::default()).unwrap().body().to_vec()).unwrap()
}

#[cfg(test)]
fn every_block_html() -> String {
    render(
        EVERY_BLOCK,
        &Locale::try_new("en").unwrap(),
        SlugMode::Ascii,
    )
    .unwrap()
    .html
}

/// Whether `css` has `.class` as a whole class name.
#[cfg(test)]
fn has_selector(css: &str, class: &str) -> bool {
    let needle = format!(".{class}");
    css.match_indices(&needle).any(|(at, _)| {
        let after = css
            .get(at + needle.len()..)
            .and_then(|rest| rest.chars().next());
        !after.is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_')
    })
}

#[test]
fn every_class_a_block_writes_is_styled() {
    let css = css();
    let html = every_block_html();
    let mut missing = Vec::new();
    for attribute in html.split("class=\"").skip(1) {
        let classes = attribute.split('"').next().unwrap();
        for class in classes.split_whitespace() {
            let highlighter = HIGHLIGHTER_PREFIXES
                .iter()
                .any(|prefix| class.starts_with(prefix));
            if !highlighter && !has_selector(&css, class) && !missing.contains(&class) {
                missing.push(class);
            }
        }
    }
    assert!(missing.is_empty(), "no rule for {missing:?}");
}

#[test]
fn the_fixture_uses_every_callout_kind() {
    let html = every_block_html();
    for kind in ["note", "tip", "warning", "summary", "read", "stat"] {
        assert!(html.contains(&format!("callout-{kind}")), "{kind}");
    }
}

#[test]
fn the_spam_trap_is_hidden_from_people() {
    let css = css();
    assert!(
        css.contains(".form-trap") && css.contains("display: none"),
        "the honeypot field must not show"
    );
}

#[test]
fn comments_in_code_are_coloured() {
    assert!(has_selector(&css(), "hl-comment"));
}

#[test]
fn block_tints_come_from_the_colour_tokens() {
    // The block rules start at the first callout rule and run to the end.
    let css = css();
    let blocks = css.split_once(".callout").map_or("", |(_, rest)| rest);
    assert_ne!(blocks, "");
    for fixed in ["#", "rgb(", "hsl("] {
        assert!(
            !blocks.contains(fixed),
            "block styles use tokens, not {fixed}"
        );
    }
}
