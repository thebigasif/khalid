//! The four writing checks (phase GEO, W1 to W4).
//!
//! W1: a heading phrased as a question is followed by a standalone answer
//! of 40 to 60 words, so a reader, and an AI answer engine, gets the
//! answer before any detail. W2: a paragraph past 150 words buries
//! its ideas; the target is 75 to 150 words, one idea each. W3: hedge
//! words, vague openers and trailing which-chains make an answer harder
//! to trust and to lift; the word lists live in the catalogue so a site
//! can read and change them. W4: a number a reader cannot check, with no
//! source and no year, teaches doubt; the fix is a link or a date. W4
//! lives in `statistics.rs`.
//!
//! The behaviour follows Steve Toth's and Koray Tugberk Gubur's writing
//! guidance for answer engines, as the GEO phase records. Where their
//! guidance gives no exact word counts, the counts come from the
//! maintainer's brief: 40 to 60 words for the capsule (decision D3),
//! 150 for a paragraph, as Yoast sets it.

use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// A question-style heading ends with a question mark.
const QUESTION_MARK: char = '?';

/// **W1: the answer capsule.**
///
/// A heading phrased as a question must be followed by a standalone
/// answer of `min_answer_words` to `max_answer_words` words (40 to 60 by
/// default, decision D3), so the answer is complete before any detail
/// starts. Only the first paragraph under the heading is counted: the
/// detail after it may run as long as the section needs.
#[derive(Debug)]
pub(crate) struct AnswerCapsule {
    pub(crate) min_answer_words: usize,
    pub(crate) max_answer_words: usize,
}

impl Rule for AnswerCapsule {
    fn id(&self) -> RuleId {
        RuleId::new("answer-capsule")
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let mut hits = Vec::new();
        for heading in question_headings(post) {
            let Some(answer) = answer_after(post, &heading.text) else {
                continue;
            };
            let words = count_words(answer);
            if words < self.min_answer_words || words > self.max_answer_words {
                hits.push(
                    Hit::about(format!(
                        "heading \"{}\" is answered in a first paragraph of {words} words, not the {} to {} of a standalone answer",
                        heading.text, self.min_answer_words, self.max_answer_words
                    ))
                    .suggesting(vec![format!(
                        "write the whole answer in one paragraph of {} to {} words under the heading",
                        self.min_answer_words, self.max_answer_words
                    )]),
                );
            }
        }
        hits
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }
}

/// **W2: paragraph length.**
///
/// A paragraph over `max_paragraph_words` words (150 by default, as in Yoast) buries
/// more than one idea. The target is 75 to 150 words, one idea each.
#[derive(Debug)]
pub(crate) struct LongParagraph {
    pub(crate) max_paragraph_words: usize,
}

impl Rule for LongParagraph {
    fn id(&self) -> RuleId {
        RuleId::new("long-paragraph")
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        post.body_text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(|line| !is_heading_line(post, line))
            .filter(|line| count_words(line) > self.max_paragraph_words)
            .map(|line| {
                Hit::about(format!(
                    "paragraph \"{}\" runs {} words, past the {} word target",
                    first_words(line, 8),
                    count_words(line),
                    self.max_paragraph_words
                ))
                .suggesting(vec![format!(
                    "split it at each change of idea, into paragraphs of 75 to {} words",
                    self.max_paragraph_words
                )])
            })
            .collect()
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }
}

/// The plain words of one line, split on whitespace.
fn count_words(line: &str) -> usize {
    line.split_whitespace().count()
}

/// The first `n` words of `line`, with an ellipsis when it holds more.
pub(crate) fn first_words(line: &str, n: usize) -> String {
    let mut words: Vec<&str> = line.split_whitespace().take(n).collect();
    if line.split_whitespace().count() > n {
        words.push("...");
    }
    words.join(" ")
}

/// The question-style headings of `post`, in body order.
fn question_headings(post: &LintPost) -> Vec<&crate::Heading> {
    post.headings
        .iter()
        .filter(|heading| heading.text.ends_with(QUESTION_MARK))
        .collect()
}

/// The answer under the heading `text`: the first line of text after it,
/// which is its first paragraph, so the detail after the answer is never
/// counted. A list, table or box that comes first is read as its first
/// item, cell or paragraph, one line each. Empty when the next heading
/// comes first. `None` when the heading's line cannot be found, so the
/// rule stays quiet rather than guessing.
fn answer_after<'a>(post: &'a LintPost, text: &str) -> Option<&'a str> {
    let start = post
        .body_text
        .lines()
        .position(|line| line.trim() == text)?;
    Some(
        post.body_text
            .lines()
            .skip(start + 1)
            .take_while(|line| !is_heading_line(post, line))
            .find(|line| !line.trim().is_empty())
            .unwrap_or_default(),
    )
}

