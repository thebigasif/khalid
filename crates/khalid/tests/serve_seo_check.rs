//! The editor's SEO panel on a running `khalid serve` (G54): the check
//! route answers a post's score, search snippet and schema graph, and a
//! search title and hand-written JSON-LD saved through the Admin API reach
//! the post file and the next check. A schema template saved through the
//! Admin API reaches `site.toml`, and the check with no restart.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

/// The demo post every request here is about.
const ROUTE: &str = "/api/admin/v1/posts/en.why-internal-links-matter/";

/// Its file in the demo site.
const FILE: &str = "content/en/posts/2026-09-14-why-internal-links-matter.md";

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

/// A copy of the demo site.
#[cfg(test)]
fn demo_site() -> tempfile::TempDir {
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

/// A key that may write posts, change live ones and change the site's
/// settings, made by `khalid keys create`.
#[cfg(test)]
fn writer_key(site: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "keys", "create", "--json"])
        .args(["--name", "Writer", "--scope", "posts:write"])
        .args(["--scope", "posts:publish", "--scope", "settings:write"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    json["key"].as_str().unwrap().to_owned()
}

/// Starts `khalid serve` on a free port.
#[cfg(test)]
fn start(site: &Path) -> Server {
    start_with(site, &[])
}

/// Starts `khalid serve` on a free port with the variables `env` set.
#[cfg(test)]
fn start_with(site: &Path, env: &[(&str, &str)]) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(["serve", "--listen", "127.0.0.1:0", "--json"])
        .envs(env.iter().copied())
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

/// Sends `method` to `path` with `key` and `body`; returns the status and
/// the body as JSON. Gives up after 30 seconds.
#[cfg(test)]
fn send(server: &Server, key: &str, (method, path): (&str, &str), body: &str) -> (u16, Value) {
    let mut stream = TcpStream::connect(&server.host).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let head = format!(
        "{method} {path} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\
         Authorization: Khalid {key}\r\nContent-Type: application/json\r\n\
         Content-Length: {}\r\n\r\n",
        server.host,
        body.len()
    );
    stream
        .write_all(format!("{head}{body}").as_bytes())
        .unwrap();
    let mut raw = String::new();
    stream.read_to_string(&mut raw).unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    let status = head.split(' ').nth(1).unwrap().parse().unwrap();
    (status, serde_json::from_str(body).unwrap_or(Value::Null))
}

/// The demo post's check.
#[cfg(test)]
fn check(server: &Server, key: &str) -> Value {
    let route = format!("{ROUTE}check/");
    let (status, json) = send(server, key, ("GET", &route), "");
    assert_eq!(status, 200, "{json}");
    json["checks"][0].clone()
}

/// The `@type` of every node in `schema`'s graph.
#[cfg(test)]
fn types(schema: &Value) -> Vec<String> {
    let nodes = schema["@graph"].as_array().unwrap();
    nodes.iter().map(|node| node["@type"].to_string()).collect()
}

#[test]
fn the_check_follows_a_saved_search_title_and_custom_schema() {
    let site = demo_site();
    let key = writer_key(site.path());
    let server = start(site.path());

    let first = check(&server, &key);
    let score = first["lint"]["score"]["value"].as_u64().unwrap();
    assert!(score <= 100, "{first}");
    let scores = &first["scores"];
    assert!(scores["seo"].as_u64().unwrap() <= 100, "{scores}");
    assert!(scores["readability"].as_u64().unwrap() <= 100, "{scores}");
    assert!(first["words"].as_u64().unwrap() > 100, "{first}");
    let snippet = &first["snippet"];
    let title = snippet["title"].as_str().unwrap();
    assert!(title.starts_with("Why internal links matter"), "{snippet}");
    let url = snippet["url"].as_str().unwrap();
    assert!(
        url.ends_with("/blog/why-internal-links-matter/"),
        "{snippet}"
    );
    let description = snippet["description"].as_str().unwrap();
    assert!(description.starts_with("Internal links help"), "{snippet}");
    assert!(!types(&first["schema"]).contains(&"\"Event\"".to_owned()));

    let (status, post) = send(&server, &key, ("GET", ROUTE), "");
    assert_eq!(status, 200, "{post}");
    let schema = "{\n  \"@type\": \"Event\",\n  \"name\": \"Internal links clinic\",\n  \
                  \"startDate\": \"2026-11-12T18:00:00+00:00\"\n}";
    let settings = json!({"seo_title": "Internal links, plainly {sep} {site}", "schema": schema});
    let edit = json!({"posts": [{"settings": settings,
        "updated_at": post["posts"][0]["updated_at"]}]});
    let (status, saved) = send(&server, &key, ("PUT", ROUTE), &edit.to_string());
    assert_eq!(status, 200, "{saved}");

    let mut file = site.path().to_path_buf();
    file.push(FILE);
    let text = fs::read_to_string(file).unwrap();
    assert!(
        text.contains("\nseo_title: \"Internal links, plainly {sep} {site}\"\n"),
        "{text}"
    );
    let after = check(&server, &key);
    let title = after["snippet"]["title"].as_str().unwrap();
    assert!(title.starts_with("Internal links, plainly "), "{after}");
    assert!(title.ends_with("Field Notes on SEO"), "{after}");
    assert!(
        types(&after["schema"]).contains(&"\"Event\"".to_owned()),
        "{after}"
    );
}

#[test]
fn the_check_names_the_author_from_their_file_as_the_built_page_does() {
    let site = demo_site();
    let mut file = site.path().to_path_buf();
    file.push(FILE);
    let text = fs::read_to_string(&file).unwrap();
    let titled = text.replacen(
        "\nauthor: asif\n",
        "\nauthor: asif\nseo_title: \"Links by {author}\"\n",
        1,
    );
    assert_ne!(text, titled);
    fs::write(&file, titled).unwrap();
    let key = writer_key(site.path());
    let server = start(site.path());

    let check = check(&server, &key);
    assert_eq!(check["snippet"]["title"], "Links by Asif", "{check}");
    let nodes = check["schema"]["@graph"].as_array().unwrap();
    let person = nodes.iter().find(|node| node["@type"] == "Person");
    let person = person.unwrap_or_else(|| panic!("no Person: {check}"));
    assert_eq!(person["name"], "Asif", "{person}");
    assert_eq!(person["jobTitle"], "Founder of SerpDad", "{person}");
    assert_eq!(
        person["sameAs"],
        json!(["https://serpdad.com/"]),
        "{person}"
    );
}

#[test]
fn a_saved_schema_template_reaches_site_toml_and_the_check_with_no_restart() {
    let site = demo_site();
    let key = writer_key(site.path());
    // Mail through Resend reads only with the key the server started with.
    let mut file = site.path().to_path_buf();
    file.push("site.toml");
    let text = fs::read_to_string(&file).unwrap();
    let mail = "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.resend]\n";
    fs::write(&file, format!("{text}{mail}")).unwrap();
    let server = start_with(
        site.path(),
        &[("KHALID_RESEND_API_KEY", "made-for-this-test")],
    );
    let (status, settings) = send(&server, &key, ("GET", "/api/admin/v1/settings/"), "");
    assert_eq!(status, 200, "{settings}");
    assert_eq!(settings["settings"]["restart_needed"], false, "{settings}");

    let route = "/api/admin/v1/schema-templates/";
    let (status, read) = send(&server, &key, ("GET", route), "");
    assert_eq!(status, 200, "{read}");
    assert_eq!(read["templates"], json!([]), "{read}");
    let collections = read["collections"].as_array().unwrap();
    assert!(collections.contains(&json!("posts")), "{read}");

    let course = json!({"applies_to": "collection", "name": "posts", "type": "Course",
        "properties": {"provider": "{site}"}});
    let change = json!({"templates": [course], "updated_at": read["updated_at"]});
    let (status, saved) = send(&server, &key, ("PUT", route), &change.to_string());
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["restart_needed"], false, "{saved}");
    let text = fs::read_to_string(file).unwrap();
    assert!(
        text.contains("[[seo.schema]]\ncollection = \"posts\"\n"),
        "{text}"
    );

    // The check follows once the pages use the saved file, within seconds.
    let course = "\"Course\"".to_owned();
    let follows = (0..200).any(|_| {
        thread::sleep(Duration::from_millis(100));
        types(&check(&server, &key)["schema"]).contains(&course)
    });
    assert!(follows, "{}", check(&server, &key));
}

