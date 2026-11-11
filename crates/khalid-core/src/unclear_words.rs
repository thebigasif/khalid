//! The words that make an answer sound unsure, shared by the linter's
//! `plain-answer` rule and the audit's `hedged-answer` check, so both read
//! a paragraph the same way and a site's own words reach both.

/// Hedge words, in lowercase, that a reader can delete without losing
/// meaning.
const HEDGES: &[&str] = &[
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

/// Sentence openers, in lowercase, that promise nothing specific.
const VAGUE_OPENERS: &[&str] = &[
    "there are",
    "there is",
    "it is important",
    "it is worth",
    "it should be noted",
    "needless to say",
    "as we all know",
    "everyone knows",
];

/// How a sentence with two trailing `, which` is named among the words
/// found.
pub const WHICH_CHAIN: &str = "which ... which";

/// The hedge words and vague openers an answer should not use: the
/// built-in English lists, plus any a site adds.
///
/// # Examples
///
/// ```
/// use khalid_core::UnclearWords;
///
/// let words = UnclearWords::default().with_own(&["Vielleicht".to_owned()], &[]);
/// let found = words.find("Es geht vielleicht. There is a way.");
/// assert_eq!(found.names(), ["vielleicht", "there is"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnclearWords {
    hedges: Vec<String>,
    openers: Vec<String>,
}

impl Default for UnclearWords {
    /// The built-in English hedge words and vague openers.
    fn default() -> Self {
        let owned = |list: &[&str]| list.iter().map(|word| (*word).to_owned()).collect();
        Self {
            hedges: owned(HEDGES),
            openers: owned(VAGUE_OPENERS),
        }
    }
}

/// What [`UnclearWords::find`] found in one paragraph, each kind in the
/// order of its list.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Unclear<'a> {
    /// The hedge words found anywhere in the paragraph.
    pub hedges: Vec<&'a str>,
    /// The vague openers found at the start of any sentence.
    pub openers: Vec<&'a str>,
    /// Whether one sentence holds two trailing `, which`.
    pub which_chain: bool,
}

impl Unclear<'_> {
    /// Whether nothing unclear was found.
    pub fn is_empty(&self) -> bool {
        self.hedges.is_empty() && self.openers.is_empty() && !self.which_chain
    }

    /// Every name found: the hedges, then the openers, then
    /// [`WHICH_CHAIN`].
    pub fn names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.hedges.clone();
        names.extend(&self.openers);
        if self.which_chain {
            names.push(WHICH_CHAIN);
        }
        names
    }
}

impl UnclearWords {
    /// These words with a site's own `hedges` and `openers` added after
    /// the built-in ones. Each is read as whole lowercase words, so
    /// `"Kind  Of"` is `"kind of"`; one with no words, or one already in
    /// the list, is left out.
    #[must_use]
    pub fn with_own(mut self, hedges: &[String], openers: &[String]) -> Self {
        add(&mut self.hedges, hedges);
        add(&mut self.openers, openers);
        self
    }

    /// The hedge words, in lowercase.
    pub fn hedges(&self) -> &[String] {
        &self.hedges
    }

    /// The vague openers, in lowercase.
    pub fn openers(&self) -> &[String] {
        &self.openers
    }

    /// The hedge words, vague openers and which-chains in `paragraph`. A
    /// word counts only whole, so "unusually" holds no "usually"; an
    /// opener counts at the start of any sentence; two `, which` in one
    /// sentence are a chain.
    pub fn find(&self, paragraph: &str) -> Unclear<'_> {
        let all = plain_words(paragraph);
        let sentences: Vec<&str> = paragraph.split(['.', '!', '?']).collect();
        let starts: Vec<String> = sentences.iter().map(|line| plain_words(line)).collect();
        Unclear {
            hedges: self
                .hedges
                .iter()
                .filter(|hedge| all.contains(&format!(" {hedge} ")))
                .map(String::as_str)
                .collect(),
            openers: self
                .openers
                .iter()
                .filter(|opener| {
                    let start = format!(" {opener} ");
                    starts.iter().any(|sentence| sentence.starts_with(&start))
                })
                .map(String::as_str)
                .collect(),
            which_chain: sentences
                .iter()
                .any(|sentence| sentence.to_lowercase().matches(", which").count() > 1),
        }
    }
}

/// Adds each of `own` to `list` as whole lowercase words, once.
fn add(list: &mut Vec<String>, own: &[String]) {
    for word in own {
        let word = plain_words(word);
        let word = word.trim();
        if !word.is_empty() && !list.iter().any(|known| known == word) {
            list.push(word.to_owned());
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hedge_counts_only_as_a_whole_word() {
        let words = UnclearWords::default();
        assert!(words.find("Links are unusually quiet helpers.").is_empty());
        assert_eq!(
            words.find("Links are usually helpers.").names(),
            ["usually"]
        );
    }

    #[test]
    fn an_opener_counts_at_the_start_of_any_sentence() {
        let words = UnclearWords::default();
        let found = words.find("Links help. There are three kinds!");
        assert_eq!(found.openers, ["there are"]);
        assert!(words.find("Readers say there are three kinds.").is_empty());
    }

    #[test]
    fn two_trailing_whiches_in_one_sentence_are_a_chain() {
        let words = UnclearWords::default();
        let found = words.find("Add links, which help, which matter.");
        assert_eq!(found.names(), [WHICH_CHAIN]);
        assert!(words.find("Add links, which help. Which ones?").is_empty());
    }

    #[test]
    fn names_list_hedges_then_openers_then_the_chain() {
        let words = UnclearWords::default();
        let found = words.find("There is perhaps a rule, which helps, which matters.");
        assert_eq!(found.names(), ["perhaps", "there is", WHICH_CHAIN]);
    }

    #[test]
    fn own_words_are_read_whole_lowercase_and_once() {
        let own = |words: &[&str]| {
            words
                .iter()
                .map(|word| (*word).to_owned())
                .collect::<Vec<_>>()
        };
        let words = UnclearWords::default().with_own(
            &own(&["Eher  Nicht", "  ", "perhaps", "eher nicht"]),
            &own(&["Es gibt"]),
        );
        assert_eq!(words.hedges().len(), HEDGES.len() + 1);
        assert_eq!(
            words.hedges().last().map(String::as_str),
            Some("eher nicht")
        );
        assert_eq!(words.openers().last().map(String::as_str), Some("es gibt"));
        let found = words.find("Es gibt Brot. Das wird eher nicht schwer.");
        assert_eq!(found.names(), ["eher nicht", "es gibt"]);
    }
}
