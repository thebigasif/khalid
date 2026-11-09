//! Hearth: a warm, classic blog for a small team or a family business
//! (docs/templates/hearth.md). The front page welcomes the reader with the
//! blog's name, then leads with one story on a warm panel, its picture in
//! a tall arch, and the other posts as a grid of picture cards. Posts read
//! in a friendly column under a centred head. Every picture sits in an
//! arch. Its card is the pack's own part, `parts-hearth.html`.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Hearth pack.
pub(crate) static HEARTH: Pack = Pack {
    name: "hearth",
    summary: "A warm, classic blog for a small team or a family business: a welcome, a picture-led lead story, a grid of picture cards with each author's face, and a friendly reading column, every picture in an arch.",
    css: PackText::file("hearth/hearth.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "hearth/picture_sizes.html"),
        PackTemplate::file("home.html", "hearth/home.html"),
        PackTemplate::file("post.html", "hearth/post.html"),
        PackTemplate::file("listing.html", "hearth/listing.html"),
        PackTemplate::file("authors.html", "hearth/authors.html"),
        PackTemplate::file("author.html", "hearth/author.html"),
        PackTemplate::file("parts-hearth.html", "hearth/parts-hearth.html"),
    ],
    tokens: PackText::file("hearth/tokens.toml"),
    reader: &[
        (ReaderSwitch::ReadingTime, true),
        (ReaderSwitch::AuthorBox, true),
    ],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::FEATURED),
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
            help: "One line under the blog's name at the top of the front page, such as who writes it and what about. The front page's intro, when written, shows in its place.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
