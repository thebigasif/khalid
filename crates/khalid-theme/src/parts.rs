//! The options that switch on a part the shared templates build once
//! (templates spec 4.2), such as the logo in the header. A pack lists the
//! ones it offers, each with its own default; the renderer keeps classic's
//! look for any part a pack leaves out.
//!
//! Each option here starts with classic's value. A pack that wants a part
//! on from the start lists it through [`on`], such as `on(LOGO)`.

use crate::{OptionKind, OptionSpec};

/// The site's logo in the header in place of its name, when the tokens
/// have `[brand] logo`.
pub const LOGO: OptionSpec = OptionSpec {
    key: "logo",
    help: "Whether the header shows the logo from the main site in place of the blog's name.",
    kind: OptionKind::Switch { default: false },
};

/// The trail from the home page to the page, above its content.
pub const BREADCRUMBS: OptionSpec = OptionSpec {
    key: "breadcrumbs",
    help: "Whether each page shows the trail of links from the home page to it.",
    kind: OptionKind::Switch { default: false },
};

/// A post's subtitle under its title, from its `description`.
pub const DEK: OptionSpec = OptionSpec {
    key: "dek",
    help: "Whether a post shows its description as a subtitle under its title.",
    kind: OptionKind::Switch { default: false },
};

/// Why a post was updated, from its `updated_note`, after its dates.
pub const UPDATED_NOTE: OptionSpec = OptionSpec {
    key: "updated_note",
    help: "Whether an updated post says why it was updated, under its dates.",
    kind: OptionKind::Switch { default: false },
};

/// The shape of a list's cards, as the `cards` option names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardShape {
    /// Picture, meta line, title and excerpt: classic's card.
    Image,
    /// Meta line, title and excerpt, no picture.
    Text,
    /// A small picture beside the text.
    Row,
    /// Title and date only.
    Compact,
    /// A numbered list of titles with their meta line.
    Ranked,
}

impl CardShape {
    /// Every shape, in the order the docs list them.
    pub const ALL: [Self; 5] = [
        Self::Image,
        Self::Text,
        Self::Row,
        Self::Compact,
        Self::Ranked,
    ];

    /// The word `[template.options]` and templates use for the shape.
    pub const fn word(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Text => "text",
            Self::Row => "row",
            Self::Compact => "compact",
            Self::Ranked => "ranked",
        }
    }
}

/// The shape of the cards in every list, `shape` unless the site sets one.
pub const fn cards(shape: CardShape) -> OptionSpec {
    OptionSpec {
        key: "cards",
        help: "The shape of the post cards in lists: image, text, row, compact or ranked.",
        kind: OptionKind::Choice {
            allowed: &["image", "text", "row", "compact", "ranked"],
            default: shape.word(),
        },
    }
}

/// A card's reading time after its date.
pub const CARD_READING_TIME: OptionSpec = OptionSpec {
    key: "card_reading_time",
    help: "Whether each post card shows how long the post takes to read.",
    kind: OptionKind::Switch { default: false },
};

/// A card author's role after their name.
pub const CARD_AUTHOR_ROLE: OptionSpec = OptionSpec {
    key: "card_author_role",
    help: "Whether each post card shows its author's role after their name.",
    kind: OptionKind::Switch { default: false },
};

/// The newest featured post above the home page and each list.
pub const FEATURED: OptionSpec = OptionSpec {
    key: "featured",
    help: "Whether the newest post marked featured shows large above the posts, and not again below.",
    kind: OptionKind::Switch { default: false },
};

/// The most editor's picks a site may ask for, the most the home page
/// carries.
const MOST_PICKS: i64 = 8;

/// How many editor's picks the home page shows, `count` unless the site
/// sets it, and kept from 0 for none to eight.
pub const fn picks(count: i64) -> OptionSpec {
    let default = if count > MOST_PICKS {
        MOST_PICKS
    } else if count < 0 {
        0
    } else {
        count
    };
    OptionSpec {
        key: "picks",
        help: "How many posts marked featured the home page lists as editor's picks, from 0 for none to 8.",
        kind: OptionKind::Count {
            least: 0,
            most: MOST_PICKS,
            default,
        },
    }
}

/// The links under a list of posts, as the `pagination` option names them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageLinks {
    /// "Newer posts" and "Older posts" with "Page 2 of 9": classic's.
    Pages,
    /// Those two and a numbered link to each page near this one.
    Numbered,
}

/// The links under a list of posts, `links` unless the site sets them.
pub const fn pagination(links: PageLinks) -> OptionSpec {
    OptionSpec {
        key: "pagination",
        help: "The links under a list of posts: pages for newer and older, or numbered for a link to each page.",
        kind: OptionKind::Choice {
            allowed: &["pages", "numbered"],
            default: match links {
                PageLinks::Pages => "pages",
                PageLinks::Numbered => "numbered",
            },
        },
    }
}