/// Writes the draft `en.anchor-words` into `site`: in the demo's
/// `on-page-seo` category, with no focus keyword, linking to one post.
#[cfg(test)]
fn write_draft(site: &Path) {
    let mut drafts = site.to_path_buf();
    drafts.push("drafts");
    drafts.push("en");
    fs::create_dir_all(&drafts).unwrap();
    drafts.push("anchor-words.md");
    let draft = "---\ntitle: Anchor words\nslug: anchor-words\ncategory: on-page-seo\n---\n\n\
                 Pick link words with care, as [sitemaps](/blog/what-a-sitemap-does/) show.\n";
    fs::write(&drafts, draft).unwrap();
}

#[test]
fn the_posts_list_counts_links_and_finds_orphans() {
    let site = demo_site();
    write_draft(site.path());
    let key = writer_key(site.path());
    let server = start(site.path());
    let route = "/api/admin/v1/posts/?filter=orphan:true%2Bhas_focus_keyword:false%2Bpillar:false";
    let (status, json) = send(&server, &key, ("GET", route), "");
    assert_eq!(status, 200, "{json}");
    let rows = json["posts"].as_array().unwrap();
    let draft = rows.iter().find(|row| row["id"] == "en.anchor-words");
    let draft = draft.unwrap_or_else(|| panic!("the draft is an orphan: {json}"));
    assert_eq!(
        draft["links"],
        json!({"inbound": 0, "outbound": 1}),
        "{json}"
    );
    assert!(draft["words"].as_u64().unwrap() > 5, "{json}");
    let linked = "/api/admin/v1/posts/?filter=orphan:false";
    let (_, json) = send(&server, &key, ("GET", linked), "");
    let ids: Vec<&str> = (json["posts"].as_array().unwrap().iter())
        .map(|row| row["id"].as_str().unwrap())
        .collect();
    assert!(ids.contains(&"en.what-a-sitemap-does"), "{json}");
    assert!(!ids.contains(&"en.anchor-words"), "{json}");
    let live = (json["posts"].as_array().unwrap().iter())
        .find(|row| row["id"] == "en.what-a-sitemap-does")
        .unwrap();
    assert_eq!(live["path"], "/blog/what-a-sitemap-does/", "{json}");
}

