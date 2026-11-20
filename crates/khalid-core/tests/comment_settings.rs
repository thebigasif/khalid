//! `[comments]` in `site.toml`: whether readers may comment on the site's
//! posts, open unless the site says closed or off.

use std::path::Path;

use khalid_core::{Config, CommentState, Environment};

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
fn a_site_without_the_table_takes_comments() {
    assert_eq!(load(SITE).unwrap().comments().state, CommentState::Open);
}

#[test]
fn a_site_can_close_comments_or_turn_them_off() {
    for (word, state) in [("closed", CommentState::Closed), ("off", CommentState::Off)] {
        let text = format!("{SITE}\n[comments]\nstate = \"{word}\"\n");
        assert_eq!(load(&text).unwrap().comments().state, state);
    }
}

#[test]
fn an_unknown_state_is_refused() {
    let text = format!("{SITE}\n[comments]\nstate = \"maybe\"\n");
    let error = load(&text).unwrap_err();
    assert!(error.contains("maybe"), "{error}");
}
