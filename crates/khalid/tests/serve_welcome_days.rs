//! A later welcome step on a running `khalid serve` (G39, backlog row 36):
//! a reader who joins gets `welcome-day-3.md` from the hourly round three
//! days on, not before, and only once, on a clock the hidden
//! `--test-clock-ahead` moves, across restarts.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use khalid_mail::StubSmtp;
use serde_json::Value;
use tempfile::TempDir;

/// A day on the clock `--test-clock-ahead` moves, in seconds.
const DAY: u32 = 24 * 60 * 60;

/// A running `khalid serve`, stopped when dropped.
struct Server {
    child: Child,
    host: String,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A copy of the demo site sending mail through `stub`, with a postal
/// address so welcomes go out, and a welcome sequence for `main`: one
/// email at once and one on day 3.
#[cfg(test)]
fn site_sending_to(stub: &StubSmtp) -> TempDir {
    let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    demo.push("../../fixtures/demo-site");
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&demo, temp.path());
    let mut settings = temp.path().to_path_buf();
    settings.push("site.toml");
    let port = stub.port();
    let mail = format!(
        "\n[mail]\nfrom = \"news@example.com\"\nname = \"Field Notes\"\n\
         postal_address = \"1 Park Row, Leeds LS1 5AB, UK\"\n\n[mail.smtp]\n\
         host = \"127.0.0.1\"\nport = {port}\nsecurity = \"none\"\nprivate_host = true\n"
    );
    let text = format!("{}{mail}", fs::read_to_string(&settings).unwrap());
    fs::write(&settings, text).unwrap();
    let mut folder = temp.path().to_path_buf();
    folder.extend(["content", "newsletters", "main"]);
    fs::create_dir_all(&folder).unwrap();
    let steps = [
        (
            "welcome.md",
            "# Welcome to Field Notes\n\nOne post a week.\n",
        ),
        (
            "welcome-day-3.md",
            "# Three days in\n\nA tip for new readers.\n",
        ),
    ];
    for (name, text) in steps {
        let mut file = folder.clone();
        file.push(name);
        fs::write(&file, text).unwrap();
    }
    temp
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let mut target = into.to_path_buf();
        target.push(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

/// Starts `khalid serve` on `site` on a free port, reading a clock
/// `seconds` ahead and running the hourly round every second.
#[cfg(test)]
fn start_ahead(site: &Path, seconds: u32) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(["serve", "--listen", "127.0.0.1:0", "--json"])
        .args(["--test-clock-ahead", &seconds.to_string()])
        .args(["--alert-every", "1"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let json: Value = serde_json::from_str(&line).unwrap_or_else(|_| panic!("{line}"));
    let address = json["address"].as_str().unwrap().strip_prefix("http://");
    let host = address.unwrap().trim_end_matches("/blog/").to_owned();
    Server { child, host }
}

/// Posts the form `body` to `path` as a browser does, and returns the
/// status and the page.
#[cfg(test)]
fn post_form(server: &Server, path: &str, body: &str) -> (u16, String) {
    let host = &server.host;
    let mut stream = TcpStream::connect(host).unwrap();
    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\
         Content-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(request.as_bytes()).unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    let (head, page) = raw.split_once("\r\n\r\n").unwrap();
    (
        head.split(' ').nth(1).unwrap().parse().unwrap(),
        page.to_owned(),
    )
}

/// The token in the confirmation link of `message`, with any line the
/// mail encoding broke joined back up.
#[cfg(test)]
fn confirm_token(message: &str) -> String {
    let body = message.replace("=\n", "").replace("=3D", "=");
    let (_, rest) = body.split_once("subscribe/confirm?token=").unwrap();
    rest.split(char::is_whitespace).next().unwrap().to_owned()
}

/// The emails `stub` received once there are `count`, or after 100 looks
/// 200 ms apart, 20 seconds in all.
#[cfg(test)]
fn received_at_least(stub: &StubSmtp, count: usize) -> Vec<String> {
    for _ in 0..100 {
        if stub.received().len() >= count {
            break;
        }
        thread::sleep(Duration::from_millis(200));
    }
    stub.received()
}

#[test]
fn the_day_three_welcome_goes_out_on_day_three_and_once() {
    let stub = StubSmtp::start().unwrap();
    let site = site_sending_to(&stub);
    let server = start_ahead(site.path(), 0);
    let (status, page) = post_form(&server, "/subscribe", "email=grace%40example.com&homepage=");
    assert_eq!(status, 200, "{page}");
    let confirming = format!("token={}", confirm_token(&stub.received()[0]));
    let (status, page) = post_form(&server, "/blog/subscribe/confirm", &confirming);
    assert_eq!(status, 200, "{page}");
    let received = received_at_least(&stub, 2);
    assert_eq!(received.len(), 2, "confirmation and welcome: {received:?}");
    assert!(received[1].contains("Subject: Welcome to Field Notes"));
    drop(server);

    // Two days on, several rounds pass and the day-3 step waits.
    let server = start_ahead(site.path(), 2 * DAY);
    thread::sleep(Duration::from_secs(3));
    assert_eq!(stub.received().len(), 2, "sent before its day");
    drop(server);

    // Four days on, a round sends it, and later rounds do not again.
    let server = start_ahead(site.path(), 4 * DAY);
    let received = received_at_least(&stub, 3);
    assert_eq!(received.len(), 3, "the day-3 step never went out");
    let message = &received[2];
    assert!(message.contains("To: grace@example.com"), "{message}");
    assert!(message.contains("Subject: Three days in"), "{message}");
    assert!(message.contains("A tip for new readers."), "{message}");
    thread::sleep(Duration::from_secs(3));
    assert_eq!(stub.received().len(), 3, "the day-3 step went out twice");
    drop(server);
}
