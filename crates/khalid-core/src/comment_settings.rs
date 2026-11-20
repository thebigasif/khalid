//! `[comments]` in `site.toml`: whether readers may comment on the site's
//! posts. A post's own `comments:` field wins over it.

use serde::Deserialize;

/// Whether readers may comment, on the whole site or on one post.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentState {
    /// Readers may comment, and approved comments show. The default.
    #[default]
    Open,
    /// Approved comments still show, but the form is gone, so no new
    /// comment comes in.
    Closed,
    /// No comments show and there is no form.
    Off,
}

impl CommentState {
    /// The state of a post that says `own`, or nothing, on a site whose
    /// posts are `site` unless they say otherwise.
    #[must_use]
    pub fn of_post(site: Self, own: Option<Self>) -> Self {
        own.unwrap_or(site)
    }
}

/// `[comments]`: how the site takes comments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CommentSettings {
    /// Whether readers may comment on a post that does not say otherwise:
    /// `open`, the default, `closed` or `off`.
    #[serde(default)]
    pub state: CommentState,
    /// Whether the site's owners and admins get an email about each new
    /// reader's comment, when the site sends mail. On unless set to
    /// `false`.
    #[serde(default = "on")]
    pub notify: bool,
}

impl Default for CommentSettings {
    fn default() -> Self {
        Self {
            state: CommentState::default(),
            notify: true,
        }
    }
}

fn on() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_post_says_its_own_state_or_takes_the_sites() {
        let closed = CommentState::Closed;
        assert_eq!(CommentState::of_post(closed, None), closed);
        let open = Some(CommentState::Open);
        assert_eq!(CommentState::of_post(closed, open), CommentState::Open);
    }
}
