//! The runs of five words in a text, the unit two texts are compared in.
//!
//! Linting one post compares it with every other post of the site, so this
//! runs once per post of the site on each lint. Each word is hashed once,
//! as its letters are read, and each run of five combines five word
//! hashes, so no word is hashed five times. A word's letters are packed
//! eight to a multiply, and a text in plain ASCII, as most are, is never
//! copied to lowercase it. [`Shingles::found_in_more_than`] then rules most
//! posts out from their [`Words`] alone, with a 1 KiB bit filter and before
//! their own set is built.

use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hasher};

/// How many words make one shingle.
const SHINGLE_WORDS: usize = 5;

/// The start of every word's hash: the 64-bit FNV offset basis.
const WORD_SEED: u64 = 0xcbf2_9ce4_8422_2325;

/// The FNV prime each letter is multiplied in with.
const WORD_PRIME: u64 = 0x0000_0100_0000_01b3;

/// One odd key for each place in a run, so the same words in another
/// order make another hash.
const RUN_KEYS: [u64; SHINGLE_WORDS] = [
    0x9e37_79b9_7f4a_7c15,
    0xc2b2_ae3d_27d4_eb4f,
    0x1656_67b1_9e37_79f9,
    0xd6e8_feb8_6659_fd93,
    0xa076_1d64_78bd_642f,
];

/// The words of a text, ignoring case and punctuation, each hashed, in
/// order. Hashes are only compared with others made here, so their exact
/// values never matter.
#[derive(Debug, Default)]
pub(crate) struct Words(Vec<u64>);

impl Words {
    /// Replaces the words held with those of `text`, reusing the memory.
    pub(crate) fn read(&mut self, text: &str) {
        if text.is_ascii() {
            self.read_ascii(text.as_bytes());
        } else {
            self.read_letters(text);
        }
    }

    /// Reads a text in plain ASCII, byte by byte, with no copy.
    fn read_ascii(&mut self, text: &[u8]) {
        self.0.clear();
        let mut word = WordHash::default();
        for &byte in text {
            if byte.is_ascii_alphanumeric() {
                word.push(byte.to_ascii_lowercase());
            } else {
                word.end(&mut self.0);
            }
        }
        word.end(&mut self.0);
    }

    /// Reads any text, letter by letter.
    fn read_letters(&mut self, text: &str) {
        self.0.clear();
        let mut word = WordHash::default();
        // The whole text at once, so a word-final Greek sigma becomes the
        // final form, as it does when one reads the text. A letter goes in
        // as its UTF-8 bytes, so a word in plain ASCII hashes the same as
        // in `read_ascii`.
        let mut utf8 = [0; 4];
        for letter in text.to_lowercase().chars() {
            if letter.is_alphanumeric() {
                letter
                    .encode_utf8(&mut utf8)
                    .bytes()
                    .for_each(|b| word.push(b));
            } else {
                word.end(&mut self.0);
            }
        }
        word.end(&mut self.0);
    }

    /// How many runs of five words the text has, repeats included.
    fn run_count(&self) -> usize {
        self.0.len().saturating_sub(SHINGLE_WORDS - 1)
    }

    /// The hash of each run of five words, in order, repeats included.
    fn runs(&self) -> impl Iterator<Item = u64> + '_ {
        self.0.windows(SHINGLE_WORDS).map(run_hash)
    }
}

/// The hash of the word being read. Letters are packed eight to a 64-bit
/// block, and only a full block or the word's end costs a multiply. UTF-8
/// bytes are never zero, so a block that is not empty is never zero.
#[derive(Debug)]
struct WordHash {
    /// The full blocks so far, mixed.
    done: u64,
    /// The letters of the block being filled, the last in the low byte.
    block: u64,
}

impl Default for WordHash {
    fn default() -> Self {
        Self {
            done: WORD_SEED,
            block: 0,
        }
    }
}

impl WordHash {
    /// Adds one byte of a lowercase letter or digit.
    fn push(&mut self, byte: u8) {
        if self.block >> 56 != 0 {
            self.fold_block();
        }
        self.block = (self.block << 8) | u64::from(byte);
    }

    /// Mixes the full block into the blocks before it and starts a new one.
    fn fold_block(&mut self) {
        self.done = self.finish();
        self.block = 0;
    }

    /// The hash of the word as read so far.
    fn finish(&self) -> u64 {
        (self.done ^ self.block).wrapping_mul(WORD_PRIME)
    }

    /// Ends the word, if one was being read, and pushes its hash onto
    /// `words`.
    fn end(&mut self, words: &mut Vec<u64>) {
        if self.block != 0 {
            words.push(self.finish());
            *self = Self::default();
        }
    }
}

/// The distinct shingles of a text: each run of five of its [`Words`],
/// hashed. The set is only counted, never listed, so its order never
/// reaches a finding.
#[derive(Debug)]
pub(crate) struct Shingles {
    /// The shingles.
    set: HashSet<u64, BuildHasherDefault<Premixed>>,
    /// One bit for each value of a hash's top [`FILTER_BITS`] bits, set
    /// when a shingle has it. A clear bit means "not a shingle", read
    /// from one small table with no branch.
    filter: Vec<u64>,
}