/// Whether `line` is one of the post's headings, by its text.
pub(crate) fn is_heading_line(post: &LintPost, line: &str) -> bool {
    post.headings
        .iter()
        .any(|heading| heading.text == line.trim())
}

/// The hedge words, vague openers and vague closers W3 flags, in
/// lowercase, kept here so a site can read and change them. Each word is
/// one a reader can delete without losing meaning.
pub(crate) const HEDGE_WORDS: &[&str] = &[
    "arguably",
    "perhaps",
    "possibly",
    "seemingly",
    "somewhat",
    "basically",
    "generally",
    "usually",
    "probably",
    "maybe",
    "kind of",
    "sort of",
    "fairly",
    "quite",
    "rather",
];

/// Openers that promise nothing specific, in lowercase, at the start of a
/// sentence.
pub(crate) const VAGUE_OPENERS: &[&str] = &[
    "there are",
    "there is",
    "it is important",
    "it is worth",
    "it should be noted",
    "needless to say",
    "as we all know",
    "everyone knows",
];

/// **W3: plain sentences in answer paragraphs.**
///
/// Hedge words, vague openers and trailing which-chains are flagged in
/// the paragraphs that answer a question-style heading, where an answer
/// engine lifts the text. Each kind found brings its own plain fix.
#[derive(Debug)]
pub(crate) struct PlainAnswer {
    pub(crate) hedges: Vec<&'static str>,
    pub(crate) openers: Vec<&'static str>,
}

impl Rule for PlainAnswer {
    fn id(&self) -> RuleId {
        RuleId::new("plain-answer")
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let mut hits = Vec::new();
        for heading in question_headings(post) {
            for line in answer_paragraphs(post, &heading.text) {
                let (found, fixes) = self.unclear(line);
                if !found.is_empty() {
                    hits.push(
                        Hit::about(format!(
                            "the answer under \"{}\" uses {}",
                            heading.text,
                            found.join(", ")
                        ))
                        .suggesting(fixes),
                    );
                }
            }
        }
        hits
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }
}

impl PlainAnswer {
    /// The hedge words, vague openers and which-chains in `paragraph`, in
    /// that order, and one plain fix for each kind found. A word counts
    /// only whole, so "unusually" holds no "usually"; an opener counts at
    /// the start of any sentence; two `, which` in one sentence are a
    /// chain. The audit's `hedged-answer` check reads a paragraph the same
    /// way.
    fn unclear(&self, paragraph: &str) -> (Vec<&'static str>, Vec<String>) {
        let mut found = Vec::new();
        let mut fixes = Vec::new();
        let all = plain_words(paragraph);
        let hedges = self.hedges.iter().filter(|hedge| has_phrase(&all, hedge));
        found.extend(hedges);
        if !found.is_empty() {
            fixes.push("say the claim outright and drop the hedge".to_owned());
        }
        let sentences: Vec<&str> = paragraph.split(['.', '!', '?']).collect();
        let words: Vec<String> = sentences
            .iter()
            .map(|sentence| plain_words(sentence))
            .collect();
        for opener in &self.openers {
            let start = format!(" {opener} ");
            if words.iter().any(|sentence| sentence.starts_with(&start)) {
                found.push(*opener);
                fixes.push(format!(
                    "start the sentence with its point, not with \"{opener}\""
                ));
            }
        }
        let chained = |sentence: &&str| sentence.to_lowercase().matches(", which").count() > 1;
        if sentences.iter().any(chained) {
            found.push("which ... which");
            fixes.push(
                "end the sentence before the second \", which\" and start a new one".to_owned(),
            );
        }
        (found, fixes)
    }
}

/// `text` in lowercase as its words with one space around each, so a
/// phrase is found only whole.
fn plain_words(text: &str) -> String {
    let words: Vec<String> = text
        .split(|c: char| !(c.is_alphanumeric() || c == '\''))
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect();
    format!(" {} ", words.join(" "))
}

/// Whether `words`, made by [`plain_words`], holds `phrase` whole.
fn has_phrase(words: &str, phrase: &str) -> bool {
    words.contains(&format!(" {phrase} "))
}

