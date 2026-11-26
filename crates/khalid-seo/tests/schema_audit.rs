//! The deeper check of hand-written JSON-LD against the whole schema.org
//! vocabulary.

#![cfg(test)]

mod common;

use common::{demo_site, internal_links_post, internal_links_post_de};
use khalid_seo::{
    AuditContext, FindingLevel, FixTarget, SchemaFinding, SchemaReport, SeoError, audit_json_ld,
    page_graph,
};

fn audit(text: &str) -> Vec<SchemaFinding> {
    audit_json_ld(text, &AuditContext::default()).unwrap()
}

/// Each finding as `path: what`, for short assertions.
fn lines(findings: &[SchemaFinding]) -> Vec<String> {
    findings
        .iter()
        .map(|f| format!("{}: {}", f.path, f.message.what()))
        .collect()
}

#[test]
fn clean_markup_has_no_findings() {
    let text = r#"{
      "@context": "https://schema.org",
      "@type": "Product",
      "name": "Lentil soup kit",
      "offers": {
        "@type": "Offer",
        "price": "19.99",
        "priceCurrency": "USD",
        "availability": "https://schema.org/InStock",
        "priceValidUntil": "2026-12-31"
      },
      "aggregateRating": {"@type": "AggregateRating", "ratingValue": 4.5, "reviewCount": 12}
    }"#;
    assert_eq!(lines(&audit(text)), Vec::<String>::new());
}

#[test]
fn every_graph_khalid_writes_is_correct_schema_org() {
    for post in [internal_links_post(), internal_links_post_de()] {
        let json = page_graph(&demo_site(), &post).unwrap().to_json();
        let found = audit(&json);
        assert_eq!(lines(&found), Vec::<String>::new());
    }
}

#[test]
fn no_snapshot_graph_has_a_schema_org_error() {
    let folder = format!("{}/tests/snapshots", env!("CARGO_MANIFEST_DIR"));
    let mut graphs = 0;
    for entry in std::fs::read_dir(folder).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let Some((_, body)) = text.split_once("\n---\n") else {
            continue;
        };
        if !body.trim_start().starts_with('{') || !body.contains("\"@type\"") {
            continue;
        }
        graphs += 1;
        let errors: Vec<String> = audit(body)
            .into_iter()
            .filter(|f| f.level == FindingLevel::Error)
            .map(|f| format!("{}: {}", f.path, f.message.what()))
            .collect();
        assert_eq!(errors, Vec::<String>::new(), "{body}");
    }
    assert!(graphs > 10, "{graphs}");
}

#[test]
fn a_slip_in_a_name_comes_with_its_fix() {
    let text = r#"{"@type": "Product", "name": "Kit",
      "offers": {"@type": "Offer", "price": "5", "pricecurrency": "USD"}}"#;
    let found = audit(text);
    assert_eq!(found.len(), 1, "{:?}", lines(&found));
    let fix = found[0].fix.clone().unwrap();
    assert_eq!(fix.target, FixTarget::Property);
    assert_eq!(fix.path, "offers.pricecurrency");
    assert_eq!(fix.label(), "Change pricecurrency to priceCurrency");
    assert_eq!(found[0].report, SchemaReport::SchemaOrg);
    assert_eq!(found[0].level, FindingLevel::Error);
}

#[test]
fn a_property_on_the_wrong_type_is_named() {
    let found = audit(r#"{"@type": "Event", "name": "Gig", "recipeIngredient": "salt"}"#);
    assert_eq!(
        lines(&found),
        ["recipeIngredient: recipeIngredient is not a property of Event."]
    );
}

#[test]
fn values_must_be_what_the_property_expects() {
    let text = r#"{"@type": "Event", "name": "Gig",
      "startDate": "next Tuesday",
      "maximumAttendeeCapacity": "forty",
      "isAccessibleForFree": "yes",
      "url": "our events page",
      "eventStatus": "scheduled"}"#;
    let found = lines(&audit(text));
    assert_eq!(
        found,
        [
            "eventStatus: eventStatus is \"scheduled\", which is not one of the values EventStatusType allows.",
            "isAccessibleForFree: isAccessibleForFree is not written as Boolean.",
            "maximumAttendeeCapacity: maximumAttendeeCapacity is not written as Integer.",
            "startDate: startDate is not written as Date or DateTime.",
            "url: url is not written as URL.",
        ]
    );
}

