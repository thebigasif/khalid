//! The Content API's answers on a running `khalid serve`: the parameters
//! it refuses, `ETag` and 304, and the tags and plain pages it lists.

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

use serde_json::Value;

use super::{Server, demo_copy, new_key, request, start};

#[test]
fn the_content_api_refuses_include_and_fields_it_ignores() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--kind", "content", "--name", "Web page"]);
    let server = start(site.path());
    for parameter in [
        "include=authors",
        "fields=title",
        "include=authors&fields=title",
    ] {
        let path = format!("/api/content/v1/posts/?key={key}&{parameter}");
        let (status, body) = request(&server, "GET", &path, &[], None);
        assert_eq!(status, 400, "{parameter}: {body}");
        let problem: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(
            problem["title"], "The list could not be read",
            "{parameter}"
        );
    }
}

#[test]
fn the_content_api_answers_etag_and_304_through_serve() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--kind", "content", "--name", "Web page"]);
    let server = start(site.path());
    let path = format!("/api/content/v1/posts/slug/fixing-slow-pages/?key={key}");
    let head = raw_head(&server, &path);
    let status_line = head.lines().next().unwrap_or_default().to_owned();
    let etag = head
        .lines()
        .find_map(|line| {
            line.split_once(": ")
                .filter(|(name, _)| name.eq_ignore_ascii_case("etag"))
        })
        .map(|(_, value)| value.trim().to_owned())
        .expect("a strong ETag on the post");
    assert!(status_line.contains("200"), "{status_line}");
    assert!(
        etag.starts_with('"') && etag.ends_with('"') && etag.len() > 10,
        "{etag}"
    );

    let (status, body) = request(&server, "GET", &path, &[("if-none-match", &etag)], None);
    assert_eq!(status, 304, "the tag matches, so nothing changed");
    assert!(body.trim().is_empty(), "a 304 carries no body: {body}");
    let (status, _) = request(
        &server,
        "GET",
        &path,
        &[("if-none-match", "\"stale\"")],
        None,
    );
    assert_eq!(status, 200, "a stale tag gets the full answer again");
}

/// The raw answer head for `path`, for headers [`exchange`] does not return.
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

/// `resource` of the Content API under the demo site's mount, read with
/// the content `key`.
#[cfg(test)]
fn content(server: &Server, key: &str, resource: &str) -> Value {
    let joint = if resource.contains('?') { '&' } else { '?' };
    let path = format!("/blog/api/content/v1/{resource}{joint}key={key}");
    let (status, body) = request(server, "GET", &path, &[], None);
    assert_eq!(status, 200, "{resource}: {body}");
    serde_json::from_str(&body).unwrap()
}

/// The `field` of every entry under `list` in `json`.
#[cfg(test)]
fn each(json: &Value, list: &str, field: &str) -> Vec<String> {
    let entries = json[list].as_array().unwrap();
    entries
        .iter()
        .map(|entry| entry[field].as_str().unwrap_or_default().to_owned())
        .collect()
}

/// Writes `tags` into the front matter of the post at `file` in `site`.
#[cfg(test)]
fn tag(site: &Path, file: &str, tags: &str) {
    let mut path = site.to_path_buf();
    path.push(file);
    let text = fs::read_to_string(&path).unwrap();
    let tagged = text.replacen("---\n", &format!("---\ntags: [{tags}]\n"), 1);
    fs::write(&path, tagged).unwrap();
}

#[test]
fn the_content_api_lists_the_tags_the_posts_name_each_in_its_language() {
    let site = demo_copy();
    tag(
        site.path(),
        "content/en/posts/2026-09-28-what-a-sitemap-does.md",
        "crawl budget, Sitemaps",
    );
    tag(
        site.path(),
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "crawl budget",
    );
    tag(
        site.path(),
        "content/de/posts/2026-09-28-was-eine-sitemap-leistet.md",
        "crawl budget",
    );
    let key = new_key(site.path(), &["--kind", "content", "--name", "Web page"]);
    let server = start(site.path());

    let tags = content(&server, &key, "tags/?limit=all");
    assert_eq!(
        each(&tags, "tags", "id"),
        ["de.crawl-budget", "en.crawl-budget", "en.sitemaps"],
        "{tags}"
    );
    assert_eq!(
        each(&tags, "tags", "name"),
        ["crawl budget", "crawl budget", "Sitemaps"]
    );
    let english = content(&server, &key, "tags/?filter=language:en");
    assert_eq!(english["meta"]["pagination"]["total"], 2, "{english}");

    let post = content(&server, &key, "posts/slug/what-a-sitemap-does/");
    assert_eq!(
        post["posts"][0]["tags"],
        serde_json::json!(["crawl-budget", "sitemaps"]),
        "{post}"
    );
    let untagged = content(&server, &key, "posts/slug/fixing-slow-pages/");
    assert_eq!(untagged["posts"][0]["tags"], serde_json::json!([]));
    let tagged = content(&server, &key, "posts/?filter=tag:crawl-budget");
    assert_eq!(
        each(&tagged, "posts", "id"),
        [
            "en.what-a-sitemap-does",
            "de.was-eine-sitemap-leistet",
            "en.how-to-write-a-title-tag",
        ],
        "{tagged}"
    );
}

#[test]
fn the_content_api_lists_the_plain_pages_at_their_built_addresses() {
    let site = demo_copy();
    let key = new_key(site.path(), &["--kind", "content", "--name", "Web page"]);
    let server = start(site.path());

    let pages = content(&server, &key, "pages/");
    let mut ids = each(&pages, "pages", "id");
    ids.sort();
    assert_eq!(ids, ["de.ueber-uns", "en.about"], "{pages}");
    let about = content(&server, &key, "pages/en.about/?formats=html,markdown");
    let about = &about["pages"][0];
    assert_eq!(about["title"], "About Field Notes");
    assert_eq!(about["excerpt"], "Who writes Field Notes on SEO, and why.");
    assert_eq!(about["url"], "https://www.example.com/blog/about/");
    let html = about["html"].as_str().unwrap();
    assert!(html.contains("<p>Field Notes is"), "{about}");
    let markdown = about["markdown"].as_str().unwrap();
    assert!(markdown.trim_start().starts_with("Field Notes"), "{about}");
    let german = content(&server, &key, "pages/slug/ueber-uns/");
    assert_eq!(
        german["pages"][0]["url"],
        "https://www.example.com/blog/de/ueber-uns/"
    );
    let posts = content(&server, &key, "posts/?filter=slug:about");
    assert_eq!(posts["meta"]["pagination"]["total"], 0, "a page is no post");
}
