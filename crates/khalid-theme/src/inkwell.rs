//! Inkwell: the archive of a long-running newsletter
//! (docs/templates/inkwell.md). An issue is a post whose facts are read by
//! position: 1 its number, 2 to 4 its parts, each with the section as the
//! label. The front page opens with the newsletter's name beside the
//! sign-up form, then the latest issue with its number set large, then
//! every issue grouped by year like bound volumes, each row with its
//! number and its parts at a glance. Its rows come from its own parts
//! file, `parts-inkwell.html`; the hero and the people grid are the shared
//! section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Inkwell pack.
pub(crate) static INKWELL: Pack = Pack {
    name: "inkwell",
    summary: "For a long-running newsletter's archive: the sign-up form beside the name, the latest issue with its number set large, and every issue numbered and grouped by year, each with its parts at a glance.",
    css: PackText::file("inkwell/inkwell.css"),
    templates: &[
        PackTemplate::file("home.html", "inkwell/home.html"),
        PackTemplate::file("post.html", "inkwell/post.html"),
        PackTemplate::file("listing.html", "inkwell/listing.html"),
        PackTemplate::file("authors.html", "inkwell/authors.html"),
        PackTemplate::file("author.html", "inkwell/author.html"),
        PackTemplate::file("parts-inkwell.html", "inkwell/parts-inkwell.html"),
    ],
    tokens: include_str!("../packs/inkwell/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
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
            help: "One short line over the newsletter's name at the top of the front page, such as how often it goes out and since when.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "signup_note",
            help: "One line in the sign-up form, such as how many letters a reader gets and that nothing else is sent.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
