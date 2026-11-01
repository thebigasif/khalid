//! Ledger: a calm, text-led look for teams that publish studies and long
//! how-tos (templates spec 3.1). Lists carry no pictures, so the words and
//! the topics lead.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, slot};

/// The longest tagline, about one line on a phone.
const TAGLINE_CHARACTERS: usize = 140;

/// The Ledger pack.
pub(crate) static LEDGER: Pack = Pack {
    name: "ledger",
    summary: "A calm, text-led blog with no pictures in its lists, for studies and long how-tos.",
    stylesheet: include_str!("../packs/ledger/ledger.css"),
    templates: &[],
    tokens: include_str!("../packs/ledger/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "tagline",
            help: "One line under the blog's name on the front page, such as what the blog is about.",
            kind: OptionKind::Line {
                most: TAGLINE_CHARACTERS,
            },
        },
        OptionSpec {
            key: "stats",
            help: "Up to three short facts beside the tagline, such as 12,000 readers.",
            kind: OptionKind::Lines { most: 3 },
        },
        OptionSpec {
            key: "lead_image",
            help: "Whether a post shows its lead image under its title.",
            kind: OptionKind::Switch { default: false },
        },
        parts::on(parts::LOGO),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::CARD_READING_TIME,
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::Inline),
        parts::contents_numbers(ContentsNumbers::None),
        parts::related(RelatedLayout::List),
        parts::related_count(5),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
    ],
};
