//! Dialogue: an interview show, each episode a conversation with one
//! guest (docs/templates/dialogue.md). An episode is a post with facts
//! (the guest, who they are, the length, one line they said): the guest
//! leads every row and the front page, their line is the quote line, and
//! each episode opens with a name plate of host and guest. Built from the
//! shared section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Dialogue pack.
pub(crate) static DIALOGUE: Pack = Pack {
    name: "dialogue",
    summary: "For an interview show: each episode led by its guest, with their name, who they are and one line they said as a large quote, a name plate of host and guest on every episode, pull quotes and transcript speakers in the text, and the topics of each conversation.",
    css: PackText::file("dialogue/dialogue.css"),
    templates: &[
        PackTemplate::file("home.html", "dialogue/home.html"),
        PackTemplate::file("post.html", "dialogue/post.html"),
        PackTemplate::file("listing.html", "dialogue/listing.html"),
        PackTemplate::file("authors.html", "dialogue/authors.html"),
        PackTemplate::file("author.html", "dialogue/author.html"),
    ],
    tokens: include_str!("../packs/dialogue/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["whatsapp", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the show's name at the top of the front page, such as who the guests are.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "listen_links",
            help: "Up to four places to listen, under the tagline, each written \"label | address\", such as \"Your podcast app | https://example.com/feed.xml\".",
            kind: OptionKind::Lines { most: 4 },
        },
        OptionSpec {
            key: "notes_count",
            help: "How many notes, posts without facts, show on the front page under the conversations.",
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
