//! Plinth: an architecture or interiors practice (docs/templates/plinth.md).
//! Projects are posts with facts (place, year, area, stage), shown on the
//! front page as spreads: the photograph on a heavy ink bar beside a
//! numbered title block of the facts. Figures in a project, such as plans,
//! are framed as numbered drawing sheets. Built from the shared section
//! parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Plinth pack.
pub(crate) static PLINTH: Pack = Pack {
    name: "plinth",
    summary: "For an architecture or interiors practice: projects as spreads with a photograph and a title block of their facts, plans framed as numbered drawing sheets, studio notes, types of work and a people page.",
    css: PackText::file("plinth/plinth.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "plinth/picture_sizes.html"),
        PackTemplate::file("home.html", "plinth/home.html"),
        PackTemplate::file("post.html", "plinth/post.html"),
        PackTemplate::file("listing.html", "plinth/listing.html"),
        PackTemplate::file("authors.html", "plinth/authors.html"),
        PackTemplate::file("author.html", "plinth/author.html"),
    ],
    tokens: include_str!("../packs/plinth/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["linkedin", "bluesky", "email"]),
        slot::newsletter("post"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the practice's name at the top of the front page, such as what it designs and where.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "notes_count",
            help: "How many studio notes, posts without facts, show after the projects on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 4,
            },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
