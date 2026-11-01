//! The options of the slots any template may fill around its pages, such
//! as share links (templates spec 4.2). A pack lists the ones it takes,
//! each with its own default, so every template switches each slot on or
//! off. A pack that lists none shows none of them, as classic does.

use crate::{OptionKind, OptionSpec};

/// Every network a share row may link to, in the order the docs list them.
/// The renderer knows each one's share address.
pub const SHARE_NETWORKS: [&str; 8] = [
    "x",
    "linkedin",
    "facebook",
    "bluesky",
    "reddit",
    "hacker-news",
    "whatsapp",
    "email",
];

/// `share`: where a post shows its share links, `default` when not set.
pub const fn share(default: &'static str) -> OptionSpec {
    OptionSpec {
        key: "share",
        help: "Where a post shows its share links: none, top under the title, end after the post, or both.",
        kind: OptionKind::Choice {
            allowed: &["none", "top", "end", "both"],
            default,
        },
    }
}

/// `newsletter`: where the newsletter form shows, `default` when not set.
/// A site that takes no sign-ups shows it nowhere.
pub const fn newsletter(default: &'static str) -> OptionSpec {
    OptionSpec {
        key: "newsletter",
        help: "Where the newsletter form shows: none, post after each post, home on the front page, or both.",
        kind: OptionKind::Choice {
            allowed: PLACES,
            default,
        },
    }
}

/// The places a slot may show: none, after each post, on the front page,
/// or both.
const PLACES: &[&str] = &["none", "post", "home", "both"];

/// `share_networks`: which networks the share links go to, in order,
/// `default` when not set.
pub const fn share_networks(default: &'static [&'static str]) -> OptionSpec {
    OptionSpec {
        key: "share_networks",
        help: "The networks the share links go to, in the order they show.",
        kind: OptionKind::Picks {
            allowed: &SHARE_NETWORKS,
            default,
        },
    }
}
