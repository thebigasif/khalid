//! Letterpress: a newsletter set like a printed periodical
//! (docs/templates/letterpress.md). Every post is an issue; its first
//! fact is the issue number and the next three are its contents. The
//! front page opens with the nameplate and the newest issue as a cover,
//! then lists every other issue as a pile of small printed covers. An
//! issue opens with a folio line and closes with an end mark. Built from
//! the shared hero part in `khalid-render/templates/sections/` and the
//! pack's own parts in `parts-letterpress.html`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Letterpress pack.
pub(crate) static LETTERPRESS: Pack = Pack {
    name: "letterpress",
    summary: "For a newsletter that treats each issue as a printed edition: the newest issue as a cover with its picture in a plate and its contents, past issues as a pile of small printed covers, and each issue opened by a folio line and closed by an end mark.",
    css: PackText::file("letterpress/letterpress.css"),
    templates: &[
        PackTemplate::file("home.html", "letterpress/home.html"),
        PackTemplate::file("post.html", "letterpress/post.html"),
        PackTemplate::file("listing.html", "letterpress/listing.html"),
        PackTemplate::file("authors.html", "letterpress/authors.html"),
        PackTemplate::file("author.html", "letterpress/author.html"),
        PackTemplate::file(
            "parts-letterpress.html",
            "letterpress/parts-letterpress.html",
        ),
    ],
    tokens: include_str!("../packs/letterpress/tokens.toml"),
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
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the newsletter's name on the front page, such as what it covers and how often it goes out.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
