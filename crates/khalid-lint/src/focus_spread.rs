//! How the focus keyword spreads through a post: through the whole text
//! and through a fair share of the subheadings, as Yoast checks them.

use crate::focus::{Keyword, words};
use crate::{HeadingLevel, Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// The fewest words before the spread of the keyword is judged: in a
/// shorter text each tenth is a sentence or two.
const DISTRIBUTION_MIN_WORDS: usize = 300;

/// The text is cut into this many parts of equal length.
const PARTS: usize = 10;

/// A text of 300 words or more where more than half of its ten equal parts
/// never use the focus keyword, so it is about the topic only in places.
#[derive(Debug)]
pub(crate) struct FocusDistribution;

impl Rule for FocusDistribution {
    fn id(&self) -> RuleId {
        RuleId::new("focus-distribution")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let Some(keyword) = Keyword::focus(post) else {
            return Vec::new();
        };
        let text = words(&post.body_text);
        if text.len() < DISTRIBUTION_MIN_WORDS {
            return Vec::new();
        }
        let mut held = [false; PARTS];
        for start in keyword.starts(&text) {
            if let Some(part) = held.get_mut(start * PARTS / text.len()) {
                *part = true;
            }
        }
        let used = held.iter().filter(|part| **part).count();
        // A text that never uses the keyword is left to `focus-too-rare`.
        if used == 0 || used * 2 >= PARTS {
            return Vec::new();
        }
        let subject = format!("{}: in {used} of {PARTS} parts", keyword.text);
        vec![Hit::about(subject)]
    }
}

/// The fewest second and third level subheadings before their share is
/// judged: with fewer, one subheading more or less swings it too far.
const SHARE_MIN_SUBHEADINGS: usize = 3;

/// Too few or too many of the second and third level subheadings hold the
/// focus keyword. None at all is left to `focus-not-in-subheading`.
#[derive(Debug)]
pub(crate) struct FocusSubheadingShare {
    /// From `Thresholds::focus_subheading_min_percent`.
    pub(crate) min_percent: usize,
    /// From `Thresholds::focus_subheading_max_percent`.
    pub(crate) max_percent: usize,
}

impl Rule for FocusSubheadingShare {
    fn id(&self) -> RuleId {
        RuleId::new("focus-subheading-share")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let Some(keyword) = Keyword::focus(post) else {
            return Vec::new();
        };
        let subheadings: Vec<&str> = post
            .headings
            .iter()
            .filter(|heading| matches!(heading.level, HeadingLevel::H2 | HeadingLevel::H3))
            .map(|heading| heading.text.as_str())
            .collect();
        if subheadings.len() < SHARE_MIN_SUBHEADINGS {
            return Vec::new();
        }
        let held = subheadings
            .iter()
            .filter(|text| keyword.found_in(text))
            .count();
        let share = held * 100 / subheadings.len();
        if held == 0 || (self.min_percent..=self.max_percent).contains(&share) {
            return Vec::new();
        }
        let all = subheadings.len();
        vec![Hit::about(format!(
            "{}: in {held} of {all} subheadings",
            keyword.text
        ))]
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Heading, LintSite};

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
    fn a_keyword_only_in_the_opening_parts_is_badly_spread() {
        let filler = ["word"; 300].join(" ");
        let mut early = post("en", "internal link");
        early.body_text = format!("Internal links help. Internal linking works. {filler}");
        let found = hits(&FocusDistribution, &early);
        assert_eq!(found, [Hit::about("internal link: in 1 of 10 parts")]);
        let spread = ["Interne Links helfen. ", &["Wort"; 30].join(" "), " "].concat();
        let mut german = post("de", "interner Link");
        german.body_text = spread.repeat(10);
        assert_eq!(hits(&FocusDistribution, &german), []);
    }

    #[test]
    fn the_subheading_share_is_judged_from_three_subheadings() {
        let rule = FocusSubheadingShare {
            min_percent: 30,
            max_percent: 75,
        };
        let mut en = post("en", "internal links");
        let heading = |text: &str| Heading::new(HeadingLevel::H2, text);
        en.headings = vec![heading("Internal links"), heading("Internal linking")];
        assert_eq!(hits(&rule, &en), [], "two subheadings are too few to judge");
        en.headings.push(heading("Why internal links help"));
        assert_eq!(
            hits(&rule, &en),
            [Hit::about("internal links: in 3 of 3 subheadings")]
        );
        en.headings.push(heading("Link text"));
        en.headings.push(heading("Navigation"));
        assert_eq!(hits(&rule, &en), [], "three of five is 60 percent");
        let mut de = post("de", "interne Links");
        de.headings = vec![
            heading("Interne Links"),
            heading("Ankertext"),
            heading("Menüs"),
            heading("Fazit"),
        ];
        assert_eq!(
            hits(&rule, &de),
            [Hit::about("interne Links: in 1 of 4 subheadings")]
        );
    }
}
