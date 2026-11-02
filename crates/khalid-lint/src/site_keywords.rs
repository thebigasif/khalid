//! Site rules about keywords and pillar posts: two posts that compete for
//! the same search, and a pillar post nobody has touched in months.

use std::collections::BTreeMap;

use chrono::{Months, NaiveDate};

use crate::focus::Keyword;
use crate::stem::WordForms;
use crate::{ClusterRole, Hit, LintPost, RuleId, Severity, SiteRule, SiteView};

/// A post whose focus keyword another post in the same language already
/// uses, in any word form. The oldest post keeps the keyword; each later
/// one is named, with the older post's address as the suggestion.
#[derive(Debug)]
pub(crate) struct DuplicateFocusKeyword;

impl SiteRule for DuplicateFocusKeyword {
    fn id(&self) -> RuleId {
        RuleId::new("duplicate-focus-keyword")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check<'site>(&self, site: &'site dyn SiteView) -> Vec<(&'site LintPost, Hit)> {
        let mut groups: BTreeMap<(&str, String), Vec<&LintPost>> = BTreeMap::new();
        for post in site.posts() {
            if let Some(keyword) = Keyword::focus(post) {
                let stems = WordForms::of(post.locale.language()).phrase(keyword.text);
                let key = (post.locale.as_str(), stems.join(" "));
                groups.entry(key).or_default().push(post);
            }
        }
        let mut hits = Vec::new();
        for mut posts in groups.into_values().filter(|posts| posts.len() > 1) {
            // Oldest first; a post without a date counts as the newest.
            posts.sort_by_key(|post| (post.date.is_none(), post.date));
            let Some((first, later)) = posts.split_first() else {
                continue;
            };
            let older = first.path.clone().unwrap_or_else(|| first.slug.to_string());
            for post in later {
                let keyword = Keyword::focus(post).map_or("", |keyword| keyword.text);
                let hit = Hit::about(keyword).suggesting(vec![older.clone()]);
                hits.push((*post, hit));
            }
        }
        hits
    }
}

/// A pillar post, the main post of a topic group, not updated for more
/// months than the site's limit.
#[derive(Debug)]
pub(crate) struct StalePillar {
    /// The most months allowed, from `Thresholds::pillar_stale_months`.
    pub(crate) months: u32,
    /// The date the linter started, read from the clock.
    pub(crate) today: NaiveDate,
}

impl SiteRule for StalePillar {
    fn id(&self) -> RuleId {
        RuleId::new("stale-pillar")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check<'site>(&self, site: &'site dyn SiteView) -> Vec<(&'site LintPost, Hit)> {
        let stale = |post: &&LintPost| {
            let pillar = post
                .cluster
                .as_ref()
                .is_some_and(|cluster| cluster.role == ClusterRole::Pillar);
            let last = post.updated.or(post.date);
            let fresh_until = last.and_then(|day| day.checked_add_months(Months::new(self.months)));
            pillar && fresh_until.is_some_and(|until| until < self.today)
        };
        site.posts()
            .iter()
            .filter(stale)
            .map(|post| (post, Hit::post()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Cluster, LintSite};

    fn post(language: &str, slug: &str, keyword: &str, date: Option<NaiveDate>) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new(slug).unwrap());
        post.focus_keyword = Some(keyword.to_owned());
        post.date = date;
        post
    }

    fn named(hits: &[(&LintPost, Hit)]) -> Vec<(String, Option<String>, Vec<String>)> {
        hits.iter()
            .map(|(post, hit)| {
                let subject = hit.subject().map(str::to_owned);
                (post.slug.to_string(), subject, hit.suggestions().to_vec())
            })
            .collect()
    }

    #[test]
    fn the_newer_post_with_the_same_keyword_in_any_form_is_named() {
        let day = |d| NaiveDate::from_ymd_opt(2026, 10, d);
        let mut site = LintSite::default();
        site.posts.extend([
            post("en", "newer", "Internal link", day(20)),
            post("en", "older", "internal links", day(1)),
            post("en", "undated", "internal links", None),
            post("de", "deutsch", "internal links", day(2)),
            post("de", "interne-links", "interne Links", day(3)),
            post("de", "interner-link", "interner Link", day(4)),
        ]);
        let found = named(&DuplicateFocusKeyword.check(&site));
        let want = |slug: &str, keyword: &str, older: &str| {
            (
                slug.to_owned(),
                Some(keyword.to_owned()),
                vec![older.to_owned()],
            )
        };
        assert_eq!(
            found,
            [
                want("interner-link", "interner Link", "interne-links"),
                want("newer", "Internal link", "older"),
                want("undated", "internal links", "older"),
            ]
        );
    }

    #[test]
    fn only_a_pillar_post_goes_stale_after_its_own_months() {
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).unwrap();
        let rule = StalePillar { months: 6, today };
        let mut pillar = post("en", "pillar", "links", NaiveDate::from_ymd_opt(2026, 4, 1));
        pillar.cluster = Some(Cluster::new("links", ClusterRole::Pillar));
        let mut member = pillar.clone();
        member.slug = Slug::try_new("member").unwrap();
        member.cluster = Some(Cluster::new("links", ClusterRole::Member));
        let mut fresh = pillar.clone();
        fresh.slug = Slug::try_new("fresh").unwrap();
        fresh.updated = NaiveDate::from_ymd_opt(2026, 9, 1);
        let mut site = LintSite::default();
        site.posts.extend([pillar, member, fresh]);
        let found = named(&rule.check(&site));
        assert_eq!(found, [("pillar".to_owned(), None, Vec::new())]);
    }
}
