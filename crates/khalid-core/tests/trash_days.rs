//! How long a deleted post waits in the trash before `khalid serve` empties
//! it: 30 days unless `site.toml` sets `trash_days`, and never when it is 0
//! (backlog 2026-11-10 row 16, G52).

use std::path::Path;

use khalid_core::{Config, Environment};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

/// Reads the site with `extra` lines added.
#[cfg(test)]
fn with(extra: &str) -> Result<Config, String> {
    let text = format!("{SITE}{extra}");
    Config::from_toml(Path::new("site.toml"), &text, &Environment::empty())
        .map_err(|error| error.to_string())
}

#[test]
fn a_deleted_post_waits_30_days_by_default() {
    assert_eq!(with("").unwrap().trash_days(), Some(30));
}

#[test]
fn a_site_sets_the_days_or_keeps_the_trash_for_good() {
    assert_eq!(with("trash_days = 7\n").unwrap().trash_days(), Some(7));
    assert_eq!(
        with("trash_days = 3650\n").unwrap().trash_days(),
        Some(3650)
    );
    assert_eq!(with("trash_days = 0\n").unwrap().trash_days(), None);
}

#[test]
fn more_than_ten_years_is_refused() {
    let expected = "site.toml, line 5, field \"trash_days\": the trash keeps a deleted post for up to 3650 days, so use 1 to 3650, or 0 to never empty it";
    assert_eq!(with("trash_days = 3651\n").unwrap_err(), expected);
}
