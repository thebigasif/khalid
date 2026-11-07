//! Reel: film and TV reviews (docs/templates/reel.md). A review is a post
//! with facts (rating, release, cast, where to watch), read by position.
//! Each verdict sits on a ticket with a star rating; the front
//! page shows the lead review, poster cards, a score sheet ranked by
//! rating and a feed of features. Built from the
//! shared section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Reel pack.
pub(crate) static REEL: Pack = Pack {
    name: "reel",
    summary: "For film and TV reviews: a star rating on a ticket, the cast and where to watch on every review, poster cards, a score sheet ranked by rating and a feed of features.",
    css: PackText::file("reel/reel.css"),
    templates: &[
        PackTemplate::file("home.html", "reel/home.html"),
        PackTemplate::file("post.html", "reel/post.html"),
        PackTemplate::file("listing.html", "reel/listing.html"),
        PackTemplate::file("authors.html", "reel/authors.html"),
        PackTemplate::file("author.html", "reel/author.html"),
    ],
    tokens: include_str!("../packs/reel/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::on(parts::FEATURED),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["bluesky", "whatsapp", "email"]),
        slot::newsletter("home"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the site's name at the top of the front page, such as what the critics review.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "score_sheet",
            help: "How many reviews, posts with facts, the score sheet on the front page ranks by rating. 0 leaves the score sheet out.",
            kind: OptionKind::Count {
                least: 0,
                most: 20,
                default: 8,
            },
        },
        OptionSpec {
            key: "features_count",
            help: "How many features, posts without facts such as lists and interviews, show in the feed on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 4,
            },
        },
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
