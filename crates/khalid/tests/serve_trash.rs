//! The trash on a running `khalid serve` (KH-EDT-6, brief G52): a deleted
//! post's file waits under `trash/`, out of `content/`, until it is
//! restored and served again or deleted for good.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use common::{Server, demo_copy, new_key, start};

/// The demo post the tests delete, and its address.
const ID: &str = "en.why-internal-links-matter";
const PAGE: &str = "/blog/why-internal-links-matter/";

/// `parts` under `site`.
#[cfg(test)]
fn under(site: &Path, parts: &[&str]) -> PathBuf {
    let mut path = site.to_path_buf();
    path.extend(parts);
    path
}

/// Deletes the demo post and answers its key in the trash.
#[cfg(test)]
fn delete(server: &Server) -> String {
    let (status, post) = server.admin("GET", &format!("/api/admin/v1/posts/{ID}/"), None);
    assert_eq!(status, 200, "{post}");
    let seen = post["posts"][0]["updated_at"].as_str().unwrap();
    let path = format!("/api/admin/v1/posts/{ID}/?updated_at={seen}");
    let (status, body) = server.admin("DELETE", &path, None);
    assert_eq!(status, 204, "{body}");
    let (status, trash) = server.admin("GET", "/api/admin/v1/trash/", None);
    assert_eq!(status, 200, "{trash}");
    assert_eq!(trash["trash"][0]["id"], ID, "{trash}");
    assert_eq!(trash["trash"][0]["status"], "published", "{trash}");
    trash["trash"][0]["key"].as_str().unwrap().to_owned()
}

#[test]
fn a_deleted_post_waits_in_the_trash_and_comes_back_when_restored() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--name", "Owner", "--scope", "admin:all"]);
    let server = start(site.path(), &key);
    let file = "2026-09-14-why-internal-links-matter.md";
    let kept = under(site.path(), &["content", "en", "posts", file]);
    let trashed = delete(&server);
    assert!(
        server.answers(PAGE, 404),
        "the deleted post is still served"
    );
    assert!(!kept.exists());
    let waiting = ["trash", &trashed, "content", "en", "posts", file];
    assert!(under(site.path(), &waiting).is_file());

    let restore = format!("/api/admin/v1/trash/{trashed}/restore/");
    let (status, body) = server.admin("POST", &restore, None);
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["posts"][0]["status"], "published", "{body}");
    assert!(server.answers(PAGE, 200), "the restored post is not served");
    assert!(kept.is_file());
    assert!(!under(site.path(), &["trash", &trashed]).exists());
    let (_, trash) = server.admin("GET", "/api/admin/v1/trash/", None);
    assert_eq!(trash["trash"], Value::Array(Vec::new()), "{trash}");
}

#[test]
fn a_post_deleted_for_good_leaves_nothing_behind() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--name", "Owner", "--scope", "admin:all"]);
    let server = start(site.path(), &key);
    let trashed = delete(&server);
    let purge = format!("/api/admin/v1/trash/{trashed}/");
    let (status, body) = server.admin("DELETE", &purge, None);
    assert_eq!(status, 204, "{body}");
    assert!(!under(site.path(), &["trash", &trashed]).exists());
    let (status, _) = server.admin("DELETE", &purge, None);
    assert_eq!(status, 404);

    // A key that climbs out of trash/ names nothing, so nothing is touched.
    // Unguarded, this key would name the site folder itself.
    fs::create_dir_all(under(site.path(), &["trash", "20261031T093000Z-x"])).unwrap();
    let climbing = "20261031T093000Z-x%2F..%2F..";
    for (method, path) in [("DELETE", ""), ("POST", "restore/")] {
        let (status, _) = server.admin(
            method,
            &format!("/api/admin/v1/trash/{climbing}/{path}"),
            None,
        );
        assert_eq!(status, 404, "{method} {path}");
    }
    assert!(under(site.path(), &["content", "en", "posts"]).is_dir());
}

/// Puts draft `slug` in the trash of `site`, deleted at `stamp`, and
/// answers its folder.
#[cfg(test)]
fn trashed_draft(site: &Path, stamp: &str, slug: &str) -> PathBuf {
    let key = format!("{stamp}-en.{slug}");
    let folder = under(site, &["trash", &key]);
    let file = under(&folder, &["drafts", "en", &format!("{slug}.md")]);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    let text = format!("---\ntitle: A deleted draft\nslug: {slug}\n---\n\nWords.\n");
    fs::write(&file, text).unwrap();
    folder
}

#[test]
fn the_trash_empties_copies_older_than_trash_days_on_its_own() {
    use khalid_core::{Clock, SystemClock};

    let site = demo_copy();
    let now = SystemClock.now();
    let stamp = |days| (now - chrono::TimeDelta::days(days)).format("%Y%m%dT%H%M%SZ");
    let old = trashed_draft(site.path(), &stamp(31).to_string(), "old-draft");
    let recent = trashed_draft(site.path(), &stamp(29).to_string(), "recent-draft");
    let key = new_key(site.path(), &["--name", "Owner", "--scope", "admin:all"]);
    let server = start(site.path(), &key);
    let mut waited = 0;
    while old.exists() && waited < 100 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        waited += 1;
    }
    assert!(!old.exists(), "a copy 31 days old is still in the trash");
    assert!(recent.is_dir(), "a copy 29 days old was emptied");
    let (_, trash) = server.admin("GET", "/api/admin/v1/trash/", None);
    assert_eq!(trash["trash"][0]["id"], "en.recent-draft", "{trash}");
    assert_eq!(trash["trash"].as_array().map(Vec::len), Some(1), "{trash}");
}

#[test]
fn a_draft_comes_back_to_drafts_unless_its_address_was_taken() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--name", "Owner", "--scope", "admin:all"]);
    let server = start(site.path(), &key);
    let new = json!({"posts": [{"title": "Choosing anchor text", "markdown": "Words."}]});
    let make = || server.admin("POST", "/api/admin/v1/posts/", Some(&new));
    let (status, first) = make();
    assert_eq!(status, 201, "{first}");
    let seen = first["posts"][0]["updated_at"].as_str().unwrap();
    let path = format!("/api/admin/v1/posts/en.choosing-anchor-text/?updated_at={seen}");
    assert_eq!(server.admin("DELETE", &path, None).0, 204);
    let draft = under(site.path(), &["drafts", "en", "choosing-anchor-text.md"]);
    assert!(!draft.exists());
    let (_, trash) = server.admin("GET", "/api/admin/v1/trash/", None);
    assert_eq!(trash["trash"][0]["status"], "draft", "{trash}");
    let trashed = trash["trash"][0]["key"].as_str().unwrap();
    let restore = format!("/api/admin/v1/trash/{trashed}/restore/");

    assert_eq!(make().0, 201);
    let (status, body) = server.admin("POST", &restore, None);
    assert_eq!(status, 409, "{body}");
    fs::remove_file(&draft).unwrap();
    let (status, body) = server.admin("POST", &restore, None);
    assert_eq!(status, 200, "{body}");
    assert!(fs::read_to_string(&draft).unwrap().contains("Words."));
}
