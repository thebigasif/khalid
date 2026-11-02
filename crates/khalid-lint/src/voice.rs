//! How the writer's sentences flow: linking words between ideas, and the
//! passive voice, as Yoast checks them in English and German.
//!
//! The passive is found by its form. English: a form of "be" followed,
//! within three words, by a past participle ("was written", "is often
//! used"). German: a form of "werden" and a past participle anywhere in
//! the sentence, because German puts it at the end ("Der Text wird von
//! allen gelesen"). A few adjectives look like participles; the lists
//! below keep the common ones out.

use crate::focus::holds;
use crate::readability::{MIN_PROSE_WORDS, all_sentences};
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView, german};

/// Linking words and phrases, in lowercase, by language, each list one
/// string with the entries between bars, so it stays short to read.
const TRANSITIONS: &[(&str, &str)] = &[
    (
        "en",
        "accordingly|additionally|afterwards|also|although|because|besides|but|\
         consequently|conversely|eventually|finally|first|firstly|furthermore|hence|however|\
         instead|later|likewise|meanwhile|moreover|namely|nevertheless|next|nonetheless|\
         otherwise|overall|second|secondly|similarly|since|so|specifically|still|then|\
         therefore|though|thus|ultimately|unless|until|whereas|while|yet|above all|after all|\
         as a result|as long as|because of|due to|even though|for example|for instance|\
         in addition|in conclusion|in contrast|in fact|in other words|in particular|in short|\
         in summary|on the other hand|so that|such as|that is why|to begin with|to sum up",
    ),
    ("de", german::TRANSITIONS),
];

/// Forms of "be" that build the English passive.
const BE: &[&str] = &["am", "is", "are", "was", "were", "be", "been", "being"];

/// English past participles that do not end in "-ed", between spaces.
const IRREGULAR: &str = " begun bitten born bought brought built caught chosen cut done drawn \
     driven eaten fallen fed felt forbidden forgiven forgotten found frozen given grown heard \
     held hidden hit hung hurt kept known laid led left lent lost made meant paid put read run \
     said seen sent set shot shown shut sold sought spent spoken spread stolen struck stuck \
     taken taught thought thrown told torn understood won worn written ";

/// English words that end in "-ed" but are not participles.
const NOT_PARTICIPLES: &[&str] = &[
    "bed", "embed", "hundred", "indeed", "naked", "red", "sacred", "shed", "wed", "wicked",
];

/// Whether `word`, in lowercase, is an English past participle.
fn english_participle(word: &str) -> bool {
    IRREGULAR.contains(&format!(" {word} "))
        || (word.len() > 3
            && word.ends_with("ed")
            && !word.ends_with("eed")
            && !NOT_PARTICIPLES.contains(&word))
}

/// Whether the sentence `words`, in lowercase, is in the passive voice in
/// `language`. A language without rules has none.
fn passive(words: &[String], language: &str) -> bool {
    match language {
        "en" => words.iter().enumerate().any(|(at, word)| {
            BE.contains(&word.as_str())
                && words
                    .iter()
                    .skip(at + 1)
                    .take(3)
                    .any(|next| english_participle(next))
        }),
        "de" => {
            words
                .iter()
                .any(|word| german::WERDEN.contains(&word.as_str()))
                && words.iter().any(|word| german::participle(word))
        }
        _ => false,
    }
}

/// The sentences of `post`'s prose in lowercase, or `None` when the prose
/// is too short for a share to mean anything.
fn lower_sentences(post: &LintPost) -> Option<Vec<Vec<String>>> {
    let all = all_sentences(post);
    if all.iter().map(Vec::len).sum::<usize>() < MIN_PROSE_WORDS {
        return None;
    }
    let lower = |sentence: &Vec<&str>| sentence.iter().map(|w| w.to_lowercase()).collect();
    Some(all.iter().map(lower).collect())
}

/// Too few sentences hold a linking word, in a language with a list.
#[derive(Debug)]
pub(crate) struct FewTransitions {
    /// From `Thresholds::transition_min_percent`.
    pub(crate) min_percent: usize,
}

