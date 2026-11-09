//! The key takeaways block, `:::takeaways` (Yoast package WP4): one list of
//! points in a box, read in full with scripts off, claiming no schema type.

use khalid_content::{BlockKind, parse_blocks, print_blocks, render};
use khalid_core::{Locale, SlugMode};

#[cfg(test)]
fn round_trip(markdown: &str, kind: &BlockKind) {
    let blocks = parse_blocks(markdown).unwrap();
    assert_eq!(blocks.len(), 1, "{markdown:?} gave {blocks:?}");
    assert_eq!(&blocks[0].kind, kind, "{markdown:?}");
    assert_eq!(print_blocks(&blocks), markdown);
}

#[cfg(test)]
fn html(markdown: &str) -> String {
    render(markdown, &Locale::try_new("en").unwrap(), SlugMode::Ascii)
        .unwrap()
        .html
}

#[cfg(test)]
fn error(markdown: &str) -> String {
    parse_blocks(markdown).unwrap_err().to_string()
}

const TAKEAWAYS: &str = "\
:::takeaways
- Link words say where the link **goes**.
- Vary the words across a site.
:::
";

#[test]
fn key_takeaways_round_trip() {
    let title = |title: &str| BlockKind::Takeaways {
        title: title.to_owned(),
    };
    round_trip(TAKEAWAYS, &title("Key takeaways"));
    round_trip(
        ":::takeaways{title=\"Das Wichtigste\"}\n1. Erstens.\n2. Zweitens.\n:::\n",
        &title("Das Wichtigste"),
    );
}

#[test]
fn renders_key_takeaways_as_a_named_box() {
    assert_eq!(
        html(TAKEAWAYS),
        "<aside class=\"takeaways\" aria-label=\"Key takeaways\">\n\
         <p class=\"takeaways-title\">Key takeaways</p>\n\
         <ul>\n<li>Link words say where the link <strong>goes</strong>.</li>\n\
         <li>Vary the words across a site.</li>\n</ul>\n</aside>\n"
    );
}

#[test]
fn key_takeaways_claim_no_schema_type() {
    let page = html(TAKEAWAYS);
    assert!(
        !page.contains("itemtype") && !page.contains("itemprop"),
        "{page}"
    );
}

#[test]
fn a_title_is_escaped() {
    let page = html(":::takeaways{title=\"<b>Points</b>\"}\n- One.\n:::\n");
    assert!(
        page.contains("aria-label=\"&lt;b&gt;Points&lt;/b&gt;\""),
        "{page}"
    );
}

#[test]
fn key_takeaways_hold_one_list() {
    let refused = "line 1: key takeaways hold one list, a line starting with - for each point";
    assert_eq!(error(":::takeaways\nJust a sentence.\n:::\n"), refused);
    assert_eq!(error(":::takeaways\n:::\n"), refused);
    assert_eq!(error(":::takeaways\n- One.\n\nA sentence.\n:::\n"), refused);
}

#[test]
fn key_takeaways_take_only_a_title() {
    let message = error(":::takeaways{kind=\"tip\"}\n- One.\n:::\n");
    assert!(message.contains("title"), "{message}");
}