#[test]
fn an_enumeration_value_in_other_capitals_gets_the_full_value_as_its_fix() {
    let found = audit(r#"{"@type": "Offer", "price": 5, "availability": "instock"}"#);
    let fix = found[0].fix.clone().unwrap();
    assert_eq!(
        (fix.target, fix.to.as_str()),
        (FixTarget::Value, "https://schema.org/InStock")
    );
    let found = audit(r#"{"@type": "Offer", "price": 5, "availability": "In stock"}"#);
    assert_eq!(
        found[0].fix.clone().unwrap().to,
        "https://schema.org/InStock"
    );
}

#[test]
fn a_currency_and_a_duration_follow_their_standards() {
    let text = r#"{"@type": "Recipe", "name": "Soup", "cookTime": "20 minutes",
      "offers": {"@type": "Offer", "price": 5, "priceCurrency": "usd"}}"#;
    let found = audit(text);
    assert_eq!(found.len(), 2, "{:?}", lines(&found));
    assert_eq!(found[0].path, "cookTime");
    assert_eq!(found[1].fix.clone().unwrap().to, "USD");
}

#[test]
fn a_nested_type_must_fit_its_property() {
    let found = lines(&audit(
        r#"{"@type": "Product", "name": "Kit", "offers": {"@type": "Person", "name": "Ann"}}"#,
    ));
    assert_eq!(
        found,
        ["offers: A Person does not fit here; this property expects Demand or Offer."]
    );
}

#[test]
fn an_id_must_point_to_a_node_on_the_page() {
    let text = r#"{"@graph": [
      {"@type": "Organization", "@id": "https://example.com/#org", "name": "Ex"},
      {"@type": "Article", "headline": "Hi",
       "publisher": {"@id": "https://example.com/#org"},
       "author": {"@id": "https://example.com/#ann"}}]}"#;
    assert_eq!(
        lines(&audit(text)),
        ["@graph[1].author.@id: The @id https://example.com/#ann points to nothing on this page."]
    );
    let known = ["https://example.com/#ann".to_owned()];
    let context = AuditContext {
        known_ids: &known,
        ..AuditContext::default()
    };
    assert_eq!(audit_json_ld(text, &context).unwrap(), []);
}

#[test]
fn a_superseded_name_points_to_its_replacement() {
    let found = audit(r#"{"@type": "Code", "name": "Script"}"#);
    let fix = found[0].fix.clone().unwrap();
    assert_eq!(
        (fix.from.as_str(), fix.to.as_str()),
        ("Code", "SoftwareSourceCode")
    );
    assert_eq!(found[0].level, FindingLevel::Warning);
}

#[test]
fn unknown_names_far_from_any_real_one_are_errors_without_a_fix() {
    let found = audit(r#"{"@type": "Widget", "name": "W"}"#);
    assert_eq!(lines(&found), ["@type: Widget is not a schema.org type."]);
    assert_eq!(found[0].fix, None);
}

#[test]
fn values_the_page_does_not_show_are_flagged_for_google() {
    let text = r#"{"@type": "Product", "name": "Lentil soup kit",
      "offers": {"@type": "Offer", "price": "29.99", "priceCurrency": "USD"}}"#;
    let context = AuditContext {
        visible_text: Some("Our lentil soup kit costs $19.99 and feeds four."),
        ..AuditContext::default()
    };
    let found = audit_json_ld(text, &context).unwrap();
    assert_eq!(
        lines(&found),
        ["offers.price: The page does not show the price \"29.99\" that the markup gives."]
    );
    assert_eq!(found[0].report, SchemaReport::Google);
}

#[test]
fn text_that_is_not_json_says_where() {
    assert_eq!(
        audit_json_ld("{\n  \"@type\": \"Event\",\n}", &AuditContext::default()),
        Err(SeoError::CustomSchemaSyntax { line: 3, column: 1 })
    );
    assert_eq!(audit("  "), []);
}
