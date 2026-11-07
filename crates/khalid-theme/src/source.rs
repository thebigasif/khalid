//! Source: a general publication, based on Ghost's Source theme (MIT
//! licence, see `packs/source/LICENSE-ghost.txt`). The logo in the middle
//! of the header, a front page that opens with one of five header styles,
//! a list or a grid of posts with hairlines between them, and a post in
//! one calm column under a wide picture.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Source pack.
pub(crate) static SOURCE: Pack = Pack {
    name: "source",
    summary: "A general publication based on Ghost's Source theme: the logo in the middle of the header, a front page that opens with a landing, highlight or magazine header, a list or a grid of posts, and a calm column for each post.",
    css: PackText::file("source/source.css"),
    templates: &[
        PackTemplate::file("home.html", "source/home.html"),
        PackTemplate::file("post.html", "source/post.html"),
    ],
    tokens: include_str!("../packs/source/tokens.toml"),
    reader: &[(ReaderSwitch::AuthorBox, false)],
    options: &[
        OptionSpec {
            key: "header_style",
            help: "How the front page opens: landing (the title and the sign-up form, centred), highlight (a large post beside smaller ones), magazine (a large post between two columns), search (the title and a search field) or off.",
            kind: OptionKind::Choice {
                allowed: &["landing", "highlight", "magazine", "search", "off"],
                default: "landing",
            },
        },
        OptionSpec {
            key: "about_sidebar",
            help: "Whether the front page's list of posts has a column beside it with the blog's name, its intro and your [template.cta].",
            kind: OptionKind::Switch { default: false },
        },
        OptionSpec {
            key: "drop_cap",
            help: "Whether the first letter of each post is set large over its first lines.",
            kind: OptionKind::Switch { default: false },
        },
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::CARD_READING_TIME,
        parts::picks(0),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
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
