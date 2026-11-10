//! `khalid build` sends `IndexNow` each live post that is new or has a new
//! `updated:` date since the last build, when the site has a key and a
//! public address (task C8, KH-SEO-11). `IndexNow` is a `wiremock`
//! stand-in.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{Value, json};
use tempfile::TempDir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// The demo post the test updates, and its public address.
const POST: &str = "content/en/posts/2026-09-28-what-a-sitemap-does.md";
const ADDRESS: &str = "https://www.example.com/blog/what-a-sitemap-does/";

/// `rest` inside `dir`.
fn at(dir: &Path, rest: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(rest);
    path
}

/// A copy of the demo site, sending `IndexNow` to `receiver` when given.
#[cfg(test)]
fn demo_site(receiver: Option<&MockServer>) -> TempDir {
    let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    demo.push("../../fixtures/demo-site/.");
    let temp = tempfile::tempdir().unwrap();
    let copied = Command::new("cp")
        .arg("-R")
        .args([&demo, temp.path()])
        .status();
    assert!(copied.unwrap().success());
    if let Some(receiver) = receiver {
        let port = receiver.address().port();
        let settings = at(temp.path(), "site.toml");
        let toml = format!(
            "{}\n[seo.indexnow]\nkey = \"3f9c0b5e2a8d4c71\"\n\
             endpoint = \"http://indexnow.test:{port}/indexnow\"\n",
            fs::read_to_string(&settings).unwrap()
        );
        fs::write(&settings, toml).unwrap();
    }
    temp
}

/// Runs `khalid build` on the site, reaching the stand-in on loopback.
#[cfg(test)]
fn build(site: &Path, more: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(["--test-hosts", "indexnow.test=127.0.0.1"])
        .arg("build")
        .args(more)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    output
}

/// Gives the post a new `updated:` date.
#[cfg(test)]
fn update_post(site: &Path) {
    let file = at(site, POST);
    let text = fs::read_to_string(&file).unwrap();
    let updated = text.replacen(
        "date: 2026-09-28\n",
        "date: 2026-09-28\nupdated: 2026-10-30\n",
        1,
    );
    assert_ne!(text, updated, "the demo post's date line moved");
    fs::write(&file, updated).unwrap();
}

/// A stand-in `IndexNow` that takes every request.
#[cfg(test)]
async fn receiver() -> MockServer {
    let receiver = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/indexnow"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&receiver)
        .await;
    receiver
}

#[tokio::test(flavor = "multi_thread")]
async fn two_builds_with_one_update_between_send_that_one_address() {
    let receiver = receiver().await;
    let site = demo_site(Some(&receiver));

    let first = build(site.path(), &[]);
    let said = String::from_utf8_lossy(&first.stdout);
    assert!(said.contains("first run with IndexNow on"), "{said}");
    assert!(receiver.received_requests().await.unwrap().is_empty());

    update_post(site.path());
    let second = build(site.path(), &["--json"]);
    let body: Value = serde_json::from_slice(&second.stdout).unwrap();
    assert_eq!(body["indexnow_sent"], json!(1), "{body}");
    let requests = receiver.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1, "IndexNow is told once");
    let sent: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(sent["urlList"], json!([ADDRESS]), "{sent}");

    let third = build(site.path(), &[]);
    let said = String::from_utf8_lossy(&third.stdout);
    assert!(said.contains("Sent 0 addresses to IndexNow"), "{said}");
    assert_eq!(receiver.received_requests().await.unwrap().len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn no_indexnow_sends_nothing_and_keeps_the_change_for_later() {
    let receiver = receiver().await;
    let site = demo_site(Some(&receiver));
    build(site.path(), &[]);
    update_post(site.path());

    let quiet = build(site.path(), &["--no-indexnow"]);
    let said = String::from_utf8_lossy(&quiet.stdout);
    assert!(!said.contains("IndexNow"), "{said}");
    assert!(receiver.received_requests().await.unwrap().is_empty());

    build(site.path(), &[]);
    let requests = receiver.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1, "the next build sends the change");
}

#[test]
fn a_site_with_no_key_sends_nothing_and_keeps_no_record() {
    let site = demo_site(None);
    let built = build(site.path(), &[]);
    let said = String::from_utf8_lossy(&built.stdout);
    assert!(!said.contains("IndexNow"), "{said}");
    assert!(!at(site.path(), "data/indexnow-sent.txt").exists());
}
