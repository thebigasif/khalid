//! Checks on what a page holds besides its words: a picture, links other
//! sites can trust, and the length a pillar post needs.

use crate::{ClusterRole, Hit, LinkKind, LintPost, Rule, RuleId, Severity, SiteView};

/// A post with no image in its body and no lead image.
#[derive(Debug)]
pub(crate) struct NoImage;

impl Rule for NoImage {
    fn id(&self) -> RuleId {
        RuleId::new("no-image")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let none = post.images.is_empty() && !post.lead_image;
        none.then(Hit::post).into_iter().collect()
    }
}

/// A post whose links to other sites all carry `nofollow`, so it vouches
/// for none of its sources.
#[derive(Debug)]
pub(crate) struct AllOutboundNofollow;

impl Rule for AllOutboundNofollow {
    fn id(&self) -> RuleId {
        RuleId::new("all-outbound-nofollow")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let mut outbound = post
            .links
            .iter()
            .filter(|link| link.kind == LinkKind::External)
            .peekable();
        let any = outbound.peek().is_some();
        let all = outbound.all(|link| {
            link.rel
                .iter()
                .any(|word| word.eq_ignore_ascii_case("nofollow"))
        });
        (any && all).then(Hit::post).into_iter().collect()
    }
}

/// A pillar post, the main post of a topic group, shorter than the floor.
#[derive(Debug)]
pub(crate) struct PillarMinWords {
    /// From `Thresholds::pillar_min_words`.
    pub(crate) min: usize,
}

impl Rule for PillarMinWords {
    fn id(&self) -> RuleId {
        RuleId::new("pillar-min-words")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let pillar = post
            .cluster
            .as_ref()
            .is_some_and(|cluster| cluster.role == ClusterRole::Pillar);
        if !pillar || post.word_count >= self.min {
            return Vec::new();
        }
        let words = post.word_count;
        vec![Hit::about(format!("{words} of {} words", self.min))]
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Cluster, Image, LintLink, LintSite};

    fn post() -> LintPost {
        LintPost::new(Locale::try_new("de").unwrap(), Slug::try_new("a").unwrap())
    }

    #[test]
    fn a_lead_image_or_a_body_image_is_enough() {
        let site = LintSite::default();
        let mut post = post();
        assert_eq!(NoImage.check(&post, &site), [Hit::post()]);
        post.lead_image = true;
        assert_eq!(NoImage.check(&post, &site), []);
        post.lead_image = false;
        post.images = vec![Image::new("/media/karte.png", Some("Karte".to_owned()))];
        assert_eq!(NoImage.check(&post, &site), []);
    }

    #[test]
    fn one_followed_outbound_link_is_enough_and_internal_links_do_not_count() {
        let site = LintSite::default();
        let mut post = post();
        assert_eq!(
            AllOutboundNofollow.check(&post, &site),
            [],
            "no outbound link"
        );
        let mut away = LintLink::new(LinkKind::External, "https://example.org/", "Quelle");
        away.rel = vec!["NoFollow".to_owned()];
        let mut home = LintLink::new(LinkKind::Internal, "/blog/b/", "Beitrag");
        home.rel = Vec::new();
        post.links = vec![away.clone(), home];
        assert_eq!(AllOutboundNofollow.check(&post, &site), [Hit::post()]);
        away.rel = vec!["noopener".to_owned()];
        post.links.push(away);
        assert_eq!(AllOutboundNofollow.check(&post, &site), []);
    }

    #[test]
    fn only_a_pillar_post_needs_the_floor() {
        let site = LintSite::default();
        let rule = PillarMinWords { min: 900 };
        let mut post = post();
        post.word_count = 400;
        assert_eq!(rule.check(&post, &site), []);
        post.cluster = Some(Cluster::new("links", ClusterRole::Member));
        assert_eq!(rule.check(&post, &site), []);
        post.cluster = Some(Cluster::new("links", ClusterRole::Pillar));
        assert_eq!(rule.check(&post, &site), [Hit::about("400 of 900 words")]);
        post.word_count = 900;
        assert_eq!(rule.check(&post, &site), []);
    }
}
