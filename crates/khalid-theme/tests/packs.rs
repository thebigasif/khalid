//! Template packs: classic is today's look and every site's default, a
//! site picks one pack by name, and each option is checked against the
//! pack's own list (templates spec 4.1).

use khalid_core::ReaderSwitch;
use khalid_theme::{OptionKind, classic, packs};

#[test]
fn classic_is_todays_look_and_changes_nothing() {
    let pack = classic();
    assert_eq!(pack.name, "classic");
    assert!(pack.stylesheet.is_empty(), "classic adds no rules");
    assert!(pack.templates.is_empty(), "classic replaces no template");
    assert!(pack.tokens.is_empty(), "classic keeps today's sizes");
    assert!(pack.reader.is_empty(), "classic keeps today's switches");
    assert!(pack.options.is_empty());
}

#[test]
fn the_trial_pack_changes_a_template_a_rule_a_size_and_a_switch() {
    let trial = packs().iter().find(|pack| pack.name == "trial").unwrap();
    assert!(trial.stylesheet.contains(".trial-greeting"));
    let names: Vec<&str> = trial.templates.iter().map(|file| file.name).collect();
    assert_eq!(names, ["not_found.html"]);
    assert!(trial.tokens.contains("[type]"));
    assert!(trial.reader.contains(&(ReaderSwitch::ReadingTime, false)));
}

#[test]
fn every_option_of_every_pack_is_documented_and_its_default_allowed() {
    for pack in packs() {
        assert!(pack.summary.ends_with('.'), "{}", pack.name);
        for option in pack.options {
            assert!(!option.help.is_empty(), "{} {}", pack.name, option.key);
            assert!(option.help.ends_with('.'), "{} {}", pack.name, option.key);
            if let OptionKind::Choice { allowed, default } = option.kind {
                assert!(allowed.contains(&default), "{} {}", pack.name, option.key);
            }
            if let OptionKind::Picks { allowed, default } = option.kind {
                assert!(!default.is_empty(), "{} {}", pack.name, option.key);
                let known = default.iter().all(|word| allowed.contains(word));
                assert!(known, "{} {}", pack.name, option.key);
            }
            if let OptionKind::Count {
                least,
                most,
                default,
            } = option.kind
            {
                let range = least..=most;
                assert!(range.contains(&default), "{} {}", pack.name, option.key);
            }
        }
    }
}

#[test]
fn classic_comes_first_and_names_are_unique() {
    let names: Vec<&str> = packs().iter().map(|pack| pack.name).collect();
    assert_eq!(names.first(), Some(&"classic"));
    let mut sorted = names.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), names.len());
}
