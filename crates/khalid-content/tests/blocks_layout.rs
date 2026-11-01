//! Pull quote and pros and cons blocks (templates spec 4.4, KH-CON-5).

use khalid_content::{BlockKind, ProsConsBlock, parse_blocks, print_blocks, render};
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

const PULL_QUOTE: &str = ":::pullquote{by=\"Ada, head of search\"}\nNo post should be more than **two clicks** from the home page.\n:::\n";

const PROS_CONS: &str = "\
:::proscons{pros=\"What we liked\" cons=\"What we missed\"}
+ Sets up in **five minutes**
+ A free plan
- No phone support
:::
";

#[test]
fn a_pull_quote_round_trips() {
    round_trip(
        PULL_QUOTE,
        &BlockKind::PullQuote {
            by: Some("Ada, head of search".to_owned()),
        },
    );
    round_trip(
        ":::pullquote\nShort posts win.\n:::\n",
        &BlockKind::PullQuote { by: None },
    );
}

#[test]
fn renders_a_pull_quote() {
    insta::assert_snapshot!(html(PULL_QUOTE));
}

#[test]
fn a_pull_quote_needs_words() {
    assert_eq!(
        error(":::pullquote{by=\"Ada\"}\n:::\n"),
        "line 1: a pull quote needs words, written between its opening and closing ::: lines"
    );
}

#[test]
fn pros_and_cons_round_trip() {
    round_trip(
        PROS_CONS,
        &BlockKind::ProsCons(ProsConsBlock {
            pros_title: "What we liked".to_owned(),
            cons_title: "What we missed".to_owned(),
            pros: vec![
                "Sets up in **five minutes**".to_owned(),
                "A free plan".to_owned(),
            ],
            cons: vec!["No phone support".to_owned()],
        }),
    );
}

#[test]
fn pros_and_cons_titles_default_to_pros_and_cons() {
    let blocks = parse_blocks(":::proscons\n+ Fast\n\n- Costly\n:::\n").unwrap();
    let BlockKind::ProsCons(block) = &blocks[0].kind else {
        panic!("{blocks:?}");
    };
    assert_eq!(
        (block.pros_title.as_str(), block.cons_title.as_str()),
        ("Pros", "Cons")
    );
}

#[test]
fn renders_pros_and_cons() {
    insta::assert_snapshot!(html(PROS_CONS));
}

#[test]
fn one_side_alone_shows_alone() {
    let html = html(":::proscons\n+ Fast\n:::\n");
    assert!(html.contains("class=\"pros\""), "{html}");
    assert!(!html.contains("class=\"cons\""), "{html}");
}

#[test]
fn pros_and_cons_refuse_a_line_with_no_mark() {
    assert_eq!(
        error(":::proscons\n+ Fast\nCostly\n:::\n"),
        "line 3: each line of pros and cons starts with + for a pro or - for a con, then a space"
    );
}

#[test]
fn pros_and_cons_need_a_line() {
    assert_eq!(
        error(":::proscons\n:::\n"),
        "line 1: pros and cons need at least one line, + for a pro or - for a con"
    );
}

#[test]
fn both_read_with_scripts_off() {
    let html = html(&[PULL_QUOTE, "\n", PROS_CONS].concat());
    assert!(!html.contains("<script"), "{html}");
    for words in [
        "two clicks",
        "Ada, head of search",
        "A free plan",
        "No phone support",
    ] {
        assert!(html.contains(words), "{words} is in the page: {html}");
    }
}

#[test]
fn their_links_and_words_are_counted() {
    let rendered = render(
        ":::proscons\n+ Read [the guide](/guides/links/)\n:::\n",
        &Locale::try_new("en").unwrap(),
        SlugMode::Ascii,
    )
    .unwrap();
    assert_eq!(rendered.links.len(), 1);
    assert_eq!(rendered.links[0].href, "/guides/links/");
}
