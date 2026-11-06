//! Dope: a blog arranged by topic, based on Ghost's Dope theme (MIT
//! licence, see `packs/dope/LICENSE-ghost.txt`). The front page is a
//! sideways shelf of topic tiles as tall as the window, each list a column
//! of wide rows with a large picture beside each summary, and a post opens
//! on its picture from edge to edge.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, PackTemplate, PackText, slot};

/// The Dope pack.
pub(crate) static DOPE: Pack = Pack {
    name: "dope",
    summary: "A blog arranged by topic: a sideways shelf of tall topic tiles on the front page, wide rows with a large picture beside each summary, and posts that open on their picture from edge to edge.",
    css: PackText::file("dope/dope.css"),
    templates: &[
        PackTemplate::file("home.html", "dope/home.html"),
        PackTemplate::file("post.html", "dope/post.html"),
    ],
    tokens: include_str!("../packs/dope/tokens.toml"),
    reader: &[
        (ReaderSwitch::ReadingTime, true),
        (ReaderSwitch::AuthorBox, true),
    ],
    options: &[
        parts::on(parts::LOGO),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::on(parts::CARD_READING_TIME),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::None),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        parts::related(RelatedLayout::Row),
        parts::related_count(3),
        slot::share("end"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("post"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
