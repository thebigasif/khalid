//! Margin: one writer's personal site of essays
//! (docs/templates/margin.md). Every page is a text column with a margin
//! beside it on a wide screen, split by one ruled line. The front page
//! has a short introduction with the writer in the margin, a sign-up band
//! with its notes in the margin, then the essays as rows with their date,
//! reading time and topic in the margin. On an essay the margin holds the
//! byline and date, then the essay's own footnotes, which stay beside the
//! text as it scrolls. The rows are the pack's own part,
//! `parts-margin.html`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The longest author slug the `writer` option takes.
const WRITER_CHARACTERS: usize = 80;

/// The most notes the sign-up band shows in its margin.
const MOST_LETTER_NOTES: usize = 3;

/// The Margin pack.
pub(crate) static MARGIN: Pack = Pack {
    name: "margin",
    summary: "One writer's personal site: a short introduction, a sign-up and essays, with dates, notes and footnotes set in a ruled margin beside the text.",
    css: PackText::file("margin/margin.css"),
    templates: &[
        PackTemplate::file("home.html", "margin/home.html"),
        PackTemplate::file("post.html", "margin/post.html"),
        PackTemplate::file("listing.html", "margin/listing.html"),
        PackTemplate::file("author.html", "margin/author.html"),
        PackTemplate::file("parts-margin.html", "margin/parts-margin.html"),
    ],
    tokens: PackText::file("margin/tokens.toml"),
    reader: &[],
    options: &[
        OptionSpec {
            key: "writer",
            help: "The slug of the author who shows in the margin of the front page's introduction, such as nell-ashby. Empty shows the newest essay's author.",
            kind: OptionKind::Line {
                most: WRITER_CHARACTERS,
            },
        },
        OptionSpec {
            key: "tagline",
            help: "The short introduction under your name on the front page. The front page's own intro wins when it has one.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "letter_notes",
            help: "Up to three short notes in the margin beside the sign-up, such as One letter a month.",
            kind: OptionKind::Lines {
                most: MOST_LETTER_NOTES,
            },
        },
        parts::LOGO,
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["bluesky", "email"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_OFF,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
