//! Harbor: a food and recipe blog (templates spec 3.5). A featured post with
//! a column of smaller posts beside it, a grid of picture cards, an offer
//! in each post and a sideways row of related reading.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Harbor pack.
pub(crate) static HARBOR: Pack = Pack {
    name: "harbor",
    summary: "A food and recipe blog: a featured post with smaller posts beside it, a strip of recipe cards with time and serving tags, a grid of picture cards, an offer in each post and a sideways row of related reading.",
    css: PackText::compressed(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/packs/harbor/harbor.css.br"
    ))),
    templates: &[
        PackTemplate::compressed(
            "home.html",
            include_bytes!(concat!(env!("OUT_DIR"), "/packs/harbor/home.html.br")),
        ),
        PackTemplate::compressed(
            "post.html",
            include_bytes!(concat!(env!("OUT_DIR"), "/packs/harbor/post.html.br")),
        ),
    ],
    tokens: include_str!("../packs/harbor/tokens.toml"),
    reader: &[
        (ReaderSwitch::ReadingTime, true),
        (ReaderSwitch::AuthorBox, true),
    ],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::CARD_READING_TIME,
        parts::on(parts::FEATURED),
        parts::picks(3),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Scroll),
        parts::related_count(10),
        parts::contents(ContentsPlacement::Inline),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
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
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::on(parts::VIDEO_SECTION),
        parts::on(parts::PODCAST_SECTION),
    ],
};
