//! What the admin shows beside a post's findings: its readability score,
//! apart from the SEO score, its word count, its reading index and its
//! most prominent words, as Yoast shows them beside its checks.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::focus::words;
use crate::readability::prose;
use crate::stem::WordForms;
use crate::words::names_subject;
use crate::{Finding, LintPost, RuleId, Score, reading};

/// The rules that judge how easy a post is to read. They count towards
/// the readability score and not towards the SEO score.
const READABILITY: &[&str] = &[
    "hard-to-read",
    "long-paragraph",
    "long-sentences",
    "long-section",
    "repeated-openers",
    "few-transitions",
    "passive-voice",
    "complex-words",
    "non-inclusive-term",
];

/// Whether `rule` judges how easy a post is to read.
pub(crate) fn is_readability(rule: RuleId) -> bool {
    READABILITY.contains(&rule.as_str())
}

/// How many prominent words the analysis lists, as Yoast does.
const PROMINENT: usize = 10;

/// Everyday English words that are not filler words but never name a
/// topic, between spaces, left out of the prominent words.
const EVERYDAY_EN: &str = " all also any been can could each even had her his how its just \
     may more most much one only other out over some such them then there these those too two \
     very were what when where which who would ";

/// Whether `word`, in lowercase, never names a topic in `language`.
fn everyday(word: &str, language: &str) -> bool {
    let spaced = format!(" {word} ");
    let listed = match language {
        "en" => EVERYDAY_EN.contains(&spaced),
        "de" => crate::german::EVERYDAY.contains(&spaced),
        _ => false,
    };
    listed || !names_subject(language, word)
}

/// A word must appear this often to count as prominent.
const PROMINENT_MIN_USES: usize = 2;

/// A post's readability score and the numbers behind it, for the admin.
///
/// Serialised as `{"word_count": 640, "reading_index": 38, "readability":
/// {"value": 95, "lost": [...]}, "prominent_words": [{"word": "links",
/// "uses": 9}]}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Analysis {
    word_count: usize,
    reading_index: Option<usize>,
    readability: Score,
    prominent_words: Vec<ProminentWord>,
}

/// A word the post uses often, in its most used form, with how often it
/// appears in any form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProminentWord {
    word: String,
    uses: usize,
}

impl ProminentWord {
    /// The word, in lowercase, in the form the post uses most.
    pub fn word(&self) -> &str {
        &self.word
    }

    /// How many times the post uses the word, in any form.
    pub fn uses(&self) -> usize {
        self.uses
    }
}

impl Analysis {
    /// The analysis of `post` from `findings`, the report about it.
    pub(crate) fn of(post: &LintPost, findings: &[Finding]) -> Self {
        let counted = words(&post.body_text).len();
        Self {
            word_count: if post.word_count > 0 {
                post.word_count
            } else {
                counted
            },
            reading_index: reading::index(prose(post), post.locale.language()),
            readability: Score::of_rules(post, findings, is_readability),
            prominent_words: prominent(post),
        }
    }

    /// How many words the body holds.
    pub fn word_count(&self) -> usize {
        self.word_count
    }

    /// The prose's LIX, or `None` under 100 words, where it means little.
    pub fn reading_index(&self) -> Option<usize> {
        self.reading_index
    }

    /// The readability score out of 100, from the readability rules only.
    pub fn readability(&self) -> &Score {
        &self.readability
    }

    /// Up to ten words the post uses most, most used first, filler words
    /// left out and word forms counted together.
    pub fn prominent_words(&self) -> &[ProminentWord] {
        &self.prominent_words
    }
}

/// The words the prose of `post` uses most, by stem, each in its most
/// used form. Ties go to the word first in the alphabet, so the list is
/// the same on every run.
fn prominent(post: &LintPost) -> Vec<ProminentWord> {
    let language = post.locale.language();
    let forms = WordForms::of(language);
    let mut stems: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for word in words(prose(post)) {
        let counts = word.chars().count() >= 3 && word.chars().all(char::is_alphabetic);
        if counts && !everyday(&word, language) {
            let stem = forms.stem(&word).into_owned();
            *stems.entry(stem).or_default().entry(word).or_default() += 1;
        }
    }
    let mut found: Vec<ProminentWord> = stems
        .into_values()
        .filter_map(|spellings| {
            let uses = spellings.values().sum();
            let most = spellings
                .iter()
                .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)));
            most.map(|(word, _)| ProminentWord {
                word: word.clone(),
                uses,
            })
        })
        .filter(|word| word.uses >= PROMINENT_MIN_USES)
        .collect();
    found.sort_by(|a, b| b.uses.cmp(&a.uses).then_with(|| a.word.cmp(&b.word)));
    found.truncate(PROMINENT);
    found
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;

    fn post(language: &str, prose: &str) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new("a").unwrap());
        post.prose_text = Some(prose.to_owned());
        post
    }

    fn listed(post: &LintPost) -> Vec<(String, usize)> {
        prominent(post)
            .into_iter()
            .map(|word| (word.word, word.uses))
            .collect()
    }

    #[test]
    fn word_forms_count_together_and_filler_words_are_left_out() {
        let en = post(
            "en",
            "Links help. Internal links rank. A link helps the readers.",
        );
        let want = [("links".to_owned(), 3), ("help".to_owned(), 2)];
        assert_eq!(listed(&en), want);
        let de = post(
            "de",
            "Interne Links helfen. Der interne Link hilft und die Links",
        );
        assert_eq!(
            listed(&de),
            [("links".to_owned(), 3), ("interne".to_owned(), 2)]
        );
    }

    #[test]
    fn at_most_ten_words_are_listed_and_ties_go_by_the_alphabet() {
        let text: Vec<String> = ('a'..='l').map(|c| format!("w{c}rd w{c}rd")).collect();
        let found = listed(&post("en", &text.join(" ")));
        assert_eq!(found.len(), 10);
        assert_eq!(found.first(), Some(&("ward".to_owned(), 2)));
    }
}
