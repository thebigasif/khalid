//! `khalid serve` with the privacy choices page and the consent banner
//! (privacy plan U4, R2, R6, R7, R8, R9, AE4): the footer of every page
//! links the choices page, a site with nothing to ask shows no banner, the
//! banner comes right after the skip link, and the choices page saves
//! with scripts off.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use serde_json::Value;
use tempfile::TempDir;

/// The site's own code, which waits for consent.
const CODE: &str = "\n[code]\nbody = \"<script>chat()</script>\"\n";

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

/// One answer: its status, its head with names lowercased, and its body.
struct Answer {
    status: u16,
    head: String,
    body: String,
}

impl Answer {
    /// The value of the header `name`, or empty.
    #[cfg(test)]
    fn header(&self, name: &str) -> &str {
        let prefix = format!("{name}: ");
        let found = self
            .head
            .lines()
            .find_map(|line| line.strip_prefix(&prefix));
        found.unwrap_or_default()
    }
}

/// A copy of the demo site with `extra` after its settings.
#[cfg(test)]
fn site_with(extra: &str) -> TempDir {
    let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    demo.push("../../fixtures/demo-site");
    let site = tempfile::tempdir().unwrap();
    copy_dir(&demo, site.path());
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    fs::write(&settings, format!("{text}{extra}")).unwrap();
    site
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

/// Starts `khalid serve` on `site` on a free port.
#[cfg(test)]
fn serve(site: &Path) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "serve", "--listen"])
        .args(["127.0.0.1:0", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    let stdout = child.stdout.take().unwrap();
    BufReader::new(stdout).read_line(&mut line).unwrap();
    let json: Value = serde_json::from_str(&line).unwrap_or_else(|_| panic!("{line}"));
    let address = json["address"].as_str().unwrap();
    let host = address.strip_prefix("http://").unwrap();
    let host = host.trim_end_matches("/blog/").to_owned();
    Server { child, host }
}

/// Sends `method` to `path` with `headers` and a form `body`, as a browser
/// with scripts off does, and keeps the answer's head.
#[cfg(test)]
fn send(server: &Server, (method, path): (&str, &str), headers: &[&str], body: &str) -> Answer {
    let host = &server.host;
    let mut stream = TcpStream::connect(host).unwrap();
    let mut request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len()
    );
    if !body.is_empty() {
        request.push_str("Content-Type: application/x-www-form-urlencoded\r\n");
    }
    for header in headers {
        request = format!("{request}{header}\r\n");
    }
    let sent = format!("{request}\r\n{body}");
    stream.write_all(sent.as_bytes()).unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    Answer {
        status: head.split(' ').nth(1).unwrap().parse().unwrap(),
        head: head.to_ascii_lowercase(),
        body: body.to_owned(),
    }
}

#[cfg(test)]
fn get(server: &Server, path: &str, headers: &[&str]) -> Answer {
    send(server, ("GET", path), headers, "")
}

#[test]
fn a_site_with_nothing_to_ask_shows_no_banner_and_still_links_its_choices() {
    let site = site_with("");
    let server = serve(site.path());
    for (page, choices) in [
        ("/blog/", "/blog/privacy-choices/"),
        ("/blog/what-a-sitemap-does/", "/blog/privacy-choices/"),
        ("/blog/de/", "/blog/de/privacy-choices/"),
    ] {
        let answer = get(&server, page, &[]);
        assert_eq!(answer.status, 200, "{page}");
        assert!(!answer.body.contains("class=\"consent\""), "{page}");
        let footer = answer.body.split_once("<footer").unwrap().1;
        assert!(
            footer.contains(&format!("<a href=\"{choices}\">")),
            "{page}"
        );
        let own = get(&server, choices, &[]);
        assert_eq!(own.status, 200, "{choices}");
        assert_eq!(own.body.matches("type=\"checkbox\"").count(), 4);
        assert!(!own.body.contains("checked"), "every switch starts off");
        assert!(own.body.contains("<meta name=\"robots\" content=\"noindex"));
    }
}

#[test]
fn the_banner_follows_the_skip_link_before_the_rest_of_the_page() {
    let site = site_with(CODE);
    let server = serve(site.path());
    let page = get(&server, "/blog/what-a-sitemap-does/", &[]).body;
    let skip = page.find("class=\"skip-link\"").unwrap();
    let banner = page.find("<section class=\"consent\"").unwrap();
    let header = page.find("<header class=\"site-header\"").unwrap();
    assert!(skip < banner && banner < header, "{page}");
    assert!(page.contains("value=\"accept-all\">Accept all</button>"));
    let choices = get(&server, "/blog/privacy-choices/", &[]).body;
    assert!(
        !choices.contains("<section class=\"consent\""),
        "one question per screen"
    );
}

#[test]
fn the_choices_page_saves_with_scripts_off_and_the_next_page_says_so() {
    let site = site_with(CODE);
    let server = serve(site.path());
    let back = "back=%2Fblog%2Fprivacy-choices%2F";
    let answers = [
        (format!("choice=save&analytics=on&{back}"), "no"),
        (
            format!("choice=save&preferences=on&analytics=on&marketing=on&{back}"),
            "yes",
        ),
        (format!("choice=accept-all&{back}"), "yes"),
        (format!("choice=reject-all&{back}"), "no"),
    ];
    for (form, kept) in answers {
        let answer = send(&server, ("POST", "/blog/_consent/"), &[], &form);
        assert_eq!(answer.status, 303, "{form}");
        assert_eq!(answer.header("location"), "/blog/privacy-choices/");
        let cookie = answer.header("set-cookie");
        assert!(
            cookie.starts_with(&format!("khalid-consent={kept};")),
            "{form}: {cookie}"
        );
        let next = get(
            &server,
            "/blog/privacy-choices/",
            &[&format!("Cookie: khalid-consent={kept}")],
        );
        assert!(
            next.body.contains("Your privacy choices are saved."),
            "{form}"
        );
        assert!(!next.body.contains("<!--khalid-chosen-->"), "{form}");
        assert_eq!(next.body.contains("chat()"), kept == "yes", "{form}");
    }
}
