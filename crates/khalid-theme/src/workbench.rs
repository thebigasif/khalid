//! Workbench: a tools-and-comparisons blog (templates spec 3.7). A
//! featured post and the newest posts up top, the newsletter twice on the
//! front page, and posts built for comparing: a subtitle, an open contents
//! box, wide "at a glance" tables, pros and cons, and a plain closing list
//! of six related links.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The Workbench pack.
pub(crate) static WORKBENCH: Pack = Pack {
    name: "workbench",
    summary: "For reviews and comparisons of tools: a featured post, the newest posts, wide comparison tables, pros and cons, and the newsletter twice on the front page.",
    stylesheet: include_str!("../packs/workbench/workbench.css"),
    templates: &[
        PackTemplate {
            name: "home.html",
            source: include_str!("../packs/workbench/home.html"),
        },
        PackTemplate {
            name: "post.html",
            source: include_str!("../packs/workbench/post.html"),
        },
    ],
    tokens: "[shape]\nradius = \"0.625rem\"\nmeasure = \"72ch\"\n\n\
             [type]\nbody = \"1.0625rem 1.125rem\"\nh1 = \"2.25rem 3rem\"\n\
             h2 = \"1.625rem 2rem\"\nh3 = \"1.25rem\"\nline_height = 1.65\n",
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::on(parts::CARD_READING_TIME),
        parts::CARD_AUTHOR_ROLE,
        parts::on(parts::FEATURED),
        parts::pagination(PageLinks::Pages),
        parts::related(RelatedLayout::Links),
        parts::related_count(6),
        parts::contents(ContentsPlacement::Inline),
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["linkedin", "x", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "lead_image",
            help: "Whether a post shows its lead picture under its title.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "closing_line",
            help: "Whether a post ends with one line naming its writers and when it was published and updated.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "newsletter_note",
            help: "One line under the newsletter form on the front page, such as how often it goes out.",
            kind: OptionKind::Lines { most: 1 },
        },
    ],
};
