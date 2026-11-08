//! Mosaic: a photo blog laid out in tiles (docs/templates/mosaic.md). The
//! front page, every list and the foot of each post show the posts as
//! tiles in a rhythm of sizes: on a wide screen a pattern of nine, a large
//! tile beside a tall one, three equal tiles, two halves, then a tall tile
//! beside a large one. Each picture is cut to its tile, and its words sit
//! under it, never on it. A post opens with its picture whole, then the
//! title row and the post's facts under it.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Mosaic pack.
pub(crate) static MOSAIC: Pack = Pack {
    name: "mosaic",
    summary: "A photo blog laid out in tiles: the posts in a rhythm of large, tall and small pictures, each cut to its tile with the place, date and title under it, never on it, and posts that open with the picture whole.",
    css: PackText::file("mosaic/mosaic.css"),
    templates: &[
        PackTemplate::file("home.html", "mosaic/home.html"),
        PackTemplate::file("post.html", "mosaic/post.html"),
        PackTemplate::file("listing.html", "mosaic/listing.html"),
        PackTemplate::file("authors.html", "mosaic/authors.html"),
        PackTemplate::file("author.html", "mosaic/author.html"),
        PackTemplate::file("parts-mosaic.html", "mosaic/parts-mosaic.html"),
    ],
    tokens: PackText::file("mosaic/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line beside the site's name at the top of the front page, such as what the pictures show and where.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "tile_summaries",
            help: "Whether the larger tiles show the post's summary under the title. The small tiles always show only the place, date and title.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "photo_facts",
            help: "Whether a post shows its facts beside the title, as short label and value pairs, such as where and when the picture was taken.",
            kind: OptionKind::Switch { default: true },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
