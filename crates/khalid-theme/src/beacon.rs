//! Beacon: a charity or a cause (docs/templates/beacon.md). Campaigns are
//! posts with facts (raised, goal, then two of the writer's own), each
//! with a goal meter on its card and in a campaign panel on its page. The
//! front page leads with the donate call beside the lead appeal, then the
//! impact figures, the campaigns and the news. Built from the shared
//! section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Beacon pack.
pub(crate) static BEACON: Pack = Pack {
    name: "beacon",
    summary: "For a charity or a cause: campaigns with a goal meter that fills as money comes in, impact figures, a news feed and a donate call in the header, the hero and a band on every page.",
    css: PackText::file("beacon/beacon.css"),
    templates: &[
        PackTemplate::file("home.html", "beacon/home.html"),
        PackTemplate::file("post.html", "beacon/post.html"),
        PackTemplate::file("listing.html", "beacon/listing.html"),
        PackTemplate::file("authors.html", "beacon/authors.html"),
        PackTemplate::file("author.html", "beacon/author.html"),
    ],
    tokens: include_str!("../packs/beacon/tokens.toml"),
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
        slot::share_networks(&["facebook", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("both"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "tagline",
            help: "One line under the charity's name at the top of the front page, such as what it does and for whom.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "campaign_count",
            help: "How many campaigns, posts with facts, show as cards on the front page after the lead appeal.",
            kind: OptionKind::Count {
                least: 0,
                most: 9,
                default: 6,
            },
        },
        OptionSpec {
            key: "news_count",
            help: "How many news posts, posts without facts, show in the news feed on the front page.",
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
