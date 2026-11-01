//! A blog under a folder of another site, as `khalid init --mount /blog/`
//! makes it, answers everything under that folder: the admin, both APIs and
//! their document, preview and sign-in links, the newsletter block and every
//! address its pages link to. So the proxy rule `khalid init --stack` prints,
//! which passes on only that folder, reaches all of it (KH-INS-3). The demo
//! site sits under `/blog/`.

mod common;

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::Command;

use common::{Server, demo_copy, new_key, start};
use khalid_mount::Stack;
use serde_json::{Value, json};

/// The scopes an editor's key has.
const EDITOR: [&str; 6] = [
    "--name",
    "Editor",
    "--scope",
    "posts:write",
    "--scope",
    "posts:publish",
];

/// The demo site's address with its mount, as the links Khalid gives out
/// start.
const BLOG: &str = "https://www.example.com/blog/";

/// Sends `method` to `path` with `headers` and `body`, and returns the
/// status, the head in lowercase and the body.
#[cfg(test)]
fn request(
    server: &Server,
    (method, path): (&str, &str),
    headers: &[(&str, &str)],
    body: &str,
) -> (u16, String, String) {
    let host = server.host();
    let mut stream = TcpStream::connect(host).unwrap();
    let mut head = format!(
        "{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len()
    );
    for (name, value) in headers {
        head = format!("{head}{name}: {value}\r\n");
    }
    stream
        .write_all(format!("{head}\r\n{body}").as_bytes())
        .unwrap();
    let mut answer = Vec::new();
    stream.read_to_end(&mut answer).unwrap();
    let raw = String::from_utf8_lossy(&answer);
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    let status = head.split(' ').nth(1).unwrap().parse().unwrap();
    (status, head.to_lowercase(), body.to_owned())
}

/// `GET path`, as [`request`] answers it.
#[cfg(test)]
fn get(server: &Server, path: &str) -> (u16, String, String) {
    request(server, ("GET", path), &[], "")
}

/// The `location` header of a lowercase `head`, if it has one.
#[cfg(test)]
fn location(head: &str) -> Option<&str> {
    head.lines()
        .find_map(|line| line.strip_prefix("location: "))
        .map(str::trim)
}

#[test]
fn the_admin_answers_under_the_mount_and_its_old_address_moves_there() {
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let (status, _, page) = get(&server, "/blog/admin/posts");
    // A copy built without `npm run build` says so; a built one names its mount.
    match status {
        200 => assert!(
            page.contains("<meta name=\"khalid-mount\" content=\"/blog/\">"),
            "{page}"
        ),
        503 => assert!(page.contains("The admin is not in this copy"), "{page}"),
        _ => panic!("the admin is not under the mount: {status} {page}"),
    }
    let (status, head, _) = get(&server, "/blog/admin");
    assert_eq!((status, location(&head)), (308, Some("/blog/admin/")));
    let (status, head, _) = get(&server, "/admin/posts");
    assert_eq!(
        (status, location(&head)),
        (308, Some("/blog/admin/posts")),
        "an old bookmark moves under the mount"
    );
}

#[test]
fn both_apis_and_their_document_answer_under_the_mount() {
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let (status, posts) = server.admin("GET", "/blog/api/admin/v1/posts/", None);
    assert_eq!(status, 200, "{posts}");
    let (status, body) = server.raw("GET", "/blog/api/content/v1/posts/", &[], None);
    assert_eq!(
        status, 401,
        "the Content API answers and asks for its key: {body}"
    );
    let (status, document) = server.admin("GET", "/blog/api/openapi.json", None);
    assert_eq!(status, 200, "{document}");
    assert_eq!(
        document["servers"][0]["url"], "https://www.example.com/blog",
        "a client calls the APIs under the mount"
    );

    let auth = format!("Khalid {}", new_key(site.path(), &EDITOR));
    let headers = [
        ("Authorization", auth.as_str()),
        ("Content-Type", "application/json"),
    ];
    let new =
        json!({"pages": [{"slug": "contact", "title": "Contact us", "markdown": "Write to us."}]});
    let made = request(
        &server,
        ("POST", "/blog/api/admin/v1/pages/"),
        &headers,
        &new.to_string(),
    );
    assert_eq!(made.0, 201, "{}", made.2);
    assert_eq!(
        location(&made.1),
        Some("/blog/api/admin/v1/pages/en.contact/")
    );
}

#[test]
fn a_preview_link_names_the_mount_and_opens_there() {
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let new = json!({"posts": [{"title": "Choosing anchor text", "author": "asif",
        "markdown": "Anchor text is the visible words of a link."}]});
    let (status, made) = server.admin("POST", "/blog/api/admin/v1/posts/", Some(&new));
    assert_eq!(status, 201, "{made}");
    let route = "/blog/api/admin/v1/posts/en.choosing-anchor-text/preview-link/";
    let (status, links) = server.admin("POST", route, None);
    assert_eq!(status, 201, "{links}");
    let url = links["preview_links"][0]["url"].as_str().unwrap();
    let path = url.strip_prefix("https://www.example.com").unwrap();
    assert!(path.starts_with("/blog/preview/"), "{url}");
    let (status, _, page) = get(&server, path);
    assert_eq!(status, 200, "{page}");
    assert!(page.contains("Anchor text is the visible words"), "{page}");
}

