//! Bulletin: a newsletter-first blog, based on Ghost's Bulletin theme (MIT
//! licence, see `packs/bulletin/LICENSE-ghost.txt`). The front page splits
//! in two: a sign-up panel in the accent colour beside the latest issue.
//! Every other issue follows as a ruled list, and a post reads as one
//! centred column.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Bulletin pack.
pub(crate) static BULLETIN: Pack = Pack {
    name: "bulletin",
    summary: "A newsletter-first blog: a sign-up panel in your accent colour beside the latest issue, a ruled list of every other issue and a centred reading column.",
    css: PackText::file("bulletin/bulletin.css"),
    templates: &[
        PackTemplate::file("home.html", "bulletin/home.html"),
        PackTemplate::file("post.html", "bulletin/post.html"),
    ],
    tokens: include_str!("../packs/bulletin/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "image_width",
            help: "How wide a post's lead image runs: small keeps it to the text, wide runs it wider, full runs it from edge to edge.",
            kind: OptionKind::Choice {
                allowed: &["small", "wide", "full"],
                default: "small",
            },
        },
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::CARD_READING_TIME,
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::None),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        parts::related(RelatedLayout::List),
        parts::related_count(3),
        slot::share("top"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
