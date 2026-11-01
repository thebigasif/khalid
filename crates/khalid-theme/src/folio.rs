//! Folio: a product team's essays and engineering notes under the blog's
//! own masthead (templates spec 3.10).

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Folio pack.
pub(crate) static FOLIO: Pack = Pack {
    name: "folio",
    summary: "A product team's essays under their own masthead: a featured story, outlined picture cards, share links and an \"All posts\" link on every essay.",
    stylesheet: include_str!("../packs/folio/folio.css"),
    templates: &[PackTemplate {
        name: "home.html",
        source: include_str!("../packs/folio/home.html"),
    }],
    tokens: TOKENS,
    reader: &[
        (ReaderSwitch::ReadingTime, false),
        (ReaderSwitch::AuthorBox, false),
        (ReaderSwitch::RelatedPosts, false),
    ],
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
        parts::on(parts::BACK_LINK),
        parts::BREADCRUMBS,
        parts::on(parts::CATEGORY_TABS),
        parts::cards(CardShape::Image),
        parts::on(parts::CARD_AUTHOR_ROLE),
        parts::on(parts::FEATURED),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(3),
        parts::contents(ContentsPlacement::None),
        slot::share("both"),
        slot::share_networks(&["x", "linkedin", "bluesky", "email"]),
        slot::newsletter("none"),
        slot::cta_band("post"),
        slot::CTA_BUTTON_ON,
    ],
};

/// Folio's type scale and spacing (templates spec 3.10). A value in the
/// site's own `theme/tokens.toml` wins.
const TOKENS: &str = r#"[type]
body = "1.0625rem 1.125rem"
h1 = "2.5rem 3.5rem"
h2 = "1.5rem 1.875rem"
line_height = 1.7

[space]
section = "4rem"

[shape]
radius = "0.75rem"
measure = "66ch"
"#;