impl Default for Shingles {
    fn default() -> Self {
        Self {
            set: HashSet::default(),
            filter: vec![0; FILTER_WORDS],
        }
    }
}

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
        self.set.clear();
        self.filter.fill(0);
        for run in words.runs() {
            self.set.insert(run);
            let (at, bit) = filter_place(run);
            if let Some(slot) = self.filter.get_mut(at) {
                *slot |= bit;
            }
        }
    }

    /// How many distinct shingles the text has.
    pub(crate) fn len(&self) -> usize {
        self.set.len()
    }

    /// Whether more than `limit` runs of `words`, repeats included, may be
    /// shingles of this text. Every shingle the two texts share is one of
    /// those runs, and a run that is a shingle always passes the filter,
    /// so this never undercounts [`Shingles::shared_with`]. It costs no
    /// set of their own, and stops as soon as the answer is known: once
    /// `limit` is passed, or once the runs left could no longer pass it.
    pub(crate) fn found_in_more_than(&self, words: &Words, limit: usize) -> bool {
        let mut left = words.run_count();
        let mut found = 0;
        for run in words.runs() {
            if found > limit {
                return true;
            }
            if found + left <= limit {
                return false;
            }
            let (at, bit) = filter_place(run);
            let slot = self.filter.get(at).copied().unwrap_or(u64::MAX);
            found += usize::from(slot & bit != 0);
            left -= 1;
        }
        found > limit
    }

    /// How many shingles this text and `other` share.
    pub(crate) fn shared_with(&self, other: &Self) -> usize {
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        small
            .set
            .iter()
            .filter(|run| large.set.contains(run))
            .count()
    }
}

/// How many top bits of a shingle's hash pick its bit in the filter.
const FILTER_BITS: u32 = 13;

/// The filter's size in 64-bit words: 1 KiB, which stays in the fastest
/// cache, and leaves a post's few hundred shingles setting under one bit
/// in ten.
const FILTER_WORDS: usize = (1 << FILTER_BITS) / 64;

/// The word of the filter and the bit in it that stand for `run`.
fn filter_place(run: u64) -> (usize, u64) {
    let top = run >> (64 - FILTER_BITS);
    let at = usize::try_from(top >> 6).unwrap_or(usize::MAX);
    (at, 1 << (top & 63))
}

/// One hash for a run of word hashes, which depends on their order, with
/// its bits spread so the set can use it as it is. Each word is multiplied
/// by its own odd key, so the five multiplies do not wait on each other.
fn run_hash(run: &[u64]) -> u64 {
    let folded = run.iter().zip(RUN_KEYS).fold(0_u64, |acc, (word, key)| {
        acc.wrapping_add(word.wrapping_mul(key))
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
        assert!(own.found_in_more_than(&words(theirs), 1));
        assert!(!own.found_in_more_than(&words(theirs), 2));
        assert!(!own.found_in_more_than(&words("z y x w v"), 0));
    }

    #[test]
    fn found_in_never_undercounts_at_any_limit() {
        let own = Shingles::of("q a b c d e f g h i j k l m n o p q");
        let theirs = words("z a b c d e f y g h i j k l x m n o p a b c d e");
        let found = theirs.runs().filter(|run| own.set.contains(run)).count();
        // abcde twice, bcdef, ghijk, hijkl.
        assert_eq!(found, 5);
        for limit in 0..found {
            assert!(own.found_in_more_than(&theirs, limit), "{limit}");
        }
        assert!(!own.found_in_more_than(&theirs, theirs.run_count()));
    }

    #[test]
    fn the_filter_rules_out_unrelated_text() {
        let text = |letter: &str| (0..400).map(|n| format!("{letter}{n}")).collect::<Vec<_>>();
        let own = Shingles::of(&text("w").join(" "));
        let other = text("x").join(" ");
        // Under one bit in ten is set, so far fewer than 100 of 396 runs
        // pass the filter.
        assert!(!own.found_in_more_than(&words(&other), 100));
    }

    #[test]
    fn both_readers_give_the_same_words() {
        let text = "  Internationalisation, of 16 READABILITY-measurements: \
                    abcdefgh abcdefghi a1b2c3d4e5f6g7h8 x";
        let mut ascii = Words::default();
        ascii.read_ascii(text.as_bytes());
        let mut letters = Words::default();
        letters.read_letters(text);
        assert_eq!(ascii.0.len(), 9);
        assert_eq!(ascii.0, letters.0);
    }

    #[test]
    fn long_words_and_ascii_words_in_other_text_hash_the_same() {
        let ascii = words("Internationalisation of readability measurements");
        let mixed = words("internationalisation of readability measurements é");
        assert_eq!(ascii.0, mixed.0.get(..4).unwrap_or_default());
        let shorter = words("internationalisatio measurement");
        assert_ne!(ascii.0.first(), shorter.0.first());
        assert_ne!(ascii.0.last(), shorter.0.last());
    }

    #[test]
    fn a_shorter_text_read_after_a_longer_one_keeps_only_its_words() {
        let mut reused = words("one two three four five six seven eight nine");
        reused.read("ten eleven");
        assert_eq!(reused.0, words("ten eleven").0);
        reused.read("...");
        assert_eq!(reused.0.len(), 0);
        reused.read("Größe x");
        assert_eq!(reused.0.len(), 2);
    }

    #[test]
    fn words_are_not_glued_across_a_break() {
        let split = Shingles::of("ab c d e f");
        let glued = Shingles::of("a bc d e f");
        assert_eq!(split.shared_with(&glued), 0);
    }
}
