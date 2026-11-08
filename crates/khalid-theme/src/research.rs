//! Research: a research lab or academic group (docs/templates/research.md).
//! Papers are posts with facts (venue, year, DOI, citations), shown as a
//! numbered reference register on the front page, on every list and on
//! each person's page. A paper ends with a "Cite this" box built from its
//! own data. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Research pack.
pub(crate) static RESEARCH: Pack = Pack {
    name: "research",
    summary: "For a research lab or academic group: papers as a numbered reference list with venue, year, DOI and citations, a news feed, research areas, a people page and a cite box on every paper.",
    css: PackText::file("research/research.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "research/picture_sizes.html"),
        PackTemplate::file("home.html", "research/home.html"),
        PackTemplate::file("post.html", "research/post.html"),
        PackTemplate::file("listing.html", "research/listing.html"),
        PackTemplate::file("authors.html", "research/authors.html"),
        PackTemplate::file("author.html", "research/author.html"),
    ],
    tokens: PackText::file("research/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["bluesky", "linkedin", "email"]),
        slot::newsletter("post"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the lab's name at the top of the front page, such as what the group studies.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "cite_box",
            help: "Whether a paper, a post with facts, ends with a Cite this box: the reference in one line and a BibTeX entry, built from the post's authors, title and facts.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "news_count",
            help: "How many news posts, posts without facts, show beside the publications on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 5,
            },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
