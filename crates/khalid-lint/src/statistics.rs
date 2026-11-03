//! W4, sourced statistics. A statistic is a figure that claims something
//! about the world: a share (47% or 47 percent), a sum of money ($1,200 or 300 euros), a
//! scale (8.5 billion, 3 million) or a multiple (3x, 4 times more), or
//! any number near a word that says it came from somewhere, such as
//! study, survey, report, found or according to. A number the writer
//! uses to instruct or count, such as "Step 3", "40 to 60 words" or "5
//! minutes", a version number or a date is not one. Headings and code
//! are never read.

use crate::writing::{first_words, is_heading_line};
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// Words after a number that make it a share, a sum of money or a scale.
const FIGURE_WORDS: &[&str] = &[
    "percent", "pct", "dollars", "euros", "pounds", "million", "billion", "trillion", "bn",
];

/// Words after "3 times" that make it a multiple, as in "3 times more".
/// A comparative ending in "er", such as "faster", counts too.
const MULTIPLE_WORDS: &[&str] = &["more", "less", "as", "likelier"];

/// Words that say a number came from somewhere a reader may want to
/// check. "According" stands for "according to".
const SOURCE_WORDS: &[&str] = &[
    "study",
    "studies",
    "survey",
    "surveys",
    "report",
    "reports",
    "research",
    "found",
    "according",
];

/// The signs that make a figure a sum of money.
const CURRENCY: &[char] = &['$', '€', '£', '¥'];

/// How many words a source word or a year may sit from its number.
const REACH: usize = 6;

/// **W4: sourced statistics.**
///
/// A statistic with no linked source on its line and no year near it
/// teaches doubt, so it is flagged; the fix is a link or a date. The rule
/// reads the writer's own prose, so code is never read, and skips headings.
#[derive(Debug)]
pub(crate) struct SourcedStatistics;

impl Rule for SourcedStatistics {
    fn id(&self) -> RuleId {
        RuleId::new("sourced-statistics")
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        post.prose_text
            .as_deref()
            .unwrap_or(&post.body_text)
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(|line| !is_heading_line(post, line))
            .filter(|line| !is_linked(post, line))
            .filter(|line| bare_statistic(line))
            .map(|line| {
                Hit::about(first_words(line, 8)).suggesting(vec![
                    "link the statistic to its source, or name the year it held".to_owned(),
                ])
            })
            .collect()
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }
}

/// Whether `line` holds a statistic with no year near it.
fn bare_statistic(line: &str) -> bool {
    if !line.contains(|c: char| c.is_ascii_digit()) {
        return false;
    }
    let words: Vec<String> = line.split_whitespace().map(clean).collect();
    (0..words.len()).any(|at| is_statistic(&words, at) && !nearby(&words, at).any(is_year))
}

/// `word` in lowercase without the punctuation around it, keeping the
/// percent and currency signs.
fn clean(word: &str) -> String {
    word.trim_matches(|c: char| !(c.is_alphanumeric() || c == '%' || CURRENCY.contains(&c)))
        .to_lowercase()
}

/// The words within [`REACH`] of the word at `at`, the word included.
fn nearby(words: &[String], at: usize) -> impl Iterator<Item = &str> {
    words
        .iter()
        .skip(at.saturating_sub(REACH))
        .take(2 * REACH + 1)
        .map(String::as_str)
}

/// Whether the word at `at` of `words` is a figure that reads as a
/// statistic: a share, a sum of money, a scale or a multiple, or a figure
/// near a word that names where it came from.
fn is_statistic(words: &[String], at: usize) -> bool {
    let word = words.get(at).map_or("", String::as_str);
    if !is_figure(word) {
        return false;
    }
    let next = words.get(at + 1).map_or("", String::as_str);
    let after = words.get(at + 2).map_or("", String::as_str);
    let multiple = word.strip_suffix('x').is_some_and(is_number)
        || (next == "times" && (MULTIPLE_WORDS.contains(&after) || after.ends_with("er")));
    word.contains('%')
        || word.contains(CURRENCY)
        || multiple
        || FIGURE_WORDS.contains(&next)
        || (next == "per" && after == "cent")
        || nearby(words, at).any(|near| SOURCE_WORDS.contains(&near))
}