#[test]
fn link_suggestions_rank_other_posts_this_one_does_not_link_to_yet() {
    let site = demo_site();
    write_draft(site.path());
    let key = writer_key(site.path());
    let server = start(site.path());

    let route = "/api/admin/v1/posts/en.anchor-words/link-suggestions/";
    let (status, json) = send(&server, &key, ("GET", route), "");
    assert_eq!(status, 200, "{json}");
    let ids: Vec<&str> = (json["suggestions"].as_array().unwrap().iter())
        .map(|each| each["id"].as_str().unwrap())
        .collect();
    let same_category = [
        "en.how-to-write-a-title-tag",
        "en.why-internal-links-matter",
    ];
    assert_eq!(ids, same_category, "by title: {json}");
    assert_eq!(json["suggestions"][0]["shares"]["category"], "on-page-seo");
    let path = &json["suggestions"][0]["path"];
    assert_eq!(path, "/blog/how-to-write-a-title-tag/", "{json}");

    // The demo post already links to the other post in its category.
    let route = format!("{ROUTE}link-suggestions/");
    let (status, json) = send(&server, &key, ("GET", &route), "");
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["suggestions"], json!([]), "{json}");
    let missing = "/api/admin/v1/posts/en.no-such-post/link-suggestions/";
    let (status, _) = send(&server, &key, ("GET", missing), "");
    assert_eq!(status, 404);
}

#[test]
fn a_copy_of_a_live_post_is_saved_as_a_new_draft_file() {
    let site = demo_site();
    let key = writer_key(site.path());
    let server = start(site.path());
    let route = format!("{ROUTE}duplicate/");
    let (status, json) = send(&server, &key, ("POST", &route), "");
    assert_eq!(status, 201, "{json}");
    assert_eq!(json["posts"][0]["id"], "en.why-internal-links-matter-copy");
    assert_eq!(json["posts"][0]["status"], "draft", "{json}");
    let mut file = site.path().to_path_buf();
    file.push("drafts/en/why-internal-links-matter-copy.md");
    let text = fs::read_to_string(&file).unwrap();
    assert!(
        text.contains("title: \"Why internal links matter (copy)\""),
        "{text}"
    );
    assert!(
        text.contains("](/blog/how-to-write-a-title-tag/)"),
        "the text: {text}"
    );
}

#[test]
fn the_site_issues_list_the_most_pressing_problems_first() {
    let site = demo_site();
    let key = writer_key(site.path());
    let server = start(site.path());
    let (status, json) = send(&server, &key, ("GET", "/api/admin/v1/lint/issues/"), "");
    assert_eq!(status, 200, "{json}");
    let issues = json["issues"].as_array().unwrap();
    assert!(issues.len() <= 10, "{json}");
    for issue in issues {
        assert!(issue["posts"].as_u64().unwrap() >= 1, "{json}");
        assert!(issue["example"].as_str().unwrap().contains('.'), "{json}");
        assert!(!issue["fix"].as_str().unwrap().is_empty(), "{json}");
    }
    let counts: Vec<(u8, u64)> = (issues.iter())
        .map(|issue| {
            (
                u8::from(issue["weight"] != "block"),
                issue["posts"].as_u64().unwrap(),
            )
        })
        .collect();
    let ranked = counts
        .windows(2)
        .all(|pair| pair[0].0 < pair[1].0 || (pair[0].0 == pair[1].0 && pair[0].1 >= pair[1].1));
    assert!(ranked, "blocks first, then the most posts: {json}");
}

#[test]
fn the_site_health_sends_the_score_khalid_audit_kept() {
    let site = demo_site();
    let key = writer_key(site.path());
    let server = start(site.path());
    let health = "/api/admin/v1/site/health/";
    let (status, json) = send(&server, &key, ("GET", health), "");
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["last"], Value::Null, "no audit has run yet: {json}");

    let audited = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args([
            "--site",
            site.path().to_str().unwrap(),
            "audit",
            "--offline",
        ])
        .output()
        .unwrap();
    assert_eq!(audited.status.code(), Some(0));
    let (status, json) = send(&server, &key, ("GET", health), "");
    assert_eq!(status, 200, "{json}");
    assert_eq!(json["last"]["score"], 100, "{json}");
    assert_eq!(json["trend"].as_array().unwrap().len(), 1, "{json}");
}
