//! The focus checks again for each secondary keyword, as tips: the title,
//! the first paragraph, the subheadings and how often the text uses it.
//!
//! A secondary keyword is a close phrase the post also wants to be found
//! for. Yoast Premium runs its keyphrase checks on each related keyphrase;
//! Khalid does the same at the info level, so a missing one is a hint and
//! never costs points or blocks.

use crate::focus::{Keyword, Place, per_thousand};
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// One hit per secondary keyword that `missing` says the post lacks,
/// named in the header's order.
fn each_missing(post: &LintPost, missing: impl Fn(&Keyword<'_>) -> bool) -> Vec<Hit> {
    Keyword::secondary(post)
        .iter()
        .filter(|keyword| missing(keyword))
        .map(|keyword| Hit::about(keyword.text))
        .collect()
}

/// A secondary keyword missing from one place: the search title, the first
/// paragraph or the subheadings.
#[derive(Debug)]
pub(crate) struct SecondaryNotIn(pub(crate) Place);

impl Rule for SecondaryNotIn {
    fn id(&self) -> RuleId {
        RuleId::new(match self.0 {
            Place::FirstParagraph => "secondary-not-in-first-paragraph",
            Place::Subheading => "secondary-not-in-subheading",
            _ => "secondary-not-in-title",
        })
    }

    fn default_severity(&self) -> Severity {
        Severity::Info
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        each_missing(post, |keyword| self.0.holds(post, keyword) == Some(false))
    }
}

/// A secondary keyword the text uses less often per thousand words than
/// the focus keyword's floor. A keyword the text never uses is left to
/// `secondary-keyword-unused`.
#[derive(Debug)]
pub(crate) struct SecondaryTooRare {
    /// From `Thresholds::focus_min_per_thousand_words`.
    pub(crate) min: usize,
}

impl Rule for SecondaryTooRare {
    fn id(&self) -> RuleId {
        RuleId::new("secondary-too-rare")
    }

    fn default_severity(&self) -> Severity {
        Severity::Info
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        each_missing(post, |keyword| {
            per_thousand(post, keyword).is_some_and(|rate| rate > 0 && rate < self.min)
        })
    }
}

/// A secondary keyword the text uses more often per thousand words than
/// the focus keyword's ceiling.
#[derive(Debug)]
pub(crate) struct SecondaryTooOften {
    /// From `Thresholds::focus_max_per_thousand_words`.
    pub(crate) max: usize,
}

impl Rule for SecondaryTooOften {
    fn id(&self) -> RuleId {
        RuleId::new("secondary-too-often")
    }

    fn default_severity(&self) -> Severity {
        Severity::Info
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        each_missing(post, |keyword| {
            per_thousand(post, keyword).is_some_and(|rate| rate > self.max)
        })
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Heading, HeadingLevel, LintSite};

    fn post(language: &str, secondary: &[&str]) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new("a").unwrap());
        post.secondary_keywords = secondary.iter().map(|&k| k.to_owned()).collect();
        post
    }

    #[test]
    fn each_missing_secondary_keyword_is_one_tip_in_header_order() {
        let mut post = post("en", &["link text", "site structure", "orphan pages"]);
        post.title = Some("Choosing link texts for a clear site structure".to_owned());
        post.body_text = "Orphan pages need links.".to_owned();
        post.headings = vec![Heading::new(HeadingLevel::H2, "Why link text matters")];
        let site = LintSite::default();
        let title = SecondaryNotIn(Place::Title).check(&post, &site);
        assert_eq!(title, [Hit::about("orphan pages")]);
        let first = SecondaryNotIn(Place::FirstParagraph).check(&post, &site);
        assert_eq!(
            first,
            [Hit::about("link text"), Hit::about("site structure")]
        );
        let sub = SecondaryNotIn(Place::Subheading).check(&post, &site);
        assert_eq!(
            sub,
            [Hit::about("site structure"), Hit::about("orphan pages")]
        );
        assert_eq!(
            SecondaryNotIn(Place::Subheading).default_severity(),
            Severity::Info
        );
    }

    #[test]
    fn german_secondary_keywords_match_in_any_word_form() {
        let mut post = post("de", &["interner Link"]);
        post.title = Some("So helfen interne Links".to_owned());
        let site = LintSite::default();
        assert_eq!(SecondaryNotIn(Place::Title).check(&post, &site), []);
    }

    #[test]
    fn density_tips_leave_an_unused_keyword_to_its_own_rule() {
        let site = LintSite::default();
        let mut rare = post("en", &["link text", "orphan pages"]);
        rare.body_text = format!("{} link text.", ["word"; 400].join(" "));
        let hits = SecondaryTooRare { min: 5 }.check(&rare, &site);
        assert_eq!(hits, [Hit::about("link text")]);
        let mut often = post("de", &["Ankertext"]);
        often.body_text = ["Ankertexte helfen."; 60].join(" ");
        let hits = SecondaryTooOften { max: 25 }.check(&often, &site);
        assert_eq!(hits, [Hit::about("Ankertext")]);
    }
}
