//! Search Console follow-ups through `khalid serve` (GEO M3): the Admin
//! API asks Google, here a stand-in, and names the heading of each post
//! that answers each long search, as `khalid search-console follow-ups`
//! does. With `report_to` and `[mail]` set, the server sends the weekly
//! email on its own (G43).

#[expect(
    dead_code,
    reason = "these tests start their own server, with the Google test flags"
)]
mod common;

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use khalid_core::{Clock, SystemClock};
use khalid_mail::StubSmtp;
use serde_json::{Value, json};
use tempfile::TempDir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Made for these tests; Google never issued them.
const SECRET: &str = "made-up-client-secret";
const REFRESH: &str = "1//made-up-refresh-token";

const OPT_IN: &str = "\n[search_console]\nclient_id = \"1234-abcd.apps.googleusercontent.com\"\n";

const ANALYTICS: &str =
    "/webmasters/v3/sites/https%3A%2F%2Fwww.example.com%2F/searchAnalytics/query";

const ROUTE: &str = "/api/admin/v1/search-console/follow-ups/";
const REPORT: &str = "/api/admin/v1/search-console/report/";

/// What `khalid keys create` makes for these tests: a key the screen could use.
const SCREEN: [&str; 4] = ["--name", "Screen", "--scope", "settings:write"];

/// A running `khalid serve`, stopped when dropped.
struct Server(Child, String);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// A copy of the demo site with `extra` added to its `site.toml`.
#[cfg(test)]
fn demo_site(extra: &str) -> TempDir {
    let temp = common::demo_copy();
    let mut settings = temp.path().to_path_buf();
    settings.push("site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    fs::write(&settings, format!("{text}{extra}")).unwrap();
    temp
}

/// Starts `khalid serve` on `site` with both Google secrets, reaching
/// `google` for Google, and reads the address it prints.
#[cfg(test)]
fn start(site: &Path, google: &MockServer) -> Server {
    let port = google.address().port();
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(["--test-hosts", "google.test=127.0.0.1"])
        .args(["--test-google", &format!("http://google.test:{port}/")])
        .args(["serve", "--listen", "127.0.0.1:0", "--json"])
        .env("KHALID_GOOGLE_CLIENT_SECRET", SECRET)
        .env("KHALID_GOOGLE_REFRESH_TOKEN", REFRESH)
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

/// GETs `route` from `server` with `key`; answers the status and
/// the body as JSON.
#[cfg(test)]
fn get(server: &Server, route: &str, key: &str) -> (u16, Value) {
    let mut stream = TcpStream::connect(&server.1).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let head = format!(
        "GET {route} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAuthorization: Khalid {key}\r\n\r\n",
        server.1
    );
    stream.write_all(head.as_bytes()).unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    let status = head.split(' ').nth(1).unwrap().parse().unwrap();
    (status, serde_json::from_str(body).unwrap_or(Value::Null))
}

#[tokio::test(flavor = "multi_thread")]
async fn the_admin_api_names_the_heading_that_answers_each_long_search() {
    let google = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "access_token": "ya29.t" })))
        .mount(&google)
        .await;
    let links = "https://www.example.com/blog/why-internal-links-matter/";
    let row = |query: &str, impressions: f64| {
        json!({ "keys": [links, query], "clicks": 2.0, "impressions": impressions,
            "ctr": 0.02, "position": 8.2 })
    };
    let german = "https://www.example.com/blog/de/warum-interne-links-wichtig-sind/";
    let rows = json!({ "rows": [
        row("internal links", 900.0),
        row("how many internal links should a blog post have", 60.0),
        row("how do search engines find and rank pages on a site", 40.0),
        { "keys": [german, "wie nutzen Suchmaschinen die Links zum Finden und Bewerten"],
            "clicks": 1.0, "impressions": 30.0, "ctr": 0.03, "position": 9.1 },
    ] });
    Mock::given(method("POST"))
        .and(path(ANALYTICS))
        .respond_with(ResponseTemplate::new(200).set_body_json(rows))
        .mount(&google)
        .await;
    let site = demo_site(OPT_IN);
    let key = common::new_key(site.path(), &SCREEN);
    let server = start(site.path(), &google);

    let (status, json) = get(&server, ROUTE, &key);
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["words"], 8);
    let post = &json["posts"][0];
    assert_eq!(post["url"], links, "{json}");
    assert_eq!(post["id"], "en.why-internal-links-matter");
    let searches = &post["searches"];
    assert_eq!(searches.as_array().map(Vec::len), Some(2), "{json}");
    assert_eq!(searches[0]["answered_by"], Value::Null);
    assert_eq!(
        searches[1]["answered_by"],
        "Search engines use them to find and rank pages"
    );
    // German filler words, such as "die" and "und", are left out of a
    // German post's searches, so its heading answers this one.
    let post = &json["posts"][1];
    assert_eq!(post["url"], german, "{json}");
    assert_eq!(
        post["searches"][0]["answered_by"],
        "Suchmaschinen nutzen sie zum Finden und Bewerten"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn the_admin_api_reports_each_posts_numbers_with_its_identity() {
    let google = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "access_token": "ya29.t" })))
        .mount(&google)
        .await;
    let row = json!({ "keys": ["https://www.example.com/blog/why-internal-links-matter/"],
        "clicks": 120.0, "impressions": 2400.0, "ctr": 0.05, "position": 3.1 });
    Mock::given(method("POST"))
        .and(path(ANALYTICS))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "rows": [row] })))
        .mount(&google)
        .await;
    let site = demo_site(OPT_IN);
    let key = common::new_key(site.path(), &SCREEN);
    let server = start(site.path(), &google);

    let (status, json) = get(&server, REPORT, &key);
    assert_eq!(status, 200, "{json}");
    let first = &json["posts"][0];
    assert_eq!(first["id"], "en.why-internal-links-matter", "{json}");
    assert_eq!(first["clicks"], 120.0);
    assert_eq!(json["week_clicks"], 120.0);
    let unseen = json["posts"].as_array().and_then(|posts| posts.last());
    assert_eq!(unseen.map(|post| &post["position"]), Some(&Value::Null));
}

