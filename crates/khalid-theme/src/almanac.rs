//! Almanac: a resource library of long guides (templates spec 3.8). A
//! row of category links, a featured guide and a call to action, the
//! newest posts as short rows, a grid of guides beside the editor's
//! picks, and posts that open with an "On this page" box and end with two
//! sets of further reading.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Almanac pack.
pub(crate) static ALMANAC: Pack = Pack {
    name: "almanac",
    summary: "For a library of long guides: category links, a featured guide, a grid of guides beside the editor's picks, and posts with an \"On this page\" box and two sets of further reading.",
    css: PackText::compressed(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/packs/almanac/almanac.css.br"
    ))),
    templates: &[
        PackTemplate::compressed(
            "home.html",
            include_bytes!(concat!(env!("OUT_DIR"), "/packs/almanac/home.html.br")),
        ),
        PackTemplate::compressed(
            "post.html",
            include_bytes!(concat!(env!("OUT_DIR"), "/packs/almanac/post.html.br")),
        ),
    ],
    tokens: include_str!("../packs/almanac/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::on(parts::UPDATED_NOTE),
        parts::cards(CardShape::Image),
        parts::CARD_READING_TIME,
        parts::CARD_AUTHOR_ROLE,
        parts::on(parts::FEATURED),
        parts::picks(5),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(10),
        parts::contents(ContentsPlacement::SideEnd),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["x", "linkedin", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("both"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "lead_image",
            help: "Whether a post shows its lead picture, wide, under its byline.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "byline_end",
            help: "Whether a post ends with its writers again and when it was last updated.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "newsletter_note",
            help: "One line under the newsletter form, such as what readers get and how you treat their address.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::on(parts::VIDEO_SECTION),
        parts::on(parts::PODCAST_SECTION),
    ],
};
