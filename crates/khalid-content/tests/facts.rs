//! A `[[fact:key]]` in a body shows the fact's current value from
//! `facts.toml` (GEO P7), and one the registry lacks stops the build.

use std::path::Path;

use khalid_content::{
    ContentError, ContentTree, SiteContent, Wikilinks, check_bodies, parse_facts, read_content,
    render_linked,
};
use khalid_core::{Config, Environment, Locale};
use khalid_routes::Routes;

const DEMO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/demo-site");

const FACTS: &str = "[pro-plan-price]\nvalue = \"29 USD a month\"\nchecked = 2026-09-01\n";

/// The demo site with the Pro plan's price in its registry, and the first
/// English post's body set to `body`.
#[cfg(test)]
fn demo_with(body: &str) -> (Config, SiteContent) {
    let site = Path::new(DEMO);
    let config = Config::load(site, &Environment::empty()).unwrap();
    let mut content = read_content(&ContentTree::walk(site).unwrap()).unwrap();
    content.facts = parse_facts(FACTS).unwrap();
    let post = content
        .posts
        .iter_mut()
        .find(|doc| doc.locale.as_str() == "en")
        .unwrap();
    post.body = body.to_owned();
    (config, content)
}

/// The HTML of `body` in English on the demo site with its facts.
#[cfg(test)]
fn html(body: &str) -> String {
    let (config, content) = demo_with(body);
    let en = Locale::try_new("en").unwrap();
    let links = Wikilinks::for_language(&content, &config, &Routes::default(), &en);
    render_linked(body, &en, config.slugs(), &links)
        .unwrap()
        .html
}

#[test]
fn a_fact_shows_its_current_value_as_plain_text() {
    let html = html("The Pro plan costs [[fact:pro-plan-price]], billed monthly.\n");
    assert!(
        html.contains("<p>The Pro plan costs 29 USD a month, billed monthly.</p>"),
        "{html}"
    );
}

#[test]
fn a_fact_with_text_after_a_pipe_still_shows_the_value() {
    let html = html("Pay [[fact:pro-plan-price|the price]] once.\n");
    assert!(html.contains("Pay 29 USD a month once."), "{html}");
    assert!(!html.contains("<a"), "{html}");
}

#[test]
fn a_fact_in_code_is_shown_as_written() {
    let html = html("Write `[[fact:pro-plan-price]]` to show the price.\n");
    assert!(
        html.contains("<code>[[fact:pro-plan-price]]</code>"),
        "{html}"
    );
}

#[test]
fn a_fact_the_registry_lacks_stops_the_check_with_its_line() {
    let (config, content) = demo_with("First line.\n\nThe plan costs [[fact:team-size]].\n");
    let error = check_bodies(&content, &config, &Routes::default()).unwrap_err();
    let ContentError::Fact { key, line, .. } = &error else {
        panic!("{error:?}");
    };
    assert_eq!(key, "team-size");
    let post = content.posts.iter().find(|d| d.locale.as_str() == "en");
    assert_eq!(Some(*line), post.map(|doc| doc.body_line + 2));
}

#[test]
fn a_known_fact_passes_the_check() {
    let (config, content) = demo_with("The plan costs [[fact:pro-plan-price]].\n");
    check_bodies(&content, &config, &Routes::default()).unwrap();
}
