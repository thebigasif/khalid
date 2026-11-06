//! Digest: a compact newsletter blog, based on Ghost's Digest theme (MIT
//! licence, see `packs/digest/LICENSE-ghost.txt`). The front page leads
//! with the latest issue, set like a post, then a subscribe box, the
//! featured issues in soft panels and every past issue as a ruled list.
//! A post opens on a grey band with its picture overlapping the band's
//! foot.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Digest pack.
pub(crate) static DIGEST: Pack = Pack {
    name: "digest",
    summary: "A compact newsletter blog: the latest issue first, a subscribe box, featured issues in soft panels and a ruled list of past issues.",
    css: PackText::file("digest/digest.css"),
    templates: &[
        PackTemplate::file("home.html", "digest/home.html"),
        PackTemplate::file("post.html", "digest/post.html"),
    ],
    tokens: include_str!("../packs/digest/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "image_width",
            help: "How wide the lead picture of the latest issue and of a post runs: small keeps it to the text, wide runs it wider, full runs it from edge to edge.",
            kind: OptionKind::Choice {
                allowed: &["small", "wide", "full"],
                default: "small",
            },
        },
        parts::on(parts::LOGO),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::CARD_READING_TIME,
        parts::picks(3),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::None),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        parts::related(RelatedLayout::List),
        parts::related_count(3),
        slot::share("top"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
