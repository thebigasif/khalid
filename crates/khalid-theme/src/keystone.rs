//! Keystone (templates spec 3.3): a calm, picture-led blog for a product or
//! a company, with a lead story, a grid of framed pictures and a reading
//! column with share links beside it. It replaces the home and post pages,
//! keeps every reader feature on, and sets a bold type scale on a wide,
//! drawn frame with one cobalt accent. Its own look is in
//! `packs/keystone/tokens.toml`.

use crate::parts::{self, CardShape, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Keystone pack.
pub(crate) static KEYSTONE: Pack = Pack {
    name: "keystone",
    summary: "A product or company blog: a lead story, a grid of framed pictures, a calm reading column with share links beside it, the author and related posts after it.",
    stylesheet: include_str!("../packs/keystone/keystone.css"),
    templates: &[
        PackTemplate {
            name: "home.html",
            source: include_str!("../packs/keystone/home.html"),
        },
        PackTemplate {
            name: "post.html",
            source: include_str!("../packs/keystone/post.html"),
        },
    ],
    tokens: include_str!("../packs/keystone/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        slot::CTA_BUTTON_ON,
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::on(parts::UPDATED_NOTE),
        parts::on(parts::FEATURED),
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("top"),
        slot::share_networks(&["x", "linkedin", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("post"),
        OptionSpec {
            key: "numbered_headings",
            help: "Whether each main heading of a post gets a chapter number: 01, 02, 03.",
            kind: OptionKind::Switch { default: false },
        },
        OptionSpec {
            key: "subscriber_line",
            help: "One line over the newsletter form on the front page, such as Join 4,000 readers.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "about",
            help: "Up to two lines about the site after each post, such as who writes it.",
            kind: OptionKind::Lines { most: 2 },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::on(parts::VIDEO_SECTION),
        parts::on(parts::PODCAST_SECTION),
    ],
};
