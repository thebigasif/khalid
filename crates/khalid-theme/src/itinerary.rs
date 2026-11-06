//! Itinerary: a travel diary of trips, filed by country
//! (docs/templates/itinerary.md). A trip is a post with facts (days,
//! route, when, cost), drawn as a ticket in every list and as a boarding
//! pass on its own page, where each main heading is a day on a dashed
//! route. Countries are categories, shown as passport stamps on the front
//! page. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Itinerary pack.
pub(crate) static ITINERARY: Pack = Pack {
    name: "itinerary",
    summary: "For a travel diary: trips filed by country as passport stamps, each trip a ticket with its days, route, season and cost, a day-by-day route on every trip and checklists for packing.",
    css: PackText::file("itinerary/itinerary.css"),
    templates: &[
        PackTemplate::file("home.html", "itinerary/home.html"),
        PackTemplate::file("post.html", "itinerary/post.html"),
        PackTemplate::file("listing.html", "itinerary/listing.html"),
        PackTemplate::file("authors.html", "itinerary/authors.html"),
        PackTemplate::file("author.html", "itinerary/author.html"),
    ],
    tokens: include_str!("../packs/itinerary/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::SideEnd),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["whatsapp", "facebook", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the diary's name at the top of the front page, such as where you travel and how.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "notes_count",
            help: "How many notes, posts without facts such as packing lists and planning tips, show under the trips on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 5,
            },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
