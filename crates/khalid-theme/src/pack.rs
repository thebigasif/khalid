//! Template packs: the built-in looks a site chooses from in `[template]`
//! of `site.toml` (templates spec 4.1, ADR "templates are built-in packs").
//!
//! A pack is not a theme a person installs. Every pack is built into the
//! binary. It holds only what it changes from the shared look: its own
//! rules after the shared stylesheet, the template files it replaces, its
//! token defaults, the reader switches it turns on or off, and its
//! options. A site's own `theme/` still wins over all of it.

use khalid_core::{PackName, ReaderSwitch};

use crate::almanac::ALMANAC;
use crate::workbench::WORKBENCH;

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
    pub stylesheet: &'static str,
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
/// set it. Templates read every option, so each has a default.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
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
    summary: "Today's look: a calm column of text, cards on the home page and the contents beside long posts.",
    stylesheet: "",
    templates: &[],
    tokens: "",
    reader: &[],
    options: &[],
};

/// Every built-in pack, classic first.
#[cfg(not(feature = "test-util"))]
static PACKS: [&Pack; 3] = [&CLASSIC, &WORKBENCH, &ALMANAC];

/// Every built-in pack, classic first, and the throwaway packs the tests
/// switch to.
#[cfg(feature = "test-util")]
static PACKS: [&Pack; 5] = [
    &CLASSIC,
    &WORKBENCH,
    &ALMANAC,
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
