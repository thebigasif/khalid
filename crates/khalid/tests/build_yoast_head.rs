//! `khalid build` with the head and settings features of the Yoast
//! comparison of 2 November 2026: archives switched off, search settings
//! per content type and feeds per format.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const SITE_TOML: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
mount = "/blog/"
mode = "static"
default_language = "en"
languages = ["en"]
"#;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Writes `text` to `rel` inside `site`, making its folders first.
#[cfg(test)]
fn put(site: &Path, rel: &str, text: &str) {
    let file = at(site, rel);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, text).unwrap();
}

/// A site with three posts by Asif, a news post dated by month, and
/// `extra` added to `site.toml`.
#[cfg(test)]
fn site(extra: &str) -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    put(temp.path(), "site.toml", &format!("{SITE_TOML}{extra}"));
    put(
        temp.path(),
        "content/en/authors/asif.md",
        "---\nname: Asif\nslug: asif\nrole: Founder of SerpDad\n---\n\nAsif writes about links.\n",
    );
    for day in 1..=3 {
        let text = format!(
            "---\ntitle: Post {day}\nslug: post-{day}\ndate: 2026-09-{day:02}\nauthor: asif\n---\n\n\
             Post {day} links to [the home page](/blog/).\n"
        );
        put(
            temp.path(),
            &format!("content/en/posts/post-{day}.md"),
            &text,
        );
    }
    put(
        temp.path(),
        "routes.toml",
        "[collections.news]\npermalink = \"/news/{year}/{month}/{slug}/\"\n",
    );
    let news = "---\ntitle: Launch day\nslug: launch\ndate: 2026-09-03\nauthor: asif\n---\n\n\
                The launch links to [the home page](/blog/).\n";
    put(temp.path(), "content/en/news/launch.md", news);
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

/// The built file at `rel` inside `public/`, or `None` when there is none.
#[cfg(test)]
fn file(site: &TempDir, rel: &str) -> Option<String> {
    fs::read_to_string(at(site.path(), &format!("public/{rel}"))).ok()
}

#[test]
fn author_and_date_archives_switched_off_are_not_built_and_redirect_home() {
    let site = site("\n[seo.index]\nauthors = \"off\"\ndates = \"off\"\n");
    build(&site);
    assert!(file(&site, "authors/asif/index.html").is_none());
    assert!(file(&site, "authors/index.html").is_none());
    assert!(file(&site, "news/2026/09/index.html").is_none());
    let redirects = file(&site, "_redirects").unwrap();
    assert!(
        redirects.contains("/blog/authors/asif/ /blog/ 301\n"),
        "{redirects}"
    );
    assert!(redirects.contains("/blog/news/2026/09/ /blog/ 301\n"));
    let sitemaps = fs::read_dir(at(site.path(), "public/sitemaps")).unwrap();
    for sitemap in sitemaps {
        let text = fs::read_to_string(sitemap.unwrap().path()).unwrap();
        assert!(!text.contains("/authors/"), "{text}");
    }
}

#[test]
fn a_collection_and_pages_can_leave_search_and_a_feed_format_can_go() {
    let site =
        site("\n[seo.index.collections]\nnews = \"noindex\"\n\n[seo.feeds]\noff = [\"atom\"]\n");
    build(&site);
    let news = file(&site, "news/2026/09/launch/index.html").unwrap();
    assert!(news.contains(r#"content="noindex, follow""#), "{news}");
    let post = file(&site, "post-1/index.html").unwrap();
    assert!(post.contains(r#"content="index, follow"#), "{post}");
    assert!(file(&site, "atom.xml").is_none());
    assert!(file(&site, "feed.xml").is_some());
    assert!(file(&site, "authors/asif/index.html").is_some());
}
