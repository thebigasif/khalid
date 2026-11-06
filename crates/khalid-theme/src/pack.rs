//! Template packs: the built-in looks a site chooses from in `[template]`
//! of `site.toml` (templates spec 4.1, ADR "templates are built-in packs").
//!
//! A pack is not a theme a person installs. Every pack is built into the
//! binary. It holds only what it changes from the shared look: its own
//! rules after the shared stylesheet, the template files it replaces, its
//! token defaults, the reader switches it turns on or off, and its
//! options. A site's own `theme/` still wins over all of it.

use std::io::Read as _;
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
    pub(crate) css: PackCss,
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
    pub fn stylesheet(&self) -> &str {
        self.css.text()
    }
}

/// A pack's stylesheet as the binary carries it. The build script minifies
/// each pack's CSS and compresses it with Brotli, because the eleven
/// stylesheets are most of the text built into the binary, and the binary
/// has a size budget. The text is read out the first time a build asks for
/// it and kept, so a site's stylesheet is the same as before.
#[derive(Debug)]
pub(crate) struct PackCss {
    /// The minified CSS, compressed with Brotli. Empty for a pack with no
    /// rules of its own.
    compressed: &'static [u8],
    /// The minified CSS, once read out.
    text: OnceLock<String>,
}

impl PackCss {
    /// The stylesheet whose Brotli bytes are `compressed`.
    pub(crate) const fn compressed(compressed: &'static [u8]) -> Self {
        Self {
            compressed,
            text: OnceLock::new(),
        }
    }

    /// A stylesheet with no rules, for the test build's sampler pack.
    #[cfg(feature = "test-util")]
    pub(crate) const fn empty() -> Self {
        Self::compressed(&[])
    }

    /// The minified CSS. The build script wrote the bytes, so they always
    /// read back; were they ever damaged, the pack would show the shared
    /// look only, which a test of every pack would catch first.
    fn text(&self) -> &str {
        self.text.get_or_init(|| {
            let mut text = String::new();
            if !self.compressed.is_empty()
                && brotli::Decompressor::new(self.compressed, 4096)
                    .read_to_string(&mut text)
                    .is_err()
            {
                text.clear();
            }
            text
        })
    }
}

/// One template file a pack replaces.
#[derive(Debug)]
#[non_exhaustive]
pub struct PackTemplate {
    /// The shared file's name, such as `post.html`.
    pub name: &'static str,
    /// The pack's template source for it.
    pub source: &'static str,
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
    css: PackCss::compressed(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/packs/classic/classic.css.br"
    ))),
    templates: &[],
    tokens: "",
    reader: &[],
    options: &[],
};

/// Every built-in pack, classic first.
#[cfg(not(feature = "test-util"))]
static PACKS: [&Pack; 11] = [
    &CLASSIC,
    &crate::almanac::ALMANAC,
    &crate::atlas::ATLAS,
    &crate::compass::COMPASS,
    &crate::folio::FOLIO,
    &crate::harbor::HARBOR,
    &crate::keystone::KEYSTONE,
    &crate::ledger::LEDGER,
    &crate::meridian::MERIDIAN,
    &crate::wire::WIRE,
    &crate::workbench::WORKBENCH,
];

/// Every built-in pack, classic first, and the throwaway packs the tests
/// switch to.
#[cfg(feature = "test-util")]
static PACKS: [&Pack; 13] = [
    &CLASSIC,
    &crate::almanac::ALMANAC,
    &crate::atlas::ATLAS,
    &crate::compass::COMPASS,
    &crate::folio::FOLIO,
    &crate::harbor::HARBOR,
    &crate::keystone::KEYSTONE,
    &crate::ledger::LEDGER,
    &crate::meridian::MERIDIAN,
    &crate::wire::WIRE,
    &crate::workbench::WORKBENCH,
    &crate::trial::TRIAL,
    &crate::trial::SAMPLER,
];

/// Every built-in pack, classic first.
pub fn packs() -> &'static [&'static Pack] {
    &PACKS
}

/// Classic, the look a site gets without `[template]`.
pub fn classic() -> &'static Pack {
    &CLASSIC
}
