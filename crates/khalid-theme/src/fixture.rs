//! Fixture: a sports club (docs/templates/fixture.md). Matches are posts
//! with facts (opponent, result or kick-off, ground, competition), shown
//! as scoreboard rows on the front page, on every list and on each
//! person's page. The front page leads with the next match and the
//! club's form; a match report opens with a scoreboard. Built from the
//! shared section parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Fixture pack.
pub(crate) static FIXTURE: Pack = Pack {
    name: "fixture",
    summary: "For a sports club: the next match and the club's form on the front page, results as scoreboard rows with the score in large figures, upcoming fixtures, club news, a squad page and a scoreboard on every match report.",
    css: PackText::file("fixture/fixture.css"),
    templates: &[
        PackTemplate::file("home.html", "fixture/home.html"),
        PackTemplate::file("post.html", "fixture/post.html"),
        PackTemplate::file("listing.html", "fixture/listing.html"),
        PackTemplate::file("authors.html", "fixture/authors.html"),
        PackTemplate::file("author.html", "fixture/author.html"),
    ],
    tokens: include_str!("../packs/fixture/tokens.toml"),
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
        slot::share_networks(&["whatsapp", "facebook", "email"]),
        slot::newsletter("post"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "tagline",
            help: "One line under the club's name at the top of the front page, such as where and what the club plays.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "results_count",
            help: "How many results, matches whose second fact starts with W, D or L, show on the front page.",
            kind: OptionKind::Count {
                least: 1,
                most: 10,
                default: 6,
            },
        },
        OptionSpec {
            key: "news_count",
            help: "How many news posts, posts without facts, show beside the results on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 4,
            },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};
