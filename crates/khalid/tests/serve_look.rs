//! The Look screen's routes on a running `khalid serve`: the templates
//! listed from the engine, and the choice saved to `site.toml` through the
//! settings route (templates spec 4.1 and 4.6, item 8).

mod common;

use std::path::PathBuf;

use common::{demo_copy, new_key, start};
use serde_json::json;

/// A key that may change the settings.
const OWNER: [&str; 4] = ["--name", "Owner", "--scope", "settings:write"];

/// The settings route under the demo's mount, where the admin calls it.
const SETTINGS: &str = "/blog/api/admin/v1/settings/";

#[test]
fn a_template_saved_through_the_settings_is_in_site_toml_and_used_after_a_restart() {
    let site = demo_copy();
    let key = new_key(site.path(), &OWNER);
    let server = start(site.path(), &key);
    let (_, read) = server.admin("GET", SETTINGS, None);
    let classic = json!({"name": "classic", "options": {}});
    assert_eq!(read["settings"]["template"], classic, "{read}");

    let seen = read["settings"]["updated_at"].as_str().unwrap();
    let choose = |name: &str, options| {
        let template = json!({"name": name, "options": options});
        json!({"settings": {"template": template, "updated_at": seen}})
    };
    let (status, body) = server.admin("PUT", SETTINGS, Some(&choose("nope", json!({}))));
    assert_eq!(status, 422, "{body}");
    assert_eq!(body["rule"], "template.name", "{body}");
    let wrong = choose("trial", json!({"greeting": "hi"}));
    let (status, body) = server.admin("PUT", SETTINGS, Some(&wrong));
    assert_eq!(status, 422, "{body}");
    assert_eq!(body["rule"], "template.options.greeting", "{body}");

    let trial = choose(
        "trial",
        json!({"greeting": "welcome", "stats": ["12,000 readers"]}),
    );
    let (status, saved) = server.admin("PUT", SETTINGS, Some(&trial));
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["settings"]["restart_needed"], true, "{saved}");
    assert_eq!(
        saved["settings"]["template"]["options"]["greeting"],
        "welcome"
    );
    let mut file = PathBuf::from(site.path());
    file.push("site.toml");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.starts_with("# The demo site's settings."), "{text}");
    assert!(text.contains("\n[template]\nname = \"trial\"\n"), "{text}");
    assert!(text.contains("greeting = \"welcome\""), "{text}");

    drop(server);
    let restarted = start(site.path(), &key);
    assert!(restarted.answers("/blog/", 200));
    let missing = restarted.raw("GET", "/blog/no-such-page/", &[], None).1;
    assert!(
        missing.contains("<p class=\"trial-greeting\">welcome</p>"),
        "{missing}"
    );
    let (_, read) = restarted.admin("GET", SETTINGS, None);
    assert_eq!(read["settings"]["restart_needed"], false, "{read}");
}

#[test]
fn the_templates_are_the_engines_packs_with_their_options() {
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &OWNER));
    assert!(server.answers("/blog/", 200));
    let (status, body) = server.admin("GET", "/blog/api/admin/v1/templates/", None);
    assert_eq!(status, 200, "{body}");
    let listed: Vec<&str> = (body["templates"].as_array().unwrap().iter())
        .map(|template| template["name"].as_str().unwrap())
        .collect();
    let packs: Vec<&str> = khalid_theme::packs().iter().map(|pack| pack.name).collect();
    assert_eq!(listed, packs, "every pack, in the engine's order");
    let trial = (body["templates"].as_array().unwrap().iter())
        .find(|template| template["name"] == "trial")
        .unwrap();
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
