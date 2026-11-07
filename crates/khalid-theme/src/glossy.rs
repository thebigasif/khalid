//! Glossy: a lifestyle magazine of picture cards
//! (docs/templates/glossy.md). The front page opens like a magazine
//! cover: the cover story's picture large beside its title and the cover
//! lines, a numbered list of the next stories. The other stories follow
//! as picture cards in a repeating rhythm of wide, tall, square and band
//! cards, then the sections as rounded links. Its cards and cover live in
//! `parts-glossy.html`; the people pages use the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Glossy pack.
pub(crate) static GLOSSY: Pack = Pack {
    name: "glossy",
    summary: "For a lifestyle magazine told in pictures: a cover story with its cover lines, then picture cards in a rhythm of wide, tall, square and band cards, sections as rounded links and posts led by a large rounded picture.",
    css: PackText::file("glossy/glossy.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "glossy/picture_sizes.html"),
        PackTemplate::file("home.html", "glossy/home.html"),
        PackTemplate::file("post.html", "glossy/post.html"),
        PackTemplate::file("listing.html", "glossy/listing.html"),
        PackTemplate::file("authors.html", "glossy/authors.html"),
        PackTemplate::file("author.html", "glossy/author.html"),
        PackTemplate::file("parts-glossy.html", "glossy/parts-glossy.html"),
    ],
    tokens: include_str!("../packs/glossy/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::CARD_READING_TIME),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["facebook", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line beside the magazine's name at the top of the front page, such as what it covers.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "cover_lines",
            help: "How many of the next stories the cover lists beside the cover story, as a magazine cover does.",
            kind: OptionKind::Count {
                least: 0,
                most: 4,
                default: 3,
            },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
