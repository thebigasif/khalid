//! The media library through a running `khalid serve`: a file uploaded
//! through the Admin API is served and kept, and a video answers in parts.

use std::io::{Read, Write};
use std::net::TcpStream;

use serde_json::Value;

use super::{Server, demo_copy, new_key, start};

/// Sends `body` as it is, and returns the status, the head in lowercase and
/// the body's bytes.
#[cfg(test)]
fn raw(
    server: &Server,
    method: &str,
    path: &str,
    key: &str,
    body: &[u8],
) -> (u16, String, Vec<u8>) {
    let headers = format!("Authorization: Khalid {key}\r\n");
    raw_with(server, (method, path), &headers, body)
}

/// [`raw`] with `headers`, each line ending in `\r\n`, in place of the key.
#[cfg(test)]
fn raw_with(
    server: &Server,
    (method, path): (&str, &str),
    headers: &str,
    body: &[u8],
) -> (u16, String, Vec<u8>) {
    let host = &server.host;
    let mut stream = TcpStream::connect(host).unwrap();
    let head = format!(
        "{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\
         {headers}Content-Length: {}\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes()).unwrap();
    stream.write_all(body).unwrap();
    let mut answer = Vec::new();
    stream.read_to_end(&mut answer).unwrap();
    let end = answer.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
    let head = String::from_utf8_lossy(&answer[..end]).to_lowercase();
    let status = head.split(' ').nth(1).unwrap().parse().unwrap();
    (status, head, answer[end + 4..].to_vec())
}

/// The media library's done line (plan 5.15, KH-EDT-5): a file uploaded
/// through the Admin API with a key from `khalid keys create` is served
/// from the blog's own address, and is still in the library after a
/// restart.
#[test]
fn a_file_uploaded_through_the_admin_api_is_served_and_kept() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--name", "Editor", "--scope", "media:write"]);
    let pdf = b"%PDF-1.7\nTen questions to ask a CRM vendor\n%%EOF\n";
    let server = start(site.path());
    let upload = "/api/admin/v1/media/?name=CRM%20checklist.pdf&folder=guides";
    let (status, _, body) = raw(&server, "POST", upload, &key, pdf);
    assert_eq!(status, 201, "{}", String::from_utf8_lossy(&body));
    let made: Value = serde_json::from_slice(&body).unwrap();
    let id = made["media"][0]["id"].as_str().unwrap().to_owned();
    let path = format!("/blog/media/library/{id}/crm-checklist.pdf");
    let url = format!("https://www.example.com{path}");
    assert_eq!(made["media"][0]["url"], url.as_str());

    let (status, head, bytes) = raw(&server, "GET", &path, "", b"");
    assert_eq!((status, bytes.as_slice()), (200, pdf.as_slice()));
    assert!(head.contains("content-type: application/pdf"), "{head}");
    assert!(
        head.contains("immutable") && head.contains("nosniff"),
        "{head}"
    );
    let wrong = format!("/blog/media/library/{id}/another-name.pdf");
    assert_eq!(raw(&server, "GET", &wrong, "", b"").0, 404);

    drop(server);
    let server = start(site.path());
    let listed = "/api/admin/v1/media/?filter=folder:guides";
    let (status, _, body) = raw(&server, "GET", listed, &key, b"");
    let list: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!((status, &list["media"][0]["id"]), (200, &Value::from(id)));
}

/// A video from the library goes out as `video/mp4` and answers a
/// `Range` request with 206, which Safari needs before it plays it.
#[test]
fn a_library_video_is_sent_as_video_and_in_parts() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--name", "Editor", "--scope", "media:write"]);
    let mut mp4 = b"\0\0\0\x18ftypisom\0\0\x02\0isomiso2".to_vec();
    mp4.extend([7_u8; 40]);
    let server = start(site.path());
    let upload = "/api/admin/v1/media/?name=Site%20tour.mp4";
    let (status, _, body) = raw(&server, "POST", upload, &key, &mp4);
    assert_eq!(status, 201, "{}", String::from_utf8_lossy(&body));
    let made: Value = serde_json::from_slice(&body).unwrap();
    let id = made["media"][0]["id"].as_str().unwrap();
    let path = format!("/blog/media/library/{id}/site-tour.mp4");

    let (status, head, bytes) = raw_with(&server, ("GET", &path), "", b"");
    assert_eq!((status, bytes.as_slice()), (200, mp4.as_slice()));
    assert!(head.contains("content-type: video/mp4"), "{head}");
    assert!(head.contains("accept-ranges: bytes"), "{head}");

    let range = "Range: bytes=4-11\r\n";
    let (status, head, bytes) = raw_with(&server, ("GET", &path), range, b"");
    assert_eq!((status, bytes.as_slice()), (206, b"ftypisom".as_slice()));
    let whole = format!("content-range: bytes 4-11/{}", mp4.len());
    assert!(head.contains(&whole), "{head}");
    assert!(head.contains("content-type: video/mp4"), "{head}");
}
