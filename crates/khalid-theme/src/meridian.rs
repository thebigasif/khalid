//! Meridian: a calm company journal, with a reference variant for long
//! evergreen explainers (templates spec 3.9).

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, slot};

/// The Meridian pack.
pub(crate) static MERIDIAN: Pack = Pack {
    name: "meridian",
    summary: "A calm company journal: category tabs, wide rows with a square picture, several authors with their roles, and a reference layout for long guides.",
    stylesheet: include_str!("../packs/meridian/meridian.css"),
    templates: &[],
    tokens: TOKENS,
    reader: &[
        (ReaderSwitch::ReadingTime, false),
        (ReaderSwitch::AuthorBox, false),
        (ReaderSwitch::RelatedPosts, false),
    ],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::DEK,
        parts::cards(CardShape::Row),
        parts::on(parts::CARD_AUTHOR_ROLE),
        parts::on(parts::CATEGORY_TABS),
        parts::picks(3),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::List),
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

/// Meridian's type scale and spacing (templates spec 3.9). A value in the
/// site's own `theme/tokens.toml` wins.
const TOKENS: &str = r#"[type]
body = "1.0625rem 1.1875rem"
h1 = "2.5rem 4rem"
h2 = "1.75rem 2.25rem"
h3 = "1.3125rem"
line_height = 1.7

[space]
section = "4.5rem"

[shape]
radius = "0.5rem"
measure = "64ch"
"#;
