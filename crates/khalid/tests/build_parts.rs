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