/// How the related posts under a post are laid out, as the `related`
/// option names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelatedLayout {
    /// The list's own cards in a grid: classic's.
    Grid,
    /// Text cards, one under another.
    List,
    /// Cards with a small picture beside the text.
    Row,
    /// Picture cards in one row that scrolls sideways.
    Scroll,
    /// Plain links, one per line.
    Links,
}

/// How the related posts under a post are laid out, `layout` unless the
/// site sets it.
pub const fn related(layout: RelatedLayout) -> OptionSpec {
    OptionSpec {
        key: "related",
        help: "How the related posts under a post are laid out: grid, list, row, scroll or links.",
        kind: OptionKind::Choice {
            allowed: &["grid", "list", "row", "scroll", "links"],
            default: match layout {
                RelatedLayout::Grid => "grid",
                RelatedLayout::List => "list",
                RelatedLayout::Row => "row",
                RelatedLayout::Scroll => "scroll",
                RelatedLayout::Links => "links",
            },
        },
    }
}

/// The fewest and the most related posts a site may ask for.
const RELATED: (i64, i64) = (3, 10);

/// How many related posts show, `count` unless the site sets it, kept
/// from 3 to 10.
pub const fn related_count(count: i64) -> OptionSpec {
    let (least, most) = RELATED;
    let default = if count > most {
        most
    } else if count < least {
        least
    } else {
        count
    };
    OptionSpec {
        key: "related_count",
        help: "How many related posts show under a post, from 3 to 10.",
        kind: OptionKind::Count {
            least,
            most,
            default,
        },
    }
}

/// Where a post's contents go, as the `contents` option names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentsPlacement {
    /// A folded box on phones and a sticky column on the end side of wide
    /// screens: classic's.
    SideEnd,
    /// The same, with the column on the start side.
    SideStart,
    /// An open box after the post's heading, at every width.
    Inline,
    /// A folded box after the post's heading, at every width.
    InlineFolded,
    /// No contents.
    None,
}

/// Where a post's contents go, `placement` unless the site sets it.
pub const fn contents(placement: ContentsPlacement) -> OptionSpec {
    OptionSpec {
        key: "contents",
        help: "Where a long post's contents go: side-end, side-start, inline, inline-folded or none.",
        kind: OptionKind::Choice {
            allowed: &["side-end", "side-start", "inline", "inline-folded", "none"],
            default: match placement {
                ContentsPlacement::SideEnd => "side-end",
                ContentsPlacement::SideStart => "side-start",
                ContentsPlacement::Inline => "inline",
                ContentsPlacement::InlineFolded => "inline-folded",
                ContentsPlacement::None => "none",
            },
        },
    }
}

/// How the contents are numbered, as the `contents_numbers` option names
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentsNumbers {
    /// 1, 2, 3 for the sections and again for each section's parts:
    /// classic's.
    Numbered,
    /// 1, 1.1, 1.2, 2 for the sections and their parts.
    Nested,
    /// No numbers.
    None,
}

/// How the contents are numbered, `numbers` unless the site sets it.
pub const fn contents_numbers(numbers: ContentsNumbers) -> OptionSpec {
    OptionSpec {
        key: "contents_numbers",
        help: "How a post's contents are numbered: numbered, nested for 1.1, or none.",
        kind: OptionKind::Choice {
            allowed: &["numbered", "nested", "none"],
            default: match numbers {
                ContentsNumbers::Numbered => "numbered",
                ContentsNumbers::Nested => "nested",
                ContentsNumbers::None => "none",
            },
        },
    }
}

/// `spec` turned on by default, when it is an on or off option. Any other
/// option comes back as it is.
pub const fn on(spec: OptionSpec) -> OptionSpec {
    match spec.kind {
        OptionKind::Switch { .. } => OptionSpec {
            kind: OptionKind::Switch { default: true },
            ..spec
        },
        _ => spec,
    }
}

/// Every part's option with classic's value, so a test can hold this list
/// and the renderer's to the same keys.
pub fn all() -> Vec<OptionSpec> {
    vec![
        LOGO,
        BREADCRUMBS,
        DEK,
        UPDATED_NOTE,
        cards(CardShape::Image),
        CARD_READING_TIME,
        CARD_AUTHOR_ROLE,
        FEATURED,
        picks(0),
        pagination(PageLinks::Pages),
        related(RelatedLayout::Grid),
        related_count(5),
        contents(ContentsPlacement::SideEnd),
        contents_numbers(ContentsNumbers::Numbered),
    ]
}
