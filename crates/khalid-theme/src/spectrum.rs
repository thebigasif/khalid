//! Spectrum: a magazine arranged by topic (docs/templates/spectrum.md).
//! Each topic keeps one colour on every page, taken from its place in the
//! main menu: a segment of the spectrum bar on the front page, a row of
//! its newest posts on a band of that colour, the square before its label
//! on every card and the band at the top of its posts and its topic page.
//! The front page leads with one story beside the latest posts. Built
//! from the shared section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Spectrum pack.
pub(crate) static SPECTRUM: Pack = Pack {
    name: "spectrum",
    summary: "For a magazine arranged by topic: each topic has its own colour, a row of its newest posts on a band of that colour on the front page, and the same colour on its posts and its page.",
    css: PackText::file("spectrum/spectrum.css"),
    templates: &[
        PackTemplate::file("home.html", "spectrum/home.html"),
        PackTemplate::file("post.html", "spectrum/post.html"),
        PackTemplate::file("listing.html", "spectrum/listing.html"),
        PackTemplate::file("authors.html", "spectrum/authors.html"),
        PackTemplate::file("author.html", "spectrum/author.html"),
        PackTemplate::file("parts-spectrum.html", "spectrum/parts-spectrum.html"),
    ],
    tokens: PackText::file("spectrum/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["facebook", "whatsapp", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the magazine's name at the top of the front page, such as what it covers.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "latest_count",
            help: "How many of the latest posts show beside the lead story on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 5,
            },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
