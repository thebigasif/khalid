//! The focus keyword: the phrase a post wants to be found for, and the
//! places a reader and Google look for it.

use khalid_core::{Slug, SlugMode};

use crate::stem::WordForms;
use crate::{Hit, LintPost, Rule, RuleId, Severity, SiteView};

/// The words of `text` in lowercase, split at anything that is not a letter
/// or a digit.
pub(crate) fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// Whether `topic` appears in `text` as whole words, in order.
pub(crate) fn holds(text: &[String], topic: &[String]) -> bool {
    !topic.is_empty() && text.windows(topic.len()).any(|run| run == topic)
}

/// A keyword and its synonyms, each kept as the stems of its words, so it
/// is found in any word form: "internal link" in "internal links", and a
/// synonym such as "site links" counts as the keyword.
#[derive(Debug)]
pub(crate) struct Keyword<'p> {
    /// The keyword as written, without outer spaces, for the finding.
    pub(crate) text: &'p str,
    /// The keyword's stems first, then each synonym's.
    phrases: Vec<Vec<String>>,
    forms: WordForms,
}

impl<'p> Keyword<'p> {
    /// `text` with `synonyms`, in the post's `language`, or `None` when
    /// `text` holds no word, as a draft from another caller may.
    pub(crate) fn new(text: &'p str, synonyms: &[String], language: &str) -> Option<Self> {
        let forms = WordForms::of(language);
        let first = forms.phrase(text);
        if first.is_empty() {
            return None;
        }
        let mut phrases = vec![first];
        let synonyms = synonyms.iter().map(|synonym| forms.phrase(synonym));
        phrases.extend(synonyms.filter(|stems| !stems.is_empty()));
        Some(Self {
            text: text.trim(),
            phrases,
            forms,
        })
    }

    /// The post's focus keyword with its synonyms, or `None` when it has
    /// none or it holds no word.
    pub(crate) fn focus(post: &'p LintPost) -> Option<Self> {
        let text = post.focus_keyword.as_deref()?;
        Self::new(text, &post.keyword_synonyms, post.locale.language())
    }

    /// The post's secondary keywords that hold a word, in the header's
    /// order, each without synonyms.
    pub(crate) fn secondary(post: &'p LintPost) -> Vec<Self> {
        let language = post.locale.language();
        post.secondary_keywords
            .iter()
            .filter_map(|text| Self::new(text, &[], language))
            .collect()
    }

    /// Whether `text` holds the keyword or a synonym, in any word form.
    pub(crate) fn found_in(&self, text: &str) -> bool {
        self.found_in_words(&words(text))
    }

    /// Whether `text`, its words in lowercase, holds the keyword or a
    /// synonym, in any word form.
    pub(crate) fn found_in_words(&self, text: &[String]) -> bool {
        self.phrases
            .iter()
            .any(|phrase| self.forms.holds(text, phrase))
    }

    /// How many times `text`, its words in lowercase, holds the keyword
    /// or a synonym, in any word form.
    pub(crate) fn uses(&self, text: &[String]) -> usize {
        self.phrases
            .iter()
            .map(|phrase| self.forms.uses(text, phrase))
            .sum()
    }
}

/// A place in a post where the focus keyword belongs, each checked by its
/// own rule with its own message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Place {
    /// The title search results show: the search title when the post sets
    /// one, else the title.
    Title,
    /// The page's main headline, checked apart from the title only when a
    /// search title takes the title's place in search results.
    Headline,
    /// The description under the title in search results.
    Description,
    /// The first paragraph of the body.
    FirstParagraph,
    /// The subheadings, where one holding the keyword is enough.
    Subheading,
    /// The last part of the web address.
    Address,
    /// The image descriptions, where one holding the keyword is enough.
    ImageAlt,
}

impl Place {
    /// The rule that checks this place.
    const fn rule(self) -> RuleId {
        RuleId::new(match self {
            Self::Title => "focus-not-in-title",
            Self::Headline => "focus-not-in-h1",
            Self::Description => "focus-not-in-description",
            Self::FirstParagraph => "focus-not-in-first-paragraph",
            Self::Subheading => "focus-not-in-subheading",
            Self::Address => "focus-not-in-address",
            Self::ImageAlt => "focus-not-in-image-alt",
        })
    }

