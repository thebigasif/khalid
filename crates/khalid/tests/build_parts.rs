//! The parts a template switches on (templates spec 4.2), end to end:
//! `khalid build` of the demo site with the test-only sampler pack and one
//! part's option on shows the part, and classic shows none of them.

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

/// Builds a copy of the demo site with `table` added to its `site.toml`
/// and each of `files` written at its path, and expects success.
#[cfg(test)]
fn built(table: &str, files: &[(&str, &str)]) -> (tempfile::TempDir, PathBuf) {
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
    for (rel, text) in files {
        let path = at(temp.path(), rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        // A line starting with `+` is added after the file's first line,
        // such as a front matter key; anything else replaces the file.
        let text = match text.strip_prefix('+') {
            Some(line) => {
                fs::read_to_string(&path)
                    .unwrap()
                    .replacen('\n', &format!("\n{line}\n"), 1)
            }
            None => (*text).to_owned(),
        };
        fs::write(path, text).unwrap();
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

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

const BRAND: (&str, &str) = (
    "theme/tokens.toml",
    "[brand]\nlogo = \"https://www.example.com/logo.svg\"\nicon = \"https://www.example.com/icon.png\"\n",
);

#[test]
fn the_sampler_pack_shows_the_logo_and_classic_keeps_the_name() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\nlogo = true\n",
        &[BRAND],
    );
    let home = page(&sampler, "index.html");
    assert!(
        home.contains("<a class=\"site-name site-logo\" href=\"/blog/\"><img src=\"https://www.example.com/logo.svg\""),
        "{home}"
    );
    let (_temp, classic) = built("", &[BRAND]);
    let home = page(&classic, "index.html");
    assert!(!home.contains("logo.svg"), "{home}");
    assert!(
        home.contains("<link rel=\"icon\" href=\"https://www.example.com/icon.png\">"),
        "{home}"
    );
}

#[test]
fn the_sampler_pack_shows_a_tab_for_each_category_in_the_order_categories_toml_lists_them() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\ncategory_tabs = true\n",
        &[],
    );
    let home = page(&sampler, "index.html");
    let tabs = "<li><a href=\"/blog/\" aria-current=\"page\">All</a></li>\n\
<li><a href=\"/blog/category/on-page-seo/\">On-page SEO</a></li>\n\
<li><a href=\"/blog/category/technical-seo/\">Technical SEO</a></li>\n\
<li><a href=\"/blog/category/content-strategy/\">Content strategy</a></li>\n</ul>";
    assert!(home.contains(tabs), "{home}");
    let german = page(&sampler, "de/index.html");
    assert!(
        german
            .contains("<li><a href=\"/blog/de/category/technical-seo/\">Technisches SEO</a></li>"),
        "{german}"
    );
    let (_temp, classic) = built("", &[]);
    let home = page(&classic, "index.html");
    assert!(!home.contains("category-tabs"), "{home}");
}

#[test]
fn the_sampler_pack_links_each_post_back_to_all_posts() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\nback_link = true\n",
        &[],
    );
    let back = "<p class=\"back-link\"><a href=\"/blog/\">All posts</a></p>";
    let post = page(&sampler, "what-a-sitemap-does/index.html");
    assert!(post.contains(back), "{post}");
    let german = page(&sampler, "de/was-eine-sitemap-leistet/index.html");
    // The German words come from the strings file; the link goes to the
    // German front page.
    assert!(
        german.contains("<p class=\"back-link\"><a href=\"/blog/de/\">"),
        "{german}"
    );
    assert!(!german.contains(">All posts<"), "{german}");
    assert!(!page(&sampler, "index.html").contains("back-link"));
    let (_temp, classic) = built("", &[]);
    let post = page(&classic, "what-a-sitemap-does/index.html");
    assert!(!post.contains("back-link"), "{post}");
}

#[test]
fn the_sampler_pack_shows_one_trail_in_the_page_and_its_schema() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\nbreadcrumbs = true\n",
        &[],
    );
    let post = page(&sampler, "what-a-sitemap-does/index.html");
    assert!(
        post.contains(
            "<li><a href=\"/blog/category/technical-seo/\">Technical SEO</a></li>\n<li><a href=\"/blog/what-a-sitemap-does/\" aria-current=\"page\">What a sitemap does, and what it does not</a></li>"
        ),
        "{post}"
    );
    assert!(
        post.contains("{\"@type\":\"ListItem\",\"position\":2,\"name\":\"Technical SEO\",\"item\":\"https://www.example.com/blog/category/technical-seo/\"}"),
        "{post}"
    );
    let (_temp, classic) = built("", &[]);
    let post = page(&classic, "what-a-sitemap-does/index.html");
    assert!(!post.contains("class=\"breadcrumbs\""), "{post}");
    assert!(post.contains("\"name\":\"Technical SEO\""), "{post}");
}

