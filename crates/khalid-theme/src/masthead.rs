//! Masthead: a general publication (docs/templates/masthead.md). The
//! site's name centred over the nameplate rule, a front page whose lead
//! story is followed by the latest posts as a grid or a list, as the
//! `front` option sets, and headline lists by section, and a calm reading
//! column on every post. Built from the shared cards and the section
//! parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Masthead pack.
pub(crate) static MASTHEAD: Pack = Pack {
    name: "masthead",
    summary: "For a general publication: the name centred over a heavy and a thin rule, a lead story, the latest posts as a grid or a list, headlines by section, and a calm reading column.",
    css: PackText::file("masthead/masthead.css"),
    templates: &[
        PackTemplate::file("home.html", "masthead/home.html"),
        PackTemplate::file("post.html", "masthead/post.html"),
        PackTemplate::file("listing.html", "masthead/listing.html"),
        PackTemplate::file("authors.html", "masthead/authors.html"),
        PackTemplate::file("author.html", "masthead/author.html"),
    ],
    tokens: PackText::file("masthead/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "front",
            help: "The shape of the posts after the lead story, on the front page and on every list: grid puts picture cards three to a row, list puts one story a row with its text beside a small picture.",
            kind: OptionKind::Choice {
                allowed: &["grid", "list"],
                default: "grid",
            },
        },
        OptionSpec {
            key: "tagline",
            help: "One line under the site's name on the front page, such as what the publication covers.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "latest_count",
            help: "How many posts after the lead story show under Latest on the front page. The rest of the page's posts show as headlines by section.",
            kind: OptionKind::Count {
                least: 3,
                most: 12,
                default: 6,
            },
        },
        parts::on(parts::FEATURED),
        parts::LOGO,
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["email", "facebook", "whatsapp"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
