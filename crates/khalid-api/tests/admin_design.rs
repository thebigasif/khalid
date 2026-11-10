//! The Design screen's route: `theme/tokens.toml` read with the contrast of
//! each colour pair, changed in place, a stale copy refused with a 409, a
//! bad value with a 422, and every role but owner and admin with a 403.

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use khalid_api::api::{self, AdminDesks};
use khalid_api::content_store::MemoryContentStore;
use khalid_api::design_desk::DesignDesk;
use khalid_api::keys::{KeyHashing, KeyToken, MemoryKeyStore, Role, Scope, StoredKey};
use khalid_api::post_store::MemoryPostStore;
use khalid_api::publish_door::{PublishDoor, Silent};
use khalid_theme::DesignFile;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Keys made up for these tests. They open nothing real.
const OWNER: &str =
    "ks_1111222233334444_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const ADMIN: &str =
    "ks_2222333344445555_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const EDITOR: &str =
    "ks_3333444455556666_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";
const SETTINGS: &str =
    "ki_4444555566667777_aaaabbbbccccddddeeeeffff00001111aaaabbbbccccddddeeeeffff00001111";

const DESIGN: &str = "/api/admin/v1/design/";

/// A synced file, with a comment beside the link and a type scale the
/// screen does not edit.
const SYNCED: &str = "locked = [\"colors.link\"]\n\n[colors]\nbackground = \"#ffffff\"\n\
    surface = \"#f6f7f9\"\ntext = \"#1f2328\"\nmuted = \"#57606a\"\n\
    link = \"#0b57d0\" # brand blue\naccent = \"#0b57d0\"\nborder = \"#d0d7de\"\n\n\
    [[fonts.files]]\nfamily = \"Inter\"\nstyle = \"normal\"\nweight = \"400\"\n\
    path = \"inter-latin.woff2\"\n\n[type]\nbody = \"1.0625rem\"\n";

/// The chosen template's tokens.
const PACK: &str = "[colors]\naccent = \"#7a3e00\"\n\n[shape]\nradius = \"2px\"\n";

#[cfg(test)]
fn pack() -> &'static str {
    PACK
}

/// The API over the site in `site`, or over none.
#[cfg(test)]
fn router(site: Option<&Path>) -> Router {
    let hashing = KeyHashing::fast_for_tests();
    let token = |text: &str| text.parse::<KeyToken>().unwrap();
    let staff = |text, role| StoredKey::for_staff(&token(text), role, &hashing).unwrap();
    let keys = MemoryKeyStore::new(vec![
        staff(OWNER, Role::Owner),
        staff(ADMIN, Role::Admin),
        staff(EDITOR, Role::Editor),
        StoredKey::for_integration(&token(SETTINGS), vec![Scope::SettingsWrite], &hashing).unwrap(),
    ]);
    let desks = AdminDesks {
        design: site.map(|site| DesignDesk::new(site, Arc::new(pack))),
        ..AdminDesks::none()
    };
    let door = PublishDoor::new(Arc::new(MemoryPostStore::default()), Arc::new(Silent));
    let content = Arc::new(MemoryContentStore::new(Vec::new()));
    let (router, _) = api::router_with_desks(content, Arc::new(door), Arc::new(keys), desks);
    router
}

/// A site folder whose `theme/tokens.toml` is `text`.
#[cfg(test)]
fn site(text: &str) -> tempfile::TempDir {
    let site = tempfile::tempdir().unwrap();
    DesignFile::from_toml(text)
        .unwrap()
        .write(site.path())
        .unwrap();
    site
}

#[cfg(test)]
async fn send(
    router: &Router,
    method: &str,
    key: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(DESIGN)
        .header("authorization", format!("Khalid {key}"))
        .header("content-type", "application/json");
    let body = body.map_or_else(Body::empty, |body| Body::from(body.to_string()));
    let response = router
        .clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// The check of `token` on `ground` in a read design.
#[cfg(test)]
fn contrast<'a>(json: &'a Value, token: &str, ground: &str) -> &'a Value {
    let checks = json["design"]["contrast"].as_array().unwrap();
    checks
        .iter()
        .find(|check| check["token"] == token && check["ground"] == ground)
        .unwrap()
}

