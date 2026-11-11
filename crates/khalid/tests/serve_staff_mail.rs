//! Sign-in links mailed to staff through `khalid serve` (gap audit 4 row
//! 37, plan section 4.6): with `[mail]` set, asking for a link sends one
//! to an account's email, and the link signs in once. An email with no
//! account gets the same answer and no mail.

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
use serde_json::{Value, json};

const ASK: &str = "/api/admin/v1/session/link/";
const REDEEM: &str = "/api/admin/v1/session/link/redeem/";

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

/// POSTs `body` as JSON to `route`; answers the status and the body.
#[cfg(test)]
fn post(server: &Server, route: &str, body: &Value) -> (u16, String) {
    let mut stream = TcpStream::connect(&server.1).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let body = body.to_string();
    let head = format!(
        "POST {route} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\
         Content-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
        server.1,
        body.len()
    );
    stream.write_all(head.as_bytes()).unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    let status = head.split(' ').nth(1).unwrap().parse().unwrap();
    (status, body.to_owned())
}

/// A copy of the demo site that sends mail through `stub`, with an owner
/// whose email is `owner@example.com`.
#[cfg(test)]
fn site_with_mail(stub: &StubSmtp) -> tempfile::TempDir {
    let site = common::demo_copy();
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    let mail = format!(
        "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\nhost = \"127.0.0.1\"\n\
         port = {}\nsecurity = \"none\"\nprivate_host = true\n",
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

/// The token after `#token=` in `mail`, read as a mail program would: the
/// text is quoted-printable, so lines fold with `=` and `=` itself is
/// `=3D`.
#[cfg(test)]
fn token_in(mail: &str) -> String {
    let unfolded = mail.replace("=\r\n", "").replace("=\n", "");
    let (_, after) = unfolded.split_once("/sign-in/link#token=3D").unwrap();
    after
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn a_staff_member_gets_a_sign_in_link_by_mail_and_it_signs_in_once() {
    let stub = StubSmtp::start().unwrap();
    let site = site_with_mail(&stub);
    let server = start(site.path());

    let (status, body) = post(&server, ASK, &json!({ "email": "owner@example.com" }));
    assert_eq!(status, 202, "{body}");
    let received = mails(&stub, 1).await;
    assert_eq!(received.len(), 1, "{received:?}");
    let mail = &received[0];
    assert!(mail.contains("To: owner@example.com"), "{mail}");
    assert!(mail.contains("Subject: Your sign-in link for"), "{mail}");

    let redeem = json!({ "token": token_in(mail) });
    let (status, body) = post(&server, REDEEM, &redeem);
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("\"role\":\"owner\""), "{body}");
    let (status, _) = post(&server, REDEEM, &redeem);
    assert_eq!(status, 401, "the link works once");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_email_with_no_account_gets_the_same_answer_and_no_mail() {
    let stub = StubSmtp::start().unwrap();
    let site = site_with_mail(&stub);
    let server = start(site.path());

    let (status, body) = post(&server, ASK, &json!({ "email": "nobody@example.com" }));
    assert_eq!(status, 202, "{body}");
    // Ask for the owner too: once its mail arrives, the first would have.
    let (status, _) = post(&server, ASK, &json!({ "email": "owner@example.com" }));
    assert_eq!(status, 202);
    mails(&stub, 1).await;
    tokio::time::sleep(Duration::from_millis(500)).await;
    let received = stub.received();
    assert_eq!(received.len(), 1, "{received:?}");
    assert!(
        received[0].contains("To: owner@example.com"),
        "{received:?}"
    );
}
