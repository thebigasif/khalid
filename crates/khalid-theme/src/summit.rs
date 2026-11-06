//! Summit: a conference, festival or event site (docs/templates/summit.md).
//! Sessions are posts with facts (day, time, room, format), shown on a
//! timetable rail by day and time on the front page, on every venue and
//! track page and on each speaker's page. A session shows its facts on a
//! ticket stub. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Summit pack.
pub(crate) static SUMMIT: Pack = Pack {
    name: "summit",
    summary: "For a conference, festival or event: the programme as a timetable by day and time, speakers, venues with their sessions, updates, and a ticket stub with the day, time, room and format on every session.",
    css: PackText::file("summit/summit.css"),
    templates: &[
        PackTemplate::file("home.html", "summit/home.html"),
        PackTemplate::file("post.html", "summit/post.html"),
        PackTemplate::file("listing.html", "summit/listing.html"),
        PackTemplate::file("authors.html", "summit/authors.html"),
        PackTemplate::file("author.html", "summit/author.html"),
    ],
    tokens: include_str!("../packs/summit/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["bluesky", "linkedin", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "event_line",
            help: "One short line above the event's name at the top of the front page, such as its dates and town.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "tagline",
            help: "One line under the event's name at the top of the front page, such as what it is about.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "speakers_count",
            help: "How many speakers, the authors of the sessions on the front page, show in its speakers grid.",
            kind: OptionKind::Count {
                least: 0,
                most: 24,
                default: 8,
            },
        },
        OptionSpec {
            key: "news_count",
            help: "How many updates, posts without facts, show beside the venues on the front page.",
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
