//! Channel: a video creator's home (docs/templates/channel.md). The newest
//! post with a video leads the front page as Now showing, found the way
//! the `video_section` row finds it; every other post is a thumbnail card
//! with the play bar, its running time read from the post's first fact;
//! and each playlist (category) is a shelf of its newest videos. A post
//! opens on its video, set wider than the text, and ends with Up next.
//! Built from the shared section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Channel pack.
pub(crate) static CHANNEL: Pack = Pack {
    name: "channel",
    summary: "For a video creator: the newest video large as Now showing, thumbnail cards with a play bar and running time, a shelf per playlist, and posts that open on a wide video with Up next under them.",
    css: PackText::file("channel/channel.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "channel/picture_sizes.html"),
        PackTemplate::file("home.html", "channel/home.html"),
        PackTemplate::file("post.html", "channel/post.html"),
        PackTemplate::file("listing.html", "channel/listing.html"),
        PackTemplate::file("authors.html", "channel/authors.html"),
        PackTemplate::file("author.html", "channel/author.html"),
    ],
    tokens: PackText::file("channel/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(6),
        parts::contents(ContentsPlacement::None),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "facebook", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the channel's name at the top of the front page, such as what the channel teaches.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "shelves",
            help: "How many playlists, one per category, show as a shelf of their newest videos on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 6,
                default: 4,
            },
        },
        parts::HERO_STATS,
    ],
};
