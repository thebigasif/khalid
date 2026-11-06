//! Ease: a help centre or documentation site, based on Ghost's Ease theme
//! (MIT, see `packs/ease/LICENSE-ghost.txt`). A band in the accent with a
//! large search box, a strip of featured articles, then one section per
//! category, two across, each a list of its newest articles.

use crate::parts::{self, PageLinks};
use crate::{Pack, PackTemplate, PackText, slot};

/// The Ease pack.
pub(crate) static EASE: Pack = Pack {
    name: "ease",
    summary: "A help centre or documentation site: a band with a large search box, a strip of featured articles, then each category as a list of its newest articles, two across.",
    css: PackText::file("ease/ease.css"),
    templates: &[
        PackTemplate::file("home.html", "ease/home.html"),
        PackTemplate::file("post.html", "ease/post.html"),
        PackTemplate::file("listing.html", "ease/listing.html"),
        PackTemplate::file("author.html", "ease/author.html"),
    ],
    tokens: include_str!("../packs/ease/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::DEK,
        parts::UPDATED_NOTE,
        parts::picks(4),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::HEADER_SEARCH,
        slot::share("top"),
        slot::share_networks(&["x", "linkedin", "facebook", "email"]),
        slot::newsletter("none"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
    ],
};
