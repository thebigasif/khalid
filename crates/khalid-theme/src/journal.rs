//! Journal: one writer's newsletter, based on Ghost's Journal theme (MIT
//! licence, see `packs/journal/LICENSE-ghost.txt`). The front page opens
//! with the latest issue in large type and no picture, then every other
//! issue as a column of text beside a side column that introduces the
//! writer, takes sign-ups and lists featured posts and topics. A band in
//! the accent colour asks for the email before the footer.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Journal pack.
pub(crate) static JOURNAL: Pack = Pack {
    name: "journal",
    summary: "One writer's newsletter: the latest issue in large type, every other issue as text beside a side column about the writer, and a sign-up band in your accent colour.",
    css: PackText::file("journal/journal.css"),
    templates: &[
        PackTemplate::file("home.html", "journal/home.html"),
        PackTemplate::file("post.html", "journal/post.html"),
    ],
    tokens: include_str!("../packs/journal/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "sidebar",
            help: "Whether the front page shows the side column with the blog's intro, the sign-up form, featured posts and topics.",
            kind: OptionKind::Switch { default: true },
        },
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::on(parts::CARD_READING_TIME),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::None),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        parts::related(RelatedLayout::List),
        parts::related_count(3),
        slot::share("end"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
