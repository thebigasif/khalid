//! The Ledger template's options: its defaults, and the tagline, its one
//! option of free text (templates spec 3.1).

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
    let text = format!("{SITE}\n[template]\nname = \"ledger\"\n[template.options]\n{options}");
    let config = Config::from_toml(Path::new("site.toml"), &text, &Environment::empty()).unwrap();
    Look::choose(config.template())
}

#[cfg(test)]
fn word(text: &str) -> CheckedOption {
    CheckedOption::Word(text.to_owned())
}

#[test]
fn ledger_turns_on_the_parts_its_spec_lists() {
    let look = choose("").unwrap();
    assert_eq!(look.pack().name, "ledger");
    let options = look.options();
    let on = CheckedOption::Switch(true);
    let off = CheckedOption::Switch(false);
    for (key, value) in [
        ("logo", on),
        ("lead_image", off.clone()),
        ("tagline", word("")),
        ("stats", CheckedOption::Lines(Vec::new())),
        ("cards", word("text")),
        ("pagination", word("numbered")),
        ("contents", word("inline")),
        ("contents_numbers", word("none")),
        ("related", word("list")),
        ("related_count", CheckedOption::Number(5)),
        ("newsletter", word("both")),
        ("cta_band", word("none")),
        ("cta_button", off),
    ] {
        assert_eq!(options.get(key), Some(&value), "{key}");
    }
    assert_eq!(options.get("share"), None, "Ledger has no share row");
}

#[test]
fn ledger_takes_a_tagline_and_three_facts() {
    let look = choose(
        "tagline = \"Studies and long how-tos on search\"\n\
         stats = [\"12,000 readers\", \"Weekly since 2024\"]\nlead_image = true\n",
    )
    .unwrap();
    let options = look.options();
    assert_eq!(
        options.get("tagline"),
        Some(&word("Studies and long how-tos on search"))
    );
    assert_eq!(
        options.get("lead_image"),
        Some(&CheckedOption::Switch(true))
    );
}

#[test]
fn a_tagline_is_one_short_line() {
    let long = "a".repeat(141);
    for wrong in [
        "\"two\\nlines\"".to_owned(),
        format!("\"{long}\""),
        "[\"a list\"]".to_owned(),
    ] {
        let error = choose(&format!("tagline = {wrong}\n")).unwrap_err();
        assert_eq!(error.key(), "tagline");
        let message = error.to_string();
        assert!(
            message.contains("takes one line of text of up to 140 characters"),
            "{wrong}: {message}"
        );
    }
}
