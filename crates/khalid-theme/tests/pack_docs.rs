//! Every template a person can pick has its page in `docs/templates/`,
//! and that page names every option the template takes (templates spec
//! 4.5).

use std::fs;
use std::path::PathBuf;

/// Classic takes no options and is described in `docs/templates.md`; the
/// others are built only for tests.
const UNDOCUMENTED: [&str; 3] = ["classic", "trial", "sampler"];

#[test]
fn every_option_of_every_template_is_documented() {
    for pack in khalid_theme::packs() {
        if UNDOCUMENTED.contains(&pack.name) {
            continue;
        }
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("../../docs/templates/{}.md", pack.name));
        let page =
            fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        for option in pack.options {
            assert!(
                page.contains(&format!("| `{}` |", option.key)),
                "{} does not document `{}`",
                path.display(),
                option.key
            );
        }
    }
}
