//! Encore: a musician's or a band's site (docs/templates/encore.md). A
//! release is a post with facts (format, release date, label, where to
//! listen), shown as a cover on its sleeve with the record half out. The
//! front page leads with the newest release, the tour dates from the
//! `tour_dates` option and the shelf of releases, then the band's notes.
//! Built with the shared section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The most shows the tour list carries: a long tour, one line each.
const MOST_SHOWS: usize = 24;

/// The Encore pack.
pub(crate) static ENCORE: Pack = Pack {
    name: "encore",
    summary: "For a musician or a band: the newest release on its sleeve, tour dates with ticket links, a shelf of every record and the band's notes from the studio and the road.",
    css: PackText::file("encore/encore.css"),
    templates: &[
        PackTemplate::file("home.html", "encore/home.html"),
        PackTemplate::file("post.html", "encore/post.html"),
        PackTemplate::file("listing.html", "encore/listing.html"),
        PackTemplate::file("authors.html", "encore/authors.html"),
        PackTemplate::file("author.html", "encore/author.html"),
    ],
    tokens: include_str!("../packs/encore/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "tagline",
            help: "One line under the band's name at the top of the front page, such as the kind of music and where the band is from.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "listen_on",
            help: "Up to four places to listen, shown under the band's name, each written name | address, such as Bandcamp | https://example.bandcamp.com/.",
            kind: OptionKind::Lines { most: 4 },
        },
        OptionSpec {
            key: "tour_dates",
            help: "The shows on the front page's tour list, one a line, written date | city | venue | ticket address or a short note such as Sold out. Write the date as 2026-11-14.",
            kind: OptionKind::Lines { most: MOST_SHOWS },
        },
        OptionSpec {
            key: "notes_count",
            help: "How many notes, posts without facts, show on the front page under the releases.",
            kind: OptionKind::Count {
                least: 0,
                most: 9,
                default: 3,
            },
        },
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
        slot::share_networks(&["bluesky", "facebook", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
