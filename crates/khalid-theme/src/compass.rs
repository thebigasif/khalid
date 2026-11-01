//! Compass (templates spec 3.4): a teaching blog where every guide opens
//! with its trail, its contents and its key points, and ends with share
//! links, its author and the newsletter. It keeps the shared post page,
//! shows the author's name without a photo at the top, turns off reading
//! time and related posts, and sets a calm type scale.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{Pack, slot};

/// The Compass pack.
pub(crate) static COMPASS: Pack = Pack {
    name: "compass",
    summary: "A teaching blog: breadcrumbs, the contents and key points before each guide, share links, the author and the newsletter after it.",
    stylesheet: include_str!("../packs/compass/compass.css"),
    templates: &[],
    tokens: TOKENS,
    reader: &[
        (ReaderSwitch::Byline, false),
        (ReaderSwitch::ReadingTime, false),
        (ReaderSwitch::RelatedPosts, false),
    ],
    options: &[
        parts::on(parts::LOGO),
        slot::CTA_BUTTON_ON,
        parts::on(parts::BREADCRUMBS),
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::Inline),
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["linkedin", "x", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("post"),
    ],
};

/// Compass's type scale, spacing and shape. A value in the site's own
/// `theme/tokens.toml` wins.
const TOKENS: &str = "[type]
body = \"1.0625rem 1.125rem\"
h1 = \"2rem 3rem\"
h2 = \"1.5rem 2rem\"
h3 = \"1.25rem\"
h4 = \"1.125rem\"
line_height = 1.7

[space]
unit = \"0.5rem\"
section = \"3.5rem\"

[shape]
radius = \"0.5rem\"
measure = \"68ch\"
";
