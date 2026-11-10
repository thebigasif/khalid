//! `khalid build`: the whole site rendered as HTML into an output folder
//! (RULES.md BL-CLI-1 to BL-CLI-3, BL-FS-2).

use khalid_core::SitePath;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

#[path = "build/ai_page.rs"]
mod ai_page;
#[path = "build/fonts.rs"]
mod fonts;
#[path = "build/gate.rs"]
mod gate;

#[cfg(test)]
fn khalid(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(args)
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

#[cfg(test)]
fn path(folder: &Path) -> &str {
    folder.to_str().unwrap()
}

#[cfg(test)]
fn demo_site() -> PathBuf {
    at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    )
}

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

#[test]
fn build_renders_every_post_and_home_page_of_the_demo_site() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert!(
        text(&output.stdout).contains("47 pages"),
        "{}",
        text(&output.stdout)
    );

    let post = page(out.path(), "why-internal-links-matter/index.html");
    assert!(post.contains("<html lang=\"en\""), "{post}");
    assert!(post.contains("Why internal links matter</h1>"), "{post}");
    assert!(
        post.contains("<h2 id=\"readers-use-them-to-keep-going\""),
        "{post}"
    );
    assert!(post.contains("https://www.example.com/blog/why-internal-links-matter/"));

    let german = page(out.path(), "de/warum-interne-links-wichtig-sind/index.html");
    assert!(german.contains("<html lang=\"de\""), "{german}");

    let home = page(out.path(), "index.html");
    let newest = home.find("/blog/planning-a-topic-cluster/");
    let oldest = home.find("/blog/why-internal-links-matter/");
    assert!(newest.is_some() && newest < oldest, "newest first:\n{home}");
    assert!(page(out.path(), "de/index.html").contains("/blog/de/"));
    assert!(page(out.path(), "404.html").contains("<h1"));
    assert!(page(out.path(), "de/404.html").contains("lang=\"de\""));

    // The AI info page: plain HTML that names who runs the site and how to
    // cite it, its own AboutPage schema, and a footer link from the home
    // page (P3). It needs no scripts: every fact is in the HTML.
    let ai = page(out.path(), "ai/index.html");
    assert!(ai.contains("runs this site"), "{ai}");
    assert!(ai.contains("Name Field Notes on SEO"), "{ai}");
    assert!(ai.contains("\"AboutPage\""), "{ai}");
    // Scripts off still shows every fact: besides the JSON-LD block, which
    // carries no behaviour, the only script is the reader script every
    // page loads deferred, which adds behaviour and no text.
    let scripts: Vec<&str> = ai.split("<script").skip(1).collect();
    assert!(
        scripts.iter().all(|tag| {
            tag.starts_with(" type=\"application/ld+json\">")
                || tag.starts_with(" src=\"/blog/assets/reader.")
        }),
        "{ai}"
    );
    let home_body = page(out.path(), "index.html");
    assert!(home_body.contains("href=\"/blog/ai/\""), "{home_body}");
    // Its text comes from site.toml in one language, so it is built once,
    // in the main language, and the German footer links to that copy.
    assert!(
        SitePath::within(out.path(), "de/ai")
            .unwrap()
            .as_path()
            .read_dir()
            .is_err()
    );
    let german_home = page(out.path(), "de/index.html");
    assert!(german_home.contains("href=\"/blog/ai/\""), "{german_home}");
}

#[test]
fn build_writes_the_author_and_plain_pages_every_link_points_to() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));

    let post = page(out.path(), "what-a-sitemap-does/index.html");
    assert!(post.contains("/blog/authors/lena-vogt/"), "{post}");
    let author = page(out.path(), "authors/lena-vogt/index.html");
    assert!(author.contains("Lena Vogt</h1>"), "{author}");
    assert!(author.contains("/blog/what-a-sitemap-does/"), "{author}");
    assert!(
        !author.contains("/blog/why-internal-links-matter/"),
        "{author}"
    );
    assert!(page(out.path(), "de/authors/asif/index.html").contains("lang=\"de\""));

    assert!(page(out.path(), "about/index.html").contains("About Field Notes</h1>"));
    assert!(page(out.path(), "de/ueber-uns/index.html").contains("Über Field Notes</h1>"));
}

