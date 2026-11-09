//! A post's two scores, as Yoast shows them: one for SEO and one for
//! readability, each out of 100, from the same lint findings as the post's
//! one score.

use khalid_lint::{Finding, Report, RuleId, Severity};
use serde::Serialize;
use utoipa::ToSchema;

/// The rules that judge how easy a post is to read, as Yoast's readability
/// analysis does. Every other rule counts towards the SEO score. A new
/// readability rule joins this list.
pub const READABILITY_RULES: &[&str] = &[
    "hard-to-read",
    "long-paragraph",
    "heading-shape",
    "answer-capsule",
    "plain-answer",
    "comparison-answer-late",
];

/// What a rule set to block costs, once, as the post's one score counts it.
const BLOCK_POINTS: u8 = 30;

/// What a rule set to warn costs, once.
const WARN_POINTS: u8 = 5;

/// The highest score a group with a blocking finding can reach.
const BLOCKED_MAX: u8 = 40;

/// A post's SEO and readability scores, each from 0 to 100.
///
/// Each rule that found a problem costs points once, 30 for a block and 5
/// for a warning, as the post's one score counts them, and a group with a
/// block scores at most 40. The rules in [`READABILITY_RULES`] make the
/// readability score; every other rule makes the SEO score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
pub struct SeoScores {
    /// How well the post is set up to be found, from 0 to 100.
    seo: u8,
    /// How easy the post is to read, from 0 to 100.
    readability: u8,
}

impl SeoScores {
    /// The scores from every finding in `report`.
    pub fn of(report: &Report) -> Self {
        let readable = |finding: &&Finding| READABILITY_RULES.contains(&finding.rule().as_str());
        let (reading, seo): (Vec<&Finding>, Vec<&Finding>) =
            report.findings().iter().partition(readable);
        Self {
            seo: score(&seo),
            readability: score(&reading),
        }
    }

    /// The SEO score.
    pub fn seo(self) -> u8 {
        self.seo
    }

    /// The readability score.
    pub fn readability(self) -> u8 {
        self.readability
    }
}

/// 100 less what each rule in `findings` costs, once.
fn score(findings: &[&Finding]) -> u8 {
    let mut costs: Vec<(RuleId, u8)> = Vec::new();
    for finding in findings {
        let points = match finding.severity() {
            Severity::Block => BLOCK_POINTS,
            Severity::Warn => WARN_POINTS,
            // A tip costs nothing, as in the linter's own score.
            Severity::Info | Severity::Off => 0,
        };
        match costs.iter_mut().find(|(rule, _)| *rule == finding.rule()) {
            Some((_, cost)) => *cost = (*cost).max(points),
            None => costs.push((finding.rule(), points)),
        }
    }
    let lost = costs
        .iter()
        .fold(0_u8, |sum, (_, cost)| sum.saturating_add(*cost));
    let value = 100_u8.saturating_sub(lost);
    if costs.iter().any(|(_, cost)| *cost == BLOCK_POINTS) {
        value.min(BLOCKED_MAX)
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use khalid_core::{FixedClock, Locale, Slug};
    use khalid_lint::{LintConfig, LintPost, LintSite, Linter};

    use super::*;

    #[cfg(test)]
    fn report(post: &LintPost) -> Report {
        let today = Utc.with_ymd_and_hms(2026, 11, 2, 9, 0, 0).unwrap();
        let linter = Linter::new(&LintConfig::default(), &FixedClock::new(today)).unwrap();
        linter.lint_post(post, &LintSite::default())
    }

    #[cfg(test)]
    fn empty() -> LintPost {
        let slug = Slug::try_new("choosing-anchor-text").unwrap();
        LintPost::new(Locale::try_new("en").unwrap(), slug)
    }

    #[test]
    fn an_empty_post_loses_seo_points_only_and_a_block_caps_them() {
        let scores = SeoScores::of(&report(&empty()));
        assert!(scores.seo() <= BLOCKED_MAX, "{scores:?}");
        assert_eq!(scores.readability(), 100, "{scores:?}");
    }

    #[test]
    fn a_long_paragraph_costs_readability_points_once() {
        let mut post = empty();
        let paragraph = "Short words fit here now. ".repeat(60);
        post.body_text = format!("{paragraph}\n\n{paragraph}\n");
        let scores = SeoScores::of(&report(&post));
        assert_eq!(scores.readability(), 100 - WARN_POINTS, "{scores:?}");
    }
}
