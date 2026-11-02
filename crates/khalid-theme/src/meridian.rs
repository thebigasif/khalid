//! Meridian: a calm company journal, with a reference variant for long
//! evergreen explainers (templates spec 3.9).

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, PackTemplate, slot};

/// The Meridian pack.
pub(crate) static MERIDIAN: Pack = Pack {
    name: "meridian",
    summary: "A calm company journal: serif titles, a lead story, category tabs, wide rows with a picture, several authors with their roles, and a reference layout for long guides.",
    stylesheet: include_str!("../packs/meridian/meridian.css"),
    templates: &[
        PackTemplate {
            name: "home.html",
            source: include_str!("../packs/meridian/home.html"),
        },
        PackTemplate {
            name: "post-reference.html",
            source: include_str!("../packs/meridian/post-reference.html"),
        },
    ],
    tokens: TOKENS,
    reader: &[(ReaderSwitch::ReadingTime, false)],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::DEK,
        parts::cards(CardShape::Row),
        parts::on(parts::CARD_AUTHOR_ROLE),
        parts::on(parts::CATEGORY_TABS),
        parts::picks(3),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Scroll),
        parts::related_count(3),
        parts::contents(ContentsPlacement::None),
        parts::contents_numbers(ContentsNumbers::Nested),
        slot::share("none"),
        slot::share_networks(&["linkedin", "x", "email"]),
        slot::newsletter("both"),
        slot::cta_band("post"),
        slot::CTA_BUTTON_ON,
    ],
};

/// Meridian's palette, serif titles, type scale and spacing (templates
/// spec 3.9): ink on a warm paper white with one deep indigo accent. A
/// value in the site's own `theme/tokens.toml` wins.
const TOKENS: &str = r##"[colors]
background = "#fcfbf8"
surface = "#f3f0e9"
text = "#17191e"
muted = "#585c64"
link = "#2c3f9c"
accent = "#2c3f9c"
border = "#e2ddd2"

[fonts]
heading = '"Iowan Old Style", Charter, "Bitstream Charter", "Sitka Text", Cambria, Georgia, serif'

[type]
body = "1.0625rem 1.1875rem"
h1 = "2.5rem 4rem"
h2 = "1.625rem 2rem"
h3 = "1.25rem 1.375rem"
h4 = "1.125rem"
small = "0.875rem"
line_height = 1.7
heading_weight = 600

[space]
unit = "0.5rem"
section = "5rem"

[shape]
radius = "0.375rem"
measure = "42rem"
"##;
