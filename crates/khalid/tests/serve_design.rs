//! The Design screen's route on a running `khalid serve`: the demo site's
//! look read with the chosen template's palette, and a saved colour on the
//! next stylesheet without a restart (phase 4, task 4.7).

mod common;

use std::thread;
use std::time::Duration;

use common::{Server, demo_copy, new_key, start};
use serde_json::json;

/// A key that may change the settings and the design.
const OWNER: [&str; 4] = ["--name", "Owner", "--scope", "settings:write"];

/// The routes under the demo's mount, where the admin calls them.
const DESIGN: &str = "/blog/api/admin/v1/design/";
const SETTINGS: &str = "/blog/api/admin/v1/settings/";

/// The stylesheet the home page links.
#[cfg(test)]
fn stylesheet(server: &Server) -> String {
    let (_, home) = server.raw("GET", "/blog/", &[], None);
    let (_, rest) = home.split_once(r#"<link rel="stylesheet" href=""#).unwrap();
    let (href, _) = rest.split_once('"').unwrap();
    server.raw("GET", href, &[], None).1
}

/// Whether the stylesheet holds `text` within 30 seconds.
#[cfg(test)]
fn styled(server: &Server, text: &str) -> bool {
    (0..300).any(|_| {
        thread::sleep(Duration::from_millis(100));
        stylesheet(server).contains(text)
    })
}

#[test]
fn a_saved_colour_reaches_the_stylesheet_over_the_templates_palette() {
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &OWNER));
    assert!(server.answers("/blog/", 200));
    let (status, read) = server.admin("GET", DESIGN, None);
    assert_eq!(status, 200, "{read}");
    assert!(
        read["design"]["updated_at"].is_null(),
        "no file yet: {read}"
    );
    assert_eq!(read["design"]["tokens"]["colors.background"], "#ffffff");

    // A template chosen on the Look screen shows its palette here at once.
    let (_, settings) = server.admin("GET", SETTINGS, None);
    let at = &settings["settings"]["updated_at"];
    let ledger =
        json!({"settings": {"template": {"name": "ledger", "options": {}}, "updated_at": at}});
    let (status, body) = server.admin("PUT", SETTINGS, Some(&ledger));
    assert_eq!(status, 200, "{body}");
    let (_, read) = server.admin("GET", DESIGN, None);
    assert_eq!(read["design"]["tokens"]["colors.background"], "#fcfcfa");

    let change = json!({"design": {
        "tokens": {"colors.link": "#b3261e"},
        "locked": ["colors.link"],
        "updated_at": null,
    }});
    let (status, saved) = server.admin("PUT", DESIGN, Some(&change));
    assert_eq!(status, 200, "{saved}");
    let link = (saved["design"]["contrast"].as_array().unwrap().iter())
        .find(|pair| pair["token"] == "colors.link" && pair["ground"] == "colors.background")
        .unwrap();
    assert_eq!(link["passes"], true, "{link}");
    assert!(
        styled(&server, "--color-link: #b3261e;"),
        "{}",
        stylesheet(&server)
    );
    let css = stylesheet(&server);
    assert!(
        css.contains("--color-background: #fcfcfa;"),
        "the pack's: {css}"
    );

    let (status, body) = server.admin("PUT", DESIGN, Some(&change));
    assert_eq!(status, 409, "the copy without a file is stale now: {body}");
}
