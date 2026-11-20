//! A post's `comments:` field opens, closes or turns off its comments in
//! place of the site's `[comments]` state.

use std::path::Path;

use khalid_content::parse_post;
use khalid_core::CommentState;

#[cfg(test)]
fn read(yaml: &str) -> Result<Option<CommentState>, String> {
    let text = format!("---\ntitle: A\n{yaml}\n---\nBody.\n");
    parse_post(Path::new("posts/a.md"), &text)
        .map(|post| post.front.comments)
        .map_err(|error| error.to_string())
}

#[test]
fn a_post_follows_the_site_unless_it_says_otherwise() {
    assert_eq!(read("date: 2026-10-27"), Ok(None));
    assert_eq!(read("comments: open"), Ok(Some(CommentState::Open)));
    assert_eq!(read("comments: closed"), Ok(Some(CommentState::Closed)));
    assert_eq!(read("comments: off"), Ok(Some(CommentState::Off)));
}

#[test]
fn an_unknown_state_names_the_field() {
    let error = read("comments: later").unwrap_err();
    assert!(error.contains("comments"), "{error}");
}