impl Rule for FewTransitions {
    fn id(&self) -> RuleId {
        RuleId::new("few-transitions")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let language = post.locale.language();
        let Some((_, list)) = TRANSITIONS.iter().find(|(lang, _)| *lang == language) else {
            return Vec::new();
        };
        let Some(all) = lower_sentences(post) else {
            return Vec::new();
        };
        let phrases: Vec<Vec<String>> = list.split('|').map(crate::focus::words).collect();
        let linked = all
            .iter()
            .filter(|sentence| phrases.iter().any(|phrase| holds(sentence, phrase)))
            .count();
        if linked * 100 >= self.min_percent * all.len() {
            return Vec::new();
        }
        let count = all.len();
        vec![Hit::about(format!(
            "{linked} of {count} sentences use a linking word"
        ))]
    }
}

/// Too many sentences are in the passive voice.
#[derive(Debug)]
pub(crate) struct PassiveVoice {
    /// From `Thresholds::passive_max_percent`.
    pub(crate) max_percent: usize,
}

impl Rule for PassiveVoice {
    fn id(&self) -> RuleId {
        RuleId::new("passive-voice")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let language = post.locale.language();
        let Some(all) = lower_sentences(post) else {
            return Vec::new();
        };
        let found = all.iter().filter(|s| passive(s, language)).count();
        if found * 100 <= self.max_percent * all.len() {
            return Vec::new();
        }
        let count = all.len();
        vec![Hit::about(format!(
            "{found} of {count} sentences are passive"
        ))]
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::LintSite;

    fn lower(sentence: &str) -> Vec<String> {
        crate::focus::words(sentence)
    }

    fn post(language: &str, sentence: &str, times: usize) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new("a").unwrap());
        post.prose_text = Some(format!("{sentence} ").repeat(times));
        post
    }

    #[test]
    fn the_passive_is_found_by_its_form_in_english_and_german() {
        assert!(passive(&lower("The post was written by Amira."), "en"));
        assert!(passive(&lower("Links are often not followed."), "en"));
        assert!(!passive(&lower("We wrote the post."), "en"));
        assert!(!passive(
            &lower("The sky is red and the feed is new."),
            "en"
        ));
        assert!(passive(&lower("Der Text wird von allen gelesen."), "de"));
        assert!(passive(&lower("Die Seite wurde gestern optimiert."), "de"));
        assert!(passive(&lower("Der Link war eingesetzt worden."), "de"));
        assert!(!passive(&lower("Anna wird morgen nach Hause gehen."), "de"));
        assert!(!passive(&lower("Er wird das Buch abgeben."), "de"));
        assert!(!passive(&lower("The post was written."), "fr"));
    }

    #[test]
    fn a_text_of_passive_sentences_is_flagged_and_a_short_one_is_not() {
        let rule = PassiveVoice { max_percent: 10 };
        let site = LintSite::default();
        let en = post("en", "The links were added by the editor last week.", 25);
        let hits = rule.check(&en, &site);
        assert_eq!(hits, [Hit::about("25 of 25 sentences are passive")]);
        let de = post("de", "Die Links werden vom Team geprüft.", 40);
        assert_eq!(rule.check(&de, &site).len(), 1);
        let short = post("en", "The links were added.", 5);
        assert_eq!(rule.check(&short, &site), [], "too short to judge");
    }

    #[test]
    fn linking_words_and_phrases_count_in_both_languages() {
        let rule = FewTransitions { min_percent: 30 };
        let site = LintSite::default();
        let flat = post("en", "Links help readers find more posts.", 40);
        let hits = rule.check(&flat, &site);
        assert_eq!(hits, [Hit::about("0 of 40 sentences use a linking word")]);
        let linked = post("en", "For example, links help readers.", 50);
        assert_eq!(rule.check(&linked, &site), []);
        let german = post("de", "Zum Beispiel helfen Links den Lesern.", 50);
        assert_eq!(rule.check(&german, &site), []);
        let flat_de = post("de", "Links helfen den Lesern sehr.", 50);
        assert_eq!(rule.check(&flat_de, &site).len(), 1);
    }
}
