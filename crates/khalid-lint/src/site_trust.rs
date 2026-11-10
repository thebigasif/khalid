//! The rules that judge what a site says about who writes it.

use khalid_core::Slug;

use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteRule, SiteView};

/// **A post whose author has no profile.**
///
/// The byline links to the author's page, which only a profile makes, so
/// without one the link leads nowhere and readers learn nothing about the
/// writer. The finding names the author. Skipped when the site's profiles
/// are unknown.
#[derive(Debug)]
pub(crate) struct AuthorWithoutProfile;

impl Rule for AuthorWithoutProfile {
    fn id(&self) -> RuleId {
        RuleId::new("author-without-profile")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        let (Some(author), Some(profiles)) = (post.author.as_deref(), site.authors()) else {
            return Vec::new();
        };
        let author = author.trim();
        let known = profiles
            .iter()
            .any(|profile| profile.locale == post.locale && profile.slug.as_str() == author);
        if author.is_empty() || known {
            return Vec::new();
        }
        vec![Hit::about(author)]
    }
}

/// **A post in a category that needs a reviewer, with none named.**
///
/// Health and money advice is trusted more when an expert checked it, so a
/// site lists such categories under `[reviewers]` in `lint.toml` and every
/// post in one must name a reviewer in `reviewed_by` (plan 4.12). Blocks by
/// default, since the site asked for it. A site that lists no category is
/// never hit.
#[derive(Debug)]
pub(crate) struct NoReviewer {
    /// The categories whose posts need a reviewer.
    pub(crate) categories: Vec<Slug>,
}

impl Rule for NoReviewer {
    fn id(&self) -> RuleId {
        RuleId::new("no-reviewer")
    }

    fn default_severity(&self) -> Severity {
        Severity::Block
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let Some(category) = post.category.as_deref() else {
            return Vec::new();
        };
        let needs = self.categories.iter().any(|slug| slug.as_str() == category);
        let named = post.reviewed_by.iter().any(|slug| !slug.trim().is_empty());
        if !needs || named {
            return Vec::new();
        }
        vec![Hit::about(category)]
    }
}

/// The slugs a site's About or contact page may carry. A reader who wants
/// to know who writes here, and an engine judging the site's trust, looks
/// for one of these (P5).
const ABOUT_SLUGS: &[&str] = &["about", "contact", "about-us", "contact-us"];

/// **P5: a site with no About or contact page.**
///
/// A reader who cannot find who stands behind a site trusts it less, and
/// an answer engine citing it has no author to credit. The rule fires
/// once for the site when none of its pages is an About or contact page.
#[derive(Debug)]
pub(crate) struct NoAboutPage;

impl SiteRule for NoAboutPage {
    fn id(&self) -> RuleId {
        RuleId::new("no-about-page")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check<'site>(&self, site: &'site dyn SiteView) -> Vec<(&'site LintPost, Hit)> {
        let has_about = site
            .pages()
            .iter()
            .any(|page| ABOUT_SLUGS.iter().any(|slug| page.slug.as_str() == *slug));
        if has_about {
            return Vec::new();
        }
        // A site rule reports about a post; this one is about the site, so
        // it reports against the first post, which `site_health` shows.
        site.posts()
            .first()
            .map(|post| (post, Hit::post()))
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::Locale;

    use super::*;
    use crate::LintSite;

    /// How many times "no reviewer" fires on a post in `category` that
    /// names `reviewed_by`, on a site that wants one in health posts.
    fn hits(category: Option<&str>, reviewed_by: &[&str]) -> usize {
        let rule = NoReviewer {
            categories: vec![Slug::try_new("health").unwrap()],
        };
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.category = category.map(str::to_owned);
        post.reviewed_by = reviewed_by.iter().map(|&slug| slug.to_owned()).collect();
        rule.check(&post, &LintSite::default()).len()
    }

    #[test]
    fn a_health_post_needs_a_reviewer_that_is_not_blank() {
        assert_eq!(hits(Some("health"), &[]), 1);
        assert_eq!(hits(Some("health"), &["  "]), 1);
        assert_eq!(hits(Some("health"), &["dr-lena-ortiz"]), 0);
    }

    #[test]
    fn a_post_outside_the_listed_categories_needs_none() {
        assert_eq!(hits(Some("recipes"), &[]), 0);
        assert_eq!(hits(None, &[]), 0);
    }
}
