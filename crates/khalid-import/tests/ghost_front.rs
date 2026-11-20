//! What a Ghost post's file says beyond its title, and what it holds that
//! front matter cannot yet.

use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use khalid_core::{AbsoluteUrl, Config, Environment};
use khalid_import::ghost::{Dropped, Export, Held, Plan, Visibility, plan, read_export};
use khalid_routes::Routes;

/// The plan of `export` from the Ghost site at `northfield.example.com`
/// into a blog on the same host, mounted at `mount`.
#[cfg(test)]
fn planned(export: &Export, mount: &str) -> Plan {
    let toml = format!(
        "name = \"Northfield Bakery\"\naddress = \"https://northfield.example.com\"\nmount = \"{mount}\"\nmode = \"server\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n"
    );
    let site = Config::from_toml(Path::new("site.toml"), &toml, &Environment::empty()).unwrap();
    let old = AbsoluteUrl::parse("https://northfield.example.com").unwrap();
    plan(export, &old, &site, &Routes::default())
}

#[cfg(test)]
fn fixture() -> Export {
    let dir = env!("CARGO_MANIFEST_DIR");
    read_export(&fs::read_to_string(format!("{dir}/fixtures/ghost/ghost-5.json")).unwrap()).unwrap()
}

#[cfg(test)]
fn url(text: &str) -> AbsoluteUrl {
    AbsoluteUrl::parse(text).unwrap()
}

#[test]
fn a_post_writes_its_search_title_people_tags_image_and_cards() {
    let plan = planned(&fixture(), "/blog/");
    let file = &plan.files[0].text;
    assert!(file.contains("date: \"2024-03-12\"\nauthor: \"mara\"\n"));
    let images = "https://northfield.example.com/content/images/2024/03";
    let fields = format!(
        "seo_title: \"Keep a sourdough starter alive | Northfield\"\n\
         co_authors:\n  - \"jonas\"\n\
         tags:\n  - \"Sourdough\"\n  - \"Rye\"\n\
         image:\n  src: \"{images}/starter-jar.jpg\"\n  alt: \"How to keep a starter alive\"\n\
         share_cards:\n  x:\n    title: \"A starter that lives\"\n  facebook:\n    title: \"Keep your starter alive\"\n    description: \"Food, warmth and time.\"\n    image: \"{images}/starter-og.jpg\"\n---\n"
    );
    assert!(file.contains(&fields), "{file}");
    let post = khalid_content::parse_post(Path::new("post.md"), file).unwrap();
    assert_eq!(post.front.tags, ["Sourdough", "Rye"]);
    assert_eq!(post.front.co_authors.len(), 1);
    assert!(post.front.image.is_some() && post.front.share_cards.x.is_some());
    assert_eq!(plan.posts[0].held, []);
    let jar = url(&format!("{images}/starter-jar.jpg"));
    assert_eq!(plan.posts[0].image, Some(jar));
    assert_eq!(plan.posts[0].dropped, [Dropped::CodeInjection]);
}

#[test]
fn the_feature_image_keeps_its_own_alt_text_and_a_page_holds_its_fields() {
    let text = r#"{"db":[{"data":{"posts":[{"id":"p1","title":"Rye","slug":"rye","status":"published","published_at":"2024-01-02T08:00:00.000Z","feature_image":"__GHOST_URL__/rye.jpg"},{"id":"p2","title":"About","slug":"about","type":"page","status":"published","feature_image":"__GHOST_URL__/us.jpg","meta_title":"About the bakery"}],"posts_meta":[{"post_id":"p1","feature_image_alt":"A dark rye loaf"}]}}]}"#;
    let plan = planned(&read_export(text).unwrap(), "/blog/");
    let file = &plan.files[0].text;
    assert!(
        file.contains("image:\n  src: \"https://northfield.example.com/rye.jpg\"\n  alt: \"A dark rye loaf\"\n"),
        "{file}"
    );
    assert!(!plan.files[1].text.contains("image:"));
    assert_eq!(
        plan.posts[1].held,
        [
            Held::SearchTitle("About the bakery".to_owned()),
            Held::FeatureImage(url("https://northfield.example.com/us.jpg")),
        ]
    );
    assert_eq!(plan.posts[1].image, None);
}

#[test]
fn pages_scheduled_and_members_posts_hold_what_they_had() {
    let plan = planned(&fixture(), "/blog/");
    let canonical = url("https://northfield.example.com/about-us/");
    assert_eq!(plan.posts[1].held, [Held::Canonical(canonical)]);
    let due = NaiveDate::from_ymd_opt(2026, 11, 2)
        .and_then(|day| day.and_hms_opt(7, 0, 0))
        .unwrap();
    assert_eq!(plan.posts[3].held, [Held::Scheduled(due)]);
    assert_eq!(plan.posts[4].held, [Held::MembersOnly(Visibility::Paid)]);
}

#[test]
fn a_post_keeps_its_canonical_and_its_old_address_as_an_alias() {
    let text = r#"{"db":[{"data":{"posts":[{"id":"c1","slug":"Rye_Bread","status":"published","published_at":"2024-01-02T08:00:00.000Z","canonical_url":"https://flour.example.net/rye/","lexical":"{\"root\":{\"children\":[{\"type\":\"signup\"},{\"type\":\"paywall\"}]}}"},{"id":"c2","slug":"spelt","status":"published","published_at":"2024-01-03T08:00:00.000Z"}]}}]}"#;
    let export = read_export(text).unwrap();
    let root = planned(&export, "/");
    let file = &root.files[0].text;
    assert!(
        file.contains(
            "aliases:\n  - \"/Rye_Bread/\"\ncanonical: \"https://flour.example.net/rye/\"\n"
        ),
        "a changed slug keeps its old address as an alias: {file}"
    );
    assert!(
        !root.files[1].text.contains("aliases"),
        "an address that stays the same needs none"
    );
    let inside = planned(&export, "/blog/");
    assert!(
        !inside.files[0].text.contains("aliases"),
        "an old address outside the blog is not an alias"
    );
    let cards = vec!["signup".to_owned(), "paywall".to_owned()];
    assert_eq!(inside.posts[0].dropped, [Dropped::Cards(cards)]);
}
