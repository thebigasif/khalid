//! London: a picture-led blog or portfolio, based on Ghost's London theme
//! (MIT licence, see `packs/london/LICENSE-ghost.txt`). The front page opens
//! with the blog's description set large in the middle, then every post as
//! a tall picture tile from edge to edge: two side by side, then one across
//! the whole width. A post without a picture becomes a dark tile with its
//! number in large faint type. A post reads in one centred column.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The London pack.
pub(crate) static LONDON: Pack = Pack {
    name: "london",
    summary: "A picture-led blog or portfolio: the blog's description set large in the middle, then tall picture tiles from edge to edge, two side by side and every third across the whole width, and a centred reading column.",
    css: PackText::file("london/london.css"),
    templates: &[
        PackTemplate::file("home.html", "london/home.html"),
        PackTemplate::file("post.html", "london/post.html"),
        PackTemplate::file("listing.html", "london/listing.html"),
        PackTemplate::file("author.html", "london/author.html"),
    ],
    tokens: include_str!("../packs/london/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "statement",
            help: "Whether the front page opens with your blog's description set large in the middle of the page.",
            kind: OptionKind::Switch { default: true },
        },
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::CARD_READING_TIME,
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::None),
        parts::contents_numbers(ContentsNumbers::None),
        parts::HEADER_SEARCH,
        parts::related(RelatedLayout::Grid),
        parts::related_count(3),
        slot::share("end"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("post"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
