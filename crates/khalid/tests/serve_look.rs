//! The Look screen's routes on a running `khalid serve`: the templates
//! listed from the engine, and the choice saved to `site.toml` through the
//! settings route (templates spec 4.1 and 4.6, item 8).

mod common;

use common::{demo_copy, new_key, start};

/// A key that may change the settings.
const OWNER: [&str; 4] = ["--name", "Owner", "--scope", "settings:write"];

#[test]
fn the_templates_are_the_engines_packs_with_their_options() {
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &OWNER));
    assert!(server.answers("/blog/", 200));
    let (status, body) = server.admin("GET", "/api/admin/v1/templates/", None);
    assert_eq!(status, 200, "{body}");
    let listed: Vec<&str> = (body["templates"].as_array().unwrap().iter())
        .map(|template| template["name"].as_str().unwrap())
        .collect();
    let packs: Vec<&str> = khalid_theme::packs().iter().map(|pack| pack.name).collect();
    assert_eq!(listed, packs, "every pack, in the engine's order");
    let trial = &body["templates"][1];
    assert_eq!(trial["reader"][0]["key"], "reading_time", "{trial}");
    assert_eq!(trial["reader"][0]["on"], false, "{trial}");
    let greeting = &trial["options"][0];
    assert_eq!(greeting["key"], "greeting", "{trial}");
    assert!(
        greeting["help"]
            .as_str()
            .is_some_and(|help| !help.is_empty())
    );
}
