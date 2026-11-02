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
fn robots_txt_and_llms_txt_follow_their_settings() {
    let site = site(
        "\n[seo.crawlers]\nextra_lines = [\"Crawl-delay: 5\"]\n\n\
         [seo.llms_txt]\nintro = \"Two people write Field Notes.\"\npages = [\"/post-2/\"]\n\
         exclude = [\"/news/\"]\n",
    );
    build(&site);
    let robots = file(&site, "robots.txt").unwrap();
    assert!(robots.contains("Crawl-delay: 5\n"), "{robots}");
    let llms = file(&site, "llms.txt").unwrap();
    assert!(llms.contains("Two people write Field Notes."), "{llms}");
    assert!(
        llms.contains("## Key pages\n\n- [Post 2](https://www.example.com/blog/post-2/)"),
        "{llms}"
    );
    assert!(!llms.contains("Launch day"), "{llms}");
    let off = self::site("\n[seo.llms_txt]\noff = true\n");
    build(&off);
    assert!(file(&off, "llms.txt").is_none());
}

#[test]
fn the_whole_site_schema_is_one_file_named_in_robots_txt() {
    let site = site("");
    build(&site);
    let schema: serde_json::Value =
        serde_json::from_str(&file(&site, "schema.json").unwrap()).unwrap();
    let graph = schema["@graph"].as_array().unwrap();
    for post in ["post-1", "post-3", "news/2026/09/launch"] {
        let id = format!("https://www.example.com/blog/{post}/");
        assert!(
            (graph.iter()).any(|node| node["@id"].as_str().is_some_and(|at| at.starts_with(&id))),
            "{id}: {schema:#}"
        );
    }
    let robots = file(&site, "robots.txt").unwrap();
    assert!(
        robots.contains("Schemamap: https://www.example.com/blog/schema.json\n"),
        "{robots}"
    );
    let off = self::site("\n[seo.schema_map]\noff = true\n");
    build(&off);
    assert!(file(&off, "schema.json").is_none());
    assert!(!file(&off, "robots.txt").unwrap().contains("Schemamap"));
}

#[test]
fn a_post_and_a_page_choose_their_schema_types_and_news_fields() {
    let site = site("");
    put(
        site.path(),
        "routes.toml",
        "[collections.news]\npermalink = \"/news/{year}/{month}/{slug}/\"\nnews = true\n",
    );
    put(
        site.path(),
        "content/en/pages/contact.md",
        "---\ntitle: Contact\nslug: contact\npage_type: ContactPage\n---\n\nWrite to us.\n",
    );
    let news = "---\ntitle: Quarterly report\nslug: quarterly\ndate: 2026-09-04\nauthor: asif\n\
                article_type: Report\nstock_tickers: [\"NASDAQ:GOOG\"]\n---\n\n\
                The report links to [the home page](/blog/).\n\n\
                ::video{src=\"/media/a.mp4\" title=\"A tour\" poster=\"/media/a.jpg\" thumbnail=\"/media/a-thumb.jpg\"}\n";
    put(site.path(), "content/en/news/quarterly.md", news);
    put(
        site.path(),
        "content/en/news/launch.md",
        "---\ntitle: Launch day\nslug: launch\ndate: 2026-09-03\nauthor: asif\nnews: false\n---\n\n\
         The launch links to [the home page](/blog/).\n",
    );
    build(&site);
    let contact = file(&site, "contact/index.html").unwrap();
    assert!(contact.contains("\"@type\":\"ContactPage\""), "{contact}");
    let report = file(&site, "news/2026/09/quarterly/index.html").unwrap();
    assert!(report.contains("\"@type\":\"Report\""), "{report}");
    assert!(
        report.contains("\"thumbnailUrl\":\"https://www.example.com/media/a-thumb.jpg\""),
        "{report}"
    );
    let launch = file(&site, "news/2026/09/launch/index.html").unwrap();
    assert!(launch.contains("\"@type\":\"NewsArticle\""), "{launch}");
}

#[test]
fn a_post_preview_on_x_and_slack_names_its_author_and_reading_time() {
    let site = site("");
    build(&site);
    let post = file(&site, "post-1/index.html").unwrap();
    assert!(
        post.contains("<meta name=\"twitter:label1\" content=\"Written by\">\n<meta name=\"twitter:data1\" content=\"Asif\">"),
        "{post}"
    );
    assert!(post.contains("<meta name=\"twitter:data2\" content=\"1 min read\">"));
    let off = self::site("\n[seo.social]\nshare_labels = false\n");
    build(&off);
    let post = file(&off, "post-1/index.html").unwrap();
    assert!(!post.contains("twitter:label1"), "{post}");
}

#[test]
fn category_and_tag_pages_use_their_own_title_description_robots_image_and_intro() {
    let site = site("");
    put(
        site.path(),
        "categories.toml",
        "[[categories]]\nslug = \"links\"\nen = \"Links\"\n\n\
         [categories.seo.en]\ntitle = \"Every link guide {sep} {site}\"\n\
         description = \"Guides on earning and checking links.\"\n\
         intro = \"Start with **the basics**.\"\n\n\
         [categories.seo.en.image]\nurl = \"https://www.example.com/media/links.png\"\n\
         alt = \"A chain of links\"\n",
    );
    put(
        site.path(),
        "tags.toml",
        "[[tags]]\nslug = \"crawl\"\n\n[tags.seo.en]\nrobots = \"noindex\"\n",
    );
    put(
        site.path(),
        "content/en/posts/linked.md",
        "---\ntitle: Linked\nslug: linked\ndate: 2026-09-20\nauthor: asif\ncategory: links\n\
         tags: [crawl]\n---\n\nLinked points to [the home page](/blog/).\n",
    );
    build(&site);
    let links = file(&site, "category/links/index.html").unwrap();
    assert!(
        links.contains("<title>Every link guide | Field Notes on SEO</title>"),
        "{links}"
    );
    assert!(links.contains(r#"content="Guides on earning and checking links.""#));
    assert!(links.contains(r#"content="https://www.example.com/media/links.png""#));
    assert!(links.contains("<strong>the basics</strong>"), "{links}");
    let crawl = file(&site, "tag/crawl/index.html").unwrap();
    assert!(crawl.contains(r#"content="noindex"#), "{crawl}");
    assert!(!crawl.contains(r#"<div class="intro">"#), "{crawl}");
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
