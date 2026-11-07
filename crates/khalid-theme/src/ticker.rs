//! Ticker: a markets newsletter (docs/templates/ticker.md). An issue is a
//! post with facts, each a market and its move. The front page opens with
//! a tape of figures, then the latest issue beside its facts as a bar
//! chart, then every other post as a data table shaded by how far each
//! figure moved. Every chart is HTML and CSS, so it draws with scripts
//! off. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Ticker pack.
pub(crate) static TICKER: Pack = Pack {
    name: "ticker",
    summary: "For a markets newsletter: a tape of figures across the front page, each issue's moves as a bar chart, and every list as a data table shaded by how far each figure moved. Charts need no script.",
    css: PackText::file("ticker/ticker.css"),
    templates: &[
        PackTemplate::file("home.html", "ticker/home.html"),
        PackTemplate::file("post.html", "ticker/post.html"),
        PackTemplate::file("listing.html", "ticker/listing.html"),
        PackTemplate::file("authors.html", "ticker/authors.html"),
        PackTemplate::file("author.html", "ticker/author.html"),
    ],
    tokens: include_str!("../packs/ticker/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "linkedin", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the newsletter's name at the top of the front page, such as what it covers and how often it goes out.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "disclaimer",
            help: "One line under every chart and at the foot of the front page, such as a note that nothing on the site is investment advice.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
