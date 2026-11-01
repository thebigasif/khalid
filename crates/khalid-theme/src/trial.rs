//! The trial pack: a throwaway look built only with the `test-util`
//! feature, so tests prove that switching packs changes the build while
//! classic stays as it is. It is never in a shipped binary.

use khalid_core::ReaderSwitch;

use crate::{OptionKind, OptionSpec, Pack, PackTemplate, slot};

/// The trial pack.
pub(crate) static TRIAL: Pack = Pack {
    name: "trial",
    summary: "A throwaway look for tests.",
    stylesheet: include_str!("../packs/trial/trial.css"),
    templates: &[PackTemplate {
        name: "not_found.html",
        source: include_str!("../packs/trial/not_found.html"),
    }],
    tokens: "[type]\nh1 = \"3rem 4rem\"\n",
    reader: &[(ReaderSwitch::ReadingTime, false)],
    options: &[
        OptionSpec {
            key: "greeting",
            help: "The word the page-not-found page shows.",
            kind: OptionKind::Choice {
                allowed: &["hello", "welcome"],
                default: "hello",
            },
        },
        OptionSpec {
            key: "banner",
            help: "Whether a banner shows.",
            kind: OptionKind::Switch { default: false },
        },
        OptionSpec {
            key: "columns",
            help: "How many columns the cards take.",
            kind: OptionKind::Count {
                least: 2,
                most: 4,
                default: 2,
            },
        },
        OptionSpec {
            key: "stats",
            help: "Short facts shown under the blog's name.",
            kind: OptionKind::Lines { most: 3 },
        },
        slot::share("end"),
        slot::share_networks(&["linkedin", "x", "email"]),
    ],
};
