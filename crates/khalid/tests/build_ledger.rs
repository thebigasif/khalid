//! The Ledger template end to end (templates spec 3.1): `khalid build` of
//! the demo site with `name = "ledger"`, a snapshot of its front page, a
//! post and an author page, and the parts each must show.

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

/// Builds a copy of the demo site with `table` added to its `site.toml`,
/// and expects success.
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

const LEDGER: &str = "\n[template]\nname = \"ledger\"\n[template.options]\n\
    tagline = \"Plain advice on search for small teams\"\n\
    stats = [\"5 field-tested guides\", \"Two writers\"]\n";

#[test]
fn ledger_front_page() {
    let (_temp, out) = built(LEDGER);
    let home = body(&out, "index.html");
    assert!(home.contains("<p class=\"tagline\">Plain advice on search for small teams</p>"));
    assert!(home.contains("<li>5 field-tested guides</li>"), "{home}");
    assert!(
        home.contains("<nav class=\"pills ledger-topics\" aria-label=\"Topics\">"),
        "topic rail: {home}"
    );
    assert!(
        home.contains("<section class=\"home-section home-latest\""),
        "lead list: {home}"
    );
    assert!(home.contains("<ol class=\"ledger-list\""), "{home}");
    insta::assert_snapshot!(home);
}

/// Adds `count` more English posts to the demo copy at `site`, one a day
/// in August, so the front page lists more than ten.
#[cfg(test)]
fn more_posts(site: &Path, count: u32) {
    for day in 1..=count {
        let post = format!(
            "---\ntitle: Field note number {day} on search\nslug: field-note-{day}\n\
             date: 2026-08-{day:02}\nauthor: asif\ncategory: on-page-seo\n\
             description: A short field note on search for small teams, number {day} of a series.\n---\n\n\
             This field note covers one small fix a small team can make to its pages this week. \
             Start with [how to write a title tag](/blog/how-to-write-a-title-tag/).\n"
        );
        fs::write(
            at(
                site,
                &format!("content/en/posts/2026-08-{day:02}-field-note-{day}.md"),
            ),
            post,
        )
        .unwrap();
    }
}

#[test]
fn ledger_draws_the_rows_after_the_tenth_as_one_line_each() {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    more_posts(temp.path(), 7);
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(LEDGER);
    fs::write(&settings, toml).unwrap();
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let home = body(&out, "index.html");
    let (_, list) = home.split_once("<ol class=\"ledger-list\"").unwrap();
    let (list, _) = list.split_once("</ol>").unwrap();
    assert_eq!(
        list.matches("<article class=\"ledger-entry\">").count(),
        10,
        "{list}"
    );
    assert_eq!(
        list.matches("<li class=\"ledger-line\">").count(),
        2,
        "{list}"
    );
    let (_, short) = list.split_once("<li class=\"ledger-line\">").unwrap();
    assert!(
        !short.contains("class=\"pill\""),
        "no topic on a short row: {short}"
    );
    assert!(
        !short.contains("card-author"),
        "no author on a short row: {short}"
    );
    assert!(
        short.contains("<time datetime="),
        "a short row keeps its date: {short}"
    );
}

#[test]
fn ledger_post_page() {
    let (_temp, out) = built(LEDGER);
    let post = body(&out, "what-a-sitemap-does/index.html");
    let (before_title, _) = post.split_once("<h1>").unwrap();
    assert!(
        before_title
            .contains("<a class=\"pill\" href=\"/blog/category/technical-seo/\">Technical SEO</a>"),
        "the category pill sits above the title: {post}"
    );
    assert!(
        post.contains("<nav class=\"toc-box\""),
        "inline contents: {post}"
    );
    assert!(
        post.contains("<ol class=\"toc-plain\">"),
        "unnumbered: {post}"
    );
    assert!(post.contains("<section class=\"author-box\">"), "{post}");
    assert!(
        post.contains("<ul class=\"cards cards-text\""),
        "related list: {post}"
    );
    assert!(!post.contains("class=\"share\""), "no share row: {post}");
    insta::assert_snapshot!(post);
}

#[test]
fn ledger_shows_the_updated_date_in_place_of_the_published_one() {
    let (_temp, out) = built(LEDGER);
    let post = body(&out, "why-internal-links-matter/index.html");
    assert!(
        post.contains("Updated <time datetime=\"2026-10-20\">"),
        "{post}"
    );
    assert!(!post.contains("Published <time"), "{post}");
}

#[test]
fn ledger_ends_the_front_page_with_the_newsletter_band_and_a_post_with_the_form() {
    let mail = "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\nhost = \"127.0.0.1\"\n\
                port = 2525\nsecurity = \"none\"\nprivate_host = true\n";
    let (_temp, out) = built(&format!("{mail}{LEDGER}"));
    let home = body(&out, "index.html");
    let (_, band) = home.split_once("<div class=\"band\">").unwrap();
    assert!(
        band.contains("<section class=\"newsletter-slot\""),
        "{home}"
    );
    let (_, after) = band.split_once("</div>").unwrap();
    assert!(
        after.trim_start().starts_with("</main>"),
        "the band comes last: {home}"
    );
    let post = body(&out, "what-a-sitemap-does/index.html");
    let related = post.find("<section class=\"related\">").unwrap();
    let form = post.find("<section class=\"newsletter-slot\"").unwrap();
    assert!(related < form, "the form follows the related posts: {post}");
}

#[test]
fn ledger_author_page() {
    let (_temp, out) = built(LEDGER);
    let author = body(&out, "authors/lena-vogt/index.html");
    assert!(
        author.contains("<ul class=\"cards cards-text\""),
        "{author}"
    );
    insta::assert_snapshot!(author);
}
