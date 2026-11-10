//! `[log]` in `site.toml`: how much `khalid serve` writes to standard error
//! and in which shape (G38).

use std::path::Path;

use khalid_core::{Config, Environment, LogFormat, LogLevel, LogSettings, MailTransport};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

#[cfg(test)]
fn load(text: &str, env: &Environment) -> Result<Config, String> {
    Config::from_toml(Path::new("site.toml"), text, env).map_err(|error| error.to_string())
}

#[test]
fn a_site_without_the_table_logs_plain_lines_at_info() {
    let log = load(SITE, &Environment::empty()).unwrap().log();
    assert_eq!(log, LogSettings::default());
    assert_eq!(log.level, LogLevel::Info);
    assert_eq!(log.format, LogFormat::Text);
}

#[test]
fn reads_every_key() {
    let text = format!("{SITE}\n[log]\nlevel = \"debug\"\nformat = \"json\"\n");
    let log = load(&text, &Environment::empty()).unwrap().log();
    assert_eq!(log.level, LogLevel::Debug);
    assert_eq!(log.format, LogFormat::Json);
}

#[test]
fn refuses_an_unknown_level_with_its_line() {
    let text = format!("{SITE}\n[log]\nlevel = \"chatty\"\n");
    let refused = load(&text, &Environment::empty()).unwrap_err();
    assert!(refused.contains("line 7"), "{refused}");
}

#[test]
fn a_variable_set_with_with_counts_as_set() {
    let text = format!("{SITE}\n[mail]\nfrom = \"news@example.com\"\n\n[mail.resend]\n");
    assert!(load(&text, &Environment::empty()).is_err());
    let env = Environment::empty().with(Environment::RESEND_API_KEY, "made-for-this-test");
    let config = load(&text, &env).unwrap();
    assert_eq!(config.mail().unwrap().transport, MailTransport::Resend);
}