#[test]
fn a_post_its_cards_and_the_home_page_name_its_category() {
    let (_temp, out) = built("", &[]);
    let post = page(&out, "what-a-sitemap-does/index.html");
    assert!(
        post.contains(
            "<p>Filed under <a href=\"/blog/category/technical-seo/\">Technical SEO</a></p>"
        ),
        "{post}"
    );
    let home = page(&out, "index.html");
    let (before, _) = home
        .split_once("<a href=\"/blog/what-a-sitemap-does/\">")
        .unwrap();
    let (_, meta) = before.rsplit_once("<p class=\"card-meta\">").unwrap();
    assert!(
        meta.contains("<a href=\"/blog/category/technical-seo/\">Technical SEO</a>"),
        "{meta}"
    );
    let (_, topics) = home.split_once("<nav class=\"topics\"").unwrap();
    let (topics, _) = topics.split_once("</nav>").unwrap();
    let names: Vec<&str> = ["On-page SEO", "Technical SEO", "Content strategy"]
        .into_iter()
        .filter(|name| topics.contains(&format!(">{name}</a>")))
        .collect();
    assert_eq!(names.len(), 3, "{topics}");
    let at = |name: &str| topics.find(name).unwrap();
    assert!(
        at("On-page SEO") < at("Technical SEO") && at("Technical SEO") < at("Content strategy"),
        "in the order of categories.toml: {topics}"
    );
}

#[test]
fn a_german_page_names_the_category_in_german() {
    let (_temp, out) = built("", &[]);
    let home = page(&out, "de/index.html");
    let (_, topics) = home.split_once("<nav class=\"topics\"").unwrap();
    assert!(topics.contains(">Technisches SEO</a>"), "{topics}");
}

#[test]
fn the_sampler_pack_shows_the_dek_from_the_description() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\ndek = true\n",
        &[],
    );
    let post = page(&sampler, "what-a-sitemap-does/index.html");
    assert!(post.contains("</h1>\n<p class=\"dek\">"), "{post}");
    let (_temp, classic) = built("", &[]);
    let post = page(&classic, "what-a-sitemap-does/index.html");
    assert!(!post.contains("class=\"dek\""), "{post}");
}

#[test]
fn the_sampler_pack_shows_ranked_cards_with_their_reading_time() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\ncards = \"ranked\"\ncard_reading_time = true\n",
        &[],
    );
    let home = page(&sampler, "index.html");
    assert!(
        home.contains("<ol class=\"cards cards-ranked\" role=\"list\">"),
        "{home}"
    );
    assert!(
        home.contains("<span class=\"card-reading-time\">"),
        "{home}"
    );
    let (_temp, classic) = built("", &[]);
    let home = page(&classic, "index.html");
    assert!(
        home.contains("<ul class=\"cards\" role=\"list\">"),
        "{home}"
    );
    assert!(!home.contains("card-reading-time"), "{home}");
}

const FEATURED: (&str, &str) = (
    "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
    "+featured: true",
);

#[test]
fn the_sampler_pack_shows_the_featured_post_and_the_picks() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\nfeatured = true\npicks = 3\n",
        &[FEATURED],
    );
    let home = page(&sampler, "index.html");
    assert!(home.contains("<section class=\"featured\""), "{home}");
    assert_eq!(
        home.matches("href=\"/blog/how-to-write-a-title-tag/\"")
            .count(),
        1,
        "{home}"
    );
    let (_temp, classic) = built("", &[FEATURED]);
    let home = page(&classic, "index.html");
    assert!(!home.contains("class=\"featured\""), "{home}");
    assert!(
        home.contains("href=\"/blog/how-to-write-a-title-tag/\""),
        "{home}"
    );
}

#[test]
fn the_sampler_pack_lays_out_related_posts_as_links() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\nrelated = \"links\"\nrelated_count = 3\n",
        &[],
    );
    let post = page(&sampler, "what-a-sitemap-does/index.html");
    let (_, related) = post.split_once("<ul class=\"related-links\">").unwrap();
    let (related, _) = related.split_once("</ul>").unwrap();
    assert_eq!(related.matches("<li>").count(), 3, "{related}");
    let (_temp, classic) = built("", &[]);
    let post = page(&classic, "what-a-sitemap-does/index.html");
    assert!(post.contains("<section class=\"related\">"), "{post}");
    assert!(!post.contains("related-links"), "{post}");
}

#[test]
fn the_sampler_pack_puts_the_contents_in_an_open_box_with_nested_numbers() {
    let (_temp, sampler) = built(
        "\n[template]\nname = \"sampler\"\n[template.options]\ncontents = \"inline\"\ncontents_numbers = \"nested\"\n",
        &[],
    );
    let post = page(&sampler, "what-a-sitemap-does/index.html");
    assert!(post.contains("<nav class=\"toc-box\""), "{post}");
    assert!(post.contains("<ol class=\"toc-nested\">"), "{post}");
    assert!(!post.contains("toc-side"), "{post}");
    let (_temp, classic) = built("", &[]);
    let post = page(&classic, "what-a-sitemap-does/index.html");
    assert!(
        post.contains("<article class=\"post post-with-toc\">"),
        "{post}"
    );
    assert!(post.contains("<nav class=\"toc-side\""), "{post}");
}
