//! Word forms: the stems that let "links" match "link" and "Links" match
//! "Link", from the Snowball stemmers for English and German.
//!
//! A stem is the part of a word its forms share, such as "link" for
//! "link", "links" and "linking". The focus checks compare stems, so a
//! writer is not told to repeat a keyword in exactly one grammatical form.
//! A language without a stemmer compares words as written.

use std::borrow::Cow;
use std::fmt;

use rust_stemmers::{Algorithm, Stemmer};

/// The stemmer for one language.
pub(crate) struct WordForms(Option<Stemmer>);

impl fmt::Debug for WordForms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WordForms")
            .field(&self.0.as_ref().map(|_| "snowball"))
            .finish()
    }
}

impl WordForms {
    /// The stemmer for `language`, such as `en` or `de`.
    pub(crate) fn of(language: &str) -> Self {
        let algorithm = match language {
            "en" => Some(Algorithm::English),
            "de" => Some(Algorithm::German),
            _ => None,
        };
        Self(algorithm.map(Stemmer::create))
    }

    /// The stem of `word`, which is in lowercase already.
    pub(crate) fn stem<'w>(&self, word: &'w str) -> Cow<'w, str> {
        match &self.0 {
            Some(stemmer) => stemmer.stem(word),
            None => Cow::Borrowed(word),
        }
    }

    /// The stems of the words of `phrase`, in order.
    pub(crate) fn phrase(&self, phrase: &str) -> Vec<String> {
        crate::focus::words(phrase)
            .iter()
            .map(|word| self.stem(word).into_owned())
            .collect()
    }

    /// Whether `word`, in lowercase, has the stem `stem`. A word whose
    /// first letter differs never does, so most words skip the stemmer and
    /// a long post stays fast to check.
    pub(crate) fn has_stem(&self, word: &str, stem: &str) -> bool {
        let first = |text: &str| text.chars().next().map(plain_letter);
        first(word) == first(stem) && self.stem(word) == stem
    }

    /// Whether `text`, its words in lowercase, holds `phrase`, given as
    /// stems, in any word form.
    pub(crate) fn holds(&self, text: &[String], phrase: &[String]) -> bool {
        self.uses(text, phrase) > 0
    }

    /// How many times `text`, its words in lowercase, holds `phrase`,
    /// given as stems, in any word form.
    pub(crate) fn uses(&self, text: &[String], phrase: &[String]) -> usize {
        if phrase.is_empty() {
            return 0;
        }
        text.windows(phrase.len())
            .filter(|run| {
                run.iter()
                    .zip(phrase)
                    .all(|(word, stem)| self.has_stem(word, stem))
            })
            .count()
    }
}

/// The letter as the German stemmer leaves it at the start of a stem,
/// without its umlaut, so "über" and its stem "uber" start alike.
fn plain_letter(letter: char) -> char {
    match letter {
        'ä' => 'a',
        'ö' => 'o',
        'ü' => 'u',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::focus::words;

    #[test]
    fn english_word_forms_share_a_stem() {
        let forms = WordForms::of("en");
        let phrase = forms.phrase("internal link");
        assert!(forms.holds(&words("Linking with internal links"), &phrase));
        assert!(!forms.holds(&words("internal lines"), &phrase));
        assert_eq!(
            forms.uses(&words("internal link, internal links"), &phrase),
            2
        );
    }

    #[test]
    fn german_word_forms_share_a_stem_umlaut_or_not() {
        let forms = WordForms::of("de");
        let phrase = forms.phrase("interner Link");
        assert!(forms.holds(&words("Mit internen Links arbeiten"), &phrase));
        let over = forms.phrase("Übersicht");
        assert!(forms.holds(&words("zwei übersichten"), &over));
    }

    #[test]
    fn a_language_without_a_stemmer_matches_words_as_written() {
        let forms = WordForms::of("fr");
        let phrase = forms.phrase("lien interne");
        assert!(forms.holds(&words("un lien interne"), &phrase));
        assert!(!forms.holds(&words("des liens internes"), &phrase));
        assert_eq!(forms.uses(&words("lien"), &[]), 0);
    }
}