/// Whether `word` holds a figure: a digit, and it is not a year, a date
/// such as 2026-10-26 or a version such as 1.3.1 or v2.
fn is_figure(word: &str) -> bool {
    let date = word.contains(['-', '/']) && word.chars().all(|c| "0123456789-/".contains(c));
    let version = word.matches('.').count() > 1 || word.strip_prefix('v').is_some_and(is_number);
    word.contains(|c: char| c.is_ascii_digit()) && !is_year(word) && !date && !version
}

/// Whether `word` is a plain number, such as 3 or 8.5.
fn is_number(word: &str) -> bool {
    word.starts_with(|c: char| c.is_ascii_digit())
        && word.chars().all(|c| "0123456789.,".contains(c))
}

/// Whether the cleaned `word` is a four-digit year a statistic can hang
/// on, from 1900 on. A sign makes it a figure: $2000 is not a year.
fn is_year(word: &str) -> bool {
    word.len() == 4
        && word
            .parse::<u16>()
            .is_ok_and(|year| (1900..=2200).contains(&year))
}

/// Whether `line` carries one of the post's links, so a number in it is
/// treated as sourced.
fn is_linked(post: &LintPost, line: &str) -> bool {
    post.links
        .iter()
        .any(|link| !link.text.is_empty() && line.contains(link.text.as_str()))
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};

    use super::*;
    use crate::{Heading, HeadingLevel, LinkKind, LintLink, LintSite};

    /// One line for each kind of statistic, none with a source or a year.
    const FLAGGED: &[&str] = &[
        "Half the posts lose 47% of readers in ten seconds.",
        "Readers who scroll stay 31 percent longer.",
        "The average site spends $1,200 on content.",
        "Agencies charge 300 euros for an audit.",
        "A full audit costs $2000.",
        "Google answers 8.5 billion searches.",
        "AI search sends 3 million visits to blogs.",
        "Answer-first posts get cited 3x more often.",
        "Long posts earn 4 times more links.",
        "One survey of 1,000 writers asked about headings.",
        "According to Ahrefs, 96 of every 100 pages get no visits.",
        "The team found 12 posts that rank for it.",
    ];

    /// Lines that hold a number but no statistic, or a dated one.
    const PASSING: &[&str] = &[
        "Cut it to 40 to 60 words.",
        "The answer runs 47 words.",
        "Reading it takes 5 minutes.",
        "Step 3: write the answer first.",
        "Khalid 1.3.1 ships the fix, so upgrade to v2 later.",
        "The update came out on 2026-10-26.",
        "Post 3 times a week, and write 12 posts before you launch.",
        "In 2024, 47% of readers left early.",
        "A 2025 survey of 1,000 writers asked about it.",
    ];

    #[test]
    fn a_statistic_is_flagged_and_a_count_a_version_or_a_date_is_not() {
        for line in FLAGGED {
            assert!(bare_statistic(line), "should flag: {line}");
        }
        for line in PASSING {
            assert!(!bare_statistic(line), "should pass: {line}");
        }
    }

    /// A post with the heading `Why 47% leave?`, then `prose`, then code.
    fn post(prose: &str) -> LintPost {
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.body_text = format!("Why 47% leave?\n{prose}\nfetch(\"/api/47%\")");
        post.prose_text = Some(format!("Why 47% leave?\n{prose}"));
        post.headings = vec![Heading::new(HeadingLevel::H2, "Why 47% leave?")];
        post
    }

    #[test]
    fn headings_code_and_linked_lines_are_never_flagged() {
        let site = LintSite::default();
        assert_eq!(
            SourcedStatistics.check(&post("It runs 47 words."), &site),
            []
        );
        let mut linked = post("Readers stay 31 percent longer, says the Ahrefs study.");
        assert_eq!(SourcedStatistics.check(&linked, &site).len(), 1);
        let link = LintLink::new(LinkKind::External, "https://ahrefs.com/", "Ahrefs study");
        linked.links = vec![link];
        assert_eq!(SourcedStatistics.check(&linked, &site), []);
    }
}
