//! A reader's comment on a running `khalid serve` emails the site's owner,
//! so it never waits unseen for approval, and `notify = false` in
//! `[comments]` stops the email. The reader's own address is never in it.

#[expect(
    dead_code,
    reason = "these tests start their own server, with mail set up"
)]
mod common;

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use khalid_mail::StubSmtp;
use serde_json::Value;

/// A running `khalid serve`, stopped when dropped.
struct Server(Child, String);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Starts `khalid serve` on `site` and reads the address it prints.
#[cfg(test)]
fn start(site: &Path) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
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

/// Sends `head` and `body` to `server` and answers the whole response.
#[cfg(test)]
fn send(server: &Server, head: &str, body: &str) -> String {
    let mut stream = TcpStream::connect(&server.1).unwrap();
    let host = &server.1;
    let head = format!(
        "{head}\r\nHost: {host}\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(head.as_bytes()).unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    raw
}

/// Posts the comment form under the sitemap post once the post shows it,
/// as a browser does with scripts off, and answers the status.
#[cfg(test)]
async fn comment(server: &Server) -> u16 {
    for _ in 0..200 {
        let page = send(server, "GET /blog/what-a-sitemap-does/ HTTP/1.1", "");
        if page.contains("<form class=\"comment-form\"") {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let body = "page=%2Fblog%2Fwhat-a-sitemap-does%2F&name=Lena&email=lena%40readers.example&comment=Very+clear%2C+thanks.&website=";
    let head = "POST /blog/comments/ HTTP/1.1\r\nContent-Type: application/x-www-form-urlencoded";
    let raw = send(server, head, body);
    raw.split(' ').nth(1).unwrap().parse().unwrap()
}

/// A copy of the demo site that sends mail through `stub`, with `extra`
/// added to `site.toml` and an owner whose email is `owner@example.com`.
#[cfg(test)]
fn site_with_mail(stub: &StubSmtp, extra: &str) -> tempfile::TempDir {
    let site = common::demo_copy();
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    let mail = format!(
        "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\nhost = \"127.0.0.1\"\n\
         port = {}\nsecurity = \"none\"\nprivate_host = true\n{extra}",
        stub.port()
    );
    fs::write(&settings, format!("{text}{mail}")).unwrap();
    let made = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "staff", "owner"])
        .args(["--email", "owner@example.com", "--name", "Asif"])
        .output()
        .unwrap();
    assert_eq!(made.status.code(), Some(0), "{made:?}");
    site
}

/// The mails `stub` holds once it holds `count`, waiting up to a minute.
#[cfg(test)]
async fn mails(stub: &StubSmtp, count: usize) -> Vec<String> {
    for _ in 0..300 {
        if stub.received().len() >= count {
            break;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    stub.received()
}

#[tokio::test(flavor = "multi_thread")]
async fn a_readers_comment_emails_the_owner_without_the_readers_address() {
    let stub = StubSmtp::start().unwrap();
    let site = site_with_mail(&stub, "");
    let server = start(site.path());
    assert_eq!(comment(&server).await, 303);
    let received = mails(&stub, 1).await;
    assert_eq!(received.len(), 1, "{received:?}");
    let mail = received[0].replace("=\r\n", "").replace("=\n", "");
    assert!(mail.contains("To: owner@example.com"), "{mail}");
    assert!(
        mail.contains("Subject: New comment from Lena, waiting for your approval"),
        "{mail}"
    );
    assert!(mail.contains("Very clear, thanks."), "{mail}");
    assert!(
        mail.contains("https://www.example.com/blog/what-a-sitemap-does/"),
        "{mail}"
    );
    assert!(mail.contains("/blog/admin/comments"), "{mail}");
    assert!(!mail.contains("readers.example"), "{mail}");
}

#[tokio::test(flavor = "multi_thread")]
async fn notify_false_sends_no_email() {
    let stub = StubSmtp::start().unwrap();
    let site = site_with_mail(&stub, "\n[comments]\nnotify = false\n");
    let server = start(site.path());
    assert_eq!(comment(&server).await, 303);
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert_eq!(stub.received(), Vec::<String>::new());
}