    /// Whether this place in `post` holds `keyword`, or `None` when the
    /// post does not have the place yet, which is left to the rule about
    /// that place.
    pub(crate) fn holds(self, post: &LintPost, keyword: &Keyword<'_>) -> Option<bool> {
        let found = |text: &str| keyword.found_in(text);
        match self {
            Self::Title => search_title(post).or(post.title.as_deref()).map(found),
            Self::Headline => search_title(post).and(post.title.as_deref()).map(found),
            Self::Description => post.description.as_deref().map(found),
            Self::FirstParagraph => first_paragraph(post).map(found),
            Self::Subheading => (!post.headings.is_empty())
                .then(|| post.headings.iter().any(|heading| found(&heading.text))),
            Self::Address => Some(in_address(post, keyword)),
            Self::ImageAlt => {
                let mut described = post
                    .images
                    .iter()
                    .filter_map(|image| image.alt.as_deref())
                    .filter(|alt| !alt.trim().is_empty())
                    .peekable();
                described.peek().is_some().then(|| described.any(found))
            }
        }
    }
}

/// The post's search title, when it sets one with any text in it.
pub(crate) fn search_title(post: &LintPost) -> Option<&str> {
    post.seo_title
        .as_deref()
        .filter(|title| !title.trim().is_empty())
}

/// The first line of the body's text that is not a heading. The plain text
/// holds one line per paragraph, heading or list item.
pub(crate) fn first_paragraph(post: &LintPost) -> Option<&str> {
    post.body_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .find(|line| !post.headings.iter().any(|h| h.text.trim() == *line))
}

/// Whether the post's address holds the keyword as written, or in its plain
/// form, such as `ueber` for "über".
fn in_address(post: &LintPost, keyword: &Keyword<'_>) -> bool {
    let address = words(post.slug.as_str());
    let plain = Slug::from_title(keyword.text, &post.locale, SlugMode::Ascii)
        .map(|slug| words(slug.as_str()))
        .unwrap_or_default();
    keyword.found_in_words(&address) || holds(&address, &plain)
}

/// A post with no focus keyword, so none of the focus checks can run and
/// its score leaves the topic out.
#[derive(Debug)]
pub(crate) struct NoFocusKeyword;

impl Rule for NoFocusKeyword {
    fn id(&self) -> RuleId {
        RuleId::new("no-focus-keyword")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        Keyword::focus(post)
            .is_none()
            .then(Hit::post)
            .into_iter()
            .collect()
    }
}

/// The focus keyword is missing from one place in the post. The finding
/// names the keyword, so the writer sees which words to add.
#[derive(Debug)]
pub(crate) struct FocusNotIn(pub(crate) Place);

impl Rule for FocusNotIn {
    fn id(&self) -> RuleId {
        self.0.rule()
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let Some(keyword) = Keyword::focus(post) else {
            return Vec::new();
        };
        if self.0.holds(post, &keyword) == Some(false) {
            vec![Hit::about(keyword.text)]
        } else {
            Vec::new()
        }
    }
}

/// The fewest words a post needs before its keyword density is measured,
/// because one use more or less swings the share of a short text too far.
const DENSITY_MIN_WORDS: usize = 100;

/// How many times per thousand words the post's body holds `keyword` or
/// one of its synonyms, or `None` when it has too few words to judge.
pub(crate) fn per_thousand(post: &LintPost, keyword: &Keyword<'_>) -> Option<usize> {
    let text = words(&post.body_text);
    if text.len() < DENSITY_MIN_WORDS {
        return None;
    }
    Some(keyword.uses(&text) * 1000 / text.len())
}

/// One hit naming the focus keyword when its rate per thousand words is
/// `outside` the band.
fn density_hit(post: &LintPost, outside: impl Fn(usize) -> bool) -> Vec<Hit> {
    let Some(keyword) = Keyword::focus(post) else {
        return Vec::new();
    };
    per_thousand(post, &keyword)
        .filter(|rate| outside(*rate))
        .map(|_| Hit::about(keyword.text))
        .into_iter()
        .collect()
}

/// The focus keyword appears fewer times per thousand words than the
/// floor, so the text does not read as being about it.
#[derive(Debug)]
pub(crate) struct FocusTooRare {
    /// From `Thresholds::focus_min_per_thousand_words`.
    pub(crate) min: usize,
}

impl Rule for FocusTooRare {
    fn id(&self) -> RuleId {
        RuleId::new("focus-too-rare")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        density_hit(post, |rate| rate < self.min)
    }
}

/// The focus keyword appears more times per thousand words than the
/// ceiling, so the text reads as stuffed.
#[derive(Debug)]
pub(crate) struct FocusTooOften {
    /// From `Thresholds::focus_max_per_thousand_words`.
    pub(crate) max: usize,
}

impl Rule for FocusTooOften {
    fn id(&self) -> RuleId {
        RuleId::new("focus-too-often")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        density_hit(post, |rate| rate > self.max)
    }
}

