//! `khalid serve` applies a changed settings or theme file without a
//! restart: `site.toml`, `routes.toml`, `categories.toml`,
//! `theme/tokens.toml` and `theme/site.css`. A settings file that no
//! longer reads keeps the last good pages, and a setting only a start
//! applies says so.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde_json::Value;
use tempfile::TempDir;

/// The pause between looks for a change.
const PAUSE: Duration = Duration::from_millis(100);

/// How many looks, 30 seconds in all, before a wait fails.
const TRIES: u32 = 300;

/// A running `khalid serve`, stopped when dropped, and every line it has
/// written to standard error.
struct Server {
    child: Child,
    host: String,
    said: Arc<Mutex<Vec<String>>>,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A copy of the demo site, so a test can change it.
#[cfg(test)]
fn demo_copy() -> TempDir {
    let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    demo.push("../../fixtures/demo-site/.");
    let temp = tempfile::tempdir().unwrap();
    let copied = Command::new("cp")
        .arg("-R")
        .args([&demo, temp.path()])
        .status();
    assert!(copied.unwrap().success());
    temp
}

/// Writes `text` to `rel` under `site`, making its folder.
#[cfg(test)]
fn put(site: &Path, rel: &str, text: &str) {
    let mut file = site.to_path_buf();
    file.push(rel);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, text).unwrap();
}

/// What `rel` under `site` holds.
#[cfg(test)]
fn read(site: &Path, rel: &str) -> String {
    let mut file = site.to_path_buf();
    file.push(rel);
    fs::read_to_string(file).unwrap()
}