/// The paragraph lines that answer the heading `text`, until the next
/// heading.
fn answer_paragraphs<'a>(post: &'a LintPost, text: &str) -> Vec<&'a str> {
    let Some(start) = post.body_text.lines().position(|line| line.trim() == text) else {
        return Vec::new();
    };
    post.body_text
        .lines()
        .skip(start + 1)
        .take_while(|line| !is_heading_line(post, line))
        .filter(|line| !line.trim().is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Heading, HeadingLevel, LintSite};

    /// The usual capsule rule, 40 to 60 words.
    const CAPSULE: AnswerCapsule = AnswerCapsule {
        min_answer_words: 40,
        max_answer_words: 60,
    };

    /// `n` plain words, as one paragraph.
    fn paragraph(n: usize) -> String {
        vec!["word"; n].join(" ")
    }

    /// A post with one question heading, the `lines` under it, then a
    /// second heading and a short paragraph.
    fn answered(lines: &[String]) -> LintPost {
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.body_text = format!(
            "Intro.\nHow long is an answer?\n{}\nWhat comes next\nMore.",
            lines.join("\n")
        );
        post.headings = vec![
            Heading::new(HeadingLevel::H2, "How long is an answer?"),
            Heading::new(HeadingLevel::H2, "What comes next"),
        ];
        post
    }

    /// What the capsule rule says about the `lines` under the question.
    fn capsule_hits(lines: &[String]) -> Vec<String> {
        let hits = CAPSULE.check(&answered(lines), &LintSite::default());
        hits.iter()
            .filter_map(Hit::subject)
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn a_capsule_followed_by_300_words_of_detail_passes() {
        let mut lines = vec![paragraph(50), String::new()];
        lines.extend(vec![paragraph(100); 3]);
        assert_eq!(capsule_hits(&lines), Vec::<String>::new());
    }

    /// A first paragraph of 15 or 120 words, a list of 6-word items, or
    /// a heading straight after the question warns, whatever follows.
    #[test]
    fn a_first_paragraph_outside_40_to_60_words_warns() {
        let first_lists_and_none = [
            (vec![paragraph(15), paragraph(40)], 15),
            (vec![paragraph(120)], 120),
            (vec![paragraph(6); 10], 6),
            (Vec::new(), 0),
        ];
        for (lines, words) in first_lists_and_none {
            let hits = capsule_hits(&lines);
            assert_eq!(hits.len(), 1, "{hits:?}");
            assert!(hits[0].contains(&format!("of {words} words")), "{hits:?}");
        }
    }

    /// What the plain-answer rule says about `answer` under the question,
    /// as its subject and its fixes.
    fn plain_hits(answer: &str) -> Vec<(String, Vec<String>)> {
        let rule = PlainAnswer {
            hedges: HEDGE_WORDS.to_vec(),
            openers: VAGUE_OPENERS.to_vec(),
        };
        rule.check(&answered(&[answer.to_owned()]), &LintSite::default())
            .iter()
            .map(|hit| {
                let subject = hit.subject().unwrap_or_default().to_owned();
                (subject, hit.suggestions().to_vec())
            })
            .collect()
    }

    #[test]
    fn a_hedge_counts_only_as_a_whole_word() {
        assert_eq!(plain_hits("Links are unusually quiet helpers."), []);
        let hits = plain_hits("Links are usually helpers.");
        assert_eq!(hits.len(), 1, "{hits:?}");
        assert!(hits[0].0.ends_with("uses usually"), "{hits:?}");
    }

    #[test]
    fn a_vague_opener_fires_at_the_start_of_any_sentence() {
        let hits = plain_hits("Links help readers. There are three kinds of link.");
        assert_eq!(hits.len(), 1, "{hits:?}");
        assert!(hits[0].0.ends_with("uses there are"), "{hits:?}");
        assert_eq!(
            hits[0].1,
            ["start the sentence with its point, not with \"there are\""]
        );
        assert_eq!(plain_hits("Readers say there are three kinds."), []);
    }

    #[test]
    fn two_trailing_whiches_in_one_sentence_are_a_chain() {
        let hits = plain_hits("Add links, which help readers, which help search engines.");
        assert_eq!(hits.len(), 1, "{hits:?}");
        assert!(hits[0].0.ends_with("uses which ... which"), "{hits:?}");
        assert_eq!(
            hits[0].1,
            ["end the sentence before the second \", which\" and start a new one"]
        );
        assert_eq!(
            plain_hits("Add links, which help readers. They help engines too."),
            []
        );
    }

    #[test]
    fn each_pattern_found_brings_its_own_fix() {
        let hits = plain_hits("There is perhaps a rule, which helps, which matters.");
        assert_eq!(hits.len(), 1, "{hits:?}");
        assert!(
            hits[0]
                .0
                .ends_with("uses perhaps, there is, which ... which"),
            "{hits:?}"
        );
        assert_eq!(hits[0].1.len(), 3, "{hits:?}");
    }
}
