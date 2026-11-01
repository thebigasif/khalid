//! Long tables that fold (templates spec 4.4, KH-CON-5).

use std::num::NonZeroU8;

use khalid_content::{BlockKind, parse_blocks, print_blocks, render};
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

const TABLE: &str = "\
:::table{fold=2}
| Tool | Price |
|---|---|
| Rank Ledger | 12 |
| Crawl Kit | 20 |
| Link Atlas | 35 |
| Page Scout | 9 |
:::
";

#[test]
fn a_folding_table_round_trips() {
    let blocks = parse_blocks(TABLE).unwrap();
    assert_eq!(
        blocks[0].kind,
        BlockKind::FoldingTable {
            fold: NonZeroU8::new(2).unwrap(),
            more: None,
        }
    );
    assert_eq!(print_blocks(&blocks), TABLE);
}

#[test]
fn fold_defaults_to_eight() {
    let blocks =
        parse_blocks(":::table{more=\"See every tool\"}\n| A |\n|---|\n| 1 |\n:::\n").unwrap();
    assert_eq!(
        blocks[0].kind,
        BlockKind::FoldingTable {
            fold: NonZeroU8::new(8).unwrap(),
            more: Some("See every tool".to_owned()),
        }
    );
}

#[test]
fn renders_a_folding_table() {
    insta::assert_snapshot!(html(TABLE));
}

#[test]
fn every_row_reads_with_scripts_off() {
    let html = html(TABLE);
    assert!(!html.contains("<script"), "{html}");
    assert!(
        html.contains("<summary>Show 2 more rows</summary>"),
        "{html}"
    );
    for row in ["Rank Ledger", "Crawl Kit", "Link Atlas", "Page Scout"] {
        assert!(html.contains(row), "{row}: {html}");
    }
}

#[test]
fn a_short_table_does_not_fold() {
    let html = html(":::table{fold=8}\n| A |\n|---|\n| 1 |\n:::\n");
    assert!(!html.contains("<details"), "{html}");
    assert!(html.contains("<td>1</td>"), "{html}");
}

#[test]
fn one_more_row_is_one_row() {
    let html = html(":::table{fold=1}\n| A |\n|---|\n| 1 |\n| 2 |\n:::\n");
    assert!(
        html.contains("<summary>Show 1 more row</summary>"),
        "{html}"
    );
}

#[test]
fn the_header_counts_once() {
    let words = |markdown: &str| {
        render(markdown, &Locale::try_new("en").unwrap(), SlugMode::Ascii)
            .unwrap()
            .words
    };
    let plain = "| Tool | Price |\n|---|---|\n| A | 1 |\n| B | 2 |\n";
    assert_eq!(
        words(&format!(":::table{{fold=1}}\n{plain}:::\n")),
        words(plain)
    );
}

#[test]
fn fold_is_a_number_from_one_to_a_hundred() {
    let message = "line 1: a table's fold is how many rows show before the rest fold away, a whole number from 1 to 100";
    for fold in ["0", "101", "many"] {
        assert_eq!(
            error(&format!(
                ":::table{{fold={fold}}}\n| A |\n|---|\n| 1 |\n:::\n"
            )),
            message
        );
    }
}

#[test]
fn a_table_block_holds_one_table() {
    let message = "line 1: a table block holds one markdown table: a header row, a line of dashes, then one row per line";
    assert_eq!(error(":::table\nJust words.\n:::\n"), message);
    assert_eq!(
        error(":::table\n| A |\n|---|\n| 1 |\n\nA note.\n:::\n"),
        message
    );
}
