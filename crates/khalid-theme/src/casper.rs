//! Casper: a classic blog, based on Ghost's Casper theme (MIT licence, see
//! `packs/casper/LICENSE-ghost.txt`). A cover band with the blog's name
//! on the accent colour and a picture under it, a feed with one large card, two
//! half cards and then three across, and a post in one serif column with
//! the sign-up box and three more posts under it.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Casper pack.
pub(crate) static CASPER: Pack = Pack {
    name: "casper",
    summary: "A classic blog based on Ghost's Casper theme: a cover band with the blog's name on the accent colour and a picture under it, a feed with one large card and then a grid, and a calm serif column for each post.",
    css: PackText::file("casper/casper.css"),
    templates: &[
        PackTemplate::file("body_attrs.html", "casper/body_attrs.html"),
        PackTemplate::file("home.html", "casper/home.html"),
        PackTemplate::file("post.html", "casper/post.html"),
    ],
    tokens: include_str!("../packs/casper/tokens.toml"),
    reader: &[(ReaderSwitch::AuthorBox, false)],
    options: &[
        OptionSpec {
            key: "navigation",
            help: "Where the header puts the logo: cover (at the start, and on the front page's cover in place of the header), middle (in the middle, the menu before it) or stacked (in the middle, the menu on a row under it).",
            kind: OptionKind::Choice {
                allowed: &["cover", "middle", "stacked"],
                default: "cover",
            },
        },
        OptionSpec {
            key: "cover",
            help: "How the front page opens: picture (the blog's name on the accent colour, the newest featured post's picture under it), accent (the accent colour alone), plain (the name on the page's own colour) or none.",
            kind: OptionKind::Choice {
                allowed: &["picture", "accent", "plain", "none"],
                default: "picture",
            },
        },
        OptionSpec {
            key: "cover_align",
            help: "Whether the front page's cover centres its words or sets them at the start.",
            kind: OptionKind::Choice {
                allowed: &["center", "start"],
                default: "center",
            },
        },
        OptionSpec {
            key: "feed",
            help: "The front page's posts: classic (one large card, two half cards, then three across), grid (three across) or list (every post as a large card).",
            kind: OptionKind::Choice {
                allowed: &["classic", "grid", "list"],
                default: "classic",
            },
        },
        OptionSpec {
            key: "post_image",
            help: "How wide a post's lead picture is: wide (wider than the text), full (the whole window), small (as wide as the text) or hidden.",
            kind: OptionKind::Choice {
                allowed: &["wide", "full", "small", "hidden"],
                default: "wide",
            },
        },
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::CARD_READING_TIME),
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("top"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("post"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
