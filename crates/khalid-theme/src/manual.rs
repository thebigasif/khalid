//! Manual: a help centre or product documentation (docs/templates/manual.md).
//! The front page leads with search and numbers the topics (categories)
//! as tiles, each with its newest articles. An article stands in three
//! columns on a wide screen: the guide's steps (its series) or more
//! articles, the text with its sections numbered in the margin like a
//! printed manual, and "On this page". Under the text, a next steps panel
//! asks whether the article helped, with links only and no script. Built
//! from the shared section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Manual pack.
pub(crate) static MANUAL: Pack = Pack {
    name: "manual",
    summary: "For a help centre or product documentation: search first, numbered topic tiles, articles with the guide's steps beside the text and the contents in a column, and a next steps panel that asks whether the article helped.",
    css: PackText::file("manual/manual.css"),
    templates: &[
        PackTemplate::file("home.html", "manual/home.html"),
        PackTemplate::file("post.html", "manual/post.html"),
        PackTemplate::file("listing.html", "manual/listing.html"),
    ],
    tokens: PackText::file("manual/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::SideEnd),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Nested),
        slot::share("none"),
        slot::share_networks(&["email", "linkedin"]),
        slot::newsletter("none"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the help centre's name at the top of the front page, such as what it helps with.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "popular_count",
            help: "How many popular articles, the posts marked featured, show on the front page. The first four also show under the search field.",
            kind: OptionKind::Count {
                least: 0,
                most: 8,
                default: 6,
            },
        },
        OptionSpec {
            key: "updates_count",
            help: "How many recently updated articles show on the front page.",
            kind: OptionKind::Count {
                least: 1,
                most: 20,
                default: 6,
            },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
