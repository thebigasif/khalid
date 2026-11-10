//! The API's rate limits on a running `khalid serve` (RULES.md BL-SEC-5,
//! audit brief G33), and the search page's, with the numbers a site sets
//! in `[limits]` (KH-OPS-5).

mod common;

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

use common::{demo_copy, new_key, start};
use khalid_core::LimitSettings;

/// Adds `table` to the end of the `site.toml` of `site`.
#[cfg(test)]
fn add_to_site_toml(site: &Path, table: &str) {
    let mut path = site.to_path_buf();
    path.push("site.toml");
    let mut file = OpenOptions::new().append(true).open(&path).unwrap();
    file.write_all(table.as_bytes()).unwrap();
    assert!(fs::read_to_string(&path).unwrap().ends_with(table));
}

/// `key` with its last letter changed, so it is a wrong key of the right
/// length.
#[cfg(test)]
fn wrong(key: &str) -> String {
    let last = if key.ends_with('0') { "1" } else { "0" };
    format!("{}{last}", key.get(..key.len() - 1).unwrap())
}

/// Sends `GET path` to `host` for the reader at `reader`, as a proxy on
/// this machine names it, and returns the status, the whole head in lower
/// case, and the body.
#[cfg(test)]
fn get_for(host: &str, path: &str, reader: &str) -> (u16, String, String) {
    let mut stream = TcpStream::connect(host).unwrap();
    let head = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nX-Forwarded-For: {reader}\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(head.as_bytes()).unwrap();
    let mut answer = Vec::new();
    stream.read_to_end(&mut answer).unwrap();
    let raw = String::from_utf8_lossy(&answer);
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    let status = head.split(' ').nth(1).unwrap().parse().unwrap();
    (status, head.to_ascii_lowercase(), body.to_owned())
}

#[test]
fn an_address_that_sends_wrong_keys_is_paused_on_both_apis() {
    let site = demo_copy();
    let content = new_key(site.path(), &["--kind", "content", "--name", "Web page"]);
    let writer = new_key(site.path(), &["--name", "Writer", "--scope", "posts:write"]);
    let server = start(site.path(), &writer);
    let posts = format!("/api/content/v1/posts/?key={content}&limit=1");
    assert!(server.answers(&posts, 200));
    assert_eq!(server.admin("GET", "/api/admin/v1/posts/", None).0, 200);

    let guess = format!("/api/content/v1/posts/?key={}&limit=1", wrong(&content));
    for _ in 0..LimitSettings::default().wrong_keys_per_minute {
        assert_eq!(server.raw("GET", &guess, &[], None).0, 401);
    }

    let (status, body) = server.raw("GET", &posts, &[], None);
    assert_eq!(status, 429, "{body}");
    assert!(body.contains("too-many-requests"), "{body}");
    let (status, body) = server.admin("GET", "/api/admin/v1/posts/", None);
    assert_eq!(status, 429, "the pause covers the Admin API too: {body}");
}

#[test]
fn a_reader_past_the_sites_own_limits_gets_a_429_with_retry_after() {
    let site = demo_copy();
    let table = "\n[limits]\nsearch_per_minute = 3\nwrong_keys_per_minute = 2\n";
    add_to_site_toml(site.path(), table);
    let content = new_key(site.path(), &["--kind", "content", "--name", "Web page"]);
    let server = start(site.path(), &content);
    assert!(server.answers("/blog/search/", 200));

    let (host, reader) = (server.host(), "198.51.100.7");
    let asks = "/blog/search/?q=links";
    for _ in 0..3 {
        let (status, head, _) = get_for(host, asks, reader);
        assert_ne!(status, 429, "{head}");
    }
    let (status, head, page) = get_for(host, asks, reader);
    assert_eq!(status, 429, "{head}");
    assert!(head.contains("retry-after: 20"), "{head}");
    assert!(
        head.contains("text/html"),
        "the search page answers: {head}"
    );
    assert!(
        page.contains("Wait a minute, then search again."),
        "the reader is told why and what to do: {page}"
    );
    assert!(page.contains("<form"), "{page}");
    assert_eq!(get_for(host, "/blog/search/?q=lin&typing=1", reader).0, 429);
    let (status, _, _) = get_for(host, "/blog/search/", reader);
    assert_eq!(status, 200, "the empty form costs nothing");
    assert_ne!(get_for(host, asks, "198.51.100.8").0, 429);

    let guess = format!("/api/content/v1/posts/?key={}&limit=1", wrong(&content));
    for _ in 0..2 {
        assert_eq!(get_for(host, &guess, reader).0, 401);
    }
    let posts = format!("/api/content/v1/posts/?key={content}&limit=1");
    assert_eq!(get_for(host, &posts, reader).0, 429);
}
