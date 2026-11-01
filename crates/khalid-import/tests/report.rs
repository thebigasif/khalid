//! The dry-run report: what an import would bring in and what it would not.

use std::fs;
use std::path::Path;

use khalid_core::{Config, Environment};
use khalid_import::Place;
use khalid_import::wordpress::{LeftReason, Plan, plan, read_redirection, read_wxr};
use khalid_routes::Routes;

#[cfg(test)]
fn read(name: &str) -> String {
    let dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{dir}/fixtures/wordpress/{name}")).unwrap()
}

/// The basic export planned into a site at `address`, with the blog in
/// `mount`, and the Redirection plugin's rules in the fixture `rules`.
#[cfg(test)]
fn planned_at(address: &str, mount: &str, rules: Option<&str>) -> Plan {
    let toml = format!(
        "name = \"Northfield Bakery\"\naddress = \"{address}\"\nmount = \"{mount}\"\nmode = \"server\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n"
    );
    let site = Config::from_toml(Path::new("site.toml"), &toml, &Environment::empty()).unwrap();
    let export = read_wxr(&read("basic.xml")).unwrap();
    let redirection = rules.map(|name| read_redirection(&read(name)).unwrap());
    plan(&export, redirection.as_ref(), &site, &Routes::default())
}

#[cfg(test)]
fn planned() -> Plan {
    planned_at(
        "https://northfield.example.com",
        "/blog/",
        Some("redirection.json"),
    )
}

#[test]
fn an_address_whose_path_stays_on_a_new_host_stays_the_same() {
    // The blog moves to a new host at its root: /about/ and the baking
    // archive keep their paths, and only the host changes.
    let plan = planned_at("https://blog.example.com", "/", None);
    let report = plan.report();
    assert_eq!((report.redirects, report.kept_addresses), (3, 2));
    assert!(
        report
            .to_string()
            .contains("Old addresses mapped: 5, of which 3 move and 2 stay the same.\n"),
        "{report}"
    );
}

#[test]
fn every_item_is_imported_failed_or_left_behind_with_a_reason() {
    let plan = planned();
    let places: Vec<(u64, Place)> = plan
        .items
        .iter()
        .map(|item| (item.id, item.place))
        .collect();
    assert_eq!(
        places,
        [(10, Place::Post), (11, Place::Page), (13, Place::Draft)]
    );
    let left: Vec<(u64, &LeftReason)> = plan
        .left_behind
        .iter()
        .map(|left| (left.id, &left.reason))
        .collect();
    assert_eq!(
        left,
        [
            (12, &LeftReason::Attachment),
            (14, &LeftReason::Kind("nav_menu_item".to_owned())),
        ]
    );
    assert_eq!(plan.skipped_rules.len(), 5);
    assert_eq!((plan.authors, plan.categories), (2, 2));
}

#[test]
fn the_report_counts_everything_in_plain_words() {
    let plan = planned();
    let report = plan.report();
    assert_eq!((report.posts, report.drafts, report.pages), (1, 1, 1));
    assert_eq!((report.redirects, report.kept_addresses), (13, 0));
    assert_eq!(
        report.to_string(),
        "This is a dry run: nothing was written.\n\
         Coming in: 1 post, 1 draft, 1 page, 2 authors and 2 categories.\n\
         Slugs kept: 2. Slugs made for posts that had none: 1.\n\
         Old addresses mapped: 13, of which 13 move and 0 stay the same.\n\
         Items with fields their files cannot hold yet: keep-a-starter-alive (tags, feature image).\n\
         Items that could not come in: 0.\n\
         Items left behind: an uploaded file whose page leads to its post (1: 12); a nav_menu_item, which Khalid has no place for (1: 14).\n\
         Redirect rules left behind: 8 /old-menu/, which answers 404 as Khalid already does; 9 /members/, which depends on the visitor (login); 10 /draft-page/, which is switched off; 11 /shop/, which is switched off; 12 /ping/, which does not redirect (nothing).\n"
    );
}
