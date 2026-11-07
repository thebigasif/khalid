//! Quill: a minimal text blog (docs/templates/quill.md). The shared pages
//! in one column of generous type, with every list one line a post, the
//! title and its date, and no pictures on lists. Its mark is the nib, a
//! small accent square on its point before the site's name and under the
//! end of every post. Quill replaces no template file, so its own rules
//! are only tokens and a short stylesheet, the smallest of every pack.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, PackText, slot};

/// The Quill pack.
pub(crate) static QUILL: Pack = Pack {
    name: "quill",
    summary: "A minimal text blog: one column of generous type, every list one line a post with its date, and no pictures on lists, so its pages load fastest.",
    css: PackText::file("quill/quill.css"),
    templates: &[],
    tokens: include_str!("../packs/quill/tokens.toml"),
    reader: &[],
    options: &[
        parts::LOGO,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Compact),
        parts::pagination(PageLinks::Pages),
        parts::related(RelatedLayout::Links),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("none"),
        slot::share_networks(&["bluesky", "email"]),
        slot::newsletter("post"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
