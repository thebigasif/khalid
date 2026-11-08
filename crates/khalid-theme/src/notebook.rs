//! Notebook: one person's public notebook of short notes and links
//! (docs/templates/notebook.md). Every page is a stream of dated notes,
//! newest first under month headings, with each date in the margin beside
//! a double margin rule, like an exercise book. A note whose first fact is
//! a web address is a link note: the fact's label names the place, and
//! the stream and the post link to it. A side column on the front page
//! says what the notebook is and lists its sections. Related notes use
//! the shared `feed_list` section part in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Notebook pack.
pub(crate) static NOTEBOOK: Pack = Pack {
    name: "notebook",
    summary: "One person's public notebook: short notes and links in a dated stream, newest first under month headings, each date in the margin beside a double margin rule, and link notes that point to what they share.",
    css: PackText::file("notebook/notebook.css"),
    templates: &[
        PackTemplate::file("home.html", "notebook/home.html"),
        PackTemplate::file("post.html", "notebook/post.html"),
        PackTemplate::file("listing.html", "notebook/listing.html"),
        PackTemplate::file("parts-notebook.html", "notebook/parts-notebook.html"),
    ],
    tokens: PackText::file("notebook/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "tagline",
            help: "One line beside the site's name at the top of the front page, such as whose notebook it is and what it holds. It also fills the About this notebook box when the front page has no intro.",
            kind: OptionKind::Lines { most: 1 },
        },
        parts::LOGO,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "hacker-news", "email"]),
        slot::newsletter("home"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
