//! Edition: a newsletter with a big cover, based on Ghost's Edition theme
//! (MIT, see `packs/edition/LICENSE-ghost.txt`). A cover from edge to edge
//! with the publication's pitch and the sign-up form, a strip of featured
//! posts, then the latest posts in one quiet column.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The most characters the cover's pitch may have: three lines of the
/// cover's large type on a wide screen.
const COVER_TEXT_CHARACTERS: usize = 160;

/// The Edition pack.
pub(crate) static EDITION: Pack = Pack {
    name: "edition",
    summary: "A newsletter with a big cover: a picture from edge to edge with the pitch and the sign-up form, a strip of featured posts, then the latest posts in one quiet column.",
    css: PackText::file("edition/edition.css"),
    templates: &[
        PackTemplate::file("home.html", "edition/home.html"),
        PackTemplate::file("post.html", "edition/post.html"),
    ],
    tokens: include_str!("../packs/edition/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "cover",
            help: "How tall the front page's cover is when a post has a picture for it: full fills the screen, half fills about half, none leaves the cover out.",
            kind: OptionKind::Choice {
                allowed: &["full", "half", "none"],
                default: "full",
            },
        },
        OptionSpec {
            key: "cover_text",
            help: "The line set large on the cover, such as what the newsletter is about. Without it the cover shows the front page's intro.",
            kind: OptionKind::Line {
                most: COVER_TEXT_CHARACTERS,
            },
        },
        OptionSpec {
            key: "feed_layout",
            help: "How the latest posts are listed: expanded rows with a picture at the start, thumbnail-end with the picture at the end, text-only, or minimal one-line rows with a date block.",
            kind: OptionKind::Choice {
                allowed: &["expanded", "thumbnail-end", "text-only", "minimal"],
                default: "expanded",
            },
        },
        parts::picks(6),
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::on(parts::CARD_READING_TIME),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Row),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::HEADER_SEARCH,
        slot::share("end"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("both"),
        slot::cta_band("post"),
        slot::CTA_BUTTON_ON,
    ],
};
