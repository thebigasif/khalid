//! Versions: a software product's changelog (docs/templates/versions.md).
//! Each change is a post: its category is its type, such as New,
//! Improved or Fixed, and its first fact is the version it shipped in.
//! The front page and every list show the changes on a version rail,
//! with a filter row of plain links to each type, the versions on the
//! page as anchor links and a box to get updates by email or feed.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Versions pack.
pub(crate) static VERSIONS: Pack = Pack {
    name: "versions",
    summary: "For a software product's changelog: every change on a version rail, a filter row by type of change (new, improved, fixed) that works without a script, the versions as anchor links and a box to get updates by email or feed.",
    css: PackText::file("versions/versions.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "versions/picture_sizes.html"),
        PackTemplate::file("home.html", "versions/home.html"),
        PackTemplate::file("post.html", "versions/post.html"),
        PackTemplate::file("listing.html", "versions/listing.html"),
        PackTemplate::file("category.html", "versions/category.html"),
        PackTemplate::file("authors.html", "versions/authors.html"),
        PackTemplate::file("author.html", "versions/author.html"),
    ],
    tokens: include_str!("../packs/versions/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("none"),
        slot::share_networks(&["linkedin", "bluesky", "email"]),
        slot::newsletter("post"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "tagline",
            help: "One line under the product's name at the top of the front page, such as what the product does.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "change_types",
            help: "The slugs of the categories that are types of change, in order: the first is marked as new, the second as improved, the third as fixed and a fourth, such as removed, with a dashed edge. Without it, new, improved and fixed.",
            kind: OptionKind::Lines { most: 4 },
        },
        OptionSpec {
            key: "feed_links",
            help: "Whether the update box and each type's page link to their feeds, so a reader can follow every change or only one type.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "version_index",
            help: "Whether the front page and lists show the versions on the page beside the timeline, as links to their place in it.",
            kind: OptionKind::Switch { default: true },
        },
    ],
};
