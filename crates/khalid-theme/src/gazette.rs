//! Gazette: a local news site with many sections (docs/templates/gazette.md).
//! A section is a category, and a story's place is its first fact. The
//! front page opens with the edition bar and a jump link per section,
//! then the top story beside a rail of the latest news grouped by day,
//! and a block per section under its flag: the section's name in reverse
//! on a heavy ink rule, the newest story with its picture and dense
//! headlines with their places. Built on the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Gazette pack.
pub(crate) static GAZETTE: Pack = Pack {
    name: "gazette",
    summary: "For a local news site with many sections: a block per section under its flag, a rail of the latest news grouped by day, dense headlines that each carry their place, and a dateline on every story.",
    css: PackText::file("gazette/gazette.css"),
    templates: &[
        PackTemplate::file("home.html", "gazette/home.html"),
        PackTemplate::file("post.html", "gazette/post.html"),
        PackTemplate::file("listing.html", "gazette/listing.html"),
        PackTemplate::file("authors.html", "gazette/authors.html"),
        PackTemplate::file("author.html", "gazette/author.html"),
        PackTemplate::file("parts-gazette.html", "gazette/parts-gazette.html"),
    ],
    tokens: include_str!("../packs/gazette/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::FEATURED),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["facebook", "bluesky", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line in the edition bar at the top of the front page, such as the towns the paper covers.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "rail_count",
            help: "How many of the newest stories the latest news rail on the front page lists.",
            kind: OptionKind::Count {
                least: 4,
                most: 20,
                default: 10,
            },
        },
    ],
};
