//! Atlas: a portal look for a blog with many topics (templates spec 3.2).
//! A bar of topic tabs, featured posts and editor's picks lead the front
//! page; posts open with breadcrumbs, share links and their contents in a
//! sticky column.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, slot};

/// The Atlas pack.
pub(crate) static ATLAS: Pack = Pack {
    name: "atlas",
    summary: "A portal for a blog with many topics: topic tabs, featured posts, editor's picks and breadcrumbs.",
    stylesheet: include_str!("../packs/atlas/atlas.css"),
    templates: &[],
    tokens: include_str!("../packs/atlas/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::FEATURED),
        parts::picks(4),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::on(parts::CARD_READING_TIME),
        parts::CARD_AUTHOR_ROLE,
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::SideStart),
        parts::contents_numbers(ContentsNumbers::Numbered),
        parts::related(RelatedLayout::List),
        parts::related_count(3),
        slot::share("both"),
        slot::share_networks(&["x", "linkedin", "facebook", "email"]),
        slot::newsletter("none"),
        slot::cta_band("both"),
        slot::CTA_BUTTON_ON,
    ],
};
