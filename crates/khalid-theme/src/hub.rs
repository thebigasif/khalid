//! Hub: a creator's link-in-bio home with the latest posts
//! (docs/templates/hub.md). One narrow column: the creator's photo and
//! name, a stack of large link tiles from the `links` option, led by the
//! call to action, then the latest posts as rows with a square picture.
//! Posts keep the same column and end with the links again. The tiles are
//! the shared `link_stack` section part in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The most link tiles the front page shows, below the call to action.
const MOST_LINKS: usize = 8;

/// The longest author slug the `creator` option takes.
const CREATOR_CHARACTERS: usize = 80;

/// The Hub pack.
pub(crate) static HUB: Pack = Pack {
    name: "hub",
    summary: "A creator's link-in-bio home: the creator's photo and name, a stack of large link buttons, then the latest posts, all in one narrow column.",
    css: PackText::file("hub/hub.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "hub/picture_sizes.html"),
        PackTemplate::file("home.html", "hub/home.html"),
        PackTemplate::file("post.html", "hub/post.html"),
    ],
    tokens: include_str!("../packs/hub/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "links",
            help: "The large link buttons on the front page, up to eight, each written label | address, or label | address | a short note, such as Watch my videos | https://www.example.com/videos. Without them, the main menu fills the stack.",
            kind: OptionKind::Lines { most: MOST_LINKS },
        },
        OptionSpec {
            key: "creator",
            help: "The slug of the author whose round photo shows at the top of the front page, such as tamsin-hale. Empty shows the newest post's author.",
            kind: OptionKind::Line {
                most: CREATOR_CHARACTERS,
            },
        },
        OptionSpec {
            key: "tagline",
            help: "One line under the creator's name on the front page, such as what you make. The front page's own intro wins when it has one.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "cta_tile",
            help: "Whether the call to action from [template.cta] leads the stack of links on the front page, filled with the accent colour.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "post_links",
            help: "Whether a post ends with the link buttons again, under a heading such as More from the site's name.",
            kind: OptionKind::Switch { default: true },
        },
        parts::LOGO,
        parts::on(parts::BACK_LINK),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["x", "bluesky", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
