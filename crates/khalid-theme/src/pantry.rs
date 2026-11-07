//! Pantry: a baking and recipe blog (docs/templates/pantry.md). A recipe
//! is a post with facts (time, yield, oven, level): its time and yield sit
//! on the tear-off ticket of every index card, all four on the ticket at
//! the top of the post, and the post opens on its ingredients card before
//! the method's numbered steps. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Pantry pack.
pub(crate) static PANTRY: Pack = Pack {
    name: "pantry",
    summary: "For a baking and recipe blog: the sections of the recipe box as divider tabs, recipes as square index cards with the time and yield on a tear-off ticket, and a recipe page that opens on its facts and ingredients card, then numbers its method.",
    css: PackText::file("pantry/pantry.css"),
    templates: &[
        PackTemplate::file("home.html", "pantry/home.html"),
        PackTemplate::file("post.html", "pantry/post.html"),
        PackTemplate::file("listing.html", "pantry/listing.html"),
    ],
    tokens: include_str!("../packs/pantry/tokens.toml"),
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
        slot::share_networks(&["facebook", "whatsapp", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the site's name at the top of the front page, such as what you bake and for whom.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "notes_count",
            help: "How many baking notes, posts without facts, show on the front page under the recipes.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 6,
            },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
