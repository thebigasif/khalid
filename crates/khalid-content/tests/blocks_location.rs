//! The location block: one of the site's `[[seo.locations]]` by name, its
//! address, hours and map drawn in place when the page is built (Yoast
//! Local comparison, 2 November 2026).

use khalid_content::{BlockKind, LOCATION_MARK, parse_blocks, print_blocks, render};
use khalid_core::{Locale, SlugMode};

const BLOCK: &str = "::location{name=\"Mill Lane & Sons\"}\n";

#[test]
fn a_location_block_round_trips() {
    let blocks = parse_blocks(BLOCK).unwrap();
    assert_eq!(print_blocks(&blocks), BLOCK);
    assert_eq!(
        blocks[0].kind,
        BlockKind::Location {
            name: "Mill Lane & Sons".to_owned()
        }
    );
}

#[test]
fn it_renders_a_mark_the_build_swaps_for_the_card() {
    let page = render(BLOCK, &Locale::try_new("en").unwrap(), SlugMode::Ascii).unwrap();
    assert_eq!(
        page.html,
        format!("{LOCATION_MARK}Mill Lane &amp; Sons-->\n")
    );
}

#[test]
fn a_location_needs_its_name_and_nothing_else() {
    let no_name = parse_blocks("::location{}\n").unwrap_err().to_string();
    assert!(no_name.contains("a location needs name"), "{no_name}");
    let other = parse_blocks("::location{name=\"A\" city=\"Leeds\"}\n").unwrap_err();
    assert!(other.to_string().contains("city"), "{other}");
}