/// Starts `khalid serve` on `site` on a free port, reads the address it
/// prints, and keeps what it writes to standard error.
#[cfg(test)]
fn start(site: &Path) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "serve", "--listen"])
        .args(["127.0.0.1:0", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut line = String::new();
    let stdout = child.stdout.take().unwrap();
    BufReader::new(stdout).read_line(&mut line).unwrap();
    let json: Value = serde_json::from_str(&line).unwrap_or_else(|_| panic!("{line}"));
    let address = json["address"].as_str().unwrap();
    let host = address.strip_prefix("http://").unwrap();
    let host = host.trim_end_matches("/blog/").to_owned();
    let said = Arc::new(Mutex::new(Vec::new()));
    let stderr = BufReader::new(child.stderr.take().unwrap());
    let keep = Arc::clone(&said);
    thread::spawn(move || {
        for line in stderr.lines().map_while(Result::ok) {
            keep.lock().unwrap().push(line);
        }
    });
    Server { child, host, said }
}

impl Server {
    /// The status and body of `path`.
    #[cfg(test)]
    fn get(&self, path: &str) -> (u16, String) {
        let host = &self.host;
        let mut stream = TcpStream::connect(host).unwrap();
        let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
        stream.write_all(request.as_bytes()).unwrap();
        let mut raw = String::new();
        stream.read_to_string(&mut raw).unwrap();
        let (head, body) = raw.split_once("\r\n\r\n").unwrap();
        (
            head.split(' ').nth(1).unwrap().parse().unwrap(),
            body.into(),
        )
    }

    /// Whether `path` answers 200 with `text` in its body within the wait.
    #[cfg(test)]
    fn shows(&self, path: &str, text: &str) -> bool {
        (0..TRIES).any(|_| {
            thread::sleep(PAUSE);
            let (status, body) = self.get(path);
            status == 200 && body.contains(text)
        })
    }

    /// The stylesheet the home page links.
    #[cfg(test)]
    fn stylesheet(&self) -> String {
        let (_, home) = self.get("/blog/");
        let (_, rest) = home.split_once(r#"<link rel="stylesheet" href=""#).unwrap();
        let (href, _) = rest.split_once('"').unwrap();
        self.get(href).1
    }

    /// The first line written to standard error that holds `text`, waiting.
    #[cfg(test)]
    fn said(&self, text: &str) -> Option<String> {
        (0..TRIES).find_map(|_| {
            thread::sleep(PAUSE);
            let said = self.said.lock().unwrap();
            said.iter().find(|line| line.contains(text)).cloned()
        })
    }
}

#[test]
fn a_changed_site_toml_shows_in_llms_txt_without_a_restart() {
    let site = demo_copy();
    let server = start(site.path());
    let intro = "Two people write Field Notes, in Berlin.";
    assert!(!server.get("/blog/llms.txt").1.contains(intro));
    let settings = read(site.path(), "site.toml");
    let added = format!("{settings}\n[seo.llms_txt]\nintro = \"{intro}\"\n");
    put(site.path(), "site.toml", &added);
    assert!(server.shows("/blog/llms.txt", intro));
}

#[test]
fn a_changed_routes_toml_moves_the_list_pages_without_a_restart() {
    let site = demo_copy();
    let server = start(site.path());
    assert_eq!(server.get("/blog/topic/technical-seo/").0, 404);
    put(
        site.path(),
        "routes.toml",
        "[taxonomies]\ncategory = \"topic\"\n",
    );
    assert!(server.shows("/blog/topic/technical-seo/", "<h1>Technical SEO</h1>"));
}

#[test]
fn a_changed_categories_toml_renames_the_category_page_without_a_restart() {
    let site = demo_copy();
    let server = start(site.path());
    let categories = read(site.path(), "categories.toml");
    let renamed = categories.replace("en = \"Technical SEO\"", "en = \"Crawling and SEO\"");
    put(site.path(), "categories.toml", &renamed);
    let page = "/blog/category/technical-seo/";
    assert!(server.shows(page, "<h1>Crawling and SEO</h1>"));
}

#[test]
fn changed_tokens_and_site_css_reach_the_stylesheet_without_a_restart() {
    let site = demo_copy();
    let server = start(site.path());
    put(
        site.path(),
        "theme/tokens.toml",
        "[colors]\nlink = \"#b3261e\"\n",
    );
    let linked = (0..TRIES).any(|_| {
        thread::sleep(PAUSE);
        server.stylesheet().contains("--color-link: #b3261e;")
    });
    assert!(linked, "{}", server.stylesheet());
    put(
        site.path(),
        "theme/site.css",
        ".reload-check { margin: 0; }\n",
    );
    let ruled = (0..TRIES).any(|_| {
        thread::sleep(PAUSE);
        server.stylesheet().contains(".reload-check")
    });
    assert!(ruled, "{}", server.stylesheet());
}

#[test]
fn a_broken_site_toml_keeps_the_last_good_pages_and_says_why() {
    let site = demo_copy();
    let server = start(site.path());
    assert!(server.shows("/blog/category/technical-seo/", "<h1>"));
    let settings = read(site.path(), "site.toml");
    put(site.path(), "site.toml", &format!("{settings}\nname = [\n"));
    let line = server.said("site.toml").expect("a line naming site.toml");
    assert!(line.contains("line"), "{line}");
    assert!(line.contains("last good"), "{line}");
    let (status, home) = server.get("/blog/");
    assert_eq!(status, 200);
    assert!(home.contains("Field Notes on SEO"), "{home}");

    let renamed = settings.replace("Field Notes on SEO", "Field Notes, Reloaded");
    put(site.path(), "site.toml", &renamed);
    assert!(server.shows("/blog/", "Field Notes, Reloaded"));
}

#[test]
fn a_broken_categories_toml_keeps_the_last_good_pages_and_says_why() {
    let site = demo_copy();
    let server = start(site.path());
    // A start render that fails stops the server, so the break comes after.
    let page = "/blog/category/technical-seo/";
    assert!(server.shows(page, "<h1>Technical SEO</h1>"));
    put(site.path(), "categories.toml", "[[categories]\n");
    let line = server.said("categories.toml").expect("a line naming it");
    assert!(line.contains("last good"), "{line}");
    assert!(server.shows(page, "<h1>Technical SEO</h1>"));
}

#[test]
fn a_changed_mount_keeps_its_start_value_and_asks_for_a_restart() {
    let site = demo_copy();
    let server = start(site.path());
    let settings = read(site.path(), "site.toml");
    let moved = settings
        .replace("mount = \"/blog/\"", "mount = \"/notes/\"")
        .replace("Field Notes on SEO", "Field Notes, Reloaded");
    put(site.path(), "site.toml", &moved);
    let line = server.said("mount").expect("a line naming mount");
    assert!(line.contains("Restart khalid serve"), "{line}");
    assert!(server.shows("/blog/", "Field Notes, Reloaded"));
    assert_eq!(server.get("/notes/").0, 404);
}
