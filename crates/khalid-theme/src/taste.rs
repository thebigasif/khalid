//! Taste: a recipe and baking blog, based on Ghost's Taste theme (MIT
//! licence, see `packs/taste/LICENSE-ghost.txt`). A centred header with a
//! large serif line and a sign-up field, a magazine block of five posts,
//! picture grids for the main topics that alternate three and four
//! across, and a centred post with the sign-up box under it.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Taste pack.
pub(crate) static TASTE: Pack = Pack {
    name: "taste",
    summary: "A recipe and baking blog based on Ghost's Taste theme: a centred header with a sign-up field, a magazine block of five posts, then picture grids for the main topics, three and four across.",
    css: PackText::file("taste/taste.css"),
    templates: &[
        PackTemplate::file("home.html", "taste/home.html"),
        PackTemplate::file("post.html", "taste/post.html"),
    ],
    tokens: include_str!("../packs/taste/tokens.toml"),
    reader: &[(ReaderSwitch::AuthorBox, false)],
    options: &[
        OptionSpec {
            key: "header_action",
            help: "What the front page's header offers under its words: subscribe (the sign-up field, or your call to action when the blog takes no sign-ups), search (a field that opens the search page) or none.",
            kind: OptionKind::Choice {
                allowed: &["subscribe", "search", "none"],
                default: "subscribe",
            },
        },
        OptionSpec {
            key: "topic_sections",
            help: "How many topics the front page shows as picture grids after its first five posts, from 0 to 6. Your [[reader.home_sections]] come first; without them, the topics with the newest posts.",
            kind: OptionKind::Count {
                least: 0,
                most: 6,
                default: 3,
            },
        },
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::CARD_READING_TIME,
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("end"),
        slot::share_networks(&["facebook", "x", "whatsapp", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
