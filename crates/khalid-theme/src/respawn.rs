//! Respawn: a games site with news, reviews and a release calendar
//! (docs/templates/respawn.md). A review is a post with facts, read by
//! position (the score out of ten, the platforms, the developer, the
//! release), and every review shows its score as a tile with a health bar
//! of ten cells: on the front page cards, at the top and end of the
//! review, and on every list. The front page leads with a story beside the
//! latest news, then the release calendar from the `release_calendar`
//! option. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Respawn pack.
pub(crate) static RESPAWN: Pack = Pack {
    name: "respawn",
    summary: "For a games site: news, reviews with a score and a health bar on every card, and a release calendar on the front page.",
    css: PackText::file("respawn/respawn.css"),
    templates: &[
        PackTemplate::file("home.html", "respawn/home.html"),
        PackTemplate::file("post.html", "respawn/post.html"),
        PackTemplate::file("listing.html", "respawn/listing.html"),
        PackTemplate::file("authors.html", "respawn/authors.html"),
        PackTemplate::file("author.html", "respawn/author.html"),
        PackTemplate::file("respawn-parts.html", "respawn/respawn-parts.html"),
    ],
    tokens: include_str!("../packs/respawn/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::FEATURED),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "reddit", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line beside the site's name at the top of the front page, such as what the site covers.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "release_calendar",
            help: "The games coming out soon, one line each, written date | game | platforms, such as 2026-11-20 | Hollowmere | PC, consoles. A date as YYYY-MM-DD shows as a calendar tile; other text, such as Spring 2027, shows as written. An optional fourth part links the game's name, such as | /hollowmere-release-date/.",
            kind: OptionKind::Lines { most: 12 },
        },
        OptionSpec {
            key: "news_count",
            help: "How many news posts, posts without facts, show in the latest news feed beside the lead story on the front page. The rest show as cards further down.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 4,
            },
        },
        OptionSpec {
            key: "verdict",
            help: "Whether a review, a post with facts, ends with the verdict: its score large with the health bar and its description as the one-line verdict.",
            kind: OptionKind::Switch { default: true },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
