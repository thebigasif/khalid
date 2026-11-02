//! Atlas: a publication look for a blog with many topics (templates spec
//! 3.2). A masthead with the topics, the featured posts, picture cards and
//! an index of every topic lead the front page; posts open with
//! breadcrumbs and a large title, with their contents in a sticky rail.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The longest tagline, about two lines under the name on a phone.
const TAGLINE_CHARACTERS: usize = 140;

/// The Atlas pack.
pub(crate) static ATLAS: Pack = Pack {
    name: "atlas",
    summary: "A publication for a blog with many topics: a masthead, featured stories, picture cards and a topic index.",
    stylesheet: include_str!("../packs/atlas/atlas.css"),
    templates: &[PackTemplate {
        name: "home.html",
        source: include_str!("../packs/atlas/home.html"),
    }],
    tokens: include_str!("../packs/atlas/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "tagline",
            help: "One line under the blog's name on the front page, such as what the blog is about.",
            kind: OptionKind::Line {
                most: TAGLINE_CHARACTERS,
            },
        },
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::FEATURED),
        parts::picks(4),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::CARD_READING_TIME),
        parts::CARD_AUTHOR_ROLE,
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::SideStart),
        parts::contents_numbers(ContentsNumbers::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(3),
        slot::share("end"),
        slot::share_networks(&["x", "linkedin", "facebook", "email"]),
        slot::newsletter("none"),
        slot::cta_band("both"),
        slot::CTA_BUTTON_ON,
    ],
};
