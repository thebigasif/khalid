//! `[comments]` in `site.toml`: whether readers may comment on the site's
//! posts, open unless the site says closed or off.

use std::path::Path;

use khalid_core::{CommentState, Config, Environment};

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
fn a_site_without_the_table_takes_comments_and_mails_about_them() {
    let comments = load(SITE).unwrap().comments();
    assert_eq!(comments.state, CommentState::Open);
    assert!(comments.notify);
}

#[test]
fn the_mail_about_new_comments_can_be_switched_off() {
    let text = format!("{SITE}\n[comments]\nnotify = false\n");
    let comments = load(&text).unwrap().comments();
    assert!(!comments.notify);
    assert_eq!(comments.state, CommentState::Open);
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

#[test]
fn each_state_writes_the_name_it_reads() {
    for state in CommentState::ALL {
        let written = serde_json::to_value(state).unwrap();
        assert_eq!(written, state.as_str());
        assert_eq!(state.to_string(), state.as_str());
        let read: CommentState = serde_json::from_value(written).unwrap();
        assert_eq!(read, state);
    }
}
