//! Episode: a podcast's blog, based on Ghost's Episode theme (MIT, see
//! `packs/episode/LICENSE-ghost.txt`). A dark band with the show's name
//! and the places to listen, the newest episode with its player, then
//! each episode as a boxed row with a square picture and a play mark.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The most characters the tagline under the show's name may have: two
/// lines of the band on a phone.
const TAGLINE_CHARACTERS: usize = 140;

/// The Episode pack.
pub(crate) static EPISODE: Pack = Pack {
    name: "episode",
    summary: "A podcast's blog: a dark band with the show's name and where to listen, the newest episode with its player, then every episode as a boxed row with a square picture.",
    css: PackText::file("episode/episode.css"),
    templates: &[
        PackTemplate::file("home.html", "episode/home.html"),
        PackTemplate::file("post.html", "episode/post.html"),
    ],
    tokens: include_str!("../packs/episode/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "tagline",
            help: "One line under the show's name on the front page, such as what the podcast is about.",
            kind: OptionKind::Line {
                most: TAGLINE_CHARACTERS,
            },
        },
        OptionSpec {
            key: "listen_on",
            help: "Up to four places to listen, shown under the front page's title, each written name | address, such as Spotify | https://open.spotify.com/show/1.",
            kind: OptionKind::Lines { most: 4 },
        },
        parts::on(parts::PODCAST_SECTION),
        OptionSpec {
            key: "lead_image",
            help: "Whether a post shows its lead image under its title. Off, the sound player is the first thing after the title.",
            kind: OptionKind::Switch { default: false },
        },
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
