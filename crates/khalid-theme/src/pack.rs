//! Template packs: the built-in looks a site chooses from in `[template]`
//! of `site.toml` (templates spec 4.1, ADR "templates are built-in packs").
//!
//! A pack is not a theme a person installs. Every pack is built into the
//! binary. It holds only what it changes from the shared look: its own
//! rules after the shared stylesheet, the template files it replaces, its
//! token defaults, the reader switches it turns on or off, and its
//! options. A site's own `theme/` still wins over all of it.

use std::collections::BTreeMap;
use std::io::Read as _;
use std::ops::Range;
use std::sync::OnceLock;

use khalid_core::{PackName, ReaderSwitch};

/// One built-in look.
#[derive(Debug)]
#[non_exhaustive]
pub struct Pack {
    /// The name `[template]` uses, such as `classic`.
    pub name: &'static str,
    /// One plain sentence that says what the look is for.
    pub summary: &'static str,
    /// The pack's own CSS rules, added after the shared ones. A site ships
    /// the shared rules and its own pack's rules, and no other pack's.
    /// Read them with [`Pack::stylesheet`].
    pub(crate) css: PackText,
    /// The template files the pack replaces, by the shared file's name.
    /// Any other file comes from the shared set.
    pub templates: &'static [PackTemplate],
    /// The pack's defaults as the text of a `theme/tokens.toml`, such as
    /// its type scale. A value in the site's own file wins.
    pub tokens: &'static str,
    /// The reader switches the pack turns on or off. A key the site writes
    /// in `[reader]` wins.
    pub reader: &'static [(ReaderSwitch, bool)],
    /// Every option `[template.options]` may set for this pack.
    pub options: &'static [OptionSpec],
}

impl Pack {
    /// The pack's own CSS rules, as the build script minified them.
    pub fn stylesheet(&self) -> &'static str {
        self.css.text()
    }
}

/// Every pack's stylesheet and template files in one text, compressed
/// with Brotli by the build script. One archive for all packs is much
/// smaller than one per file, because the packs share many patterns, and
/// the binary has a size budget.
static ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/packs.br"));

/// Where each file starts and ends in the archive's text: one line per
/// file, `<pack>/<file>`, its start and its end, split by tabs.
static INDEX: &str = include_str!(concat!(env!("OUT_DIR"), "/packs.index"));

/// The archive, read out the first time a build asks for a pack's file and
/// kept, so every site is the same as before.
#[derive(Debug, Default)]
struct Archive {
    /// Every file's text, one after the other.
    text: String,
    /// Each file's place in `text`, by `<pack>/<file>`.
    files: BTreeMap<&'static str, Range<usize>>,
}

impl Archive {
    /// The archive the build script wrote. Were its bytes ever damaged,
    /// every pack would fall back to the shared look and files, which a
    /// test of every pack would catch first.
    fn read() -> Self {
        let mut text = String::new();
        if brotli::Decompressor::new(ARCHIVE, 4096)
            .read_to_string(&mut text)
            .is_err()
        {
            return Self::default();
        }
        let files = INDEX
            .lines()
            .filter_map(|line| {
                let mut parts = line.split('\t');
                let name = parts.next()?;
                let start = parts.next()?.parse().ok()?;
                let end = parts.next()?.parse().ok()?;
                Some((name, start..end))
            })
            .collect();
        Self { text, files }
    }

    /// The text of `file`, or nothing when the archive has no such file.
    fn file(&self, file: &str) -> &str {
        self.files
            .get(file)
            .and_then(|place| self.text.get(place.clone()))
            .unwrap_or_default()
    }
}

/// The archive, once read out.
fn archive() -> &'static Archive {
    static ARCHIVE_TEXT: OnceLock<Archive> = OnceLock::new();
    ARCHIVE_TEXT.get_or_init(Archive::read)
}

/// A pack's stylesheet or template file, by its place in the archive.
#[derive(Debug)]
pub(crate) struct PackText {
    /// The file, as `<pack>/<file>`, such as `harbor/harbor.css`. Empty
    /// for a pack with no rules of its own.
    file: &'static str,
}

impl PackText {
    /// The file `file`, as `<pack>/<file>` under `packs/`.
    pub(crate) const fn file(file: &'static str) -> Self {
        Self { file }
    }

    /// A stylesheet with no rules, for the test build's sampler pack.
    #[cfg(feature = "test-util")]
    pub(crate) const fn empty() -> Self {
        Self::file("")
    }

    /// The text: a stylesheet minified, a template file as written.
    fn text(&self) -> &'static str {
        archive().file(self.file)
    }
}

