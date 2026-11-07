//! Ruby: a magazine of picture cards, based on Ghost's Ruby theme (MIT
//! licence, see `packs/ruby/LICENSE-ghost.txt`). The front page is a grid
//! of white cards, four across on a wide screen, on a page that fades to
//! light grey. Each card shows its picture, its topic, its title, a short
//! summary and a footer with the date and the author's photo. A post reads
//! in one column under a wide picture, with more cards in a grey band
//! after it.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks, RelatedLayout};
use crate::{Pack, PackTemplate, PackText, slot};

/// The Ruby pack.
pub(crate) static RUBY: Pack = Pack {
    name: "ruby",
    summary: "A magazine of picture cards based on Ghost's Ruby theme: a grid of white cards four across, each with its picture, topic, title, summary and author, and a calm column for each post with more cards after it.",
    css: PackText::file("ruby/ruby.css"),
    templates: &[
        PackTemplate::file("home.html", "ruby/home.html"),
        PackTemplate::file("post.html", "ruby/post.html"),
    ],
    tokens: include_str!("../packs/ruby/tokens.toml"),
    reader: &[(ReaderSwitch::AuthorBox, false)],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::CARD_READING_TIME,
        parts::pagination(PageLinks::Numbered),
        parts::related(RelatedLayout::Grid),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::contents_numbers(ContentsNumbers::None),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        slot::share("end"),
        slot::share_networks(&["x", "facebook", "linkedin", "email"]),
        slot::newsletter("post"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
