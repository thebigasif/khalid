//! Every built-in template has its page in `docs/templates/`, and the page
//! names every option the template takes (templates spec 4.5).

use std::fs;
use std::path::PathBuf;

/// The packs only tests switch to, which have no page.
const TEST_ONLY: [&str; 2] = ["trial", "sampler"];

#[test]
fn every_template_documents_each_of_its_options() {
    for pack in khalid_theme::packs() {
        if pack.name == "classic" || TEST_ONLY.contains(&pack.name) {
            continue;
        }
        let mut page = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        page.push(format!("../../docs/templates/{}.md", pack.name));
        let text =
            fs::read_to_string(&page).unwrap_or_else(|error| panic!("{}: {error}", page.display()));
        for option in pack.options {
            assert!(
                text.contains(&format!("| `{}` |", option.key)),
                "{} does not document the option {}",
                page.display(),
                option.key
            );
        }
    }
}