/// A secondary keyword the body never uses, one hit per keyword in the
/// header's order.
#[derive(Debug)]
pub(crate) struct SecondaryKeywordUnused;

impl Rule for SecondaryKeywordUnused {
    fn id(&self) -> RuleId {
        RuleId::new("secondary-keyword-unused")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, _site: &dyn SiteView) -> Vec<Hit> {
        let text = words(&post.body_text);
        post.secondary_keywords
            .iter()
            .map(|keyword| keyword.trim())
            .filter(|keyword| {
                Keyword::new(keyword, &[], post.locale.language())
                    .is_some_and(|keyword| !keyword.found_in_words(&text))
            })
            .map(Hit::about)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use khalid_core::Locale;

    use super::*;
    use crate::{Heading, HeadingLevel};

    #[test]
    fn a_topic_matches_whole_words_in_order_only() {
        let text = words("Why Internal-Links matter");
        assert!(holds(&text, &words("internal links")));
        assert!(!holds(&text, &words("links internal")));
        assert!(!holds(&text, &words("intern")));
        assert!(!holds(&text, &[]));
    }

    #[test]
    fn the_first_paragraph_skips_headings_and_blank_lines() {
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.body_text = "\nWhy links\n\nLinks help.\nMore.".to_owned();
        post.headings = vec![Heading::new(HeadingLevel::H2, "Why links")];
        assert_eq!(first_paragraph(&post), Some("Links help."));
    }

    #[test]
    fn a_german_keyword_matches_its_plain_address() {
        let mut post = LintPost::new(
            Locale::try_new("de").unwrap(),
            Slug::try_new("ueber-interne-links").unwrap(),
        );
        post.focus_keyword = Some("über interne Links".to_owned());
        let hits = FocusNotIn(Place::Address).check(&post, &crate::LintSite::default());
        assert_eq!(hits, []);
    }

    #[test]
    fn without_a_search_title_only_the_title_rule_reads_the_title() {
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.focus_keyword = Some("internal links".to_owned());
        post.title = Some("Why links matter".to_owned());
        post.seo_title = Some("  ".to_owned());
        let site = crate::LintSite::default();
        let hits = FocusNotIn(Place::Title).check(&post, &site);
        assert_eq!(hits, [Hit::about("internal links")]);
        assert_eq!(FocusNotIn(Place::Headline).check(&post, &site), []);
    }

    #[test]
    fn another_word_form_of_the_keyword_counts_in_english_and_german() {
        let site = crate::LintSite::default();
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.focus_keyword = Some("internal link".to_owned());
        post.title = Some("Linking with internal links".to_owned());
        assert_eq!(FocusNotIn(Place::Title).check(&post, &site), []);
        let mut german = LintPost::new(Locale::try_new("de").unwrap(), Slug::try_new("a").unwrap());
        german.focus_keyword = Some("interner Link".to_owned());
        german.title = Some("Warum interne Links helfen".to_owned());
        assert_eq!(FocusNotIn(Place::Title).check(&german, &site), []);
        german.title = Some("Warum Verweise helfen".to_owned());
        let hits = FocusNotIn(Place::Title).check(&german, &site);
        assert_eq!(hits, [Hit::about("interner Link")]);
    }

    #[test]
    fn a_synonym_counts_as_the_keyword_in_every_place_and_the_density() {
        let site = crate::LintSite::default();
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.focus_keyword = Some("internal links".to_owned());
        post.keyword_synonyms = vec!["site links".to_owned()];
        post.title = Some("Why site links matter".to_owned());
        assert_eq!(FocusNotIn(Place::Title).check(&post, &site), []);
        post.body_text = format!("{} internal links and site links.", ["word"; 96].join(" "));
        let keyword = Keyword::focus(&post).unwrap();
        assert_eq!(per_thousand(&post, &keyword), Some(2000 / 101));
    }

    #[test]
    fn a_keyword_without_words_is_no_keyword() {
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new("a").unwrap());
        post.focus_keyword = Some(" - ".to_owned());
        post.secondary_keywords = vec![String::new()];
        post.title = Some("A title".to_owned());
        post.body_text = ["word"; 120].join(" ");
        let site = crate::LintSite::default();
        assert_eq!(FocusNotIn(Place::Title).check(&post, &site), []);
        assert_eq!(FocusTooRare { min: 5 }.check(&post, &site), []);
        assert_eq!(SecondaryKeywordUnused.check(&post, &site), []);
        assert_eq!(NoFocusKeyword.check(&post, &site), [Hit::post()]);
    }
}
