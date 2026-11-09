//! The plugin fields read in `seo_social.rs`, written into each imported
//! post's front matter (Yoast package WP4): keywords, share cards and the
//! main post of a topic group. What front matter cannot hold stays in the
//! report.

use std::fs;
use std::path::Path;

use khalid_core::{Config, Environment};
use khalid_import::wordpress::{Held, Plan, plan, read_wxr};
use khalid_routes::Routes;

#[cfg(test)]
fn planned() -> Plan {
    let dir = env!("CARGO_MANIFEST_DIR");
    let text = fs::read_to_string(format!("{dir}/fixtures/wordpress/seo/social.xml")).unwrap();
    let toml = "name = \"Northfield Bakery\"\naddress = \"https://northfield.example.com\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n";
    let site = Config::from_toml(Path::new("site.toml"), toml, &Environment::empty()).unwrap();
    let plan = plan(&read_wxr(&text).unwrap(), None, &site, &Routes::default());
    assert!(plan.failed.is_empty(), "{:?}", plan.failed);
    plan
}

/// The front matter written for the post at `index`.
#[cfg(test)]
fn front(index: usize) -> String {
    let plan = planned();
    let text = &plan.files[index].text;
    text.split("---\n").nth(1).unwrap().to_owned()
}

#[test]
fn a_yoast_cornerstone_post_becomes_its_categorys_main_post() {
    assert_eq!(
        front(0),
        "title: \"Why rye bread needs time\"\n\
         slug: \"rye-needs-time\"\n\
         date: \"2024-04-02\"\n\
         category: \"sourdough\"\n\
         focus_keyword: \"rye bread\"\n\
         secondary_keywords:\n  - \"rye starter\"\n  - \"rye flour\"\n  - \"dark rye\"\n  - \"sourdough rye\"\n\
         share_cards:\n  x:\n    title: \"Rye takes a day & a night\"\n  facebook:\n    \
         title: \"Why rye bread needs time: the slow way\"\n    \
         description: \"Rye wants a day, not an hour.\"\n    \
         image: \"https://northfield.example.com/wp-content/uploads/rye-card.jpg\"\n\
         topic_group: \"sourdough\"\n\
         main_post: true\n"
    );
}

#[test]
fn keywords_beyond_four_secondary_stay_in_the_report() {
    let text = front(1);
    assert!(text.contains("focus_keyword: \"bread flour\"\n"), "{text}");
    let secondary = "secondary_keywords:\n  - \"strong flour\"\n  - \"flour protein\"\n  - \"baker's flour\"\n  - \"high gluten flour\"\n";
    assert!(text.contains(secondary), "{text}");
    let plan = planned();
    let held = &plan.items[1].held;
    let beyond = Held::FocusKeywords(vec!["wheat flour".to_owned()]);
    assert!(held.contains(&beyond), "{held:?}");
}

#[test]
fn a_pillar_post_without_a_category_leads_its_own_group() {
    let text = front(1);
    assert!(
        text.contains("  facebook:\n    title: \"Which flour makes the best loaf?\"\n"),
        "{text}"
    );
    assert!(
        text.contains("  x:\n    description: \"Protein is what counts.\"\n"),
        "{text}"
    );
    let group = "topic_group: \"choosing-bread-flour\"\nmain_post: true\n";
    assert!(text.contains(group), "{text}");
}

#[test]
fn the_breadcrumb_title_and_schema_type_are_in_the_report() {
    let plan = planned();
    let held = &plan.items[1].held;
    let listed = |key: &str| {
        held.iter()
            .any(|h| matches!(h, Held::Unread(f) if f.key == key))
    };
    assert!(listed("rank_math_breadcrumb_title"), "{held:?}");
    assert!(listed("rank_math_rich_snippet"), "{held:?}");
}

#[test]
fn additional_keyphrases_become_secondary_keywords() {
    let text = front(2);
    let secondary = "secondary_keywords:\n  - \"banneton\"\n  - \"brotform\"\n";
    assert!(text.contains(secondary), "{text}");
    assert!(!text.contains("main_post"), "{text}");
    assert!(
        !text.contains("focus_keyword"),
        "no focus keyword was set: {text}"
    );
}
