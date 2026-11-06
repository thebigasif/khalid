//! Meridian: a calm company journal, with a reference variant for long
//! evergreen explainers (templates spec 3.9).

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, PackTemplate, PackText, slot};

/// The template files the meridian pack replaces.
static MERIDIAN_TEMPLATES: [PackTemplate; 2] = [
    PackTemplate::compressed(
        "home.html",
        include_bytes!(concat!(env!("OUT_DIR"), "/packs/meridian/home.html.br")),
    ),
    PackTemplate::compressed(
        "post-reference.html",
        include_bytes!(concat!(
            env!("OUT_DIR"),
            "/packs/meridian/post-reference.html.br"
        )),
    ),
];

/// The Meridian pack.
pub(crate) static MERIDIAN: Pack = Pack {
    name: "meridian",
    summary: "A calm company journal: serif titles, a lead story, category tabs, wide rows with a picture, several authors with their roles, and a reference layout for long guides.",
    css: PackText::compressed(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/packs/meridian/meridian.css.br"
    ))),
    templates: &MERIDIAN_TEMPLATES,
    tokens: include_str!("../packs/meridian/tokens.toml"),
    reader: &[(ReaderSwitch::ReadingTime, false)],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::DEK,
        parts::cards(CardShape::Row),
        parts::on(parts::CARD_AUTHOR_ROLE),
        parts::on(parts::CATEGORY_TABS),
        parts::picks(3),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Scroll),
        parts::related_count(3),
        parts::contents(ContentsPlacement::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Nested),
        slot::share("none"),
        slot::share_networks(&["linkedin", "x", "email"]),
        slot::newsletter("both"),
        slot::cta_band("post"),
        slot::CTA_BUTTON_ON,
        parts::HERO_STATS,
        parts::on(parts::TOPIC_TABS),
        parts::on(parts::VIDEO_SECTION),
        parts::on(parts::PODCAST_SECTION),
    ],
};
