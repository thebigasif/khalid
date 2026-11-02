//! Words that can exclude or hurt readers, as Yoast's inclusive language
//! check finds them, with the words to use instead.
//!
//! The list lives in the catalogue, under the `non-inclusive-term` entry,
//! so it is read and changed in one place with the rule's message. The
//! rule is a tip: some words fit in one sentence and not in another, so
//! the writer decides.

use crate::catalogue::Swap;
use crate::focus::{Keyword, words};
use crate::readability::prose;
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// A term from the catalogue's list in the writer's prose, one hit per
/// term, with the words to use instead as the suggestion.
#[derive(Debug)]
pub(crate) struct NonInclusiveTerm {
    /// The catalogue's terms, in its order.
    pub(crate) terms: Vec<Swap>,
}

impl Rule for NonInclusiveTerm {
    fn id(&self) -> RuleId {
        RuleId::new("non-inclusive-term")
    }

    fn default_severity(&self) -> Severity {
        Severity::Info
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let language = post.locale.language();
        let text = words(prose(post));
        self.terms
            .iter()
            .filter(|term| term.lang == language)
            .filter(|term| {
                Keyword::new(&term.term, &[], language)
                    .is_some_and(|keyword| keyword.found_in_words(&text))
            })
            .map(|term| {
                Hit::about(format!("\"{}\"", term.term)).suggesting(vec![term.instead.clone()])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Catalogue, LintSite};

    fn rule() -> NonInclusiveTerm {
        let catalogue = Catalogue::builtin().unwrap();
        let entry = catalogue.get("non-inclusive-term").unwrap();
        NonInclusiveTerm {
            terms: entry.terms().to_vec(),
        }
    }

    fn post(language: &str, prose: &str) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new("a").unwrap());
        post.prose_text = Some(prose.to_owned());
        post
    }

    #[test]
    fn a_listed_term_in_any_word_form_is_a_tip_with_its_alternative() {
        let site = LintSite::default();
        let hits = rule().check(&post("en", "Add spam hosts to your blacklists."), &site);
        let want = Hit::about("\"blacklist\"").suggesting(vec!["blocklist, deny list".to_owned()]);
        assert_eq!(hits, [want]);
        let german = rule().check(&post("de", "Die Putzfrau kommt montags."), &site);
        assert_eq!(german.len(), 1);
        assert_eq!(german.first().and_then(Hit::subject), Some("\"Putzfrau\""));
    }

    #[test]
    fn a_term_in_another_language_or_in_a_quote_does_not_count() {
        let site = LintSite::default();
        assert_eq!(
            rule().check(&post("de", "Der chairman spricht."), &site),
            []
        );
        let mut quoted = post("en", "Our team keeps a short list of blocked hosts.");
        quoted.body_text = "Our team keeps a blacklist, said the old manual.".to_owned();
        assert_eq!(rule().check(&quoted, &site), []);
        assert_eq!(rule().default_severity(), Severity::Info);
    }
}
