//! Solo: a personal site and blog for one writer or creator, based on
//! Ghost's Solo theme (MIT licence, see `packs/solo/LICENSE-ghost.txt`).
//! The front page opens with a large headline and a sign-up field, then a
//! feed of posts with large titles in one of three layouts. Ghost's
//! parallax feed is a plain layout here: each post's picture fills its
//! card under the title, and nothing moves.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The longest headline, about three lines at its largest size.
const HEADLINE_CHARACTERS: usize = 140;

/// The Solo pack.
pub(crate) static SOLO: Pack = Pack {
    name: "solo",
    summary: "A personal site for one writer or creator: a large headline with a sign-up field, then a feed of posts with large titles.",
    css: PackText::file("solo/solo.css"),
    templates: &[
        PackTemplate::file("home.html", "solo/home.html"),
        PackTemplate::file("post.html", "solo/post.html"),
    ],
    tokens: include_str!("../packs/solo/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "feed",
            help: "How the front page lists posts: classic (the title beside a picture), typographic (large titles and summaries, no pictures) or parallax (each picture fills its card under the title).",
            kind: OptionKind::Choice {
                allowed: &["classic", "typographic", "parallax"],
                default: "classic",
            },
        },
        OptionSpec {
            key: "headline",
            help: "The large line at the top of the front page, such as who you are. Empty shows the blog's name.",
            kind: OptionKind::Line {
                most: HEADLINE_CHARACTERS,
            },
        },
        OptionSpec {
            key: "profile_photo",
            help: "Whether the front page shows a round photo of the newest post's author above the headline.",
            kind: OptionKind::Switch { default: true },
        },
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::CARD_READING_TIME),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        parts::related(RelatedLayout::List),
        parts::related_count(3),
        slot::share("end"),
        slot::share_networks(&["x", "linkedin", "bluesky", "email"]),
        slot::newsletter("home"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
