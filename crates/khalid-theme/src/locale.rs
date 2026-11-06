//! Locale: a city guide (docs/templates/locale.md). A place is a post
//! with facts, read by position (the kind of place, the price level, the
//! opening hours, the address), and an area is a category. The front page
//! lists every area's places as listing rows: the name, a dotted leader
//! and the price meter, then the kind and the hours. A place opens with
//! its picture beside an At a glance card. Notes, posts without facts,
//! show apart as picture cards. Built on the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Locale pack.
pub(crate) static LOCALE: Pack = Pack {
    name: "locale",
    summary: "For a city guide: places listed by area, each with its kind, opening hours and a price meter, an At a glance card on every place, and notes from the city.",
    css: PackText::file("locale/locale.css"),
    templates: &[
        PackTemplate::file("home.html", "locale/home.html"),
        PackTemplate::file("post.html", "locale/post.html"),
        PackTemplate::file("listing.html", "locale/listing.html"),
        PackTemplate::file("authors.html", "locale/authors.html"),
        PackTemplate::file("author.html", "locale/author.html"),
        PackTemplate::file("parts-locale.html", "locale/parts-locale.html"),
    ],
    tokens: include_str!("../packs/locale/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
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
            key: "tagline",
            help: "One line under the guide's name at the top of the front page, such as the city it covers.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
