//! Brief: a short digest newsletter (docs/templates/brief.md). Every page
//! is one narrow sheet on a tinted desk. An issue is a post whose facts
//! give its number and how many items it holds: the front page leads with
//! the latest issue and its item ticks, then every earlier issue as a
//! dated row, and inside an issue each item's number hangs in the margin.
//! Built from the shared section parts in `khalid-render/templates/sections/`
//! and its own `parts-brief.html`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Brief pack.
pub(crate) static BRIEF: Pack = Pack {
    name: "brief",
    summary: "For a short daily or weekly digest: one narrow sheet per page, the latest issue with a tick for each item, earlier issues as dated rows, and numbered items that each end on a link.",
    css: PackText::file("brief/brief.css"),
    templates: &[
        PackTemplate::file("home.html", "brief/home.html"),
        PackTemplate::file("post.html", "brief/post.html"),
        PackTemplate::file("listing.html", "brief/listing.html"),
        PackTemplate::file("authors.html", "brief/authors.html"),
        PackTemplate::file("author.html", "brief/author.html"),
        PackTemplate::file("parts-brief.html", "brief/parts-brief.html"),
    ],
    tokens: PackText::file("brief/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::None),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["email", "bluesky", "linkedin"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the newsletter's name at the top of the front page, such as what it covers and how often it goes out.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
