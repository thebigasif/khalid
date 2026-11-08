//! Atelier: a fashion label, a stylist or a fashion writer
//! (docs/templates/atelier.md). A look is a post with facts, the pieces
//! worn, and a picture. The front page is a lookbook: the cover look as a
//! spread with its pieces beside the picture, then the other looks as a
//! numbered grid of tall pictures, the collections and a journal of
//! notes. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Atelier pack.
pub(crate) static ATELIER: Pack = Pack {
    name: "atelier",
    summary: "For a fashion label, stylist or fashion writer: a lookbook of numbered looks, each a tall picture with the pieces worn beside or under it, collections, a journal of notes and posts that open as a two-page spread.",
    css: PackText::file("atelier/atelier.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "atelier/picture_sizes.html"),
        PackTemplate::file("home.html", "atelier/home.html"),
        PackTemplate::file("post.html", "atelier/post.html"),
        PackTemplate::file("listing.html", "atelier/listing.html"),
        PackTemplate::file("authors.html", "atelier/authors.html"),
        PackTemplate::file("author.html", "atelier/author.html"),
    ],
    tokens: PackText::file("atelier/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["bluesky", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the label's name at the top of the front page, such as what the label makes.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "journal_count",
            help: "How many notes, posts without facts, show in the journal on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 4,
            },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
