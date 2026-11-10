//! `[limits]` in `site.toml`: how many requests the APIs and the search
//! page take a minute before they answer 429 (KH-OPS-5, G33).

use std::path::Path;

use khalid_core::{Config, Environment, LimitSettings};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

#[cfg(test)]
fn load(text: &str) -> Result<Config, String> {
    Config::from_toml(Path::new("site.toml"), text, &Environment::default())
        .map_err(|error| error.to_string())
}

#[test]
fn a_site_without_the_table_keeps_the_usual_limits() {
    let limits = *load(SITE).unwrap().limits();
    assert_eq!(limits, LimitSettings::default());
    assert_eq!(limits.admin_per_minute, 600);
    assert_eq!(limits.content_per_minute, 6000);
    assert_eq!(limits.wrong_keys_per_minute, 10);
    assert_eq!(limits.search_per_minute, 120);
}

#[test]
fn reads_every_key() {
    let text = format!(
        "{SITE}\n[limits]\nadmin_per_minute = 60\ncontent_per_minute = 100000\n\
         wrong_keys_per_minute = 3\nsearch_per_minute = 30\n"
    );
    let limits = *load(&text).unwrap().limits();
    assert_eq!(limits.admin_per_minute, 60);
    assert_eq!(limits.content_per_minute, 100_000);
    assert_eq!(limits.wrong_keys_per_minute, 3);
    assert_eq!(limits.search_per_minute, 30);
}

#[test]
fn a_key_left_out_keeps_its_usual_limit() {
    let text = format!("{SITE}\n[limits]\nsearch_per_minute = 30\n");
    let limits = *load(&text).unwrap().limits();
    assert_eq!(limits.search_per_minute, 30);
    assert_eq!(limits.admin_per_minute, 600);
}

#[test]
fn refuses_a_limit_outside_one_to_a_hundred_thousand() {
    for (key, count) in [
        ("admin_per_minute", 0),
        ("content_per_minute", 100_001),
        ("wrong_keys_per_minute", 0),
        ("search_per_minute", 0),
    ] {
        let text = format!("{SITE}\n[limits]\n{key} = {count}\n");
        assert_eq!(
            load(&text).unwrap_err(),
            format!(
                "site.toml, line 7, field \"{key}\": a limit counts 1 to 100000 requests a minute, so use a number in that range"
            ),
            "{key} = {count}"
        );
    }
}

#[test]
fn refuses_an_unknown_key() {
    let text = format!("{SITE}\n[limits]\nuploads_per_minute = 5\n");
    let error = load(&text).unwrap_err();
    assert!(error.contains("uploads_per_minute"), "{error}");
}
