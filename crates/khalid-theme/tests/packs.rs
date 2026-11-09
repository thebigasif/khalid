//! Template packs: classic is today's look and every site's default, a
//! site picks one pack by name, and each option is checked against the
//! pack's own list (templates spec 4.1).

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use khalid_core::ReaderSwitch;
use khalid_theme::{OptionKind, classic, packs};

/// The packs built only for tests, which ship in no binary and have no
/// `pack.toml`.
const TEST_ONLY: [&str; 2] = ["trial", "sampler"];

#[test]
fn classic_is_the_default_look_and_only_restyles() {
    let pack = classic();
    assert_eq!(pack.name, "classic");
    assert!(
        pack.stylesheet().contains("--frame"),
        "classic sets its own frame"
    );
    assert!(pack.templates.is_empty(), "classic replaces no template");
    assert!(pack.tokens().is_empty(), "classic keeps today's sizes");
    assert!(pack.reader.is_empty(), "classic keeps today's switches");
    assert!(pack.options.is_empty());
}

#[test]
fn the_trial_pack_changes_a_template_a_rule_a_size_and_a_switch() {
    let trial = packs().iter().find(|pack| pack.name == "trial").unwrap();
    assert!(trial.stylesheet().contains(".trial-greeting"));
    let names: Vec<&str> = trial.templates.iter().map(|file| file.name).collect();
    assert_eq!(names, ["not_found.html"]);
    assert!(trial.tokens().contains("[type]"));
    assert!(trial.reader.contains(&(ReaderSwitch::ReadingTime, false)));
}

/// Each pack's token defaults sit in the packs' Brotli archive and come
/// out byte for byte as its `tokens.toml` says.
#[test]
fn every_pack_s_tokens_read_as_its_file_says() {
    let mut checked = 0;
    for pack in packs() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push(format!("packs/{}/tokens.toml", pack.name));
        let Ok(text) = fs::read_to_string(&file) else {
            continue;
        };
        assert_eq!(pack.tokens(), text, "{}", pack.name);
        checked += 1;
    }
    assert!(checked >= 40, "{checked}");
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
/// them back as the text of the files in `packs/`, with each comment
/// emptied as the build script empties it.
/// Empties every `{# … #}` comment of a template and keeps its `-` marks,
/// which trim the space around it. A comment renders nothing either way, so
/// every page is the same; the binary just stops carrying the comments'
/// words.
fn strip_comments(template: &str) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{#") {
        out.push_str(rest.get(..start).unwrap_or_default());
        let comment = rest.get(start..).unwrap_or_default();
        let Some(end) = comment.find("#}") else {
            out.push_str(comment);
            return out;
        };
        let inner = comment.get(2..end).unwrap_or_default();
        out.push_str("{#");
        if inner.starts_with('-') {
            out.push('-');
        }
        out.push(' ');
        if inner.len() > 1 && inner.ends_with('-') {
            out.push('-');
        }
        out.push_str("#}");
        rest = comment.get(end.saturating_add(2)..).unwrap_or_default();
    }
    out.push_str(rest);
    out
}

#[test]
fn every_pack_template_reads_back_as_its_file() {
    for pack in khalid_theme::packs() {
        for file in pack.templates {
            let path = format!(
                "{}/packs/{}/{}",
                env!("CARGO_MANIFEST_DIR"),
                pack.name,
                file.name
            );
            let written =
                std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{path}: {error}"));
            assert_eq!(file.source(), strip_comments(&written), "{path}");
        }
    }
}

/// The scripts list the shipped templates from the folders in `packs/` that
/// hold a `pack.toml` (`scripts/template-packs.sh`), so those folders are
/// exactly the packs a binary ships, and each file gives the kind of blog
/// the template suits, its heading and its minimum frame.
#[test]
fn every_shipped_pack_and_no_other_has_a_pack_toml() {
    let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    folder.push("packs");
    let mut described = BTreeSet::new();
    for entry in fs::read_dir(&folder).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        let mut file = entry.path();
        file.push("pack.toml");
        let Ok(text) = fs::read_to_string(&file) else {
            continue;
        };
        let table: toml::Table = toml::from_str(&text).unwrap();
        for key in ["kind", "kind_label"] {
            let value = table.get(key).and_then(toml::Value::as_str);
            assert!(
                value.is_some_and(|value| !value.is_empty()),
                "{}: {key}",
                file.display()
            );
        }
        let frame = table.get("frame").and_then(toml::Value::as_integer);
        assert!(
            frame.is_some_and(|frame| frame > 0),
            "{}: frame",
            file.display()
        );
        described.insert(name);
    }
    let shipped: BTreeSet<String> = packs()
        .iter()
        .map(|pack| pack.name)
        .filter(|name| !TEST_ONLY.contains(name))
        .map(str::to_owned)
        .collect();
    assert_eq!(described, shipped);
}
