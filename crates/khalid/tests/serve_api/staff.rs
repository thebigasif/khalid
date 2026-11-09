//! Staff on a running `khalid serve`: sign-in and staff tokens, and the
//! first owner made by `khalid staff owner` and the link that signs them in.

use std::path::Path;
use std::process::Command;

use serde_json::Value;

use super::{demo_copy, exchange, new_key, request, start};

#[test]
fn khalid_serve_answers_staff_sign_in_and_keeps_staff_tokens_working() {
    let site = demo_copy();
    let staff_token = new_key(
        site.path(),
        &["--kind", "staff", "--name", "Admin app", "--role", "owner"],
    );
    let server = start(site.path());
    let sign_in =
        serde_json::json!({"email": "nobody@example.com", "password": "correct horse battery"});
    let (status, media, body) = exchange(
        &server,
        "POST",
        "/api/admin/v1/session/",
        &[],
        Some(&sign_in),
    );
    assert_eq!(
        (status, media.as_str()),
        (401, "application/problem+json"),
        "{body}"
    );
    let (status, _) = request(&server, "GET", "/api/admin/v1/session/", &[], None);
    assert_eq!(status, 401, "no session cookie");
    let authorization = format!("Khalid {staff_token}");
    let auth = [("Authorization", authorization.as_str())];
    let (status, body) = request(&server, "GET", "/api/admin/v1/staff/", &auth, None);
    assert_eq!(status, 200, "an owner's staff token manages staff: {body}");
    let invite = serde_json::json!({"email": "mia@example.com", "name": "Mia", "role": "writer"});
    let (status, body) = request(
        &server,
        "POST",
        "/api/admin/v1/staff/",
        &auth,
        Some(&invite),
    );
    assert_eq!(status, 201, "{body}");
    let (_, body) = request(&server, "GET", "/api/admin/v1/staff/", &auth, None);
    assert!(
        body.contains("mia@example.com"),
        "kept in the site's database: {body}"
    );
}

/// Runs `khalid --site <site> staff owner --json` with `email` and `args`.
#[cfg(test)]
fn owner(site: &Path, email: &str, args: &[&str]) -> (Option<i32>, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "--json", "staff", "owner"])
        .args(["--email", email, "--name", "Asif"])
        .args(args)
        .output()
        .unwrap();
    let text = [output.stdout, output.stderr].concat();
    (output.status.code(), serde_json::from_slice(&text).unwrap())
}

#[test]
fn a_new_owner_gets_an_author_file_named_after_them() {
    let site = demo_copy();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "staff", "owner"])
        .args(["--email", "noor@example.com", "--name", "Noor Haddad"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let mut file = site.path().to_path_buf();
    file.push("content/en/authors/noor-haddad.md");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.contains("name: \"Noor Haddad\""), "{text}");
    // Asking again signs the same owner in, and makes no second file.
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "staff", "owner"])
        .args(["--email", "noor@example.com", "--name", "Noor Haddad"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(std::fs::read_to_string(&file).unwrap(), text);
}

#[test]
fn the_owner_link_also_names_where_khalid_serve_listens_by_default() {
    let site = demo_copy();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "staff", "owner"])
        .args(["--email", "asif@example.com", "--name", "Asif"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let said = String::from_utf8(output.stdout).unwrap();
    assert!(
        said.contains("https://www.example.com/blog/admin/sign-in/link#token="),
        "{said}"
    );
    assert!(
        said.contains("http://localhost:8080/blog/admin/sign-in/link#token="),
        "{said}"
    );
    assert!(said.contains("`khalid serve`"), "{said}");
}

#[test]
fn the_first_owner_signs_in_with_the_link_khalid_staff_owner_shows_once() {
    let site = demo_copy();
    let server = start(site.path());
    let (_, port) = server.host.split_once(':').unwrap();
    // No --listen: the link names the port the running server took.
    let (code, made) = owner(site.path(), "asif@example.com", &[]);
    assert_eq!(code, Some(0), "{made}");
    let link = made["link"].as_str().unwrap();
    let (admin, token) = link.split_once("/sign-in/link#token=").unwrap();
    assert!(admin.ends_with("/admin"), "{link}");
    let local = made["local_link"].as_str().unwrap();
    let on_this_computer = format!("http://localhost:{port}/blog/admin/sign-in/link#token={token}");
    assert_eq!(
        local, on_this_computer,
        "the same link, where serve listens"
    );
    let (code, second) = owner(site.path(), "someone@example.com", &[]);
    assert_eq!(code, Some(1), "a second owner is refused: {second}");
    let redeem = serde_json::json!({ "token": token });
    let path = "/api/admin/v1/session/link/redeem/";
    let (status, body) = request(&server, "POST", path, &[], Some(&redeem));
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("\"role\":\"owner\""), "{body}");
    // Asif already had an author file, so the owner writes as it.
    assert!(body.contains("\"author\":\"en.asif\""), "{body}");
    let (status, _) = request(&server, "POST", path, &[], Some(&redeem));
    assert_eq!(status, 401, "the link works once");
}
