//! What a suggestion is for, the instructions each kind sends, and the
//! length each option is checked against.

use std::fmt::Write as _;

use khalid_lint::Thresholds;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// What a suggestion is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AiTask {
    /// The title search results show.
    SeoTitle,
    /// The meta description search results show under the title.
    MetaDescription,
    /// The title on a share card, on X or Facebook.
    ShareTitle,
    /// The description on a share card, on X or Facebook.
    ShareDescription,
    /// Alt text for a picture.
    AltText,
    /// A way to fix a check that failed.
    LintFix,
    /// The points of a key takeaways block.
    Takeaways,
}

/// The fewest and most characters an option may have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct LengthRule {
    /// The fewest characters.
    pub min: usize,
    /// The most characters.
    pub max: usize,
}

impl LengthRule {
    /// From `min` to `max` characters.
    pub const fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

/// The lengths options are checked against: the linter's for titles and
/// descriptions, so a suggestion never fails the check it is written for,
/// and fixed ones for the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Lengths {
    /// The SEO title's.
    pub title: LengthRule,
    /// The meta description's.
    pub description: LengthRule,
}

impl Default for Lengths {
    fn default() -> Self {
        Self::from_thresholds(&Thresholds::default())
    }
}

impl Lengths {
    /// The lengths a site's `lint.toml` sets.
    pub fn from_thresholds(thresholds: &Thresholds) -> Self {
        Self {
            title: LengthRule::new(thresholds.title_min_chars, thresholds.title_max_chars),
            description: LengthRule::new(
                thresholds.description_min_chars,
                thresholds.description_max_chars,
            ),
        }
    }

    /// The rule for `task`.
    pub fn rule(&self, task: AiTask) -> LengthRule {
        match task {
            AiTask::SeoTitle => self.title,
            AiTask::MetaDescription => self.description,
            AiTask::ShareTitle => SHARE_TITLE,
            AiTask::ShareDescription => SHARE_DESCRIPTION,
            AiTask::AltText => ALT_TEXT,
            AiTask::LintFix => LINT_FIX,
            AiTask::Takeaways => TAKEAWAY,
        }
    }

    /// Why `option` does not fit `task`'s rule, or `None` when it fits.
    pub fn misfit(&self, task: AiTask, option: &str) -> Option<String> {
        let rule = self.rule(task);
        let length = option.chars().count();
        if length > rule.max {
            return Some(format!(
                "Too long: {length} characters, and {} is the most that shows in full.",
                rule.max
            ));
        }
        (length < rule.min).then(|| {
            format!(
                "Too short: {length} characters, and it needs at least {} to say enough.",
                rule.min
            )
        })
    }
}

/// Share cards on X and Facebook show about 70 characters of a title.
const SHARE_TITLE: LengthRule = LengthRule::new(10, 70);

/// And about 200 of a description.
const SHARE_DESCRIPTION: LengthRule = LengthRule::new(30, 200);

/// Screen readers read alt text in one go; 125 characters keeps it short.
const ALT_TEXT: LengthRule = LengthRule::new(5, 125);

/// A fix is an instruction or a rewritten line, never an essay.
const LINT_FIX: LengthRule = LengthRule::new(5, 400);

/// Each takeaway is one sentence.
const TAKEAWAY: LengthRule = LengthRule::new(10, 160);

/// The most characters of a post's text a request sends, so a long post
/// costs the owner no more than a short one.
const MOST_TEXT: usize = 12_000;

/// The words a request is written from. Only what the task needs is set.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct AiWords {
    /// The post's language, such as `en`.
    pub language: Option<String>,
    /// The post's title.
    pub title: Option<String>,
    /// The post's text, in markdown.
    pub text: Option<String>,
    /// The post's focus keyword.
    pub focus_keyword: Option<String>,
    /// The failing check, for a fix: its rule and message.
    pub finding: Option<(String, String)>,
    /// The picture, for alt text: its file name, caption and current alt.
    pub picture: Option<(String, Option<String>, Option<String>)>,
}

impl AiTask {
    /// How many options to ask for.
    pub fn count(self) -> u8 {
        match self {
            Self::LintFix => 3,
            _ => 5,
        }
    }

