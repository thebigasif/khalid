//! The built-in templates through the Admin API, `GET /templates/`: each
//! with its summary, the reader features it turns on and off, and every
//! option with the values it takes (templates spec 4.1).

use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::MemoryContentStore;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_api::resources::{AdminTemplate, OptionTakes, ReaderDefault, TemplateOptionSpec};
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const OWNER: &str =
    "ki_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const EDITOR: &str =
    "ki_2222333344445555_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

/// A site listing `templates`, or listing none at all.
#[cfg(test)]
fn router(templates: Option<Vec<AdminTemplate>>) -> Router {
    let hashing = KeyHashing::fast_for_tests();
    let token = |text: &str| text.parse::<KeyToken>().unwrap();
    let key = |text, scopes| StoredKey::for_integration(&token(text), scopes, &hashing).unwrap();
    let keys = MemoryKeyStore::new(vec![
        key(OWNER, vec![Scope::SettingsWrite]),
        key(EDITOR, vec![Scope::PostsWrite, Scope::PostsPublish]),
    ]);
    let desks = AdminDesks {
        templates: templates.map(Arc::from),
        ..AdminDesks::none()
    };
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(MemoryContentStore::new(Vec::new()));
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

/// Two templates: one with no options, one with an option of each kind.
#[cfg(test)]
fn two() -> Vec<AdminTemplate> {
    let words = |words: &[&str]| words.iter().map(|word| (*word).to_owned()).collect();
    let option = |key: &str, takes| TemplateOptionSpec {
        key: key.to_owned(),
        help: format!("What {key} changes."),
        takes,
    };
    vec![
        AdminTemplate {
            name: "classic".to_owned(),
            summary: "Today's look.".to_owned(),
            reader: Vec::new(),
            options: Vec::new(),
        },
        AdminTemplate {
            name: "ledger".to_owned(),
            summary: "A data-led blog.".to_owned(),
            reader: vec![ReaderDefault {
                key: "reading_time".to_owned(),
                on: false,
            }],
            options: vec![
                option("logo", OptionTakes::Switch { default: true }),
                option(
                    "contents",
                    OptionTakes::Choice {
                        allowed: words(&["inline", "none"]),
                        default: "inline".to_owned(),
                    },
                ),
                option(
                    "related_count",
                    OptionTakes::Count {
                        least: 3,
                        most: 10,
                        default: 5,
                    },
                ),
                option("stats", OptionTakes::Lines { most: 3 }),
                option(
                    "share_networks",
                    OptionTakes::Picks {
                        allowed: words(&["x", "email"]),
                        default: words(&["email"]),
                    },
                ),
            ],
        },
    ]
}

#[cfg(test)]
async fn list(router: &Router, key: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .uri("/api/admin/v1/templates/")
        .header("authorization", format!("Khalid {key}"))
        .body(Body::empty())
        .unwrap();
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn the_templates_list_each_option_with_what_it_takes() {
    let (status, json) = list(&router(Some(two())), OWNER).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let templates = &json["templates"];
    assert_eq!(templates[0]["name"], "classic", "{json}");
    assert_eq!(templates[0]["options"], json!([]), "{json}");
    let ledger = &templates[1];
    assert_eq!(
        ledger["reader"],
        json!([{"key": "reading_time", "on": false}])
    );
    let takes: Vec<&Value> = (ledger["options"].as_array().unwrap().iter())
        .map(|option| &option["takes"])
        .collect();
    assert_eq!(
        takes,
        [
            &json!({"kind": "switch", "default": true}),
            &json!({"kind": "choice", "allowed": ["inline", "none"], "default": "inline"}),
            &json!({"kind": "count", "least": 3, "most": 10, "default": 5}),
            &json!({"kind": "lines", "most": 3}),
            &json!({"kind": "picks", "allowed": ["x", "email"], "default": ["email"]}),
        ]
    );
    assert_eq!(ledger["options"][0]["help"], "What logo changes.");
}

#[tokio::test]
async fn the_list_needs_settings_write_and_a_server_that_has_templates() {
    let (status, json) = list(&router(Some(two())), EDITOR).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["rule"], "settings:write", "{json}");
    let (status, json) = list(&router(None), OWNER).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
}