/// One template file a pack replaces.
#[derive(Debug)]
#[non_exhaustive]
pub struct PackTemplate {
    /// The shared file's name, such as `post.html`.
    pub name: &'static str,
    /// The pack's template source for it. Read it with
    /// [`PackTemplate::source`].
    text: PackText,
}

impl PackTemplate {
    /// The file `name`, whose source is `file` (`<pack>/<file>`) in the
    /// archive.
    pub(crate) const fn file(name: &'static str, file: &'static str) -> Self {
        Self {
            name,
            text: PackText::file(file),
        }
    }

    /// The pack's template source for the file.
    pub fn source(&self) -> &'static str {
        self.text.text()
    }
}

/// One option of a pack: its key, what it does, and the values it takes.
#[derive(Debug)]
#[non_exhaustive]
pub struct OptionSpec {
    /// The key in `[template.options]`, such as `lead_image`.
    pub key: &'static str,
    /// One plain sentence that says what the option changes.
    pub help: &'static str,
    /// The values it takes, and its default.
    pub kind: OptionKind,
}

/// The values one option takes, and the one it has when the site does not
/// set it. Templates read every option, so each has a default. The enum is
/// exhaustive on purpose, like [`crate::CheckedOption`]: the Admin API's
/// list matches it, so a new kind stops the build until it is described.
#[derive(Debug, Clone, Copy)]
pub enum OptionKind {
    /// `true` or `false`.
    Switch {
        /// The value when not set.
        default: bool,
    },
    /// One word from a fixed list, such as a contents placement.
    Choice {
        /// Every word allowed, in the order the docs list them.
        allowed: &'static [&'static str],
        /// The word when not set.
        default: &'static str,
    },
    /// A whole number in a range, such as how many related posts.
    Count {
        /// The smallest allowed.
        least: i64,
        /// The largest allowed.
        most: i64,
        /// The number when not set.
        default: i64,
    },
    /// One line of text, such as a tagline, empty when not set.
    Line {
        /// The most characters allowed.
        most: usize,
    },
    /// A short list of lines of text, empty when not set.
    Lines {
        /// The most lines allowed.
        most: usize,
    },
    /// One or more words from a fixed list, each once, in the order
    /// written, such as the networks a share row links to.
    Picks {
        /// Every word allowed, in the order the docs list them.
        allowed: &'static [&'static str],
        /// The words when not set.
        default: &'static [&'static str],
    },
}

/// Classic: the look every site had before packs, and every site's default.
static CLASSIC: Pack = Pack {
    name: PackName::CLASSIC,
    summary: "The default look: a lead story and a grid of cards on list pages, a readable column for posts and the contents beside long posts.",
    css: PackText::file("classic/classic.css"),
    templates: &[],
    tokens: "",
    reader: &[],
    options: &[],
};

/// Every built-in pack, classic first, then the rest by name, and with the
/// `test-util` feature the throwaway packs the tests switch to. A slice, so
/// a new pack is one line here and no count changes.
static PACKS: &[&Pack] = &[
    &CLASSIC,
    &crate::almanac::ALMANAC,
    &crate::alto::ALTO,
    &crate::atlas::ATLAS,
    &crate::bulletin::BULLETIN,
    &crate::casper::CASPER,
    &crate::compass::COMPASS,
    &crate::dawn::DAWN,
    &crate::digest::DIGEST,
    &crate::dope::DOPE,
    &crate::ease::EASE,
    &crate::edge::EDGE,
    &crate::edition::EDITION,
    &crate::episode::EPISODE,
    &crate::folio::FOLIO,
    &crate::harbor::HARBOR,
    &crate::headline::HEADLINE,
    &crate::journal::JOURNAL,
    &crate::keystone::KEYSTONE,
    &crate::ledger::LEDGER,
    &crate::london::LONDON,
    &crate::meridian::MERIDIAN,
    &crate::research::RESEARCH,
    &crate::ruby::RUBY,
    &crate::respawn::RESPAWN,
    &crate::solo::SOLO,
    &crate::source::SOURCE,
    &crate::wave::WAVE,
    &crate::taste::TASTE,
    &crate::wire::WIRE,
    &crate::workbench::WORKBENCH,
    #[cfg(feature = "test-util")]
    &crate::trial::TRIAL,
    #[cfg(feature = "test-util")]
    &crate::trial::SAMPLER,
];

/// Every built-in pack, classic first.
pub fn packs() -> &'static [&'static Pack] {
    PACKS
}

/// Classic, the look a site gets without `[template]`.
pub fn classic() -> &'static Pack {
    &CLASSIC
}
