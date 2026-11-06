//! Dawn: a calm, text-led blog and newsletter, based on Ghost's Dawn theme
//! (MIT, see `packs/dawn/LICENSE-ghost.txt`). A small centred cover with
//! the sign-up, a row of featured posts, then every post as one line with
//! a day and month block, the title and the reading time.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The most characters the featured row's heading may have.
const FEATURED_TITLE_CHARACTERS: usize = 60;

/// The Dawn pack.
pub(crate) static DAWN: Pack = Pack {
    name: "dawn",
    summary: "A calm, text-led blog and newsletter: a small centred cover with the sign-up, a row of featured posts, then every post as one line with a day and month block, the title and the reading time.",
    css: PackText::file("dawn/dawn.css"),
    templates: &[
        PackTemplate::file("home.html", "dawn/home.html"),
        PackTemplate::file("post.html", "dawn/post.html"),
        PackTemplate::file("listing.html", "dawn/listing.html"),
        PackTemplate::file("author.html", "dawn/author.html"),
    ],
    tokens: include_str!("../packs/dawn/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "featured_title",
            help: "The heading over the front page's row of featured posts. Without it the row is headed Featured.",
            kind: OptionKind::Line {
                most: FEATURED_TITLE_CHARACTERS,
            },
        },
        parts::picks(6),
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::on(parts::CARD_READING_TIME),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::HEADER_SEARCH,
        slot::share("top"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("home"),
        slot::cta_band("post"),
        slot::CTA_BUTTON_ON,
    ],
};
