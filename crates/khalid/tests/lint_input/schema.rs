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
fn a_category_schema_template_reaches_its_posts_by_the_category_name() {
    // The template names the category as `categories.toml` does, as the
    // built page does (G54); the post in another category is left alone.
    let clinic = post("Links clinic", "category: events\n", "Come along.");
    let notes = post("Links notes", "category: notes\n", "Read along.");
    let posts = [
        ("posts/clinic.md", clinic.as_str()),
        ("posts/notes.md", notes.as_str()),
    ];
    let site = site(None, &posts);
    let named = "[[categories]]\nslug = \"events\"\nen = \"Events and talks\"\n\n\
                 [[categories]]\nslug = \"notes\"\nen = \"Notes\"\n";
    fs::write(within(site.path(), "categories.toml"), named).unwrap();
    let settings = within(site.path(), "site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    let template = "\n[[seo.schema]]\ncategory = \"Events and talks\"\ntype = \"Event\"\n\
                    [seo.schema.properties]\nname = \"{title}\"\nlocation = \"Leeds\"\n";
    fs::write(&settings, format!("{text}{template}")).unwrap();
    let report = lint(site.path());
    assert_eq!(
        found(&report, RULE, "links-clinic"),
        ["Event needs startDate"],
        "{report}"
    );
    assert_eq!(found(&report, RULE, "links-notes"), Vec::<String>::new());
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

#[test]
fn hand_written_json_ld_is_checked_against_the_whole_vocabulary() {
    // The organisation's @id is the site's own, so it points to a node on
    // the page; the speaker's points to nothing.
    let event = "schema: |\n  {\"@type\": \"Event\", \"name\": \"Internal links clinic\", \
                 \"startDate\": \"next Tuesday\", \"location\": \"Leeds\", \
                 \"recipeIngredient\": \"salt\", \
                 \"organizer\": {\"@id\": \"ORG\"}, \
                 \"performer\": {\"@id\": \"https://elsewhere.example/#speaker\"}}\n";
    let event = event.replace("ORG", "https://www.example.com/blog/#organization");
    let text = post(
        "Links clinic",
        &event,
        "Come along to the Internal links clinic.",
    );
    let posts = [("posts/clinic.md", text.as_str())];
    let site = site(None, &posts);
    let report = lint(site.path());
    let found = found(&report, "schema-markup-mistake", "links-clinic");
    let starts = [
        "recipeIngredient: recipeIngredient is not a property of Event.",
        "startDate: startDate is not written as Date or DateTime.",
        "performer.@id: The @id https://elsewhere.example/#speaker points to nothing on this page.",
        "recipeIngredient: The page does not show the recipeIngredient \"salt\" that the markup gives.",
    ];
    assert_eq!(found.len(), starts.len(), "{found:?}");
    for (line, start) in found.iter().zip(starts) {
        assert!(line.starts_with(start), "{found:?}");
    }
    let severities: Vec<&str> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["rule"] == "schema-markup-mistake")
        .filter_map(|f| f["severity"].as_str())
        .collect();
    assert_eq!(severities, ["warn"; 4], "{report}");
}
