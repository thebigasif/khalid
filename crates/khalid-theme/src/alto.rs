//! Alto: a clean, minimal blog, based on Ghost's Alto theme (MIT licence,
//! see `packs/alto/LICENSE-ghost.txt`). A strip of square featured cards
//! that scrolls sideways, then one post under another with a wide picture,
//! and a post with a centred header, one calm column and the author box.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, PackTemplate, PackText, slot};

/// The Alto pack.
pub(crate) static ALTO: Pack = Pack {
    name: "alto",
    summary: "A clean, minimal blog based on Ghost's Alto theme: a sideways strip of featured posts, then one post under another with a wide picture, and a calm column for each post.",
    css: PackText::file("alto/alto.css"),
    templates: &[
        PackTemplate::file("home.html", "alto/home.html"),
        PackTemplate::file("post.html", "alto/post.html"),
    ],
    tokens: include_str!("../packs/alto/tokens.toml"),
    reader: &[
        (ReaderSwitch::ReadingTime, true),
        (ReaderSwitch::AuthorBox, true),
    ],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::DEK,
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::CARD_READING_TIME),
        parts::picks(8),
        parts::pagination(PageLinks::Pages),
        parts::related(RelatedLayout::Grid),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("end"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("none"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
