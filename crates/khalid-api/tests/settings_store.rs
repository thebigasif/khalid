//! The site's core settings as the Admin API keeps them (KH-API-2).

use std::collections::BTreeMap;

use chrono::{DateTime, TimeZone, Utc};
use khalid_api::resources::{AdminSettings, SettingsChange, TemplateSetting, TemplateValue};
use khalid_api::settings_store::{MemorySettings, SettingsError, SettingsStore};
use khalid_core::FixedClock;
use serde_json::json;

#[cfg(test)]
fn at(day: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, day, 9, 0, 0).unwrap()
}

/// The demo's settings, last changed on 20 October.
#[cfg(test)]
fn demo() -> MemorySettings {
    let settings = AdminSettings {
        name: "Field Notes on SEO".to_owned(),
        address: "https://www.example.com".to_owned(),
        mount: "/blog/".to_owned(),
        default_language: "en".to_owned(),
        languages: vec!["en".to_owned(), "de".to_owned()],
        template: TemplateSetting {
            name: "classic".to_owned(),
            options: BTreeMap::new(),
        },
        updated_at: at(20),
        restart_needed: false,
    };
    MemorySettings::new(settings).with_clock(FixedClock::new(at(27)))
}

#[cfg(test)]
fn change(value: serde_json::Value) -> SettingsChange {
    serde_json::from_value(value).unwrap()
}

#[tokio::test]
async fn the_settings_are_sent_in_api_shape() {
    let settings = demo().read().await.unwrap();
    assert_eq!(
        serde_json::to_value(settings).unwrap(),
        json!({
            "name": "Field Notes on SEO", "address": "https://www.example.com",
            "mount": "/blog/", "default_language": "en", "languages": ["en", "de"],
            "template": {"name": "classic", "options": {}},
            "updated_at": "2026-10-20T09:00:00Z", "restart_needed": false,
        })
    );
}

#[tokio::test]
async fn a_template_change_reads_each_kind_of_value_and_replaces_every_option() {
    let store = demo();
    let sent = json!({"template": {"name": "ledger", "options": {
        "logo": false, "related_count": 6, "contents": "inline", "stats": ["12,000 readers"],
    }}, "updated_at": "2026-10-20T09:00:00Z"});
    let saved = store.change(change(sent)).await.unwrap();
    assert_eq!(saved.template.name, "ledger");
    let options: Vec<&TemplateValue> = saved.template.options.values().collect();
    assert_eq!(
        options,
        [
            &TemplateValue::Word("inline".to_owned()),
            &TemplateValue::Switch(false),
            &TemplateValue::Number(6),
            &TemplateValue::Words(vec!["12,000 readers".to_owned()]),
        ]
    );
    let nested = json!({"template": {"name": "ledger", "options": {"related": {"count": 5}}},
        "updated_at": "2026-10-27T09:00:00Z"});
    let refused = serde_json::from_value::<SettingsChange>(nested);
    assert!(refused.is_err(), "an option is never a table");
}

#[tokio::test]
async fn a_change_on_the_latest_copy_sets_only_what_it_sends() {
    let store = demo();
    let sent = json!({"name": "Field Notes", "updated_at": "2026-10-20T09:00:00Z"});
    let saved = store.change(change(sent)).await.unwrap();
    assert_eq!(saved.name, "Field Notes");
    assert_eq!(saved.mount, "/blog/");
    assert_eq!(saved.updated_at, at(27));
}

#[tokio::test]
async fn a_stale_or_unstartable_change_saves_nothing() {
    let store = demo();
    let stale = json!({"name": "Lost", "updated_at": "2026-10-19T09:00:00Z"});
    let refused = store.change(change(stale)).await;
    assert!(matches!(refused, Err(SettingsError::Stale { updated_at }) if updated_at == at(20)));
    let orphan = json!({"default_language": "fr", "updated_at": "2026-10-20T09:00:00Z"});
    let refused = store.change(change(orphan)).await;
    assert!(
        matches!(&refused, Err(SettingsError::Refused { field, .. }) if field == "default_language"),
        "{refused:?}"
    );
    assert_eq!(store.read().await.unwrap().name, "Field Notes on SEO");
    let unknown = serde_json::from_value::<SettingsChange>(
        json!({"mode": "static", "updated_at": "2026-10-20T09:00:00Z"}),
    );
    assert!(unknown.is_err(), "mode is not changed through the API");
}
