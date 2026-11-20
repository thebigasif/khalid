//! A post's ties to other posts, as the Admin API reads and writes
//! them (plan 4.12).

use khalid_api::resources::{ClusterPlace, PostSettings, SettingsEdit};
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