#[test]
fn the_newsletter_block_posts_under_the_mount() {
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let block = "::newsletter{label=\"Your email\" button=\"Join\"}\n";
    let new = json!({"pages": [{"slug": "letters", "title": "Letters", "markdown": block}]});
    let (status, made) = server.admin("POST", "/blog/api/admin/v1/pages/", Some(&new));
    assert_eq!(status, 201, "{made}");
    assert!(server.answers("/blog/letters/", 200));
    let (_, _, page) = get(&server, "/blog/letters/");
    assert!(page.contains("action=\"/blog/subscribe\""), "{page}");
    assert!(!page.contains("action=\"/subscribe\""), "{page}");
    let form = (
        b"email=sam%40example.com".as_slice(),
        Some("application/x-www-form-urlencoded"),
    );
    let (status, page) = server.send("POST", "/blog/subscribe", &[], form);
    assert_ne!(status, 404, "the sign-up answers under the mount: {page}");
}

#[test]
fn the_owner_sign_in_link_opens_the_admin_under_the_mount() {
    let site = demo_copy();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args([
            "--site",
            site.path().to_str().unwrap(),
            "--json",
            "staff",
            "owner",
        ])
        .args(["--email", "asif@example.com", "--name", "Asif"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let made: Value = serde_json::from_slice(&output.stdout).unwrap();
    let link = made["link"].as_str().unwrap();
    assert!(
        link.starts_with(&format!("{BLOG}admin/sign-in/link#token=")),
        "{link}"
    );
    let local = made["local_link"].as_str().unwrap();
    let here = "http://localhost:8080/blog/admin/sign-in/link#token=";
    assert!(local.starts_with(here), "{local}");
}

/// Every address in the `href`, `src` and `action` attributes of `html`
/// that starts at the host's root, without its query or fragment.
#[cfg(test)]
fn addresses(html: &str) -> Vec<String> {
    let mut found = Vec::new();
    for attribute in ["href=\"", "src=\"", "action=\""] {
        for rest in html.split(attribute).skip(1) {
            let value = rest.split('"').next().unwrap();
            let path = value.split(['?', '#']).next().unwrap();
            if path.starts_with('/') && !path.starts_with("//") {
                found.push(path.to_owned());
            }
        }
    }
    found
}

/// The status and any `Location` of each address the demo's pages lead
/// to, from its home page on, and of each address Khalid serves things
/// at, inside and outside the mount.
#[cfg(test)]
fn answers(server: &Server) -> BTreeMap<String, (u16, Option<String>)> {
    let families = [
        "/",
        "/admin",
        "/admin/",
        "/api/openapi.json",
        "/api/content/v1/posts/",
        "/api/admin/v1/posts/",
        "/api/hooks/git",
        "/subscribe",
        "/unsubscribe",
        "/comments/",
        "/forms/contact",
        "/preview/en.a/",
        "/search/",
        "/_consent/",
        "/robots.txt",
        "/sitemap.xml",
        "/feed.xml",
    ];
    let mut queue: Vec<String> = families.iter().map(|path| (*path).to_owned()).collect();
    queue.extend(families.iter().map(|path| format!("/blog{path}")));
    let mut answered = BTreeMap::new();
    while let Some(path) = queue.pop() {
        if answered.contains_key(&path) || answered.len() > 400 {
            continue;
        }
        let (status, head, body) = get(server, &path);
        if status == 200 && head.contains("content-type: text/html") {
            queue.extend(addresses(&body));
        }
        answered.insert(path, (status, location(&head).map(str::to_owned)));
    }
    answered
}

/// How each kind of printed rule passes on everything under `/blog/`.
const PASSES_ON_THE_MOUNT: [&str; 8] = [
    "location /blog/ {",
    "handle /blog/* {",
    "ProxyPass /blog/ ",
    "PathPrefix(`/blog/`)",
    "url.pathname.startsWith(\"/blog/\")",
    "\"source\": \"/blog/:path*\"",
    "source: \"/blog/:path*\"",
    "/blog/*  ",
];

/// The proxy rule `khalid init` prints for `stack` and the mount `/blog/`.
#[cfg(test)]
fn printed_rule(stack: Stack, folder: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args([
            "init",
            folder.to_str().unwrap(),
            "--address",
            "https://www.example.com",
        ])
        .args(["--mount", "/blog/", "--stack", stack.name(), "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{stack}");
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    json["proxy"]["rule"].as_str().unwrap().to_owned()
}

#[test]
fn every_printed_proxy_rule_reaches_everything_the_server_answers() {
    for stack in Stack::ALL {
        let folder = tempfile::tempdir().unwrap();
        let rule = printed_rule(stack, folder.path());
        assert!(
            PASSES_ON_THE_MOUNT.iter().any(|form| rule.contains(form)),
            "{stack} passes on all of /blog/:\n{rule}"
        );
    }
    let site = demo_copy();
    let server = start(site.path(), &new_key(site.path(), &EDITOR));
    let answered = answers(&server);
    let mut outside = Vec::new();
    for (path, (status, moved)) in &answered {
        let inside = path.starts_with("/blog/") || path == "/blog";
        let moves_inside = moved
            .as_deref()
            .is_some_and(|to| to.starts_with("/blog/") || to.starts_with(BLOG));
        // An address outside the mount may answer as its twin inside does,
        // for a program that reaches Khalid without the proxy.
        if *status == 404 || inside || moves_inside {
            continue;
        }
        let twin = get(&server, &format!("/blog{path}")).0;
        if twin != *status {
            outside.push(format!("{path} answers {status}, /blog{path} {twin}"));
        }
    }
    assert!(outside.is_empty(), "outside the mount: {outside:#?}");
    assert!(
        answered.len() > 40,
        "the crawl reached the demo's pages: {answered:#?}"
    );
}
