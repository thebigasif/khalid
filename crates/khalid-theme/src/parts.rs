//! The options that switch on a part the shared templates build once
//! (templates spec 4.2), such as the logo in the header. A pack lists the
//! ones it offers, each with its own default; the renderer keeps classic's
//! look for any part a pack leaves out.
//!
//! Each option here starts with classic's value. A pack that wants a part
//! on from the start lists it through [`on`], such as `on(LOGO)`.

use crate::{OptionKind, OptionSpec};

/// The site's logo in the header in place of its name, when the tokens
/// have `[brand] logo`.
pub const LOGO: OptionSpec = OptionSpec {
    key: "logo",
    help: "Whether the header shows the logo from the main site in place of the blog's name.",
    kind: OptionKind::Switch { default: false },
};

/// The trail from the home page to the page, above its content.
pub const BREADCRUMBS: OptionSpec = OptionSpec {
    key: "breadcrumbs",
    help: "Whether each page shows the trail of links from the home page to it.",
    kind: OptionKind::Switch { default: false },
};

/// `spec` turned on by default, when it is an on or off option. Any other
/// option comes back as it is.
pub const fn on(spec: OptionSpec) -> OptionSpec {
    match spec.kind {
        OptionKind::Switch { .. } => OptionSpec {
            kind: OptionKind::Switch { default: true },
            ..spec
        },
        _ => spec,
    }
}

/// Every part's option with classic's value, so a test can hold this list
/// and the renderer's to the same keys.
pub fn all() -> Vec<OptionSpec> {
    vec![LOGO, BREADCRUMBS]
}
