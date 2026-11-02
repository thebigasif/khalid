//! Folio: a product team's essays and engineering notes under the blog's
//! own masthead (templates spec 3.10).

use crate::parts::{self, CardShape, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Folio pack.
pub(crate) static FOLIO: Pack = Pack {
    name: "folio",
    summary: "A product team's notebook under a large masthead on ruled paper: a featured story, picture cards, and notes with a contents rail, framed code and an author box.",
    stylesheet: include_str!("../packs/folio/folio.css"),
    templates: &[PackTemplate {
        name: "home.html",
        source: include_str!("../packs/folio/home.html"),
    }],
    tokens: TOKENS,
    reader: &[],
    options: &[
        OptionSpec {
            key: "masthead_name",
            help: "The name the masthead shows, one line; the blog's name when not set.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "tagline",
            help: "One line under the masthead's name.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "quote",
            help: "A quote centred under the cards: its words, then who said them.",
            kind: OptionKind::Lines { most: 2 },
        },
        parts::on(parts::LOGO),
        parts::BACK_LINK,
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::on(parts::CATEGORY_TABS),
        parts::cards(CardShape::Image),
        parts::CARD_AUTHOR_ROLE,
        parts::on(parts::FEATURED),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(3),
        parts::contents(ContentsPlacement::SideEnd),
        slot::share("end"),
        slot::share_networks(&["x", "linkedin", "bluesky", "email"]),
        slot::newsletter("none"),
        slot::cta_band("post"),
        slot::CTA_BUTTON_ON,
    ],
};

/// Folio's palette, fonts, type scale and spacing (templates spec 3.10):
/// a neutral page, ink-dark text, one violet accent, and a tight scale
/// in the system's own interface face. A value in the site's own
/// `theme/tokens.toml` wins.
const TOKENS: &str = r##"[colors]
background = "#ffffff"
surface = "#f4f4f2"
text = "#141418"
muted = "#5b5b66"
link = "#4b36d1"
accent = "#4b36d1"
border = "#e4e4df"

[fonts]
heading = "\"Inter Display\", Inter, \"SF Pro Display\", -apple-system, BlinkMacSystemFont, \"Segoe UI Variable Display\", \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif"
body = "Inter, -apple-system, BlinkMacSystemFont, \"Segoe UI Variable Text\", \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif"
mono = "\"JetBrains Mono\", \"SF Mono\", ui-monospace, Menlo, Consolas, monospace"

[type]
body = "1rem 1.0625rem"
h1 = "2.25rem 3.5rem"
h2 = "1.5rem 1.875rem"
h3 = "1.25rem"
h4 = "1.0625rem"
small = "0.875rem"
line_height = 1.65
heading_weight = 700

[space]
unit = "0.5rem"
section = "5rem"

[shape]
radius = "0.75rem"
measure = "42rem"
"##;
