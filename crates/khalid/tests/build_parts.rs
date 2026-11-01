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
