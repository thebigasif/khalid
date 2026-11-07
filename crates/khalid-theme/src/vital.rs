//! Vital: a health and wellness site of programmes and routines
//! (docs/templates/vital.md). A routine is a post with facts (time,
//! level, how often, what you need): its time sits in the session ring on
//! every card and on the post's session panel, and the post's numbered
//! list is drawn as the routine track. Built from the shared section
//! parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Vital pack.
pub(crate) static VITAL: Pack = Pack {
    name: "vital",
    summary: "For a health and wellness site: programmes as categories, routines as picture cards with their time in a ring, a session panel and a care note on every routine, its steps drawn as a track, and a notes feed.",
    css: PackText::file("vital/vital.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "vital/picture_sizes.html"),
        PackTemplate::file("home.html", "vital/home.html"),
        PackTemplate::file("post.html", "vital/post.html"),
        PackTemplate::file("listing.html", "vital/listing.html"),
        PackTemplate::file("authors.html", "vital/authors.html"),
        PackTemplate::file("author.html", "vital/author.html"),
    ],
    tokens: include_str!("../packs/vital/tokens.toml"),
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
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["whatsapp", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the site's name at the top of the front page, such as who the routines are for.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "care_note",
            help: "One plain safety line shown under every routine's session panel and beside the notes on the front page, such as \"General wellbeing ideas, not medical advice.\" Nothing shows until you write it.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "notes_count",
            help: "How many notes, posts without facts, show on the front page under the routines.",
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
