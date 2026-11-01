//! Wire: a news and columns site (templates spec 3.6). A lead story beside
//! tabs of the latest posts and the editor's picks, a fast one-column
//! feed, and posts that open with a subtitle and the writer's card.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Wire pack.
pub(crate) static WIRE: Pack = Pack {
    name: "wire",
    summary: "A news and columns site: a lead story beside tabs of the latest posts and the editor's picks, a fast one-column feed, and posts that open with a subtitle and the writer's card.",
    stylesheet: include_str!("../packs/wire/wire.css"),
    templates: &[PackTemplate {
        name: "home.html",
        source: include_str!("../packs/wire/home.html"),
    }],
    tokens: include_str!("../packs/wire/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::on(parts::CARD_READING_TIME),
        parts::CARD_AUTHOR_ROLE,
        parts::on(parts::FEATURED),
        parts::picks(5),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Row),
        parts::related_count(3),
        parts::contents(ContentsPlacement::None),
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "newsletter_after",
            help: "After how many posts in the front page's feed the newsletter form shows.",
            kind: OptionKind::Count {
                least: 2,
                most: 20,
                default: 8,
            },
        },
        OptionSpec {
            key: "author_card",
            help: "Whether a post shows its writer's card, with a bio that folds open, right after the picture.",
            kind: OptionKind::Switch { default: true },
        },
    ],
};