#[test]
fn build_writes_a_page_for_each_glossary_term_where_its_route_puts_it() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));

    let term = page(out.path(), "glossary/topical-authority/index.html");
    assert!(term.contains("Topical authority</h1>"), "{term}");
    assert!(term.contains("https://www.example.com/blog/glossary/topical-authority/"));
    let german = page(out.path(), "de/glossary/thematische-autoritaet/index.html");
    assert!(german.contains("<html lang=\"de\""), "{german}");
}

#[test]
fn build_turns_each_wikilink_of_the_demo_site_into_a_link_to_its_page() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));

    let post = page(out.path(), "why-internal-links-matter/index.html");
    let link = "<a href=\"/blog/glossary/topical-authority/\">Topical authority</a>";
    assert!(post.contains(link), "{post}");
    let german = page(out.path(), "de/warum-interne-links-wichtig-sind/index.html");
    let link = "<a href=\"/blog/de/glossary/thematische-autoritaet/\">";
    assert!(german.contains(link), "{german}");
    let term = page(out.path(), "glossary/topical-authority/index.html");
    assert!(
        term.contains("<a href=\"/blog/planning-a-topic-cluster/\">"),
        "{term}"
    );
}

#[test]
fn build_draws_the_maths_of_a_post_as_mathml() {
    let temp = tempfile::tempdir().unwrap();
    let site = path(temp.path());
    khalid(&["init", site, "--address", "https://www.example.com"]);
    let post = "---\ntitle: Compound growth\nslug: compound-growth\ndate: 2026-09-14\n\
                author: asif\n---\n\n\
                Traffic grows as $v(1+r)^n$ each month, as [the home page](/) says.\n\n\
                :::maths{label=\"Visits after n months\"}\nv_n = v_0 (1 + r)^n\n:::\n";
    fs::write(at(temp.path(), "content/en/posts/compound-growth.md"), post).unwrap();

    let output = khalid(&["--site", site, "build"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let page = page(&at(temp.path(), "public"), "compound-growth/index.html");
    assert!(page.contains("<math"), "{page}");
    assert!(!page.contains("$v(1+r)^n$"), "{page}");
    assert!(
        page.contains("<figure class=\"maths\">\n<math display=\"block\""),
        "{page}"
    );
    assert!(
        page.contains("<figcaption>Visits after n months</figcaption>"),
        "{page}"
    );
    assert!(!page.contains("v_n = v_0"), "{page}");
}

#[test]
fn build_stops_at_a_broken_wikilink_and_names_its_file_and_line() {
    let out = tempfile::tempdir().unwrap();
    let site = at(&demo_site(), "../invalid/content-broken-wikilink");
    let output = khalid(&["--site", path(&site), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stdout));
    let said = text(&output.stderr);
    assert!(
        said.contains("why-links-matter.md, line 10: [[link-equity]] matches no post"),
        "{said}"
    );
    assert!(!at(out.path(), "index.html").exists());
}

#[test]
fn build_writes_each_post_where_routes_toml_puts_it() {
    let temp = tempfile::tempdir().unwrap();
    let site = path(temp.path());
    let made = khalid(&["init", site, "--address", "https://www.example.com"]);
    assert_eq!(made.status.code(), Some(0), "{}", text(&made.stderr));
    let routes = "[collections.posts]\npermalink = \"/{year}/{slug}/\"\n";
    fs::write(at(temp.path(), "routes.toml"), routes).unwrap();
    let post = "---\ntitle: Dated links\nslug: dated-links\ndate: 2026-09-14\nauthor: asif\n\
                ---\n\nText that links to [the home page](/).\n";
    fs::write(at(temp.path(), "content/en/posts/dated-links.md"), post).unwrap();

    let output = khalid(&["--site", site, "build"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let public = at(temp.path(), "public");
    assert!(page(&public, "2026/dated-links/index.html").contains("Dated links</h1>"));
    assert!(page(&public, "index.html").contains("/2026/dated-links/\""));
}

#[test]
fn build_refuses_a_glossary_term_whose_route_needs_a_date() {
    let temp = tempfile::tempdir().unwrap();
    let site = path(temp.path());
    khalid(&["init", site, "--address", "https://www.example.com"]);
    let routes = "[collections.glossary]\npermalink = \"/{year}/{slug}/\"\n";
    fs::write(at(temp.path(), "routes.toml"), routes).unwrap();
    fs::create_dir_all(at(temp.path(), "content/en/glossary")).unwrap();
    let term = "---\nterm: Crawl budget\nslug: crawl-budget\n---\n\nText.\n";
    fs::write(at(temp.path(), "content/en/glossary/crawl-budget.md"), term).unwrap();

    let output = khalid(&["--site", site, "build", "--json"]);
    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stdout));
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let what = json["error"]["what"].as_str().unwrap();
    assert!(what.contains("crawl-budget.md has no address"), "{what}");
    assert!(!at(temp.path(), "public/index.html").exists());
}

#[test]
fn build_writes_the_stylesheet_each_page_links_with_its_compressed_copies() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));

    let linked = |file: &str| {
        let html = page(out.path(), file);
        let (_, rest) = html.split_once(r#"<link rel="stylesheet" href=""#).unwrap();
        let (href, _) = rest.split_once('"').unwrap();
        assert_eq!(html.matches(r#"<link rel="stylesheet""#).count(), 1);
        href.strip_prefix("/blog/").unwrap().to_owned()
    };
    let (home, post) = (
        linked("index.html"),
        linked("why-internal-links-matter/index.html"),
    );
    for file in [&home, &post] {
        let hash = file
            .strip_prefix("assets/site.")
            .and_then(|r| r.strip_suffix(".css"));
        assert_eq!(hash.map(str::len), Some(8), "{file}");
        let css = page(out.path(), file);
        assert!(css.contains("--color-background: #ffffff;"), "{css}");
        for copy in [".br", ".gz"] {
            let size = fs::metadata(at(out.path(), &format!("{file}{copy}")))
                .unwrap()
                .len();
            assert!(size > 0 && size < 10 * 1024, "{copy}: {size} bytes");
        }
    }
    // Each page links the rules its HTML can use: the rules of a post's
    // header are not in the home page's stylesheet.
    assert_ne!(home, post);
    assert!(page(out.path(), &post).contains(".post-header"));
    assert!(!page(out.path(), &home).contains(".post-header"));
}

#[test]
fn init_then_new_then_build_writes_the_post_to_public() {
    let temp = tempfile::tempdir().unwrap();
    let site = path(temp.path());
    let made = khalid(&[
        "init",
        site,
        "--address",
        "https://www.example.com",
        "--mount",
        "/blog/",
    ]);
    assert_eq!(made.status.code(), Some(0), "{}", text(&made.stderr));
    let new = khalid(&["--site", site, "new", "Why links matter"]);
    assert_eq!(new.status.code(), Some(0), "{}", text(&new.stderr));

    let public = at(temp.path(), "public");
    let unready = khalid(&["--site", site, "build"]);
    assert_eq!(
        unready.status.code(),
        Some(0),
        "the rest of the site builds"
    );
    assert!(text(&unready.stdout).contains("This post has no author."));
    assert!(public.exists(), "the home page is written");
    assert!(
        !at(&public, "why-links-matter").exists(),
        "a new post has no author yet"
    );

    let posts = at(temp.path(), "content/en/posts");
    let file = fs::read_dir(&posts)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let written = fs::read_to_string(&file).unwrap();
    let mut finished = written.replacen("---\n", "---\nauthor: asif\n", 1);
    finished.push_str(
        "Links help readers find [the home page](/blog/) and [a guide](https://example.org/).\n",
    );
    fs::write(&file, finished).unwrap();

    let output = khalid(&["--site", site, "build", "--json"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["out"], path(&public));
    // The post, home, search, 404, archive, authors, sitemap, privacy, AI
    // info and privacy choices pages.
    assert_eq!(json["pages"], 10);
    let post = page(&public, "why-links-matter/index.html");
    assert!(post.contains("Why links matter</h1>"), "{post}");
}

#[test]
fn build_copies_a_folder_posts_image_beside_its_page() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let chart = "content/en/posts/2026-10-05-fixing-slow-pages/page-weight.png";
    assert_eq!(
        fs::read(at(out.path(), "fixing-slow-pages/page-weight.png")).unwrap(),
        fs::read(at(&demo, chart)).unwrap(),
    );
}

#[test]
fn a_posts_lead_image_is_drawn_at_once_and_shared() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let post = page(out.path(), "fixing-slow-pages/index.html");
    let (_, lead) = post.split_once("<figure class=\"image\">").expect(&post);
    let (lead, _) = lead.split_once("</figure>").unwrap();
    assert!(
        lead.contains("alt=\"A bar chart of page weight by part\""),
        "{lead}"
    );
    assert!(lead.contains(" fetchpriority=\"high\""), "{lead}");
    assert!(!lead.contains("loading=\"lazy\""), "{lead}");
    assert!(lead.contains("/blog/media/"), "{lead}");
    assert!(
        lead.contains("<figcaption>Images weigh the most.</figcaption>"),
        "{lead}"
    );
    assert!(
        post.contains("<meta property=\"og:image\" content=\"https://www.example.com/blog/media/"),
        "{post}"
    );
    let plain = page(out.path(), "why-internal-links-matter/index.html");
    assert!(!plain.contains("fetchpriority"), "{plain}");
}

#[test]
fn a_posts_faq_block_reaches_its_json_ld() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let post = page(out.path(), "fixing-slow-pages/index.html");
    let (_, graph) = post.split_once("application/ld+json").expect(&post);
    let (graph, _) = graph.split_once("</script>").unwrap();
    assert!(graph.contains("\"FAQPage\""), "{graph}");
    assert!(graph.contains("Which page should I fix first?"), "{graph}");
}

