//! Chapter: an author's site (docs/templates/chapter.md). Books, events
//! and writing are categories, named by the `books_category` and
//! `events_category` options. The front page opens with the newest book
//! beside its cover, then shows the books as a shelf of covers, the
//! writing as a contents page with dotted leaders and the events as dated
//! rows, each part opened like a chapter. An excerpt is set as a page of
//! the book. Built from the shared section parts in
//! `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The longest category slug an option takes.
const SLUG_CHARACTERS: usize = 60;

/// The Chapter pack.
pub(crate) static CHAPTER: Pack = Pack {
    name: "chapter",
    summary: "For an author: the newest book beside its cover, the books as a shelf of covers, excerpts and essays as a book's contents page, and events as dated rows. Excerpts are set as a page of the book.",
    css: PackText::file("chapter/chapter.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "chapter/picture_sizes.html"),
        PackTemplate::file("home.html", "chapter/home.html"),
        PackTemplate::file("post.html", "chapter/post.html"),
        PackTemplate::file("listing.html", "chapter/listing.html"),
        PackTemplate::file("authors.html", "chapter/authors.html"),
        PackTemplate::file("author.html", "chapter/author.html"),
    ],
    tokens: PackText::file("chapter/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(4),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the author's name at the top of the front page, such as what they write.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "books_category",
            help: "The slug of the category that holds your books. Empty means books.",
            kind: OptionKind::Line {
                most: SLUG_CHARACTERS,
            },
        },
        OptionSpec {
            key: "events_category",
            help: "The slug of the category that holds your events. Empty means events.",
            kind: OptionKind::Line {
                most: SLUG_CHARACTERS,
            },
        },
        OptionSpec {
            key: "excerpts_category",
            help: "The slug of the category whose posts are set as a page of the book, with a drop cap. Empty means excerpts.",
            kind: OptionKind::Line {
                most: SLUG_CHARACTERS,
            },
        },
        OptionSpec {
            key: "events_count",
            help: "How many events show on the front page.",
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
