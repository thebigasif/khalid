//! `khalid build` fills the home page's sections from `[reader]
//! home_sections` in the owner's order: the featured posts, a category,
//! the newsletter sign-up, a series in reading order and the newest posts,
//! each named and linked as its own list page is.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const SITE_TOML: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
mount = "/blog/"
default_language = "en"
languages = ["en", "de"]

[reader]
home_sections = [
  "featured",
  { category = "clusters" },
  "newsletter",
  { series = "topic-clusters-from-scratch" },
  "latest",
  { category = "no-such-topic" },
]

[mail]
from = "news@example.com"
name = "Field Notes"

[mail.smtp]
host = "127.0.0.1"
port = 2525
security = "none"
private_host = true
"#;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

#[cfg(test)]
fn put(site: &Path, rel: &str, text: &str) {
    let file = at(site, rel);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, text).unwrap();
}

/// Three parts of a series in the `clusters` category, the middle one
/// featured, and one post in German with no category.
#[cfg(test)]
fn site() -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    put(temp.path(), "site.toml", SITE_TOML);
    for (slug, date, extra) in [
        (
            "plan-the-posts",
            "2026-09-10",
            "featured: true\nseries:\n  name: Topic clusters from scratch\n  order: 2\n",
        ),
        (
            "pick-the-subject",
            "2026-09-20",
            "series:\n  name: Topic clusters from scratch\n  order: 1\n",
        ),
        (
            "link-the-cluster",
            "2026-09-05",
            "series:\n  name: Topic clusters from scratch\n  order: 3\n",
        ),
    ] {
        let title = slug.replace('-', " ");
        put(
            temp.path(),
            &format!("content/en/posts/{slug}.md"),
            &format!(
                "---\ntitle: {title}\nslug: {slug}\ndate: {date}\nauthor: asif\ncategory: clusters\n{extra}---\n\n\
                 A part that links to [the home page](/blog/).\n"
            ),
        );
    }
    put(
        temp.path(),
        "content/de/posts/warum-links-zaehlen.md",
        "---\ntitle: Warum Links zählen\nslug: warum-links-zaehlen\ndate: 2026-09-01\nauthor: asif\n---\n\n\
         Ein Beitrag mit einem Link zur [Startseite](/blog/de/).\n",
    );
    for language in ["en", "de"] {
        put(
            temp.path(),
            &format!("content/{language}/authors/asif.md"),
            "---\nname: Asif\nslug: asif\n---\n\nAsif writes about links.\n",
        );
    }
    temp
}

#[cfg(test)]
fn build(site: &TempDir) {
    let output: Output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site.path())
        .arg("build")
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{said}");
}

#[cfg(test)]
fn main_of(site: &TempDir, rel: &str) -> String {
    let page = fs::read_to_string(at(site.path(), &format!("public/{rel}"))).unwrap();
    let (_, main) = page.split_once("<main").unwrap();
    let (main, _) = main.split_once("</main>").unwrap();
    main.to_owned()
}

#[test]
fn the_sections_show_in_the_owner_order_each_linked_to_its_list() {
    let site = site();
    build(&site);
    let main = main_of(&site, "index.html");
    let at = |mark: &str| {
        main.find(mark)
            .unwrap_or_else(|| panic!("{mark} in {main}"))
    };
    let order = [
        "<h2>Editor&#39;s picks</h2>",
        "<h2>clusters</h2>",
        "id=\"home-newsletter-title\"",
        "<h2>Topic clusters from scratch</h2>",
        "<h2>Latest posts</h2>",
    ]
    .map(at);
    assert!(order.is_sorted(), "{order:?} in {main}");
    assert_eq!(
        main.matches("<form class=\"subscribe\"").count(),
        1,
        "{main}"
    );
    for link in [
        "href=\"/blog/category/clusters/\"",
        "href=\"/blog/series/topic-clusters-from-scratch/\"",
        "href=\"/blog/archive/\"",
    ] {
        assert!(main.contains(link), "{link} in {main}");
    }
    assert!(!main.contains("no-such-topic"), "{main}");
    let (_, series) = main
        .split_once("<h2>Topic clusters from scratch</h2>")
        .unwrap();
    let series = series.split("<h2>").next().unwrap();
    let first = series.find("/blog/pick-the-subject/").unwrap();
    let second = series.find("/blog/plan-the-posts/").unwrap();
    assert!(first < second, "reading order: {series}");
    let (_, picks) = main.split_once("<h2>Editor&#39;s picks</h2>").unwrap();
    let picks = picks.split("<h2>").next().unwrap();
    assert!(picks.contains("/blog/plan-the-posts/"), "{picks}");
    assert!(!picks.contains("/blog/pick-the-subject/"), "{picks}");
}

#[test]
fn a_language_shows_only_the_sections_it_has_posts_for() {
    let site = site();
    build(&site);
    let main = main_of(&site, "de/index.html");
    assert!(!main.contains("<h2>clusters</h2>"), "{main}");
    assert!(!main.contains("Topic clusters from scratch"), "{main}");
    assert!(main.contains("/blog/de/warum-links-zaehlen/"), "{main}");
    assert!(main.contains("href=\"/blog/de/archive/\""), "{main}");
    assert!(main.contains("id=\"home-newsletter-title\""), "{main}");
}
