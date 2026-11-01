//! A site's chosen look: the `[template]` table checked against the packs,
//! with a plain sentence for a wrong name, option or value (spec 4.1).

use std::path::Path;

use khalid_core::{Config, Environment};
use khalid_theme::{CheckedOption, Look, LookError};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

#[cfg(test)]
fn choose(table: &str) -> Result<Look, LookError> {
    let text = format!("{SITE}\n{table}");
    let config = Config::from_toml(Path::new("site.toml"), &text, &Environment::empty()).unwrap();
    Look::choose(config.template())
}

#[cfg(test)]
fn word(text: &str) -> CheckedOption {
    CheckedOption::Word(text.to_owned())
}

#[test]
fn a_site_without_the_table_gets_classic_with_no_options() {
    let look = choose("").unwrap();
    assert_eq!(look.pack().name, "classic");
    assert!(look.options().is_empty());
    assert_eq!(look.pack().name, Look::classic().pack().name);
}

#[test]
fn an_unknown_name_lists_the_names_there_are() {
    let error = choose("[template]\nname = \"ledgr\"\n").unwrap_err();
    assert_eq!(error.key(), "name");
    assert_eq!(error.table(), "[template]");
    let message = error.to_string();
    assert!(message.starts_with("there is no template named ledgr; use one of: classic"));
}

#[test]
fn classic_takes_no_options() {
    let error = choose("[template]\n[template.options]\nlead_image = true\n").unwrap_err();
    assert_eq!(error.key(), "lead_image");
    assert_eq!(error.table(), "[template.options]");
    assert_eq!(
        error.to_string(),
        "the classic template has no option lead_image; it takes no options, so remove the line"
    );
}

#[test]
fn an_unknown_option_lists_the_options_the_pack_takes() {
    let error = choose("[template]\nname = \"trial\"\n[template.options]\nbaner = true\n");
    assert_eq!(
        error.unwrap_err().to_string(),
        "the trial template has no option baner; it takes greeting, banner, columns, stats, share, share_networks, newsletter, cta_band, cta_button"
    );
}

#[test]
fn the_trial_pack_fills_every_option_with_its_default() {
    let look = choose("[template]\nname = \"trial\"\n").unwrap();
    let options = look.options();
    assert_eq!(options.get("greeting"), Some(&word("hello")));
    assert_eq!(options.get("banner"), Some(&CheckedOption::Switch(false)));
    assert_eq!(options.get("columns"), Some(&CheckedOption::Number(2)));
    assert_eq!(
        options.get("stats"),
        Some(&CheckedOption::Lines(Vec::new()))
    );
}

#[test]
fn the_trial_pack_takes_the_values_written() {
    let look = choose(
        "[template]\nname = \"trial\"\n[template.options]\ngreeting = \"welcome\"\n\
         banner = true\ncolumns = 3\nstats = [\"12,000 readers\"]\n",
    )
    .unwrap();
    let options = look.options();
    assert_eq!(options.get("greeting"), Some(&word("welcome")));
    assert_eq!(options.get("banner"), Some(&CheckedOption::Switch(true)));
    assert_eq!(options.get("columns"), Some(&CheckedOption::Number(3)));
    let stats = CheckedOption::Lines(vec!["12,000 readers".to_owned()]);
    assert_eq!(options.get("stats"), Some(&stats));
}

#[test]
fn each_wrong_value_says_what_the_option_takes() {
    for (line, wanted) in [
        ("greeting = \"hi\"", "takes one of hello, welcome"),
        ("greeting = true", "takes one of hello, welcome"),
        ("banner = \"yes\"", "takes true or false"),
        ("columns = 9", "takes a whole number from 2 to 4"),
        (
            "stats = [\"a\", \"b\", \"c\", \"d\"]",
            "takes a list of up to 3 lines",
        ),
        ("stats = { a = 1 }", "takes a list of up to 3 lines"),
    ] {
        let table = format!("[template]\nname = \"trial\"\n[template.options]\n{line}\n");
        let error = choose(&table).unwrap_err();
        assert_eq!(error.table(), "[template.options]");
        let message = error.to_string();
        assert!(message.contains(wanted), "{line}: {message}");
        assert!(message.starts_with("the option "), "{message}");
    }
}

#[test]
fn share_networks_take_known_names_once_each() {
    let look = choose(
        "[template]\nname = \"trial\"\n[template.options]\nshare = \"end\"\n\
         share_networks = [\"email\", \"linkedin\"]\n",
    )
    .unwrap();
    let picked = CheckedOption::Lines(vec!["email".to_owned(), "linkedin".to_owned()]);
    assert_eq!(look.options().get("share_networks"), Some(&picked));
    assert_eq!(look.options().get("share"), Some(&word("end")));
    for wrong in ["[\"myspace\"]", "[]", "[\"email\", \"email\"]", "\"email\""] {
        let table =
            format!("[template]\nname = \"trial\"\n[template.options]\nshare_networks = {wrong}\n");
        let message = choose(&table).unwrap_err().to_string();
        assert!(
            message.contains("takes a list of one or more of x, linkedin, facebook"),
            "{wrong}: {message}"
        );
    }
}

#[test]
fn the_look_keeps_the_sites_call_to_action() {
    assert!(choose("").unwrap().cta().is_none());
    let look = choose("[template.cta]\nlabel = \"Book a demo\"\nhref = \"/demo/\"\n").unwrap();
    assert_eq!(look.pack().name, "classic");
    assert_eq!(
        look.cta().map(|cta| cta.label.as_str()),
        Some("Book a demo")
    );
}

#[test]
fn options_reach_templates_as_plain_values() {
    let look = choose("[template]\nname = \"trial\"\n").unwrap();
    let json = serde_json::to_string(look.options()).unwrap();
    assert_eq!(
        json,
        r#"{"banner":false,"columns":2,"cta_band":"both","cta_button":true,"greeting":"hello","newsletter":"both","share":"end","share_networks":["linkedin","x","email"],"stats":[]}"#
    );
}
