//! The focus keyword's own shape: not too long, not only filler words,
//! and early in the title, as Yoast checks it.

use crate::focus::{Keyword, search_title, words};
use crate::words::names_subject;
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// A focus keyword with more words that carry meaning than the limit, so
/// few people search for exactly that phrase.
#[derive(Debug)]
pub(crate) struct FocusTooLong {
    /// From `Thresholds::focus_max_words`.
    pub(crate) max: usize,
}

impl Rule for FocusTooLong {
    fn id(&self) -> RuleId {
        RuleId::new("focus-too-long")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let language = post.locale.language();
        Keyword::focus(post)
            .filter(|keyword| {
                let content = words(keyword.text)
                    .iter()
                    .filter(|word| names_subject(language, word))
                    .count();
                content > self.max
            })
            .map(|keyword| Hit::about(keyword.text))
            .into_iter()
            .collect()
    }
}

/// A focus keyword made only of filler and question words, such as "how
/// to", which names no topic.
#[derive(Debug)]
pub(crate) struct FocusOnlyFiller;

impl Rule for FocusOnlyFiller {
    fn id(&self) -> RuleId {
        RuleId::new("focus-only-filler")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let language = post.locale.language();
        Keyword::focus(post)
            .filter(|keyword| {
                let all = words(keyword.text);
                all.iter().all(|word| !names_subject(language, word))
            })
            .map(|keyword| Hit::about(keyword.text))
            .into_iter()
            .collect()
    }
}

/// A search title that holds the focus keyword, but only after other words
/// that carry meaning. Filler and question words before it are fine, so
/// "Why internal links matter" starts with "internal links".
#[derive(Debug)]
pub(crate) struct FocusNotAtTitleStart;

impl Rule for FocusNotAtTitleStart {
    fn id(&self) -> RuleId {
        RuleId::new("focus-not-at-title-start")
    }

    fn default_severity(&self) -> Severity {
        Severity::Info
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let (Some(keyword), Some(title)) = (
            Keyword::focus(post),
            search_title(post).or(post.title.as_deref()),
        ) else {
            return Vec::new();
        };
        let title = words(title);
        // A title without the keyword is left to `focus-not-in-title`.
        let Some(&first) = keyword.starts(&title).first() else {
            return Vec::new();
        };
        let language = post.locale.language();
        let before = title.get(..first).unwrap_or_default();
        if before.iter().all(|word| !names_subject(language, word)) {
            Vec::new()
        } else {
            vec![Hit::about(keyword.text)]
        }
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::LintSite;

    fn post(language: &str, keyword: &str) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new("a").unwrap());
        post.focus_keyword = Some(keyword.to_owned());
        post
    }

    fn hits(rule: &dyn Rule, post: &LintPost) -> Vec<Hit> {
        rule.check(post, &LintSite::default())
    }

    #[test]
    fn a_long_keyword_counts_only_words_that_carry_meaning() {
        let rule = FocusTooLong { max: 4 };
        let fine = post("en", "how to choose the best link text");
        assert_eq!(hits(&rule, &fine), []);
        let long = post("en", "choose best internal link text fast");
        assert_eq!(
            hits(&rule, &long),
            [Hit::about("choose best internal link text fast")]
        );
        let german = post("de", "die besten internen Links für kleine Blogs");
        let found = hits(&rule, &german);
        assert_eq!(
            found,
            [Hit::about("die besten internen Links für kleine Blogs")]
        );
    }

    #[test]
    fn a_keyword_of_filler_words_names_no_topic_in_either_language() {
        assert_eq!(
            hits(&FocusOnlyFiller, &post("en", "how to")),
            [Hit::about("how to")]
        );
        assert_eq!(
            hits(&FocusOnlyFiller, &post("de", "wie und wo")),
            [Hit::about("wie und wo")]
        );
        assert_eq!(hits(&FocusOnlyFiller, &post("en", "how to link")), []);
    }

    #[test]
    fn filler_and_question_words_before_the_keyword_still_count_as_the_start() {
        let mut en = post("en", "internal links");
        en.title = Some("Why internal links matter".to_owned());
        assert_eq!(hits(&FocusNotAtTitleStart, &en), []);
        en.seo_title = Some("A guide to internal links".to_owned());
        assert_eq!(
            hits(&FocusNotAtTitleStart, &en),
            [Hit::about("internal links")]
        );
        let mut de = post("de", "interne Links");
        de.title = Some("Wie interne Links helfen".to_owned());
        assert_eq!(hits(&FocusNotAtTitleStart, &de), []);
        de.title = Some("Ratgeber: interne Links".to_owned());
        assert_eq!(
            hits(&FocusNotAtTitleStart, &de),
            [Hit::about("interne Links")]
        );
        de.title = Some("Ratgeber".to_owned());
        assert_eq!(hits(&FocusNotAtTitleStart, &de), []);
    }
}
