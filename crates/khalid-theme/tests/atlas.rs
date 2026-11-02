//! The Atlas template's options and their defaults (templates spec 3.2).

use std::path::Path;

use khalid_core::{Config, Environment};
use khalid_theme::{CheckedOption, Look, LookError};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

#[cfg(test)]
fn choose(options: &str) -> Result<Look, LookError> {
    let text = format!("{SITE}\n[template]\nname = \"atlas\"\n[template.options]\n{options}");
    let config = Config::from_toml(Path::new("site.toml"), &text, &Environment::empty()).unwrap();
    Look::choose(config.template())
}

#[cfg(test)]
fn word(text: &str) -> CheckedOption {
    CheckedOption::Word(text.to_owned())
}

#[test]
fn atlas_turns_on_the_parts_its_spec_lists() {
    let look = choose("").unwrap();
    assert_eq!(look.pack().name, "atlas");
    let options = look.options();
    let on = CheckedOption::Switch(true);
    for (key, value) in [
        ("logo", on.clone()),
        ("breadcrumbs", on.clone()),
        ("featured", on.clone()),
        ("picks", CheckedOption::Number(4)),
        ("cards", word("image")),
        ("card_reading_time", on.clone()),
        ("pagination", word("numbered")),
        ("contents", word("side-start")),
        ("contents_numbers", word("numbered")),
        ("related", word("grid")),
        ("related_count", CheckedOption::Number(3)),
        ("share", word("end")),
        ("newsletter", word("none")),
        ("cta_band", word("both")),
        ("cta_button", on),
    ] {
        assert_eq!(options.get(key), Some(&value), "{key}");
    }
    let networks = ["x", "linkedin", "facebook", "email"].map(str::to_owned);
    assert_eq!(
        options.get("share_networks"),
        Some(&CheckedOption::Lines(networks.to_vec()))
    );
}

#[test]
fn atlas_takes_the_values_written() {
    let look = choose("share = \"both\"\npicks = 0\ncontents = \"side-end\"\n").unwrap();
    let options = look.options();
    assert_eq!(options.get("share"), Some(&word("both")));
    assert_eq!(options.get("picks"), Some(&CheckedOption::Number(0)));
    assert_eq!(options.get("contents"), Some(&word("side-end")));
}

#[test]
fn atlas_takes_a_tagline_of_one_short_line() {
    assert_eq!(
        choose("").unwrap().options().get("tagline"),
        Some(&word(""))
    );
    let look = choose("tagline = \"Search, plainly\"\n").unwrap();
    assert_eq!(
        look.options().get("tagline"),
        Some(&word("Search, plainly"))
    );
    let long = "a".repeat(141);
    let error = choose(&format!("tagline = \"{long}\"\n")).unwrap_err();
    assert_eq!(error.key(), "tagline");
}

#[test]
fn atlas_has_no_masthead_facts() {
    let error = choose("stats = [\"12,000 readers\"]\n").unwrap_err();
    assert_eq!(error.key(), "stats");
    assert!(
        error
            .to_string()
            .starts_with("the atlas template has no option stats")
    );
}
