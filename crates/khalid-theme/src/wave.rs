//! Wave: an interview show's blog, based on Ghost's Wave theme (MIT, see
//! `packs/wave/LICENSE-ghost.txt`). A quiet white page in one narrow
//! column: the show's name and the places to listen, the newest episode
//! in a slim player card, then every episode as a row with square
//! artwork and a round play mark in the accent.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Wave pack.
pub(crate) static WAVE: Pack = Pack {
    name: "wave",
    summary: "An interview show's blog: the newest episode in a slim player card, then every episode as a row with square artwork and a round play mark.",
    css: PackText::file("wave/wave.css"),
    templates: &[
        PackTemplate::file("home.html", "wave/home.html"),
        PackTemplate::file("post.html", "wave/post.html"),
    ],
    tokens: include_str!("../packs/wave/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "listen_on",
            help: "Up to four places to listen, shown under the front page's title, each written name | address, such as Spotify | https://open.spotify.com/show/1.",
            kind: OptionKind::Lines { most: 4 },
        },
        parts::on(parts::PODCAST_SECTION),
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::CARD_READING_TIME,
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Row),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("end"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
    ],
};
