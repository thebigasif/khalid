//! Answers read straight off the socket: the drawn share card as PNG bytes,
//! and the `Link` header that carries the stylesheet as an early hint.

use std::io::{Read, Write};
use std::net::TcpStream;

use super::{Server, demo_copy, get, start};

/// Fetches `path` and returns its status and raw bytes, for files that are
/// not text.
#[cfg(test)]
fn get_bytes(server: &Server, path: &str) -> (u16, Vec<u8>) {
    let host = &server.host;
    let mut stream = TcpStream::connect(host).unwrap();
    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).unwrap();
    let end = raw.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
    let head = String::from_utf8_lossy(&raw[..end]).to_ascii_lowercase();
    let status = head
        .split_whitespace()
        .find_map(|word| word.parse::<u16>().ok())
        .unwrap_or_default();
    (status, raw[end + 4..].to_vec())
}

#[test]
fn serve_answers_the_drawn_share_card_of_a_page_as_png_bytes() {
    let site = demo_copy();
    let server = start(site.path());
    let home = get(&server, "/blog/").body;
    let (_, rest) = home
        .split_once(r#"<meta property="og:image" content=""#)
        .unwrap();
    let (url, _) = rest.split_once('"').unwrap();
    let href = url.replace("https://www.example.com", "");
    assert!(href.starts_with("/blog/share/"), "{href}");

    let (status, png) = get_bytes(&server, &href);
    assert_eq!(status, 200, "{href}");
    assert_eq!(&png[..4], b"\x89PNG", "the PNG signature");
    assert!(png.len() > 1000, "a real card, not a stub");
    assert_eq!(
        get_bytes(&server, "/blog/share/0000000000000000.png").0,
        404
    );
    assert_eq!(get_bytes(&server, "/blog/share/not-a-key.png").0, 404);
}

#[test]
fn serve_sends_the_stylesheet_as_an_early_hint_link() {
    let site = demo_copy();
    let server = start(site.path());
    let home = get(&server, "/blog/");
    assert_eq!(home.status, 200);
    // The head names the stylesheet; the same answer carries a Link header
    // that says to preload it, which an HTTP stack can turn into a 103.
    let (_, rest) = home
        .body
        .split_once(r#"<link rel="stylesheet" href=""#)
        .unwrap();
    let (href, _) = rest.split_once('"').unwrap();
    let head = raw_head(&server, "/blog/");
    let link = head
        .lines()
        .find_map(|line| {
            line.split_once(": ")
                .filter(|(name, _)| name.eq_ignore_ascii_case("link"))
        })
        .map(|(_, value)| value.trim().to_owned());
    let want = format!("<{href}>; rel=preload; as=style");
    assert_eq!(link.as_deref(), Some(want.as_str()), "the head: {head}");
}

/// The raw answer head for `path`, for headers the helpers do not return.
#[cfg(test)]
fn raw_head(server: &Server, path: &str) -> String {
    let host = &server.host;
    let mut stream = TcpStream::connect(host).unwrap();
    stream
        .write_all(
            format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n").as_bytes(),
        )
        .unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    raw.split_once("\r\n\r\n")
        .map_or(String::new(), |(head, _)| head.to_owned())
}
