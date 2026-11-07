//! Edge: a picture-led site for photographers, illustrators and studios,
//! based on Ghost's Edge theme (MIT licence, see `packs/edge/LICENSE-ghost.txt`).
//! The front page is a wall of lead pictures with no text between them,
//! each keeping its own shape, and a post opens with its picture wide
//! above a short title and one calm reading column. Ghost's masonry
//! script is a CSS grid here: each picture spans rows in proportion to
//! its height, so the wall reads across in date order with no script.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Edge pack.
pub(crate) static EDGE: Pack = Pack {
    name: "edge",
    summary: "A picture-led site for creative work: a wall of lead pictures on the front page, and posts that open with a wide picture above one calm reading column.",
    css: PackText::file("edge/edge.css"),
    templates: &[
        PackTemplate::file("home.html", "edge/home.html"),
        PackTemplate::file("post.html", "edge/post.html"),
    ],
    tokens: include_str!("../packs/edge/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "captions",
            help: "Whether the picture wall shows each post's title, category and date under its picture (below) or shows the pictures alone (hidden). Screen readers hear the titles either way.",
            kind: OptionKind::Choice {
                allowed: &["hidden", "below"],
                default: "hidden",
            },
        },
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(6),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("top"),
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
