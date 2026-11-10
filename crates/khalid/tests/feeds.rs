//! The feeds the binary writes and serves pass the checks `khalid-seo`
//! runs on its own: the `rss` crate's validator, RFC 4287's required Atom
//! elements, and the JSON Feed 1.1 fields read strictly (KH-SEO-6).

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use rss::validation::Validate;
use serde::Deserialize;
use serde_json::Value;
use tempfile::TempDir;

/// The folder each language's feeds sit in, below the blog's root.
const LANGUAGES: [&str; 2] = ["", "de/"];

/// JSON Feed 1.1 as jsonfeed.org defines it, refusing unknown fields.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[expect(dead_code, reason = "the fields are read only to be checked")]
struct JsonFeed {
    version: String,
    title: String,
    home_page_url: Option<String>,
    feed_url: Option<String>,
    description: Option<String>,
    language: Option<String>,
    authors: Option<Vec<Author>>,
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[expect(dead_code, reason = "the fields are read only to be checked")]
struct Author {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[expect(dead_code, reason = "the fields are read only to be checked")]
struct Item {
    id: String,
    url: Option<String>,
    title: Option<String>,
    content_html: Option<String>,
    summary: Option<String>,
    image: Option<String>,
    date_published: Option<String>,
    date_modified: Option<String>,
    authors: Option<Vec<Author>>,
    tags: Option<Vec<String>>,
}

/// Reads `xml` as RSS 2.0 and validates it, returning how many items it
/// has.
#[cfg(test)]
fn rss_items(xml: &str) -> usize {
    let channel = rss::Channel::read_from(xml.as_bytes()).unwrap();
    channel.validate().unwrap();
    channel.items().len()
}

/// Reads `xml` as Atom and checks RFC 4287 section 4.1: a feed and each
/// entry have an id, a title and an alternate link, and every entry has
/// an author, its own or the feed's. Returns how many entries it has.
#[cfg(test)]
fn atom_entries(xml: &str) -> usize {
    let feed = atom_syndication::Feed::read_from(xml.as_bytes()).unwrap();
    assert!(!feed.id().is_empty() && !feed.title().is_empty(), "{xml}");
    for entry in feed.entries() {
        assert!(!entry.id().is_empty() && !entry.title().is_empty(), "{xml}");
        let authored = !entry.authors().is_empty() || !feed.authors().is_empty();
        assert!(authored, "{} has no author", entry.id());
        let linked = entry.links().iter().any(|link| link.rel() == "alternate");
        assert!(linked, "{} has no alternate link", entry.id());
    }
    feed.entries().len()
}

/// Reads `json` into the JSON Feed 1.1 types, returning how many items it
/// has.
#[cfg(test)]
fn json_items(json: &str) -> usize {
    let feed: JsonFeed = serde_json::from_str(json).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(feed.version, "https://jsonfeed.org/version/1.1");
    feed.items.len()
}

/// Checks the three feeds of one language and that each holds `posts`.
#[cfg(test)]
fn check(feeds: [String; 3], language: &str) {
    let [rss, atom, json] = feeds;
    let posts = rss_items(&rss);
    assert!(posts > 0, "{language}feed.xml is empty");
    assert_eq!(atom_entries(&atom), posts, "{language}atom.xml");
    assert_eq!(json_items(&json), posts, "{language}feed.json");
}

/// A copy of the demo site, so neither build nor serve writes into the
/// fixture.
#[cfg(test)]
fn demo_copy() -> TempDir {
    let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    demo.push("../../fixtures/demo-site");
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&demo, temp.path());
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

#[cfg(test)]
fn read(out: &Path, rel: &str) -> String {
    let mut file = out.to_path_buf();
    file.push(rel);
    fs::read_to_string(file).unwrap()
}

#[test]
fn the_built_feeds_of_each_language_pass_the_feed_checks() {
    let site = demo_copy();
    let out = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "build", "--out"])
        .arg(out.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    for language in LANGUAGES {
        let feeds = ["feed.xml", "atom.xml", "feed.json"]
            .map(|name| read(out.path(), &format!("{language}{name}")));
        check(feeds, language);
    }
}

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

/// Starts `khalid serve` on `site` on a free port and reads the address it
/// prints.
#[cfg(test)]
fn start(site: &Path) -> Server {
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
    let port = address.strip_prefix("http://127.0.0.1:").unwrap();
    let host = format!("127.0.0.1:{}", port.trim_end_matches("/blog/"));
    Server { child, host }
}

/// The head and body `path` answers with.
#[cfg(test)]
fn answer(server: &Server, path: &str) -> (String, String) {
    let host = &server.host;
    let mut stream = TcpStream::connect(host).unwrap();
    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    (head.to_owned(), body.to_owned())
}

/// The body `path` answers with, which must be a 200.
#[cfg(test)]
fn get(server: &Server, path: &str) -> String {
    let (head, body) = answer(server, path);
    assert!(head.starts_with("HTTP/1.1 200"), "{path}: {head}");
    body
}

#[test]
fn the_served_feeds_of_each_language_pass_the_feed_checks() {
    let site = demo_copy();
    let server = start(site.path());
    for language in LANGUAGES {
        let feeds = ["feed.xml", "atom.xml", "feed.json"]
            .map(|name| get(&server, &format!("/blog/{language}{name}")));
        check(feeds, language);
    }
}

/// The demo site with a second collection of two guides and a channel of
/// Lena's posts in `routes.toml`, and a category no post is filed under in
/// `categories.toml`.
#[cfg(test)]
fn site_with_lists() -> TempDir {
    let site = demo_copy();
    let mut routes = site.path().to_path_buf();
    routes.push("routes.toml");
    let lists = "[collections.guides]\npermalink = \"/guides/{slug}/\"\n\n\
                 [channels.from-lena]\npath = \"/from-lena/\"\nfilter = { author = \"lena-vogt\" }\n";
    fs::write(routes, lists).unwrap();
    let mut guides = site.path().to_path_buf();
    guides.push("content/en/guides");
    fs::create_dir_all(&guides).unwrap();
    for (slug, date) in [
        ("audit-a-page", "2026-10-02"),
        ("plan-a-month", "2026-10-09"),
    ] {
        let text = format!(
            "---\ntitle: How to {slug}\nslug: {slug}\ndate: {date}\nauthor: asif\n\
             description: A short guide for small teams.\n---\n\n\
             Start with [internal links](/blog/why-internal-links-matter/).\n"
        );
        let mut file = guides.clone();
        file.push(format!("{slug}.md"));
        fs::write(file, text).unwrap();
    }
    let mut categories = read(site.path(), "categories.toml");
    categories.push_str("\n[[categories]]\nslug = \"link-building\"\nen = \"Link building\"\n");
    let mut file = site.path().to_path_buf();
    file.push("categories.toml");
    fs::write(file, categories).unwrap();
    site
}

/// Each collection, category and channel folder below the blog's root,
/// and the slugs of the posts its feeds hold, newest first.
const LIST_FEEDS: [(&str, &[&str]); 7] = [
    ("guides/", &["plan-a-month", "audit-a-page"]),
    (
        "category/on-page-seo/",
        &["how-to-write-a-title-tag", "why-internal-links-matter"],
    ),
    (
        "category/technical-seo/",
        &["fixing-slow-pages", "what-a-sitemap-does"],
    ),
    ("category/content-strategy/", &["planning-a-topic-cluster"]),
    (
        "de/category/technical-seo/",
        &[
            "langsame-seiten-schneller-machen",
            "was-eine-sitemap-leistet",
        ],
    ),
    ("from-lena/", &["fixing-slow-pages", "what-a-sitemap-does"]),
    (
        "de/from-lena/",
        &[
            "langsame-seiten-schneller-machen",
            "was-eine-sitemap-leistet",
        ],
    ),
];

/// Category folders with no posts in their language, so no feeds.
const NO_FEEDS: [&str; 2] = ["category/link-building/", "de/category/link-building/"];

/// Checks the three feeds of the list at `folder` and that they hold
/// exactly the posts `slugs`, in order.
#[cfg(test)]
fn check_list(feeds: [String; 3], folder: &str, slugs: &[&str]) {
    let channel = rss::Channel::read_from(feeds[0].as_bytes()).unwrap();
    let held: Vec<&str> = (channel.items().iter())
        .filter_map(|item| item.link())
        .filter_map(|link| link.trim_end_matches('/').rsplit('/').next())
        .collect();
    assert_eq!(held, slugs, "{folder}feed.xml");
    check(feeds, folder);
}

#[test]
fn the_built_list_feeds_hold_only_their_own_posts() {
    let site = site_with_lists();
    let out = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "build", "--out"])
        .arg(out.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    for (folder, slugs) in LIST_FEEDS {
        let feeds = ["feed.xml", "atom.xml", "feed.json"]
            .map(|name| read(out.path(), &format!("{folder}{name}")));
        check_list(feeds, folder, slugs);
        let page = read(out.path(), &format!("{folder}index.html"));
        let href = format!("https://www.example.com/blog/{folder}");
        for (media, name) in [
            ("application/rss+xml", "feed.xml"),
            ("application/atom+xml", "atom.xml"),
            ("application/feed+json", "feed.json"),
        ] {
            let link = format!("<link rel=\"alternate\" type=\"{media}\" href=\"{href}{name}\">");
            assert!(page.contains(&link), "{folder}index.html lacks {link}");
        }
    }
    for folder in NO_FEEDS {
        let mut feed = out.path().to_path_buf();
        feed.push(format!("{folder}feed.xml"));
        assert!(!feed.exists(), "{}", feed.display());
    }
}

#[test]
fn the_served_list_feeds_hold_only_their_own_posts() {
    let site = site_with_lists();
    let server = start(site.path());
    for (folder, slugs) in LIST_FEEDS {
        let feeds = ["feed.xml", "atom.xml", "feed.json"]
            .map(|name| get(&server, &format!("/blog/{folder}{name}")));
        check_list(feeds, folder, slugs);
    }
    for folder in NO_FEEDS {
        let (head, _) = answer(&server, &format!("/blog/{folder}feed.xml"));
        assert!(head.starts_with("HTTP/1.1 404"), "{folder}: {head}");
    }
}
