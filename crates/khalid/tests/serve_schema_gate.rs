//! The publish door reads all of a post's schema (plan D-9): a draft whose
//! hand-written JSON-LD misses a property Google needs is refused by the
//! Admin API's publish route and by `khalid publish`, and never served.

mod common;

use std::fs;
use std::path::Path;
use std::process::Command;

use common::{demo_copy, new_key, start};
use serde_json::{Value, json};

/// The scopes the test key carries: whatever an editor may do.
const EDITOR: [&str; 6] = [
    "--name",
    "Editor",
    "--scope",
    "posts:write",
    "--scope",
    "posts:publish",
];

/// A draft about an event whose JSON-LD names no start date and no place.
const DRAFT: &str = "---
title: Internal links clinic
slug: internal-links-clinic
author: asif
description: A free evening clinic where we read your internal links together and fix the worst.
schema: |
  {\"@type\": \"Event\", \"name\": \"Internal links clinic\"}
---

Bring a post you want more readers to find. We read its links together.
";

/// Writes [`DRAFT`] into the drafts of `site`.
#[cfg(test)]
fn write_draft(site: &Path) {
    let mut drafts = site.to_path_buf();
    drafts.push("drafts");
    drafts.push("en");
    fs::create_dir_all(&drafts).unwrap();
    drafts.push("internal-links-clinic.md");
    fs::write(drafts, DRAFT).unwrap();
}

/// The subjects of every `invalid-structured-data` finding in `lint`.
#[cfg(test)]
fn gaps(lint: &Value) -> Vec<String> {
    let findings = lint["findings"].as_array().cloned().unwrap_or_default();
    findings
        .iter()
        .filter(|finding| finding["rule"] == "invalid-structured-data")
        .map(|finding| finding["subject"].as_str().unwrap_or_default().to_owned())
        .collect()
}

#[test]
fn the_publish_route_refuses_json_ld_that_misses_what_google_needs() {
    let site = demo_copy();
    write_draft(site.path());
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let route = "/api/admin/v1/posts/en.internal-links-clinic/";
    let (status, draft) = server.admin("GET", route, None);
    assert_eq!(status, 200, "{draft}");
    let publish = json!({"posts": [{
        "status": "published",
        "updated_at": draft["posts"][0]["updated_at"],
    }]});
    let (status, refused) = server.admin("PUT", route, Some(&publish));
    assert_eq!(status, 422, "{refused}");
    assert_eq!(
        gaps(&refused["lint"]),
        ["Event needs location", "Event needs startDate"],
        "{refused}"
    );
    assert!(server.answers("/blog/internal-links-clinic/", 404));
}

#[test]
fn khalid_publish_refuses_json_ld_that_misses_what_google_needs() {
    let site = demo_copy();
    write_draft(site.path());
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site.path())
        .args(["publish", "internal-links-clinic", "--json"])
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(1), "{said}");
    let answer: Value = serde_json::from_str(&said).unwrap();
    assert_eq!(
        gaps(&answer["lint"]),
        ["Event needs location", "Event needs startDate"],
        "{said}"
    );
}
