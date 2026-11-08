//! Aperture: one photographer's site (docs/templates/aperture.md). Each
//! post is one picture and the story behind it. The front page shows the
//! newest posts as full-width single-image stories, each with an exposure
//! plate of its camera details read from the post's facts by position
//! (camera, lens, exposure, date taken), then the older ones in a light
//! contact sheet. Not a portfolio grid: one picture at a time.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Aperture pack.
pub(crate) static APERTURE: Pack = Pack {
    name: "aperture",
    summary: "For one photographer: each post is one picture shown the full width of the page with the story behind it and its camera, lens and exposure, and older pictures in a light contact sheet.",
    css: PackText::file("aperture/aperture.css"),
    templates: &[
        PackTemplate::file("home.html", "aperture/home.html"),
        PackTemplate::file("post.html", "aperture/post.html"),
        PackTemplate::file("listing.html", "aperture/listing.html"),
        PackTemplate::file("authors.html", "aperture/authors.html"),
        PackTemplate::file("author.html", "aperture/author.html"),
    ],
    tokens: PackText::file("aperture/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(6),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "facebook", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line beside the photographer's name at the top of the front page, such as what they photograph and where.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "stories",
            help: "How many of the newest posts the front page shows as full-width stories. The other posts on the page go into the contact sheet under them.",
            kind: OptionKind::Count {
                least: 1,
                most: 12,
                default: 5,
            },
        },
        OptionSpec {
            key: "camera_plate",
            help: "Whether each picture shows its camera details under it: the post's facts, read as camera, lens, exposure and the date it was taken.",
            kind: OptionKind::Switch { default: true },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
