//! Keystone (templates spec 3.3): a personal guide site with a big featured
//! post, a grid of pictures and numbered chapters. It replaces the home
//! and post pages, turns off the author box, related posts, reading time
//! and contents, and sets a larger type scale with wide spacing.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Keystone pack.
pub(crate) static KEYSTONE: Pack = Pack {
    name: "keystone",
    summary: "A personal guide site: one big featured post, a grid of pictures, numbered chapters and share links beside the post.",
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
    reader: &[
        (ReaderSwitch::ReadingTime, false),
        (ReaderSwitch::AuthorBox, false),
        (ReaderSwitch::RelatedPosts, false),
    ],
    options: &[
        parts::on(parts::LOGO),
        slot::CTA_BUTTON_ON,
        parts::on(parts::FEATURED),
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::None),
        slot::share("top"),
        slot::share_networks(&["x", "linkedin", "facebook", "email"]),
        slot::newsletter("home"),
        slot::cta_band("none"),
        OptionSpec {
            key: "numbered_headings",
            help: "Whether each main heading of a post gets a chapter number: 01, 02, 03.",
            kind: OptionKind::Switch { default: true },
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
const TOKENS: &str = "[type]
body = \"1.125rem 1.25rem\"
h1 = \"2.5rem 3.75rem\"
h2 = \"1.75rem 2.25rem\"
h3 = \"1.375rem\"
h4 = \"1.125rem\"
line_height = 1.7
heading_weight = 800

[space]
unit = \"0.5rem\"
section = \"4rem\"

[shape]
radius = \"0.75rem\"
measure = \"66ch\"
";
