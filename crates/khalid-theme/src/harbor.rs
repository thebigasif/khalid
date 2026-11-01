//! Harbor: a busy marketing blog (templates spec 3.5). A featured post with
//! three more beside it, rows of recent posts, an offer in each post and a
//! sideways row of related reading.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Harbor pack.
pub(crate) static HARBOR: Pack = Pack {
    name: "harbor",
    summary: "A busy marketing blog: a featured post with three more beside it, rows of recent posts, an offer in each post and a sideways row of related reading.",
    stylesheet: include_str!("../packs/harbor/harbor.css"),
    templates: &[PackTemplate {
        name: "home.html",
        source: include_str!("../packs/harbor/home.html"),
    }],
    tokens: include_str!("../packs/harbor/tokens.toml"),
    reader: &[
        (ReaderSwitch::ReadingTime, false),
        (ReaderSwitch::AuthorBox, false),
    ],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::CARD_READING_TIME,
        parts::on(parts::FEATURED),
        parts::picks(3),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Scroll),
        parts::related_count(10),
        parts::contents(ContentsPlacement::Inline),
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["linkedin", "x", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "offer_card",
            help: "Whether each post shows your [template.cta] as an offer card under the author's name.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "section_marks",
            help: "Whether a small drawn mark in your accent colour sits above each section heading of a post.",
            kind: OptionKind::Switch { default: true },
        },
    ],
};
