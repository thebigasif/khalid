//! "Copy schema from a URL" through `khalid serve` (KH-SEO-4): the Admin
//! API fetches a page, here a stand-in, through the shared outbound client
//! and answers with its JSON-LD. A page inside the network is refused.

#[expect(
    dead_code,
    reason = "these tests start their own server, with the test hosts flag"
)]
mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ROUTE: &str = "/api/admin/v1/schema/copies/";

/// What `khalid keys create` makes for these tests: a key the editor could use.
const EDITOR: [&str; 4] = ["--name", "Editor", "--scope", "posts:write"];

/// A page with one JSON-LD script, as a recipe site might write it.
const PAGE: &str = r#"<!doctype html><html><head><title>Lentil soup</title>
<script type="application/ld+json">{"@context": "https://schema.org", "@graph": [
  {"@type": "WebSite", "@id": "https://recipes.test/#site", "name": "Recipes"},
  {"@type": "Recipe", "@id": "https://recipes.test/soup/#recipe", "name": "Lentil soup",
   "isPartOf": {"@id": "https://recipes.test/#site"}, "recipeYield": "4 bowls"}
]}</script></head><body><h1>Lentil soup</h1></body></html>"#;

/// A running `khalid serve`, stopped when dropped.
struct Server(Child, String);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Starts `khalid serve` on `site`, with `recipes.test` answered by the
/// stand-in on loopback, and reads the address it prints.
#[cfg(test)]
fn start(site: &Path) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(["--test-hosts", "recipes.test=127.0.0.1"])
        .args(["serve", "--listen", "127.0.0.1:0", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let json: Value = serde_json::from_str(&line).unwrap_or_else(|_| panic!("{line}"));
    let address = json["address"].as_str().unwrap();
    let host = address.strip_prefix("http://").unwrap();
    Server(child, host.trim_end_matches("/blog/").to_owned())
}

/// POSTs `url` to the copy route with `key`; answers the status and the
/// body as JSON.
#[cfg(test)]
fn copy(server: &Server, key: &str, url: &str) -> (u16, Value) {
    let body = json!({ "url": url }).to_string();
    let mut stream = TcpStream::connect(&server.1).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let head = format!(
        "POST {ROUTE} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAuthorization: Khalid {key}\r\n\
         Content-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
        server.1,
        body.len()
    );
    stream.write_all(head.as_bytes()).unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    let status = head.split(' ').nth(1).unwrap().parse().unwrap();
    (status, serde_json::from_str(body).unwrap_or(Value::Null))
}

#[tokio::test(flavor = "multi_thread")]
async fn the_editor_copies_a_pages_schema_without_the_other_sites_ids() {
    let recipes = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/soup/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(PAGE))
        .expect(1)
        .mount(&recipes)
        .await;
    let site = common::demo_copy();
    let key = common::new_key(site.path(), &EDITOR);
    let server = start(site.path());

    let port = recipes.address().port();
    let (status, json) = copy(&server, &key, &format!("http://recipes.test:{port}/soup/"));
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["types"], json!(["WebSite", "Recipe"]));
    assert_eq!(json["problem"], Value::Null, "{json}");
    let json_ld: Value = serde_json::from_str(json["json_ld"].as_str().unwrap()).unwrap();
    assert_eq!(
        json_ld["@graph"][1],
        json!({"@type": "Recipe", "name": "Lentil soup", "recipeYield": "4 bowls"})
    );

    // The private-network guard holds: the test table lets loopback in,
    // and nothing else inward.
    let (status, json) = copy(&server, &key, "http://10.0.0.1/admin/");
    assert_eq!(status, 422, "{json}");
    assert!(
        json["detail"].as_str().unwrap().contains("inside"),
        "{json}"
    );
}
