//! A post's ties to other posts and its listing choices, as the Admin
//! API reads and writes them (plan 4.12, 5.2 and 5.4).

use khalid_api::resources::{ARTICLE_TYPES, ClusterPlace, PAGE_TYPES, PostSettings, SettingsEdit};
use khalid_core::{ArticleSchemaType, PageSchemaType};
use serde_json::{Value, json};

#[cfg(test)]
fn edit(json: Value) -> Result<SettingsEdit, serde_json::Error> {
    serde_json::from_value(json)
}

#[test]
fn every_tie_is_set_through_one_edit_and_written_as_sent() {
    let sent = json!({"connections": {
        "translation": "anchor-text",
        "related": ["internal-links", "link-text"],
        "series": {"name": "Links: a course", "order": 2},
        "cluster": {"name": "links", "role": "pillar"},
        "topic_group": "internal-links",
        "main_post": true,
    }});
    let settings = edit(sent.clone())
        .unwrap()
        .applied_to(PostSettings::default());
    assert_eq!(
        settings.connections.cluster.as_ref().unwrap().role,
        ClusterPlace::Pillar
    );
    let json = serde_json::to_value(&settings).unwrap();
    assert_eq!(json["connections"], sent["connections"]);
}

#[test]
fn a_tie_left_out_keeps_its_value_and_null_clears_it() {
    let set = json!({"connections": {"translation": "a", "related": ["b"]}});
    let before = edit(set).unwrap().applied_to(PostSettings::default());
    let cleared = edit(json!({"connections": {"translation": null}})).unwrap();
    let after = cleared.applied_to(before);
    assert_eq!(after.connections.translation, None);
    let json = serde_json::to_value(&after).unwrap();
    assert_eq!(json["connections"]["related"], json!(["b"]));
}

#[test]
fn a_cluster_without_a_role_is_a_member() {
    let sent = json!({"connections": {"cluster": {"name": "links"}}});
    let settings = edit(sent).unwrap().applied_to(PostSettings::default());
    let json = serde_json::to_value(&settings).unwrap();
    assert_eq!(json["connections"]["cluster"]["role"], "member");
}

#[test]
fn ties_refuse_what_front_matter_would_refuse() {
    let refused = |connections: Value| edit(json!({ "connections": connections })).is_err();
    let six = json!(["a", "b", "c", "d", "e", "f"]);
    let too_many = edit(json!({"connections": {"related": six}})).unwrap_err();
    assert!(too_many.to_string().contains("at most 5"), "{too_many}");
    let twice = edit(json!({"connections": {"related": ["a", "a"]}})).unwrap_err();
    assert!(twice.to_string().contains("listed twice"), "{twice}");
    assert!(refused(json!({"related": ["Not a slug"]})));
    assert!(refused(json!({"series": {"name": " "}})));
    assert!(refused(json!({"series": {"name": "A", "order": 0}})));
    assert!(refused(
        json!({"cluster": {"name": "links", "role": "hub"}})
    ));
    assert!(refused(json!({"translation": ""})));
    assert!(
        refused(json!({"sitemap": false})),
        "sitemap is a listing choice"
    );
}

#[test]
fn every_listing_choice_is_set_through_one_edit_and_written_as_sent() {
    let sent = json!({"listing": {
        "sitemap": false,
        "news": false,
        "breadcrumb_title": "Anchor text",
        "page_type": "FAQPage",
        "article_type": "Report",
        "keyword_synonyms": ["link words"],
        "reviewed_by": ["mira"],
    }});
    let settings = edit(sent.clone())
        .unwrap()
        .applied_to(PostSettings::default());
    assert_eq!(settings.listing.page_type, Some(PageSchemaType::FaqPage));
    assert_eq!(
        settings.listing.article_type,
        Some(ArticleSchemaType::Report)
    );
    let json = serde_json::to_value(&settings).unwrap();
    assert_eq!(json["listing"], sent["listing"]);
}

#[test]
fn listing_choices_refuse_unknown_types_and_a_twice_listed_synonym() {
    let refused = |listing: Value| edit(json!({ "listing": listing })).is_err();
    assert!(refused(json!({"page_type": "BlogPage"})));
    assert!(refused(json!({"article_type": "blogposting"})));
    assert!(refused(json!({"reviewed_by": ["Mira Lee"]})));
    assert!(refused(json!({"breadcrumb_title": "  "})));
    let twice = edit(json!({"listing": {"keyword_synonyms": ["Site links", "site links"]}}));
    assert!(twice.unwrap_err().to_string().contains("listed twice"));
}

#[test]
fn the_named_types_are_the_ones_the_api_reads() {
    for name in PAGE_TYPES {
        let read = serde_json::from_value::<PageSchemaType>(json!(name)).unwrap();
        assert_eq!(read.as_str(), name);
    }
    for name in ARTICLE_TYPES {
        let read = serde_json::from_value::<ArticleSchemaType>(json!(name)).unwrap();
        assert_eq!(read.as_str(), name);
    }
}
