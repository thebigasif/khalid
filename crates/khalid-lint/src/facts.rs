//! Rules that hold posts to the site's fact registry, `facts.toml`: one
//! value per fact everywhere on the site, and every fact checked often.

use chrono::{Months, NaiveDate};

use crate::address::FACT;
use crate::topics::mentions;
use crate::{Hit, LintFact, LintPost, Rule, RuleId, Severity, SiteRule, SiteView};

/// The words a reader sees in `post`, in lowercase: its titles, its
/// description and its body.
fn visible_text(post: &LintPost) -> String {
    let parts = [&post.title, &post.seo_title, &post.description];
    let mut text: Vec<&str> = parts.into_iter().filter_map(Option::as_deref).collect();
    text.push(&post.body_text);
    text.join("\n").to_lowercase()
}

/// Whether `text`, already in lowercase, gives `value` as whole words.
fn gives(text: &str, value: &str) -> bool {
    let value = value.trim().to_lowercase();
    !value.is_empty() && mentions(text, &value)
}

/// A `[[fact:key]]` in a post whose key the registry lacks, named in the
/// finding as written. The build stops at it too, so the page never shows
/// a gap.
#[derive(Debug)]
pub(crate) struct FactUnknown;

impl Rule for FactUnknown {
    fn id(&self) -> RuleId {
        RuleId::new("fact-unknown")
    }

    fn default_severity(&self) -> Severity {
        Severity::Block
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        post.wikilinks
            .iter()
            .map(|target| target.trim())
            .filter(|target| {
                target
                    .strip_prefix(FACT)
                    .is_some_and(|key| !site.facts().iter().any(|fact| fact.key == key.trim()))
            })
            .map(Hit::about)
            .collect()
    }
}

/// A post that gives a value the fact registry lists as one a fact held
/// before, named in the finding with the current value as the fix.
///
/// An old value that is part of the current one, such as "14 days" when
/// the trial is now "14 days, no card needed", is never flagged, because a
/// post that gives the current value would match it.
#[derive(Debug)]
pub(crate) struct FactOldValue;

impl Rule for FactOldValue {
    fn id(&self) -> RuleId {
        RuleId::new("fact-old-value")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        // A site without facts pays nothing for this rule on any post.
        if site.facts().iter().all(|fact| fact.old.is_empty()) {
            return Vec::new();
        }
        let text = visible_text(post);
        let mut hits = Vec::new();
        for fact in site.facts() {
            let current = fact.value.to_lowercase();
            let old = fact.old.iter().filter(|old| {
                let old_lower = old.trim().to_lowercase();
                !current.contains(&old_lower) && gives(&text, old)
            });
            hits.extend(old.map(|old| Hit::about(old.trim()).suggesting(vec![fact.value.clone()])));
        }
        hits
    }
}

/// The `(post, hit)` for each post of `site` that gives the value of one of
/// `facts`, each hit naming the fact's key and suggesting its source.
fn uses<'site>(site: &'site dyn SiteView, facts: &[&LintFact]) -> Vec<(&'site LintPost, Hit)> {
    if facts.is_empty() {
        return Vec::new();
    }
    let mut hits = Vec::new();
    for post in site.posts() {
        let text = visible_text(post);
        let used = facts.iter().filter(|fact| gives(&text, &fact.value));
        hits.extend(used.map(|fact| {
            let source = fact.source.iter().cloned().collect();
            (post, Hit::about(fact.key.as_str()).suggesting(source))
        }));
    }
    hits
}

/// A post that gives the value of a fact nobody has checked for longer
/// than the site allows, named in the finding by the fact's key, with the
/// fact's source as the place to check it.
#[derive(Debug)]
pub(crate) struct FactNotChecked {
    /// The most months allowed, from `Thresholds::fact_check_months`.
    pub(crate) months: u32,
    /// The date the linter started, read from the clock.
    pub(crate) today: NaiveDate,
}

impl FactNotChecked {
    /// Whether `fact` was last checked longer ago than allowed.
    fn overdue(&self, fact: &LintFact) -> bool {
        let due = fact.checked.checked_add_months(Months::new(self.months));
        due.is_some_and(|due| due < self.today)
    }
}

impl SiteRule for FactNotChecked {
    fn id(&self) -> RuleId {
        RuleId::new("fact-not-checked")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check<'site>(&self, site: &'site dyn SiteView) -> Vec<(&'site LintPost, Hit)> {
        let overdue: Vec<&LintFact> = site.facts().iter().filter(|f| self.overdue(f)).collect();
        uses(site, &overdue)
    }
}

/// A post that gives the value of a fact whose `checked` date is still to
/// come, which would keep `fact-not-checked` quiet for good. A date one day
/// ahead passes, since the writer's today may be ahead of the server's.
#[derive(Debug)]
pub(crate) struct FactCheckedInFuture {
    /// The date the linter started, read from the clock.
    pub(crate) today: NaiveDate,
}

impl FactCheckedInFuture {
    /// Whether `fact` claims a check after tomorrow.
    fn ahead(&self, fact: &LintFact) -> bool {
        self.today
            .succ_opt()
            .is_some_and(|tomorrow| fact.checked > tomorrow)
    }
}

impl SiteRule for FactCheckedInFuture {
    fn id(&self) -> RuleId {
        RuleId::new("fact-checked-in-future")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check<'site>(&self, site: &'site dyn SiteView) -> Vec<(&'site LintPost, Hit)> {
        let ahead: Vec<&LintFact> = site.facts().iter().filter(|f| self.ahead(f)).collect();
        uses(site, &ahead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_value_counts_only_as_whole_words() {
        assert!(gives("we started in 2019.", "2019"));
        assert!(!gives("we have 20190 readers", "2019"));
        assert!(!gives("anything", "  "), "an empty value never matches");
    }

    #[test]
    fn a_fact_checked_exactly_at_the_limit_is_not_overdue() {
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).unwrap();
        let rule = FactNotChecked { months: 6, today };
        let on_time = LintFact::new(
            "price",
            "29 USD",
            NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
        );
        let late = LintFact::new(
            "price",
            "29 USD",
            NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
        );
        assert!(!rule.overdue(&on_time));
        assert!(rule.overdue(&late));
    }

    #[test]
    fn a_fact_checked_tomorrow_passes_and_the_day_after_is_ahead() {
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).unwrap();
        let rule = FactCheckedInFuture { today };
        let on = |day| {
            LintFact::new(
                "price",
                "29 USD",
                NaiveDate::from_ymd_opt(2026, 10, day).unwrap(),
            )
        };
        assert!(!rule.ahead(&on(26)));
        assert!(!rule.ahead(&on(27)), "the writer may be a day ahead");
        assert!(rule.ahead(&on(28)));
    }
}