#[tokio::test(flavor = "multi_thread")]
async fn serve_sends_the_weekly_email_on_its_own_and_keeps_the_day() {
    let google = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "access_token": "ya29.t" })))
        .mount(&google)
        .await;
    let row = json!({ "keys": ["https://www.example.com/blog/what-a-sitemap-does/"],
        "clicks": 8.0, "impressions": 160.0, "ctr": 0.05, "position": 3.1 });
    Mock::given(method("POST"))
        .and(path(ANALYTICS))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "rows": [row] })))
        .mount(&google)
        .await;
    let stub = StubSmtp::start().unwrap();
    let mail = format!(
        "report_to = \"owner@example.com\"\n\n[mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\n\
         host = \"127.0.0.1\"\nport = {}\nsecurity = \"none\"\nprivate_host = true\n",
        stub.port()
    );
    let site = demo_site(&format!("{OPT_IN}{mail}"));
    let _server = start(site.path(), &google);

    for _ in 0..300 {
        if !stub.received().is_empty() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    let received = stub.received();
    assert_eq!(received.len(), 1, "{received:?}");
    assert!(
        received[0].contains("To: owner@example.com"),
        "{received:?}"
    );
    assert!(
        received[0].contains("Clicks from Google: 8 this week"),
        "{received:?}"
    );
    let mut kept = site.path().to_path_buf();
    kept.push("data/search-console-email.txt");
    let today = SystemClock.now().date_naive();
    assert_eq!(fs::read_to_string(kept).unwrap(), format!("{today}\n"));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_site_that_did_not_opt_in_is_told_how_and_google_is_not_asked() {
    let google = MockServer::start().await;
    let site = demo_site("");
    let key = common::new_key(site.path(), &SCREEN);
    let server = start(site.path(), &google);

    let (status, json) = get(&server, ROUTE, &key);
    assert_eq!(status, 409, "{json}");
    assert_eq!(json["type"], "https://khalid.dev/problems/not-set-up");
    assert!(
        json["fix"]
            .as_str()
            .is_some_and(|fix| fix.ends_with("then restart khalid serve.")),
        "{json}"
    );
    assert_eq!(google.received_requests().await.map(|r| r.len()), Some(0));
}
