//! The blog's most common lint problems, for the dashboard's task list,
//! as Yoast's SEO tasks give them: each rule that fires on the live site,
//! on how many posts, with its plain message and a post to open first.

use std::collections::{BTreeMap, BTreeSet};

use khalid_lint::{Finding, Report, Severity};
use serde::Serialize;
use utoipa::ToSchema;

/// The most problems the task list shows.
pub const MOST_ISSUES: usize = 10;

/// How much a problem matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssueWeight {
    /// It stops a post from going live.
    Block,
    /// A warning, which does not.
    Warn,
    /// A tip, which costs no points and comes after every warning.
    Tip,
}

impl IssueWeight {
    /// The weight of a finding with `severity`, or `None` for a rule that is off.
    const fn of(severity: Severity) -> Option<Self> {
        match severity {
            Severity::Block => Some(Self::Block),
            Severity::Warn => Some(Self::Warn),
            Severity::Info => Some(Self::Tip),
            Severity::Off => None,
        }
    }
}

/// One lint rule that fires on the site, as one task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct SiteIssue {
    /// The rule, such as `missing-alt`.
    rule: String,
    /// Whether it stops posts going live or warns.
    weight: IssueWeight,
    /// How many posts it fires on.
    posts: u32,
    /// What is wrong, in one sentence.
    what: String,
    /// Why it matters.
    why: String,
    /// What to do.
    fix: String,
    /// The identity of one post it fires on, to open first.
    #[schema(example = "en.choosing-anchor-text")]
    example: String,
}

/// Up to [`MOST_ISSUES`] rules of `report`, those that stop posts first,
/// then warnings, then tips, each group with those on the most posts first.
pub fn top_issues(report: &Report) -> Vec<SiteIssue> {
    let mut by_rule: BTreeMap<&str, (IssueWeight, &Finding, BTreeSet<String>)> = BTreeMap::new();
    for finding in report.findings() {
        let Some(weight) = IssueWeight::of(finding.severity()) else {
            continue;
        };
        let at = finding.location();
        let entry = by_rule.entry(finding.rule().as_str());
        let (kept, first, posts) = entry.or_insert_with(|| (weight, finding, BTreeSet::new()));
        // A rule counts at its heaviest weight on any post, shown on that post.
        if weight < *kept {
            *kept = weight;
            *first = finding;
        }
        posts.insert(format!("{}.{}", at.locale, at.slug));
    }
    let mut issues: Vec<SiteIssue> = (by_rule.into_iter())
        .map(|(rule, (weight, first, posts))| SiteIssue {
            rule: rule.to_owned(),
            weight,
            posts: u32::try_from(posts.len()).unwrap_or(u32::MAX),
            what: first.message().what().to_owned(),
            why: first.message().why().to_owned(),
            fix: first.message().fix().to_owned(),
            // The identity the Admin API gives a post: its language, a full stop, its slug.
            example: format!("{}.{}", first.location().locale, first.location().slug),
        })
        .collect();
    issues.sort_by(|a, b| (a.weight, b.posts, &a.rule).cmp(&(b.weight, a.posts, &b.rule)));
    issues.truncate(MOST_ISSUES);
    issues
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use khalid_core::{FixedClock, Locale, Slug};
    use khalid_lint::{LintConfig, LintPost, LintSite, Linter};

    use super::*;

    #[test]
    fn tips_weigh_least_and_rules_that_are_off_are_left_out() {
        assert_eq!(IssueWeight::of(Severity::Info), Some(IssueWeight::Tip));
        assert_eq!(IssueWeight::of(Severity::Off), None);
        assert!(IssueWeight::Block < IssueWeight::Warn && IssueWeight::Warn < IssueWeight::Tip);
    }

    #[test]
    fn rules_that_stop_posts_come_first_counted_once_per_post() {
        let today = Utc.with_ymd_and_hms(2026, 11, 2, 9, 0, 0).unwrap();
        let linter = Linter::new(&LintConfig::default(), &FixedClock::new(today)).unwrap();
        let mut site = LintSite::default();
        for slug in ["anchor-text", "page-speed"] {
            let post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new(slug).unwrap());
            site.posts.push(post);
        }
        let issues = top_issues(&linter.lint_site(&site));
        assert!(
            !issues.is_empty() && issues.len() <= MOST_ISSUES,
            "{issues:?}"
        );
        let first = issues.first().unwrap();
        assert_eq!(first.weight, IssueWeight::Block, "{issues:?}");
        assert_eq!(first.posts, 2, "{issues:?}");
        assert_eq!(first.example, "en.anchor-text");
        assert!(!first.what.is_empty() && !first.fix.is_empty());
        let weights: Vec<IssueWeight> = issues.iter().map(|issue| issue.weight).collect();
        assert!(weights.is_sorted(), "{weights:?}");
    }
}
