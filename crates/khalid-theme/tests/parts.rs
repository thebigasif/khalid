//! The part options a pack lists start with classic's value, and `on`
//! turns an on or off part on by default (templates spec 4.2).

use khalid_theme::{OptionKind, parts};

#[test]
fn every_part_starts_off_and_on_turns_a_switch_on() {
    for spec in parts::all() {
        if let OptionKind::Switch { default } = spec.kind {
            assert!(!default, "{} starts on", spec.key);
        }
    }
    let logo = parts::on(parts::LOGO);
    assert_eq!(logo.key, "logo");
    assert!(matches!(logo.kind, OptionKind::Switch { default: true }));
}

#[test]
fn the_cards_option_takes_every_card_shape_and_nothing_else() {
    let words: Vec<&str> = parts::CardShape::ALL
        .iter()
        .map(|shape| shape.word())
        .collect();
    let spec = parts::cards(parts::CardShape::Ranked);
    let OptionKind::Choice { allowed, default } = spec.kind else {
        panic!("cards is not a choice");
    };
    assert_eq!(allowed, words.as_slice());
    assert_eq!(default, "ranked");
}

#[test]
fn every_choice_starts_on_one_of_its_words() {
    for spec in parts::all() {
        if let OptionKind::Choice { allowed, default } = spec.kind {
            assert!(allowed.contains(&default), "{}: {default}", spec.key);
        }
    }
}
