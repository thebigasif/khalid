//! Long, rare words, as Yoast Premium's word complexity check finds them.
//!
//! A word is complex when it is long and not among the words readers meet
//! every day. English: more than seven letters, not capitalised, so names
//! do not count. German: more than ten letters, because German joins
//! words into long everyday nouns, capitalised or not. The lists of
//! everyday long words are short on purpose: they hold the words a blog
//! post uses most, and a word missing from them only nudges the share.

use crate::readability::{MIN_PROSE_WORDS, all_sentences};
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView, german};

/// Long English words that readers meet every day, in lowercase, between
/// spaces.
const COMMON_EN: &str = " actually addition although analysis anything approach argument \
     audience available beautiful becoming behavior behaviour beginning benefits birthday \
     bookmark building business calendar campaign category certainly character checklist \
     children clicking colleague comments commercial community companies complete computer \
     concerns condition conference consider continue contract creative currently customer \
     customers database daughter decision definition describe designed detailed different \
     difficult directly discover discussion distance document download economic education \
     electric employee employees engagement engineer especially essential everybody everyone \
     everything everywhere evidence examples excellent exercise experience experiment external \
     favorite favourite features feedback financial followers following football frequently \
     friendly function generally government greatest guidelines happened headline headlines \
     homepage hospital immediately important improve including increase individual industry \
     information instance interest interesting internal internet interview keywords knowledge \
     language learning listening literally location marketing material measures meetings \
     membership messages millions minister navigation necessary negative newsletter \
     newspaper normally obviously official organisation organization original otherwise \
     overview paragraph paragraphs particular partners password patients percentage \
     performance personal photograph pictures platform political population position \
     positive possible practical practice president pressure previous probably problems \
     products professional programme progress projects property provided publication \
     published publishing purchase question questions reaching readers reading recently \
     recommend relationship remember research resources response restaurant results \
     schedule security sentence sentences services settings shipping shopping shoulder \
     situation software solution something sometimes specific standard starting statement \
     straight strategy strength structure students subscribe subscribers successful \
     suggestion summary surprise teachers teaching technology template terrible thinking \
     thousand together tomorrow traditional training transfer tutorial understand university \
     valuable visitors websites whatever whenever workshop yesterday yourself ";

/// Whether `word`, as written, is long and rare in `language`. In English
/// a capital marks a name, unless the word is `at` the start of its sentence. A
/// language without a list has no complex words.
fn complex(word: &str, language: &str, at: usize) -> bool {
    let letters = word.chars().filter(|c| c.is_alphabetic()).count();
    let lower = format!(" {} ", word.to_lowercase());
    match language {
        "en" => {
            let capital = word.chars().next().is_some_and(char::is_uppercase);
            letters > 7 && (at == 0 || !capital) && !COMMON_EN.contains(&lower)
        }
        "de" => letters > 10 && !german::COMMON_LONG.contains(&lower),
        _ => false,
    }
}

/// Too large a share of the prose's words are long and rare.
#[derive(Debug)]
pub(crate) struct ComplexWords {
    /// From `Thresholds::complex_words_max_percent`.
    pub(crate) max_percent: usize,
}

impl Rule for ComplexWords {
    fn id(&self) -> RuleId {
        RuleId::new("complex-words")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let language = post.locale.language();
        let sentences = all_sentences(post);
        let all = sentences.iter().map(Vec::len).sum::<usize>();
        if all < MIN_PROSE_WORDS {
            return Vec::new();
        }
        let hard: Vec<&str> = sentences
            .iter()
            .flat_map(|sentence| sentence.iter().enumerate())
            .filter(|(at, word)| complex(word, language, *at))
            .map(|(_, word)| *word)
            .collect();
        if hard.len() * 100 <= self.max_percent * all {
            return Vec::new();
        }
        let count = hard.len();
        let example = hard.first().copied().unwrap_or_default();
        vec![Hit::about(format!(
            "{count} of {all} words are long and rare, such as \"{example}\""
        ))]
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::LintSite;

    fn post(language: &str, sentence: &str, times: usize) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new("a").unwrap());
        post.prose_text = Some(format!("{sentence} ").repeat(times));
        post
    }

    #[test]
    fn long_rare_words_count_and_names_and_everyday_words_do_not() {
        assert!(complex("infrastructure", "en", 1));
        assert!(!complex("Wellington", "en", 1), "a name");
        assert!(complex("Interoperability", "en", 0), "opens a sentence");
        assert!(!complex("everything", "en", 1), "an everyday word");
        assert!(!complex("short", "en", 1));
        assert!(complex("Rechtsschutzversicherung", "de", 1));
        assert!(!complex("Informationen", "de", 1), "an everyday word");
        assert!(!complex("Lesbarkeit", "de", 1), "ten letters");
        assert!(!complex("infrastructure", "fr", 1));
    }

    #[test]
    fn a_text_full_of_rare_words_is_flagged_in_both_languages() {
        let rule = ComplexWords { max_percent: 10 };
        let site = LintSite::default();
        let en = post("en", "Our infrastructure needs interoperability.", 60);
        let hits = rule.check(&en, &site);
        let found = "120 of 240 words are long and rare, such as \"infrastructure\"";
        assert_eq!(hits, [Hit::about(found)]);
        let plain = post("en", "Links help readers find more posts.", 40);
        assert_eq!(rule.check(&plain, &site), []);
        let de = post("de", "Die Rechtsschutzversicherung zahlt.", 70);
        assert_eq!(rule.check(&de, &site).len(), 1);
    }
}
