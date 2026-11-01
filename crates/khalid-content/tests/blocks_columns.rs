//! Side-by-side columns (templates spec 4.4, KH-CON-5).

use khalid_content::{BlockKind, Column, parse_blocks, print_blocks, render};
use khalid_core::{Locale, SlugMode};

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

const COLUMNS: &str = "\
::::columns
:::column{title=\"Hosted checkout\"}
Live in **an afternoon**.
:::

:::column
- Full control
- More work
:::
::::
";

#[test]
fn columns_round_trip() {
    let blocks = parse_blocks(COLUMNS).unwrap();
    assert_eq!(blocks.len(), 1, "{blocks:?}");
    assert_eq!(
        blocks[0].kind,
        BlockKind::Columns {
            columns: vec![
                Column {
                    title: Some("Hosted checkout".to_owned()),
                    text: "Live in **an afternoon**.".to_owned(),
                },
                Column {
                    title: None,
                    text: "- Full control\n- More work".to_owned(),
                },
            ],
        }
    );
    assert_eq!(print_blocks(&blocks), COLUMNS);
}

#[test]
fn renders_columns() {
    insta::assert_snapshot!(html(COLUMNS));
}

#[test]
fn columns_read_with_scripts_off() {
    let html = html(COLUMNS);
    assert!(!html.contains("<script"), "{html}");
    for words in ["Hosted checkout", "an afternoon", "More work"] {
        assert!(html.contains(words), "{words}: {html}");
    }
}

#[test]
fn columns_need_two_to_four() {
    let one = "::::columns\n:::column\nAlone.\n:::\n::::\n";
    let five = format!("::::columns\n{}::::\n", ":::column\nA.\n:::\n".repeat(5));
    let message = "line 1: columns hold two to four :::column blocks; one column is a callout, and five do not fit";
    assert_eq!(error(one), message);
    assert_eq!(error(&five), message);
}

#[test]
fn columns_hold_only_columns() {
    assert_eq!(
        error("::::columns\nA stray line.\n::::\n"),
        "line 2: columns hold only :::column blocks, each with its words, then :::"
    );
}

#[test]
fn a_column_takes_only_a_title() {
    assert_eq!(
        error("::::columns\n:::column{width=2}\nA.\n:::\n:::column\nB.\n:::\n::::\n"),
        "line 2: a column does not take width; it takes title"
    );
}
