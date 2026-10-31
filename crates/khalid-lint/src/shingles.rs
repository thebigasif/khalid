//! The runs of five words in a text, the unit two texts are compared in.
//!
//! Linting one post compares it with every other post of the site, so this
//! runs once per post of the site on each lint. Each word is hashed once,
//! as its letters are read, and each run of five combines five word
//! hashes, so no word is hashed five times, and a text in plain ASCII, as
//! most are, is never copied to lowercase it. [`Shingles::found_in`] then rules most posts out from their
//! [`Words`] alone, before their own set is built.

use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hasher};

/// How many words make one shingle.
const SHINGLE_WORDS: usize = 5;

/// The start of every word's hash: the 64-bit FNV offset basis.
const WORD_SEED: u64 = 0xcbf2_9ce4_8422_2325;

/// The FNV prime each letter is multiplied in with.
const WORD_PRIME: u64 = 0x0000_0100_0000_01b3;

/// The odd number a run's word hashes are folded together with.
const RUN_PRIME: u64 = 0x9e37_79b9_7f4a_7c15;

/// The words of a text, ignoring case and punctuation, each hashed, in
/// order. Hashes are only compared with others made here, so their exact
/// values never matter.
#[derive(Debug, Default)]
pub(crate) struct Words(Vec<u64>);

impl Words {
    /// Replaces the words held with those of `text`, reusing the memory.
    pub(crate) fn read(&mut self, text: &str) {
        self.0.clear();
        if text.is_ascii() {
            let letters = text
                .bytes()
                .map(|byte| char::from(byte.to_ascii_lowercase()));
            hash_words(letters, &mut self.0);
        } else {
            // The whole text at once, so a word-final Greek sigma becomes
            // the final form, as it does when one reads the text.
            hash_words(text.to_lowercase().chars(), &mut self.0);
        }
    }

    /// The hash of each run of five words, in order, repeats included.
    fn runs(&self) -> impl Iterator<Item = u64> + '_ {
        self.0.windows(SHINGLE_WORDS).map(run_hash)
    }
}

/// The distinct shingles of a text: each run of five of its [`Words`],
/// hashed. The set is only counted, never listed, so its order never
/// reaches a finding.
#[derive(Debug, Default)]
pub(crate) struct Shingles(HashSet<u64, BuildHasherDefault<Premixed>>);

impl Shingles {
    /// The shingles of `text`.
    pub(crate) fn of(text: &str) -> Self {
        let mut words = Words::default();
        words.read(text);
        let mut shingles = Self::default();
        shingles.refill(&words);
        shingles
    }

    /// Replaces the shingles held with those of `words`, reusing the memory.
    pub(crate) fn refill(&mut self, words: &Words) {
        self.0.clear();
        self.0.extend(words.runs());
    }

    /// How many distinct shingles the text has.
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    /// How many runs of `words`, repeats included, are shingles of this
    /// text. Every shingle the two texts share is one of those runs, so
    /// this is never less than [`Shingles::shared_with`] would give, and
    /// it costs no set of their own.
    pub(crate) fn found_in(&self, words: &Words) -> usize {
        words.runs().filter(|run| self.0.contains(run)).count()
    }

    /// How many shingles this text and `other` share.
    pub(crate) fn shared_with(&self, other: &Self) -> usize {
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        small.0.iter().filter(|run| large.0.contains(run)).count()
    }
}

/// Pushes onto `words` the hash of each word in `letters`, which are already
/// lowercase. A word is a run of letters and digits; anything else ends it.
fn hash_words(letters: impl Iterator<Item = char>, words: &mut Vec<u64>) {
    let mut word = WORD_SEED;
    let mut in_word = false;
    for letter in letters {
        if letter.is_alphanumeric() {
            word = (word ^ u64::from(u32::from(letter))).wrapping_mul(WORD_PRIME);
            in_word = true;
        } else if in_word {
            words.push(word);
            word = WORD_SEED;
            in_word = false;
        }
    }
    if in_word {
        words.push(word);
    }
}

