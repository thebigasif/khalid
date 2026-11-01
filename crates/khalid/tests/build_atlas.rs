//! The Atlas template end to end (templates spec 3.2): `khalid build` of
//! the demo site with `name = "atlas"`, three posts marked featured and a
//! call to action, snapshots of its front page, a post and an author
//! page, and the parts each must show.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Copies the folder `from` into `into`, file by file.
#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = at(into, &entry.file_name().to_string_lossy());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// The demo posts marked `featured: true`, newest first once listed.
const FEATURED: [&str; 3] = [
    "content/en/posts/2026-10-12-planning-a-topic-cluster.md",
    "content/en/posts/2026-09-28-what-a-sitemap-does.md",
    "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
];

/// Builds a copy of the demo site in Atlas with `table` added to its
/// `site.toml` and the `FEATURED` posts marked, and expects success.
#[cfg(test)]
fn built(table: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(table);
    fs::write(&settings, toml).unwrap();
    for rel in FEATURED {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(&path, text.replacen('\n', "\nfeatured: true\n", 1)).unwrap();
    }
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    (temp, out)
}

/// The `<body>` of the page at `rel`: the head only differs by the
/// stylesheet's hash.
#[cfg(test)]
fn body(out: &Path, rel: &str) -> String {
    let html = fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"));
    let (_, body) = html.split_once("<body>").unwrap();
    body.to_owned()
}

/// The part of `html` from `start` to the next `end` after it.
#[cfg(test)]
fn between<'a>(html: &'a str, start: &str, end: &str) -> &'a str {
    let (_, rest) = html
        .split_once(start)
        .unwrap_or_else(|| panic!("{start}: {html}"));
    rest.split_once(end).map_or(rest, |(part, _)| part)
}

const ATLAS: &str = "\n[template]\nname = \"atlas\"\n\n[template.cta]\n\
    label = \"Get the checklist\"\nhref = \"https://www.example.com/checklist/\"\n\
    note = \"One page, free.\"\n";

#[test]
fn atlas_front_page() {
    let (_temp, out) = built(ATLAS);
    let home = body(&out, "index.html");
    let bar = between(&home, "<nav class=\"catbar\"", "</nav>");
    assert!(
        bar.contains("<a href=\"/blog/\" aria-current=\"page\">All posts</a>"),
        "{bar}"
    );
    assert!(bar.contains(">Technical SEO</a>"), "{bar}");
    let leads = between(&home, "<section class=\"leads\"", "</section>");
    assert_eq!(leads.matches("story-lead").count(), 2, "{leads}");
    assert!(
        leads.contains("Planning a topic cluster"),
        "newest first: {leads}"
    );
    let picks = between(&home, "<section class=\"picks\"", "</section>");
    assert!(picks.contains("How to write a title tag"), "{picks}");
    let feed = between(&home, "<section class=\"feed\"", "</section>");
    assert_eq!(feed.matches("<li>").count(), 3, "leads stay out: {feed}");
    assert_eq!(home.matches("<section class=\"topic-block\"").count(), 3);
    assert!(home.contains("<div class=\"cta-band\">"), "{home}");
    assert!(
        home.contains(
            "<a class=\"button header-button\" href=\"https://www.example.com/checklist/\">"
        ),
        "{home}"
    );
    insta::assert_snapshot!(home);
}

#[test]
fn atlas_without_featured_posts_opens_with_the_feed() {
    let (_temp, out) =
        built("\n[template]\nname = \"atlas\"\n[template.options]\nfeatured = false\npicks = 0\n");
    let home = body(&out, "index.html");
    assert!(!home.contains("class=\"leads\""), "{home}");
    assert!(!home.contains("class=\"picks\""), "{home}");
    let feed = between(&home, "<section class=\"feed\"", "</section>");
    assert_eq!(feed.matches("<li>").count(), 5, "{feed}");
}

#[test]
fn atlas_post_page() {
    let (_temp, out) = built(ATLAS);
    let post = body(&out, "what-a-sitemap-does/index.html");
    let trail = between(&post, "<nav class=\"breadcrumbs\"", "</nav>");
    assert!(trail.contains(">Technical SEO</a>"), "{trail}");
    assert!(
        post.contains("<article class=\"post post-with-toc post-toc-start\">"),
        "contents on the start side: {post}"
    );
    let side = between(&post, "<nav class=\"toc-side\"", "</nav>");
    assert!(side.contains("<ol>"), "numbered: {side}");
    assert_eq!(
        post.matches("<div class=\"share\">").count(),
        2,
        "top and end"
    );
    let (_, header) = post.split_once("<header class=\"post-header\">").unwrap();
    let (title, rest) = header.split_once("</header>").unwrap();
    assert!(title.contains("<h1>"), "{title}");
    assert!(
        rest.trim_start().starts_with("<div class=\"share\">"),
        "share under the title: {rest}"
    );
    let author = post.find("<section class=\"author-box\">").unwrap();
    let band = post.find("<div class=\"cta-band\">").unwrap();
    let related = post.find("<section class=\"related\">").unwrap();
    assert!(
        author < band && band < related,
        "author, band, related: {post}"
    );
    let list = between(&post, "<section class=\"related\">", "</section>");
    assert!(list.contains("<ul class=\"cards cards-text\""), "{list}");
    assert!(list.matches("<li>").count() <= 3, "{list}");
    insta::assert_snapshot!(post);
}

#[test]
fn atlas_author_page() {
    let (_temp, out) = built(ATLAS);
    let author = body(&out, "authors/asif/index.html");
    assert!(author.contains("<nav class=\"breadcrumbs\""), "{author}");
    assert!(
        author.contains("<ul class=\"cards cards-text\""),
        "{author}"
    );
    assert!(
        author.contains("<span class=\"card-reading-time\">"),
        "{author}"
    );
    insta::assert_snapshot!(author);
}
