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
