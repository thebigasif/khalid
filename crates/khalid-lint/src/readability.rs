//! Yoast's readability checks on the writer's own prose: sentence length,
//! text between subheadings, and sentences that start alike.
//!
//! Each rule reads [`prose`], which leaves out code, quotes, tables and the
//! parts Khalid writes itself, so a long code line is never a long
//! sentence. Sentences end as LIX counts them in `reading.rs`, with each
//! language's short forms such as "e.g." or "z. B." kept inside them.

use crate::reading::short_forms;
use crate::{HeadingLevel, Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// The fewest words of prose before a share of its sentences is judged:
/// in a shorter text one sentence swings the share too far.
pub(crate) const MIN_PROSE_WORDS: usize = 200;

/// The prose the readability rules read: the caller's `prose_text`, or
/// the whole `body_text` when it sends none.
pub(crate) fn prose(post: &LintPost) -> &str {
    post.prose_text.as_deref().unwrap_or(&post.body_text)
}

/// Whether `line` is one of the post's headings, at any level.
fn is_heading(post: &LintPost, line: &str) -> bool {
    post.headings.iter().any(|h| h.text.trim() == line.trim())
}

/// The paragraphs of the prose: each line that holds text and is not a
/// heading, in order.
pub(crate) fn paragraphs(post: &LintPost) -> Vec<&str> {
    prose(post)
        .lines()
        .filter(|line| !line.trim().is_empty() && !is_heading(post, line))
        .collect()
}

/// The sentences of `paragraph` in `language`, each as its words without
/// the punctuation around them. A token without a letter, such as "2026"
/// or "-", is not a word.
pub(crate) fn sentences<'t>(paragraph: &'t str, language: &str) -> Vec<Vec<&'t str>> {
    let short = short_forms(language);
    let mut all = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    for token in paragraph.split_whitespace() {
        let word = token.trim_matches(|c: char| !c.is_alphanumeric());
        if word.chars().any(char::is_alphabetic) {
            current.push(word);
        }
        let bare = token.trim_end_matches(['"', '\'', ')', '\u{201d}', '\u{2019}']);
        let ends = bare.ends_with(['.', '!', '?']);
        if ends && !current.is_empty() && !short.contains(&bare.to_lowercase().as_str()) {
            all.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        all.push(current);
    }
    all
}

/// Every sentence of the post's prose, paragraph by paragraph.
pub(crate) fn all_sentences(post: &LintPost) -> Vec<Vec<&str>> {
    let language = post.locale.language();
    paragraphs(post)
        .into_iter()
        .flat_map(|paragraph| sentences(paragraph, language))
        .collect()
}

/// Too large a share of the sentences run past the word limit.
#[derive(Debug)]
pub(crate) struct LongSentences {
    /// From `Thresholds::sentence_max_words`.
    pub(crate) max_words: usize,
    /// From `Thresholds::long_sentences_max_percent`.
    pub(crate) max_percent: usize,
}

impl Rule for LongSentences {
    fn id(&self) -> RuleId {
        RuleId::new("long-sentences")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let all = all_sentences(post);
        if all.iter().map(Vec::len).sum::<usize>() < MIN_PROSE_WORDS {
            return Vec::new();
        }
        let long = all.iter().filter(|s| s.len() > self.max_words).count();
        if long * 100 <= self.max_percent * all.len() {
            return Vec::new();
        }
        let (count, max) = (all.len(), self.max_words);
        vec![Hit::about(format!(
            "{long} of {count} sentences are longer than {max} words"
        ))]
    }
}

/// A stretch of prose longer than the limit with no second or third level
/// subheading, named by the subheading it starts under.
#[derive(Debug)]
pub(crate) struct LongSection {
    /// From `Thresholds::section_max_words`.
    pub(crate) max_words: usize,
}

impl Rule for LongSection {
    fn id(&self) -> RuleId {
        RuleId::new("long-section")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let splits = |line: &str| {
            post.headings.iter().any(|h| {
                matches!(h.level, HeadingLevel::H2 | HeadingLevel::H3) && h.text.trim() == line
            })
        };
        let mut sections: Vec<(Option<&str>, usize)> = vec![(None, 0)];
        for line in prose(post).lines().map(str::trim) {
            if splits(line) {
                sections.push((Some(line), 0));
            } else if let Some((_, words)) = sections.last_mut() {
                *words += line.split_whitespace().count();
            }
        }
        sections
            .into_iter()
            .filter(|(_, words)| *words > self.max_words)
            .map(|(heading, words)| {
                Hit::about(match heading {
                    Some(text) => format!("section \"{text}\" runs {words} words"),
                    None => format!("the opening runs {words} words"),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Heading, LintSite};

    fn post(language: &str, prose: &str) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new("a").unwrap());
        post.prose_text = Some(prose.to_owned());
        post
    }

    fn hits(rule: &dyn Rule, post: &LintPost) -> Vec<Hit> {
        rule.check(post, &LintSite::default())
    }

    #[test]
    fn sentences_keep_short_forms_and_drop_tokens_without_letters() {
        let en = sentences("Use short words, e.g. these ones. In 2026 - fine!", "en");
        assert_eq!(
            en,
            [
                vec!["Use", "short", "words", "e.g", "these", "ones"],
                vec!["In", "fine"]
            ]
        );
        let de = sentences("Kurze Wörter, z. B. diese, helfen. Fertig.", "de");
        assert_eq!(de.len(), 2);
    }

    #[test]
    fn a_text_of_long_sentences_is_flagged_in_both_languages() {
        let rule = LongSentences {
            max_words: 20,
            max_percent: 25,
        };
        let long = ["word"; 25].join(" ");
        let short = "Short and clear.";
        let en = post(
            "en",
            &format!("{long}. {short} {long}. {long}. {long}. {long}. {long}. {long}. {long}."),
        );
        assert_eq!(
            hits(&rule, &en),
            [Hit::about("8 of 9 sentences are longer than 20 words")]
        );
        let wort = ["Wort"; 22].join(" ");
        let de = post(
            "de",
            &[format!("{wort}."), "Kurz und klar. ".repeat(70)].join(" "),
        );
        assert_eq!(
            hits(&rule, &de),
            [],
            "one long sentence in seventy-one is fine"
        );
    }

    #[test]
    fn code_in_the_body_never_counts_once_the_prose_is_known() {
        let rule = LongSentences {
            max_words: 20,
            max_percent: 25,
        };
        let code = ["let x = compute(a, b, c) + offset * scale"; 30].join(" ");
        let mut with_code = post("en", "Short and clear. Easy to read.");
        with_code.body_text = code;
        assert_eq!(hits(&rule, &with_code), []);
        with_code.prose_text = None;
        assert_eq!(
            hits(&rule, &with_code).len(),
            1,
            "without prose, body_text is read"
        );
    }

    #[test]
    fn a_long_stretch_without_a_subheading_is_named_by_its_heading() {
        let rule = LongSection { max_words: 300 };
        let words = ["Wort"; 310].join(" ");
        let mut de = post(
            "de",
            &format!("Kurz.\nWarum Links\n{words}\nDetails\n{words}"),
        );
        de.headings = vec![
            Heading::new(HeadingLevel::H2, "Warum Links"),
            Heading::new(HeadingLevel::H4, "Details"),
        ];
        let found = hits(&rule, &de);
        assert_eq!(
            found,
            [Hit::about("section \"Warum Links\" runs 621 words")]
        );
        let en = post("en", &["word"; 301].join(" "));
        assert_eq!(hits(&rule, &en), [Hit::about("the opening runs 301 words")]);
    }
}