    /// The instructions every request of this task carries.
    pub fn system(self, lengths: &Lengths) -> String {
        let rule = lengths.rule(self);
        let (min, max) = (rule.min, rule.max);
        let what = match self {
            Self::SeoTitle => format!(
                "Write SEO titles for the blog post below, each {min} to {max} characters, with the focus keyword near the start when one is given. Each must be true to the post and give a reason to click."
            ),
            Self::MetaDescription => format!(
                "Write meta descriptions for the blog post below, each {min} to {max} characters: one or two full sentences that say what the reader gets, with the focus keyword when one is given."
            ),
            Self::ShareTitle => format!(
                "Write titles for the post's share card on social media, each {min} to {max} characters, that make someone stop scrolling while staying true to the post."
            ),
            Self::ShareDescription => format!(
                "Write descriptions for the post's share card on social media, each {min} to {max} characters, that say why the post is worth opening."
            ),
            Self::AltText => format!(
                "Write alt text for the picture described below, each {min} to {max} characters: what it shows that matters to the post. Never start with \"image of\" or \"picture of\"."
            ),
            Self::LintFix => format!(
                "A check on the blog post below failed. Write ways to fix it, each at most {max} characters: the rewritten sentence, title or description itself when the check is about one, otherwise one short instruction."
            ),
            Self::Takeaways => format!(
                "Write the key takeaways of the blog post below: three to five points, each one plain sentence of at most {max} characters, in the order the post makes them."
            ),
        };
        format!(
            "{what} Write in the post's language. Answer with at most {} options, one per line, and nothing else: no numbers, no quotes, no notes.",
            self.count()
        )
    }

    /// The words this task sends, from `words`.
    pub fn prompt(self, words: &AiWords) -> String {
        let mut prompt = String::new();
        let mut line = |name: &str, value: Option<&str>| {
            if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
                let _ = writeln!(prompt, "{name}: {}", value.trim());
            }
        };
        line("Language", words.language.as_deref());
        line("Title", words.title.as_deref());
        line("Focus keyword", words.focus_keyword.as_deref());
        if let Some((rule, message)) = &words.finding {
            line("Failed check", Some(rule));
            line("What it found", Some(message));
        }
        if let Some((name, caption, alt)) = &words.picture {
            line("Picture file", Some(name));
            line("Caption", caption.as_deref());
            line("Alt text now", alt.as_deref());
        }
        let text = words.text.as_deref().unwrap_or_default();
        if self != Self::AltText && !text.trim().is_empty() {
            let cut: String = text.chars().take(MOST_TEXT).collect();
            let _ = write!(prompt, "\nPost:\n{}", cut.trim());
        }
        prompt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_use_the_linters_lengths() {
        let lengths = Lengths::default();
        assert_eq!(lengths.misfit(AiTask::SeoTitle, &"a".repeat(60)), None);
        assert_eq!(
            lengths.misfit(AiTask::SeoTitle, &"a".repeat(61)).unwrap(),
            "Too long: 61 characters, and 60 is the most that shows in full."
        );
        assert_eq!(
            lengths.misfit(AiTask::MetaDescription, "Short.").unwrap(),
            "Too short: 6 characters, and it needs at least 50 to say enough."
        );
    }

    #[test]
    fn letters_count_once_whatever_their_bytes() {
        let lengths = Lengths::default();
        assert_eq!(lengths.misfit(AiTask::SeoTitle, &"ü".repeat(60)), None);
    }

    #[test]
    fn alt_text_never_sends_the_post() {
        let words = AiWords {
            title: Some("Choosing anchor text".to_owned()),
            text: Some("The whole post.".to_owned()),
            picture: Some(("anchors.png".to_owned(), None, None)),
            ..AiWords::default()
        };
        let prompt = AiTask::AltText.prompt(&words);
        assert!(prompt.contains("Picture file: anchors.png"));
        assert!(!prompt.contains("The whole post."));
    }

    #[test]
    fn a_long_post_is_cut() {
        let words = AiWords {
            text: Some("word ".repeat(10_000)),
            ..AiWords::default()
        };
        let prompt = AiTask::Takeaways.prompt(&words);
        assert!(prompt.chars().count() < MOST_TEXT + 20);
    }
}
