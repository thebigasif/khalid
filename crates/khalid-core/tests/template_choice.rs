//! `[template]` in `site.toml`: which built-in look the site uses, and that
//! look's options, read as written so the theme can check them (spec 4.1).

use std::path::Path;

use khalid_core::{Config, Environment, OptionValue, ReaderSwitch};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

#[cfg(test)]
fn load(text: &str) -> Result<Config, String> {
    let text = format!("{SITE}\n{text}");
    Config::from_toml(Path::new("site.toml"), &text, &Environment::empty())
        .map_err(|error| error.to_string())
}

#[test]
fn a_site_without_the_table_uses_classic_with_no_options() {
    let config = load("").unwrap();
    assert_eq!(config.template().name().as_str(), "classic");
    assert!(config.template().options().is_empty());
}

#[test]
fn reads_the_name_and_every_kind_of_option() {
    let config = load(
        "[template]\nname = \"ledger\"\n[template.options]\nlead_image = true\n\
         contents = \"side-end\"\nstats = [\"12,000 readers\"]\n\
         related = { layout = \"list\", count = 5 }\n",
    )
    .unwrap();
    let template = config.template();
    assert_eq!(template.name().as_str(), "ledger");
    let options = template.options();
    assert_eq!(options.get("lead_image"), Some(&OptionValue::Switch(true)));
    assert_eq!(
        options.get("contents"),
        Some(&OptionValue::Word("side-end".to_owned()))
    );
    assert_eq!(
        options.get("stats"),
        Some(&OptionValue::Words(vec!["12,000 readers".to_owned()]))
    );
    let Some(OptionValue::Table(related)) = options.get("related") else {
        panic!("related is a table");
    };
    assert_eq!(related.get("count"), Some(&OptionValue::Number(5)));
}

#[test]
fn refuses_a_name_that_is_not_a_plain_word() {
    let error = load("[template]\nname = \"Ledger Pro\"\n").unwrap_err();
    assert!(error.contains("line 7"), "{error}");
    assert!(error.contains("lowercase letters"), "{error}");
}

#[test]
fn refuses_an_option_of_a_kind_no_template_takes() {
    let error =
        load("[template]\nname = \"ledger\"\n[template.options]\nscale = 1.5\n").unwrap_err();
    assert!(error.contains("on or off, a whole number"), "{error}");
}

#[test]
fn refuses_an_unknown_key_in_the_table() {
    let error = load("[template]\nname = \"ledger\"\ncolour = \"red\"\n").unwrap_err();
    assert!(error.contains("colour"), "{error}");
}

#[test]
fn a_template_default_fills_only_the_switches_the_site_left_out() {
    let config = load("[reader]\nbyline = true\n").unwrap();
    let defaults = [
        (ReaderSwitch::Byline, false),
        (ReaderSwitch::ReadingTime, false),
    ];
    let config = config.with_reader_defaults(&defaults);
    assert!(config.reader().byline, "the site's own key wins");
    assert!(
        !config.reader().reading_time,
        "the template's default fills"
    );
    assert!(
        config.reader().author_box,
        "a switch neither names stays on"
    );
}

#[test]
fn every_switch_has_its_reader_key() {
    let keys: Vec<&str> = ReaderSwitch::ALL.iter().map(|s| s.key()).collect();
    assert_eq!(keys.len(), 15);
    for key in keys {
        let config = load(&format!("[reader]\n{key} = false\n"));
        assert!(config.is_ok(), "{key} is a [reader] key");
    }
}
