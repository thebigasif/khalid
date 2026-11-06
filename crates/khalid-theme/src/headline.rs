//! Headline: a local news site, based on Ghost's Headline theme (MIT
//! licence, see `packs/headline/LICENSE-ghost.txt`). A stacked header with
//! the logo over the menu, a front page of the latest posts and then one
//! section per topic, each a large lead story over small rows with a
//! square-cut picture, then short lists of headlines for more topics, and
//! a dark footer.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Headline pack.
pub(crate) static HEADLINE: Pack = Pack {
    name: "headline",
    summary: "A local news site based on Ghost's Headline theme: the logo over the menu, the latest posts, then a section for each topic with a lead story and small rows, short lists of headlines for more topics, and a dark footer.",
    css: PackText::file("headline/headline.css"),
    templates: &[
        PackTemplate::file("home.html", "headline/home.html"),
        PackTemplate::file("post.html", "headline/post.html"),
    ],
    tokens: include_str!("../packs/headline/tokens.toml"),
    reader: &[(ReaderSwitch::AuthorBox, false)],
    options: &[
        OptionSpec {
            key: "latest_count",
            help: "How many of the newest posts the front page shows under Latest, the first one large.",
            kind: OptionKind::Count {
                least: 3,
                most: 13,
                default: 7,
            },
        },
        OptionSpec {
            key: "topic_sections",
            help: "How many topics get a section of their own on the front page, with a lead story and small rows.",
            kind: OptionKind::Count {
                least: 0,
                most: 6,
                default: 3,
            },
        },
        OptionSpec {
            key: "topic_lists",
            help: "How many more topics show as short lists of headlines under the sections.",
            kind: OptionKind::Count {
                least: 0,
                most: 6,
                default: 3,
            },
        },
        OptionSpec {
            key: "post_image",
            help: "How wide a post's lead picture is: standard (as wide as the text) or wide (as wide as the page).",
            kind: OptionKind::Choice {
                allowed: &["standard", "wide"],
                default: "standard",
            },
        },
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::CARD_READING_TIME,
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Row),
        parts::related_count(3),
        parts::contents(ContentsPlacement::None),
        parts::contents_numbers(ContentsNumbers::Numbered),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("top"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
