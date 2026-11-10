//! `khalid theme check`: drift between the saved tokens and the main site,
//! faint colours even when locked, and the way to sync first.

use std::fs;

use super::{HOME, Names, RESTYLED, STYLE, json, khalid, lock_link, site, stand_in, text, within};

/// A site synced from `home`, whose site.toml names `home` as its address.
#[cfg(test)]
async fn synced_site(home: &str) -> tempfile::TempDir {
    let site = site();
    let settings = within(site.path(), "site.toml");
    let settings_text = fs::read_to_string(&settings).unwrap();
    let moved: Vec<String> = settings_text
        .lines()
        .map(|line| {
            if line.starts_with("address = ") {
                format!("address = \"{home}\"")
            } else {
                line.to_owned()
            }
        })
        .collect();
    fs::write(&settings, moved.join("\n")).unwrap();
    let first = khalid(site.path(), &["theme", "sync", home], Names::StandIn).await;
    assert_eq!(first.status.code(), Some(0), "{}", text(&first.stderr));
    site
}

#[tokio::test(flavor = "multi_thread")]
async fn check_after_a_sync_finds_nothing_and_reads_the_address_from_the_site() {
    let (_server, home) = stand_in(HOME, Some(STYLE)).await;
    let site = synced_site(&home).await;

    let output = khalid(site.path(), &["theme", "check"], Names::StandIn).await;

    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let said = text(&output.stdout);
    assert!(said.contains(&format!("against {home}.")), "{said}");
    assert!(
        said.contains("matches the main site, and every colour is readable"),
        "{said}"
    );

    let body = json(&khalid(site.path(), &["--json", "theme", "check"], Names::StandIn).await);
    assert_eq!(body["ok"], true, "{body}");
    assert_eq!(body["drift"], serde_json::json!([]));
}

#[tokio::test(flavor = "multi_thread")]
async fn check_reports_drift_from_a_redesigned_site_and_changes_nothing() {
    let (_before, home) = stand_in(HOME, Some(STYLE)).await;
    let site = synced_site(&home).await;
    let file = within(site.path(), "theme/tokens.toml");
    let saved = fs::read_to_string(&file).unwrap();
    let (_after, redesigned) = stand_in(HOME, Some(RESTYLED)).await;

    let args = ["theme", "check", redesigned.as_str()];
    let output = khalid(site.path(), &args, Names::StandIn).await;

    assert_eq!(output.status.code(), Some(1));
    let said = text(&output.stdout);
    assert!(
        said.contains("colors.text is #2b2118 here, and the main site now uses #1b1b1f."),
        "{said}"
    );
    assert!(
        said.contains(&format!("Run `khalid theme sync {redesigned}`")),
        "{said}"
    );
    assert_eq!(fs::read_to_string(&file).unwrap(), saved);

    let args = ["--json", "theme", "check", redesigned.as_str()];
    let output = khalid(site.path(), &args, Names::StandIn).await;
    assert_eq!(output.status.code(), Some(1));
    let body = json(&output);
    assert_eq!(body["ok"], false);
    let text_drift = body["drift"]
        .as_array()
        .unwrap()
        .iter()
        .find(|change| change["token"] == "colors.text")
        .unwrap();
    assert_eq!(text_drift["kept"], false);
    assert_eq!(text_drift["file"], "#2b2118");
    assert_eq!(text_drift["site"], "#1b1b1f");
}

#[tokio::test(flavor = "multi_thread")]
async fn check_reports_a_faint_colour_even_when_it_is_locked() {
    let (_server, home) = stand_in(HOME, Some(STYLE)).await;
    let site = synced_site(&home).await;
    lock_link(site.path(), "#f0c040");

    let output = khalid(site.path(), &["theme", "check"], Names::StandIn).await;

    assert_eq!(output.status.code(), Some(1));
    let said = text(&output.stdout);
    assert!(
        said.contains("colors.link stays #f0c040, because it is locked."),
        "{said}"
    );
    assert!(
        said.contains("colors.link is #f0c040, with a contrast of 1."),
        "{said}"
    );
    assert!(said.contains("Change the faint colours"), "{said}");

    let output = khalid(site.path(), &["--json", "theme", "check"], Names::StandIn).await;
    let body = json(&output);
    assert_eq!(body["ok"], false);
    assert_eq!(body["drift"][0]["kept"], true);
    let faint = &body["contrast"][0];
    assert_eq!(faint["token"], "colors.link");
    assert_eq!(faint["color"], "#f0c040");
    assert_eq!(faint["target"], 4.5);
    assert_eq!(faint["nearest_passes"], true);
}

#[tokio::test(flavor = "multi_thread")]
async fn check_without_a_tokens_file_says_to_sync_first() {
    let site = site();

    let output = khalid(site.path(), &["--json", "theme", "check"], Names::Real).await;

    assert_eq!(output.status.code(), Some(1));
    let error = &json(&output)["error"];
    assert_eq!(error["what"], "This site has no theme/tokens.toml yet.");
    let fix = error["fix"].as_str().unwrap();
    assert!(
        fix.contains("khalid theme sync https://www.example.com/"),
        "{fix}"
    );
}
