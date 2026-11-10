//! Deleting a library file on a running `khalid serve` (G51): a file a
//! post still names stays, and once the post lets go of it, it goes.

mod common;

use std::fs;

use serde_json::Value;

const PDF: &[u8] = b"%PDF-1.7\nA checklist for choosing a CRM\n%%EOF\n";
const POST: &str = "content/en/posts/2026-09-21-how-to-write-a-title-tag.md";

#[test]
fn a_file_a_post_names_stays_until_the_post_lets_go() {
    let site = common::demo_copy();
    let key = common::new_key(site.path(), &["--name", "Maria", "--scope", "media:write"]);
    let server = common::start(site.path(), &key);
    let auth = format!("Khalid {key}");
    let headers = [("Authorization", auth.as_str())];
    let upload = "/api/admin/v1/media/?name=CRM%20checklist.pdf";
    let (status, body) = server.send(
        "POST",
        upload,
        &headers,
        (PDF, Some("application/octet-stream")),
    );
    assert_eq!(status, 201, "{body}");
    let made: Value = serde_json::from_str(&body).unwrap();
    let item = &made["media"][0];
    let (id, url) = (item["id"].as_str().unwrap(), item["url"].as_str().unwrap());
    let updated_at = item["updated_at"].as_str().unwrap();
    let delete = format!("/api/admin/v1/media/{id}/?updated_at={updated_at}");

    let mut post = site.path().to_path_buf();
    post.push(POST);
    let text = fs::read_to_string(&post).unwrap();
    fs::write(&post, format!("{text}\n[The checklist]({url})\n")).unwrap();
    let (status, refused) = server.admin("DELETE", &delete, None);
    assert_eq!(status, 409, "{refused}");
    let detail = refused["detail"].as_str().unwrap();
    assert!(detail.contains(POST), "{detail}");
    let path = url.strip_prefix("https://www.example.com").unwrap();
    assert!(server.answers(path, 200), "the file is still served");

    fs::write(&post, text).unwrap();
    let (status, body) = server.admin("DELETE", &delete, None);
    assert_eq!(status, 204, "{body}");
    assert!(server.answers(path, 404), "the file is gone");
}