/// Every `.html` file under `folder`.
#[cfg(test)]
fn html_files(folder: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(folder).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            html_files(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "html") {
            found.push(path);
        }
    }
}

#[test]
fn every_image_on_every_built_page_has_its_five_attributes() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let mut pages = Vec::new();
    html_files(out.path(), &mut pages);
    let mut images = 0;
    for page in pages {
        let html = fs::read_to_string(&page).unwrap();
        for tag in html.split("<img ").skip(1) {
            let tag = tag.split_once('>').unwrap().0;
            for attribute in ["width", "height", "srcset", "sizes", "alt"] {
                assert!(
                    tag.contains(&format!(" {attribute}=\"")) || tag.starts_with(attribute),
                    "{}: <img {tag}> has no {attribute}",
                    page.display()
                );
            }
            images += 1;
        }
    }
    assert!(images > 0, "the demo site shows at least one image");
    let post = page(out.path(), "fixing-slow-pages/index.html");
    let srcset = post.split("srcset=\"/blog/").nth(1).unwrap();
    let webp = srcset.split(' ').next().unwrap();
    assert!(
        webp.starts_with("media/") && webp.contains(".webp"),
        "{webp}"
    );
    assert!(at(out.path(), webp).is_file(), "{webp} is written");
}

#[test]
fn build_needs_a_site_folder() {
    let output = khalid(&["build"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(text(&output.stderr).contains("--site"));
}

#[test]
fn build_refuses_to_write_into_the_content_folder() {
    let temp = tempfile::tempdir().unwrap();
    let site = path(temp.path());
    khalid(&["init", site, "--address", "https://www.example.com"]);
    let content = at(temp.path(), "content/html");
    let output = khalid(&["--site", site, "build", "--out", path(&content), "--json"]);
    assert_eq!(output.status.code(), Some(1));
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(json["error"]["why"].as_str().unwrap().contains("content"));
    assert!(!content.exists());
}

#[test]
fn static_builds_carry_the_referrer_meta_tags() {
    let out = tempfile::tempdir().unwrap();
    let demo = demo_site();
    let output = khalid(&["--site", path(&demo), "build", "--out", path(out.path())]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    for rel in [
        "index.html",
        "why-internal-links-matter/index.html",
        "404.html",
    ] {
        let html = page(out.path(), rel);
        assert!(
            html.contains(r#"<meta name="referrer" content="strict-origin-when-cross-origin">"#),
            "{rel} carries the referrer policy as a meta tag, for hosts that drop headers"
        );
    }
}
