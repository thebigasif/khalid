//! Rules that compare a post with related content: near copies, glossary
//! terms and topic groups.

use crate::link_graph::{links_to, page_key};
use crate::shingles::{Shingles, Words};
use crate::{ClusterRole, Hit, LinkKind, LintPost, Rule, RuleId, Severity, SiteView};

/// The fewest shingles a text needs before it is compared at all, so two
/// one-line drafts never count as copies of each other.
const MIN_SHINGLES: usize = 10;

/// Another post in the same language that shares more than the site's limit
/// of its shingles with this one, named in the finding.
#[derive(Debug)]
pub(crate) struct NearDuplicate {
    /// The largest share in percent, from `Thresholds::near_duplicate_percent`.
    pub(crate) max_percent: usize,
}

impl Rule for NearDuplicate {
    fn id(&self) -> RuleId {
        RuleId::new("near-duplicate")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        let own = Shingles::of(&post.body_text);
        if own.len() < MIN_SHINGLES {
            return Vec::new();
        }
        let (mut words, mut theirs) = (Words::default(), Shingles::default());
        let others = site.posts().iter();
        let others = others.filter(|o| o.locale == post.locale && o.slug != post.slug);
        let copies = others.filter(|other| {
            words.read(&other.body_text);
            // The union is never smaller than our own shingles, and the
            // shared ones never outnumber `found_in`, so a post this rules
            // out could not pass the exact test below either.
            if own.found_in(&words) * 100 <= self.max_percent * own.len() {
                return false;
            }
            theirs.refill(&words);
            if theirs.len() < MIN_SHINGLES {
                return false;
            }
            let shared = own.shared_with(&theirs);
            let union = own.len() + theirs.len() - shared;
            shared * 100 > self.max_percent * union
        });
        copies
            .map(|other| Hit::about(other.slug.as_str()))
            .collect()
    }
}

/// Whether `term` appears in `text` as whole words. Both are lowercase.
pub(crate) fn mentions(text: &str, term: &str) -> bool {
    text.match_indices(term).any(|(at, _)| {
        let before = text.get(..at).and_then(|head| head.chars().next_back());
        let after = text
            .get(at + term.len()..)
            .and_then(|tail| tail.chars().next());
        !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric)
    })
}

/// A glossary term in the post's language that the body uses but never
/// links to, named in the finding.
#[derive(Debug)]
pub(crate) struct GlossaryTermNotLinked;

impl Rule for GlossaryTermNotLinked {
    fn id(&self) -> RuleId {
        RuleId::new("glossary-term-not-linked")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        let body = post.body_text.to_lowercase();
        let linked = |slug: &str| {
            post.wikilinks.iter().any(|target| target.trim() == slug)
                || post.links.iter().any(|link| {
                    link.kind == LinkKind::Internal
                        && page_key(&link.target).rsplit('/').next() == Some(slug)
                })
        };
        site.glossary()
            .iter()
            .filter(|term| term.locale == post.locale && !linked(term.slug.as_str()))
            .filter(|term| mentions(&body, &term.term.to_lowercase()))
            .map(|term| Hit::about(term.term.clone()))
            .collect()
    }
}

/// A member of a topic group that does not link to the group's main post in
/// its language, named in the finding. Skipped when the group has none.
#[derive(Debug)]
pub(crate) struct ClusterWithoutPillar;

impl Rule for ClusterWithoutPillar {
    fn id(&self) -> RuleId {
        RuleId::new("cluster-without-pillar")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        let Some(cluster) = post
            .cluster
            .as_ref()
            .filter(|c| c.role == ClusterRole::Member)
        else {
            return Vec::new();
        };
        let is_pillar = |other: &&LintPost| {
            other.locale == post.locale
                && other.slug != post.slug
                && other.cluster.as_ref().is_some_and(|theirs| {
                    theirs.role == ClusterRole::Pillar && theirs.name == cluster.name
                })
        };
        let pillar = site.posts().iter().find(is_pillar);
        pillar
            .filter(|pillar| !links_to(post, pillar))
            .map(|pillar| Hit::about(pillar.slug.as_str()))
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::LintSite;

    /// The plain definition the rule must agree with: the distinct runs of
    /// five lowercase words, compared as text, with no hashing at all.
    fn copies_by_definition(own: &str, theirs: &str, max_percent: usize) -> bool {
        let runs = |text: &str| -> BTreeSet<Vec<String>> {
            let lower = text.to_lowercase();
            let words: Vec<String> = lower
                .split(|c: char| !c.is_alphanumeric())
                .filter(|word| !word.is_empty())
                .map(str::to_owned)
                .collect();
            words.windows(5).map(<[String]>::to_vec).collect()
        };
        let (own, theirs) = (runs(own), runs(theirs));
        let shared = own.intersection(&theirs).count();
        let union = own.len() + theirs.len() - shared;
        own.len() >= MIN_SHINGLES
            && theirs.len() >= MIN_SHINGLES
            && shared * 100 > max_percent * union
    }

    fn post(slug: &str, body: &str) -> LintPost {
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new(slug).unwrap());
        post.body_text = body.to_owned();
        post
    }

    /// A text of `len` words, `Word0, word1 ...`, with every `every`th word
    /// swapped for another and the first `skip` left out.
    fn variant(len: usize, every: usize, skip: usize) -> String {
        (skip..len)
            .map(|at| match at % 7 {
                _ if every > 0 && at % every == 0 => format!("other{at}"),
                0 => format!("Word{at},"),
                _ => format!("word{at}"),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn the_rule_agrees_with_the_plain_definition_around_the_limit() {
        let base = variant(80, 0, 0);
        let mut found = 0;
        for every in 0..=24 {
            for skip in [0, 5, 20, 39, 41, 60, 72] {
                let other = variant(80, every, skip);
                let site = LintSite {
                    posts: vec![post("base", &base), post("other", &other)],
                    ..LintSite::default()
                };
                let rule = NearDuplicate { max_percent: 50 };
                let hits = rule.check(&site.posts[0], &site);
                let expected = copies_by_definition(&base, &other, 50);
                assert_eq!(!hits.is_empty(), expected, "every {every}, skip {skip}");
                found += usize::from(expected);
            }
        }
        // Both answers occur, so the limit itself was crossed.
        assert!(found > 10 && found < 150, "{found} copies");
    }
}
