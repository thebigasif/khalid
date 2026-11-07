//! Dispatch: a newsletter whose site's first job is to win sign-ups
//! (docs/templates/dispatch.md). The front page opens on the sign-up, a
//! reply slip with the email field or the call to action, beside the
//! latest issue. Each issue is a post whose first fact is its number,
//! shown in a postage stamp on every list and on the issue itself, and
//! the reply slip follows every issue and every list.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Dispatch pack.
pub(crate) static DISPATCH: Pack = Pack {
    name: "dispatch",
    summary: "For a newsletter whose site wins sign-ups: the sign-up first on the front page beside the latest issue, each issue marked by its number in a postage stamp, and a reply slip to join after every issue and list.",
    css: PackText::file("dispatch/dispatch.css"),
    templates: &[
        PackTemplate::file("home.html", "dispatch/home.html"),
        PackTemplate::file("post.html", "dispatch/post.html"),
        PackTemplate::file("listing.html", "dispatch/listing.html"),
        PackTemplate::file("authors.html", "dispatch/authors.html"),
        PackTemplate::file("author.html", "dispatch/author.html"),
        PackTemplate::file("parts-dispatch.html", "dispatch/parts-dispatch.html"),
    ],
    tokens: include_str!("../packs/dispatch/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::BREADCRUMBS,
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Row),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(3),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::None),
        slot::share("end"),
        slot::share_networks(&["email", "bluesky", "linkedin"]),
        slot::newsletter("both"),
        slot::cta_band("none"),
        slot::CTA_BUTTON_ON,
        OptionSpec {
            key: "tagline",
            help: "The newsletter's promise in one line, under its name at the top of the front page.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "cadence",
            help: "When the newsletter goes out, such as every Saturday morning, shown above its name and in every sign-up box when the call to action has no note.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "promises",
            help: "Up to three short lines on what each issue holds, listed with a tick under the promise on the front page.",
            kind: OptionKind::Lines { most: 3 },
        },
        OptionSpec {
            key: "feed_link",
            help: "Whether every sign-up box also links to the feed, for readers who would rather not give an email address.",
            kind: OptionKind::Switch { default: true },
        },
        parts::HERO_STATS,
    ],
};
