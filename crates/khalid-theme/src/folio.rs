//! Folio: a product team's essays and engineering notes under the blog's
//! own masthead (templates spec 3.10).

use crate::parts::{self, CardShape, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Folio pack.
pub(crate) static FOLIO: Pack = Pack {
    name: "folio",
    summary: "A product team's notebook under a large masthead on ruled paper: a featured story, picture cards, and notes with a contents rail, framed code and an author box.",
    css: PackText::file("folio/folio.css"),
    templates: &[
        PackTemplate::file("home.html", "folio/home.html"),
        PackTemplate::file("post.html", "folio/post.html"),
    ],
    tokens: include_str!("../packs/folio/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "masthead_name",
            help: "The name the masthead shows, one line; the blog's name when not set.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "tagline",
            help: "One line under the masthead's name.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "quote",
            help: "A quote centred under the cards: its words, then who said them.",
            kind: OptionKind::Lines { most: 2 },
        },
        parts::on(parts::LOGO),
        parts::BACK_LINK,
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::on(parts::CATEGORY_TABS),
        parts::cards(CardShape::Image),
        parts::CARD_AUTHOR_ROLE,
        parts::on(parts::FEATURED),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(3),
        parts::contents(ContentsPlacement::SideEnd),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("end"),
        slot::share_networks(&["x", "linkedin", "bluesky", "email"]),
        slot::newsletter("none"),
        slot::cta_band("post"),
        slot::CTA_BUTTON_ON,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::on(parts::VIDEO_SECTION),
        parts::on(parts::PODCAST_SECTION),
    ],
};
