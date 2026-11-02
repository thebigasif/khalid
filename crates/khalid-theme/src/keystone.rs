//! Keystone (templates spec 3.3): a calm, picture-led blog for a product or
//! a company, with a lead story, a grid of framed pictures and a reading
//! column with share links beside it. It replaces the home and post pages,
//! keeps every reader feature on, and sets a tight type scale on a white
//! page with one blue accent.

use crate::parts::{self, CardShape, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Keystone pack.
pub(crate) static KEYSTONE: Pack = Pack {
    name: "keystone",
    summary: "A product or company blog: a lead story, a grid of framed pictures, a calm reading column with share links beside it, the author and related posts after it.",
    stylesheet: include_str!("../packs/keystone/keystone.css"),
    templates: &[
        PackTemplate {
            name: "home.html",
            source: include_str!("../packs/keystone/home.html"),
        },
        PackTemplate {
            name: "post.html",
            source: include_str!("../packs/keystone/post.html"),
        },
    ],
    tokens: TOKENS,
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        slot::CTA_BUTTON_ON,
        parts::on(parts::DEK),
        parts::on(parts::FEATURED),
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::None),
        slot::share("top"),
        slot::share_networks(&["x", "linkedin", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("post"),
        OptionSpec {
            key: "numbered_headings",
            help: "Whether each main heading of a post gets a chapter number: 01, 02, 03.",
            kind: OptionKind::Switch { default: false },
        },
        OptionSpec {
            key: "subscriber_line",
            help: "One line over the newsletter form on the front page, such as Join 4,000 readers.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "about",
            help: "Up to two lines about the site after each post, such as who writes it.",
            kind: OptionKind::Lines { most: 2 },
        },
    ],
};

/// Keystone's type scale, spacing and shape. A value in the site's own
/// `theme/tokens.toml` wins.
const TOKENS: &str = "[colors]
background = \"#ffffff\"
surface = \"#f5f7fa\"
text = \"#0d1526\"
muted = \"#4f5b6d\"
link = \"#2340c4\"
accent = \"#3355e0\"
border = \"#e3e8ef\"

[fonts]
body = \"-apple-system, BlinkMacSystemFont, \\\"Segoe UI Variable Text\\\", \\\"Segoe UI\\\", system-ui, Roboto, \\\"Helvetica Neue\\\", sans-serif\"
heading = \"-apple-system, BlinkMacSystemFont, \\\"Segoe UI Variable Display\\\", \\\"Segoe UI\\\", system-ui, Roboto, \\\"Helvetica Neue\\\", sans-serif\"

[type]
body = \"1.0625rem 1.125rem\"
h1 = \"2.25rem 3.25rem\"
h2 = \"1.5rem 1.875rem\"
h3 = \"1.1875rem 1.3125rem\"
h4 = \"1.0625rem\"
small = \"0.875rem 0.9375rem\"
line_height = 1.65
heading_weight = 500

[space]
unit = \"0.5rem\"
section = \"5rem\"

[shape]
radius = \"0.625rem\"
measure = \"42rem\"
";