/// One hash for a run of word hashes, which depends on their order, with
/// its bits spread so the set can use it as it is.
fn run_hash(run: &[u64]) -> u64 {
    let folded = run.iter().fold(0_u64, |acc, word| {
        (acc.rotate_left(27) ^ word).wrapping_mul(RUN_PRIME)
    });
    spread(folded)
}

/// The `splitmix64` finaliser: every bit of the result depends on every bit
/// of `value`.
fn spread(value: u64) -> u64 {
    let value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    let value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

/// A hasher for keys that are already well mixed hashes: it hands the key
/// back as it is, so a lookup costs no second hash.
#[derive(Debug, Default)]
struct Premixed(u64);

impl Hasher for Premixed {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = self.0.rotate_left(8) ^ u64::from(*byte);
        }
    }

    fn write_u64(&mut self, value: u64) {
        self.0 = value;
    }
}

#[cfg(test)]
mod tests {
    use super::{Shingles, Words};

    fn words(text: &str) -> Words {
        let mut words = Words::default();
        words.read(text);
        words
    }

    #[test]
    fn a_text_of_five_words_has_one_shingle() {
        assert_eq!(Shingles::of("one two three four five").len(), 1);
        assert_eq!(Shingles::of("one two three four").len(), 0);
        assert_eq!(Shingles::of("").len(), 0);
    }

    #[test]
    fn case_and_punctuation_do_not_count() {
        let plain = Shingles::of("links help readers find the next page");
        let marked = Shingles::of("Links, help READERS find... the next page!");
        assert_eq!(plain.len(), 3);
        assert_eq!(plain.shared_with(&marked), 3);
    }

    #[test]
    fn letters_outside_ascii_are_read_as_words() {
        let german = Shingles::of("Größe und Maße der Straße heute");
        let lower = Shingles::of("größe und maße der straße heute");
        assert_eq!(german.len(), 2);
        assert_eq!(german.shared_with(&lower), 2);
    }

    #[test]
    fn a_capital_greek_text_matches_its_written_lowercase() {
        let capitals = Shingles::of("ΟΔΟΣ ΚΑΙ ΔΡΟΜΟΣ ΤΗΣ ΠΟΛΗΣ");
        let written = Shingles::of("οδος και δρομος της πολης");
        assert_eq!(capitals.len(), 1);
        assert_eq!(capitals.shared_with(&written), 1);
    }

    #[test]
    fn a_repeated_run_counts_once() {
        let text = "a b c d e a b c d e";
        // abcde, bcdea, cdeab, deabc, eabcd, then abcde again.
        assert_eq!(Shingles::of(text).len(), 5);
    }

    #[test]
    fn word_order_matters() {
        let one = Shingles::of("one two three four five");
        let other = Shingles::of("five four three two one");
        assert_eq!(one.shared_with(&other), 0);
    }

    #[test]
    fn refill_forgets_the_last_text() {
        let mut shingles = Shingles::of("one two three four five six");
        shingles.refill(&words("seven eight nine ten eleven"));
        let fresh = Shingles::of("seven eight nine ten eleven");
        assert_eq!(shingles.len(), 1);
        assert_eq!(shingles.shared_with(&fresh), 1);
    }

    #[test]
    fn found_in_counts_repeats_so_it_never_undercounts() {
        let own = Shingles::of("a b c d e f");
        let theirs = "a b c d e a b c d e";
        assert_eq!(own.shared_with(&Shingles::of(theirs)), 1);
        assert_eq!(own.found_in(&words(theirs)), 2);
        assert_eq!(own.found_in(&words("z y x w v")), 0);
    }

    #[test]
    fn words_are_not_glued_across_a_break() {
        let split = Shingles::of("ab c d e f");
        let glued = Shingles::of("a bc d e f");
        assert_eq!(split.shared_with(&glued), 0);
    }
}
