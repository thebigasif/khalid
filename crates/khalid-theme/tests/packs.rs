//! Template packs: classic is today's look and every site's default, a
//! site picks one pack by name, and each option is checked against the
//! pack's own list (templates spec 4.1).

use khalid_core::ReaderSwitch;
use khalid_theme::{OptionKind, classic, packs};

#[test]
fn classic_is_the_default_look_and_only_restyles() {
    let pack = classic();
    assert_eq!(pack.name, "classic");
    assert!(
        pack.stylesheet().contains("--frame"),
        "classic sets its own frame"
    );
    assert!(pack.templates.is_empty(), "classic replaces no template");
    assert!(pack.tokens.is_empty(), "classic keeps today's sizes");
    assert!(pack.reader.is_empty(), "classic keeps today's switches");
    assert!(pack.options.is_empty());
}

#[test]
fn the_trial_pack_changes_a_template_a_rule_a_size_and_a_switch() {
    let trial = packs().iter().find(|pack| pack.name == "trial").unwrap();
    assert!(trial.stylesheet().contains(".trial-greeting"));
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

/// Every pack's stylesheet reads back from the compressed copy the binary
/// carries, as the minified text with no comment left in it.
#[test]
fn every_pack_stylesheet_reads_back_from_the_binary() {
    for pack in khalid_theme::packs() {
        let css = pack.stylesheet();
        if css.is_empty() {
            // Only the test build's sampler has no rules of its own.
            assert_eq!(pack.name, "sampler");
            continue;
        }
        assert!(css.contains('{'), "{} has no rules", pack.name);
        assert!(!css.contains("/*"), "{} kept a comment", pack.name);
        assert!(css.ends_with('\n'), "{}", pack.name);
    }
}

/// The binary carries each pack's template files compressed, and reads
/// them back as the very text of the files in `packs/`.
#[test]
fn every_pack_template_reads_back_as_its_file() {
    let packs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("packs");
    for pack in khalid_theme::packs() {
        for file in pack.templates {
            let written = std::fs::read_to_string(packs.join(pack.name).join(file.name))
                .unwrap_or_else(|error| panic!("{}/{}: {error}", pack.name, file.name));
            assert_eq!(file.source(), written, "{}/{}", pack.name, file.name);
        }
    }
}
