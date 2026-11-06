//! Lattice: a science explainer (docs/templates/lattice.md). Every
//! picture is a numbered figure plate on a lattice mat, a post's facts are
//! its key numbers, footnotes are small numbered tabs over a ruled notes
//! panel, and glossary links are marked terms. Built from the shared
//! section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Lattice pack.
pub(crate) static LATTICE: Pack = Pack {
    name: "lattice",
    summary: "For a science explainer site: every picture a numbered figure plate, key numbers from each post's facts, footnotes as numbered tabs over a notes panel, marked glossary terms and a picture-led front page.",
    css: PackText::file("lattice/lattice.css"),
    templates: &[
        PackTemplate::file("home.html", "lattice/home.html"),
        PackTemplate::file("post.html", "lattice/post.html"),
        PackTemplate::file("listing.html", "lattice/listing.html"),
    ],
    tokens: include_str!("../packs/lattice/tokens.toml"),
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
        slot::share_networks(&["bluesky", "facebook", "email"]),
        slot::newsletter("post"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the site's name at the top of the front page, such as what the site explains.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "key_numbers",
            help: "Whether a post's facts show as a row of key numbers under its lead picture, and under the lead explainer on the front page.",
            kind: OptionKind::Switch { default: true },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
