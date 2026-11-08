//! Frequency: a podcast's site (docs/templates/frequency.md). An episode
//! is a post with a player and facts (the number, the running time, the
//! guest, then one of the writer's own). The front page leads with the
//! show beside the On air console, which plays the newest episode, then
//! lists every episode as a numbered row with a running-time dial. The
//! listen links, one app a line, show on the front page and after every
//! episode. Built from the shared section parts in
//! `khalid-render/templates/sections/` and the pack's own
//! `parts-frequency.html`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The most listen links the show takes: the large apps and a feed.
const MOST_LISTEN_LINKS: usize = 6;

/// The Frequency pack.
pub(crate) static FREQUENCY: Pack = Pack {
    name: "frequency",
    summary: "For a podcast: the newest episode plays on the front page, every episode is a numbered row with a running-time dial, and the listen links show on the front page and after each episode.",
    css: PackText::file("frequency/frequency.css"),
    templates: &[
        PackTemplate::file("home.html", "frequency/home.html"),
        PackTemplate::file("post.html", "frequency/post.html"),
        PackTemplate::file("listing.html", "frequency/listing.html"),
        PackTemplate::file("authors.html", "frequency/authors.html"),
        PackTemplate::file("author.html", "frequency/author.html"),
        PackTemplate::file("parts-frequency.html", "frequency/parts-frequency.html"),
    ],
    tokens: PackText::file("frequency/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("both"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "tagline",
            help: "One line under the show's name at the top of the front page, such as what the show is about and how often it comes out. The front page's own intro wins when it has one.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "listen_on",
            help: "Where to listen, up to six links, each written label | address, such as Apple Podcasts | https://podcasts.apple.com/podcast/id123. They show as tiles under the show's name and after every episode.",
            kind: OptionKind::Lines {
                most: MOST_LISTEN_LINKS,
            },
        },
        OptionSpec {
            key: "post_listen",
            help: "Whether every episode ends with the listen links under a Listen on heading.",
            kind: OptionKind::Switch { default: true },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