#[tokio::test]
async fn the_owner_reads_the_values_pages_use_with_each_contrast() {
    let site = site(SYNCED);
    let (status, json) = send(&router(Some(site.path())), "GET", OWNER, None).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    let design = &json["design"];
    assert_eq!(design["tokens"]["colors.link"], "#0b57d0", "the file's");
    assert_eq!(design["tokens"]["shape.radius"], "2px", "the pack's");
    assert_eq!(design["tokens"]["brand.logo"], "", "no logo");
    assert_eq!(design["locked"], json!(["colors.link"]));
    assert!(design["updated_at"].is_string(), "{json}");
    let text = contrast(&json, "colors.text", "colors.background");
    assert_eq!(text["ratio"], 15.7, "{text}");
    assert_eq!(text["passes"], true);
    assert_eq!(design["contrast"].as_array().unwrap().len(), 8);
    let fonts = design["fonts"].as_array().unwrap();
    let inter = fonts.iter().find(|font| font["label"] == "Inter");
    assert!(
        inter.is_some(),
        "the self-hosted family is offered: {fonts:?}"
    );
}

#[tokio::test]
async fn a_change_is_saved_in_place_and_a_replay_of_the_old_copy_is_a_409() {
    let site = site(SYNCED);
    let router = router(Some(site.path()));
    let (_, read) = send(&router, "GET", ADMIN, None).await;
    let change = json!({"design": {
        "tokens": {"colors.link": "#60a5fa", "fonts.heading": "Georgia, serif"},
        "locked": ["colors.link", "fonts.heading"],
        "updated_at": read["design"]["updated_at"],
    }});
    let (status, json) = send(&router, "PUT", ADMIN, Some(change.clone())).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["design"]["tokens"]["colors.link"], "#60a5fa");
    let link = contrast(&json, "colors.link", "colors.background");
    assert_eq!(link["passes"], false, "a faint link: {link}");
    assert!(link["nearest"].is_string(), "a shade that passes: {link}");
    let text = DesignFile::read(site.path()).unwrap().to_toml();
    assert!(text.contains("link = \"#60a5fa\" # brand blue"), "{text}");
    assert!(text.contains("[type]\nbody = \"1.0625rem\""), "{text}");
    assert!(
        text.contains("locked = [\"colors.link\", \"fonts.heading\"]"),
        "{text}"
    );
    let (status, json) = send(&router, "PUT", ADMIN, Some(change)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{json}");
    assert_eq!(json["rule"], "updated_at", "{json}");
}

#[tokio::test]
async fn a_site_without_the_file_gets_one_with_only_the_change() {
    let site = tempfile::tempdir().unwrap();
    let router = router(Some(site.path()));
    let (status, read) = send(&router, "GET", SETTINGS, None).await;
    assert_eq!(status, StatusCode::OK, "{read}");
    assert!(read["design"]["updated_at"].is_null(), "{read}");
    assert_eq!(read["design"]["tokens"]["colors.accent"], "#7a3e00");
    let logo = "https://www.example.com/media/logo.svg";
    let change = json!({"design": {
        "tokens": {"brand.logo": logo}, "updated_at": null,
    }});
    let (status, json) = send(&router, "PUT", SETTINGS, Some(change)).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["design"]["tokens"]["brand.logo"], logo);
    let text = DesignFile::read(site.path()).unwrap().to_toml();
    assert!(
        !text.contains("[colors]"),
        "the pack keeps its colours: {text}"
    );
}

#[tokio::test]
async fn a_bad_value_is_a_422_in_plain_words_and_nothing_is_saved() {
    let site = site(SYNCED);
    let router = router(Some(site.path()));
    let (_, read) = send(&router, "GET", OWNER, None).await;
    let at = &read["design"]["updated_at"];
    let cases = [
        (
            "colors.link",
            "blue",
            "a colour must be written #rrggbb or #rgb",
        ),
        ("fonts.body", "Inter; x", "a font list may hold only names"),
        ("colors.glow", "#ffffff", "Design screen changes only"),
    ];
    for (token, value, words) in cases {
        let change = json!({"design": {"tokens": {token: value}, "updated_at": at}});
        let (status, json) = send(&router, "PUT", OWNER, Some(change)).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
        assert_eq!(json["rule"], token, "{json}");
        let detail = json["detail"].as_str().unwrap();
        assert!(detail.contains(words), "{detail}");
        assert!(json["fix"].as_str().unwrap().contains("nothing was saved"));
    }
    let locked = json!({"design": {"locked": ["colors.glow"], "updated_at": at}});
    let (status, json) = send(&router, "PUT", OWNER, Some(locked)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(DesignFile::read(site.path()).unwrap().to_toml(), SYNCED);
}

#[tokio::test]
async fn only_the_owner_and_admins_reach_the_design() {
    let site = site(SYNCED);
    let router = router(Some(site.path()));
    let (status, json) = send(&router, "GET", EDITOR, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["rule"], "settings:write", "{json}");
    let change = json!({"design": {"tokens": {"colors.link": "#000000"}, "updated_at": null}});
    let (status, _) = send(&router, "PUT", EDITOR, Some(change)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(DesignFile::read(site.path()).unwrap().to_toml(), SYNCED);
    let (status, json) = send(&self::router(None), "GET", OWNER, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{json}");
}
