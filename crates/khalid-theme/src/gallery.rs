//! Gallery: a studio's or an illustrator's site
//! (docs/templates/gallery.md). The front page is a wall of pictures:
//! every work on a mat at its own shape, never cropped, with a numbered
//! wall label under it, then the list of works under the same numbers.
//! A post is a project page: the title beside a sheet of the post's facts
//! read by position (client, year, medium, size or edition), the work on
//! its mat, then one reading column. Not single-image stories (Aperture)
//! and not an even grid of cropped looks (Atelier).

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Gallery pack.
pub(crate) static GALLERY: Pack = Pack {
    name: "gallery",
    summary: "For a studio or an illustrator: a wall of pictures, each on a mat at its own shape with a numbered label, a list of works under the same numbers, and project pages with a sheet of the client, year, medium and size.",
    css: PackText::file("gallery/gallery.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "gallery/picture_sizes.html"),
        PackTemplate::file("home.html", "gallery/home.html"),
        PackTemplate::file("post.html", "gallery/post.html"),
        PackTemplate::file("listing.html", "gallery/listing.html"),
        PackTemplate::file("authors.html", "gallery/authors.html"),
        PackTemplate::file("author.html", "gallery/author.html"),
        PackTemplate::file("parts-gallery.html", "gallery/parts-gallery.html"),
    ],
    tokens: include_str!("../packs/gallery/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "facebook", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "statement",
            help: "One or two lines beside the studio's name on the front page, such as what the studio makes and for whom. The home page's intro, when it has one, comes first.",
            kind: OptionKind::Lines { most: 2 },
        },
        OptionSpec {
            key: "checklist",
            help: "Whether the front page closes with the list of works: every work on the page under its wall number, with its client and year.",
            kind: OptionKind::Switch { default: true },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
