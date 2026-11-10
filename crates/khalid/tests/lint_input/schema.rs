//! The publish check reads all of a post's schema, as the build writes it:
//! the schema of its blocks, its collection's schema templates and its
//! hand-written JSON-LD. A property Google needs that is missing blocks the
//! post (plan D-9).

use std::fs;

use super::{found, lint, post, site, within};

/// The rule a missing required property breaks.
const RULE: &str = "invalid-structured-data";

/// A recipe with its lists, and `image` among its attributes.
fn recipe(image: &str) -> String {
    format!(
        "::::recipe{{name=\"Oat bars\" {image}}}\n:::ingredients\n- 250 g rolled oats\n\
         - 125 g butter\n:::\n\n:::method\n1. Melt the butter and stir in the oats.\n\
         2. Bake at 180 C for 30 minutes.\n:::\n::::"
    )
}

#[test]
fn hand_written_json_ld_that_misses_a_required_property_blocks_the_post() {
    let event = "schema: |\n  {\"@type\": \"Event\", \"name\": \"Internal links clinic\"}\n";
    let text = post("Links clinic", event, "Come along.");
    let posts = [("posts/clinic.md", text.as_str())];
    let site = site(None, &posts);
    let report = lint(site.path());
    assert_eq!(
        found(&report, RULE, "links-clinic"),
        ["Event needs location", "Event needs startDate"],
        "{report}"
    );
}

#[test]
fn hand_written_json_ld_with_what_google_needs_passes() {
    let event = "schema: |\n  {\"@type\": \"Event\", \"name\": \"Internal links clinic\", \
                 \"startDate\": \"2026-11-12T18:00:00+00:00\", \"location\": \"Leeds\"}\n";
    let text = post("Links clinic", event, "Come along.");
    let posts = [("posts/clinic.md", text.as_str())];
    let site = site(None, &posts);
    let report = lint(site.path());
    assert_eq!(found(&report, RULE, "links-clinic"), Vec::<String>::new());
}

#[test]
fn a_recipe_block_with_no_photo_blocks_the_post_and_one_with_a_photo_passes() {
    let site = site(
        None,
        &[
            ("posts/bare.md", &post("Oat bars", "", &recipe(""))),
            (
                "posts/shown.md",
                &post(
                    "Oat bars shown",
                    "",
                    &recipe("image=\"/media/oat-bars.jpg\""),
                ),
            ),
        ],
    );
    let report = lint(site.path());
    assert_eq!(found(&report, RULE, "oat-bars"), ["Recipe needs image"]);
    assert_eq!(found(&report, RULE, "oat-bars-shown"), Vec::<String>::new());
}

#[test]
fn a_collection_schema_template_that_misses_a_required_property_blocks_its_posts() {
    let text = post("Links clinic", "", "Come along.");
    let posts = [("posts/clinic.md", text.as_str())];
    let site = site(None, &posts);
    let settings = within(site.path(), "site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    let template = "\n[[seo.schema]]\ncollection = \"posts\"\ntype = \"Event\"\n\
                    [seo.schema.properties]\nname = \"{title}\"\nlocation = \"Leeds\"\n";
    fs::write(&settings, format!("{text}{template}")).unwrap();
    let report = lint(site.path());
    assert_eq!(
        found(&report, RULE, "links-clinic"),
        ["Event needs startDate"],
        "{report}"
    );
}
