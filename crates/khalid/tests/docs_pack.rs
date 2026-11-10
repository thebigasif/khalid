//! `scripts/docs-bundle.sh` packs Khalid's user docs for
//! <https://serpdad.com/khalid/docs/>: every doc the website lists is in the
//! pack with its title and order, and every link and picture in the pack
//! resolves inside it or is a full address. A link or picture that does not
//! resolve stops the pack.

mod shipped;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::Value;
use tempfile::TempDir;

/// Where the website serves the docs.
const BASE: &str = "/khalid/docs/";

/// The docs R13 publishes before the template guides, with the guides
/// under `deploy`. A guide for each template follows `templates`
/// (`shipped::templates`).
const BEFORE_GUIDES: [&str; 5] = [
    "getting-started",
    "cli",
    "deploy",
    "deploy/cloudflare",
    "templates",
];

/// The docs R13 publishes after the template guides.
const AFTER_GUIDES: [&str; 7] = [
    "routes",
    "seo-settings",
    "seo-tools",
    "reading",
    "newsletters",
    "charts-embeds-tracking",
    "ghost-api",
];

/// The front matter of a page in the pack.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Front {
    title: String,
    order: u32,
    slug: String,
    parent: Option<String>,
    source: String,
}

#[cfg(test)]
fn within(dir: &Path, relative: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(relative);
    path
}

#[cfg(test)]
fn repository() -> PathBuf {
    within(Path::new(env!("CARGO_MANIFEST_DIR")), "../..")
}

/// Runs the script into a new folder, on `root` or on this repository.
#[cfg(test)]
fn pack(root: Option<&Path>) -> (TempDir, Output) {
    let temp = tempfile::tempdir().unwrap();
    let mut command = Command::new("bash");
    command
        .arg(within(&repository(), "scripts/docs-bundle.sh"))
        .arg(within(temp.path(), "docs"));
    if let Some(root) = root {
        command.arg(root);
    }
    let output = command.output().unwrap();
    (temp, output)
}

#[cfg(test)]
fn packed(root: Option<&Path>) -> TempDir {
    let (temp, output) = pack(root);
    assert!(output.status.success(), "{}", stderr(&output));
    temp
}

#[cfg(test)]
fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Splits a page into its front matter and its body.
#[cfg(test)]
fn page(pack: &Path, file: &str) -> (Front, String) {
    let text = fs::read_to_string(within(pack, file)).unwrap();
    let rest = text.strip_prefix("---\n").unwrap();
    let (front, body) = rest.split_once("\n---\n").unwrap();
    (serde_saphyr::from_str(front).unwrap(), body.to_owned())
}

/// The page as the website draws it, with raw HTML kept.
#[cfg(test)]
fn html(body: &str) -> Html {
    let mut options = comrak::Options::default();
    options.extension.table = true;
    options.render.r#unsafe = true;
    Html::parse_fragment(&comrak::markdown_to_html(body, &options))
}

#[cfg(test)]
fn list(pack: &Path) -> Vec<Value> {
    let text = fs::read_to_string(within(pack, "docs.json")).unwrap();
    let json: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json["base"], BASE);
    json["docs"].as_array().unwrap().clone()
}

/// A small repository with one doc, `docs/a.md`, whose body is `body`.
#[cfg(test)]
fn fixture(body: &str) -> TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(within(root.path(), "docs/guide")).unwrap();
    fs::write(within(root.path(), "README.md"), "# Read me\n").unwrap();
    let a = format!("---\ntitle: \"A\"\norder: 10\n---\n\n# A\n\n{body}\n");
    fs::write(within(root.path(), "docs/a.md"), a).unwrap();
    let guide = "---\ntitle: \"Guide\"\norder: 20\n---\n\n# Guide\n";
    fs::write(within(root.path(), "docs/guide.md"), guide).unwrap();
    let b = "---\ntitle: \"B\"\norder: 10\n---\n\n# B\n\nBack to [A](../a.md).\n";
    fs::write(within(root.path(), "docs/guide/b.md"), b).unwrap();
    root
}

#[test]
fn the_pack_lists_every_doc_in_sidebar_order_with_its_title() {
    let temp = packed(None);
    let pack = within(temp.path(), "docs");
    let docs = list(&pack);
    let slugs: Vec<&str> = docs
        .iter()
        .map(|doc| doc["slug"].as_str().unwrap())
        .collect();
    // The guides sit between the other docs, Classic's first and the rest
    // in the order their front matter gives, one for each template.
    let guides: Vec<String> = shipped::templates()
        .iter()
        .map(|name| format!("templates/{name}"))
        .collect();
    let (before, rest) = slugs.split_at(BEFORE_GUIDES.len());
    let (listed, after) = rest.split_at(guides.len());
    assert_eq!(before, BEFORE_GUIDES.as_slice(), "{slugs:?}");
    assert_eq!(after, AFTER_GUIDES.as_slice(), "{slugs:?}");
    assert_eq!(listed.first(), Some(&"templates/classic"), "{slugs:?}");
    let mut sorted = listed.to_vec();
    sorted.sort_unstable();
    assert_eq!(sorted, guides, "{slugs:?}");
    let orders: Vec<u64> = docs
        .iter()
        .skip(BEFORE_GUIDES.len())
        .take(guides.len())
        .map(|doc| doc["order"].as_u64().unwrap())
        .collect();
    assert!(orders.is_sorted(), "{orders:?}");
    for doc in &docs {
        let slug = doc["slug"].as_str().unwrap();
        assert_eq!(doc["url"], format!("{BASE}{slug}/"));
        let (front, body) = page(&pack, doc["file"].as_str().unwrap());
        assert_eq!(front.slug, slug);
        assert_eq!(doc["title"], front.title.as_str());
        assert_eq!(doc["order"], front.order);
        assert_eq!(doc["parent"].as_str(), front.parent.as_deref());
        assert_eq!(front.source, format!("docs/{slug}.md"));
        let page = html(&body);
        let h1s: Vec<String> = page
            .select(&Selector::parse("h1").unwrap())
            .map(|h1| h1.text().collect())
            .collect();
        assert_eq!(h1s, [front.title], "{slug} needs one h1, its title");
    }
    let tarball = within(temp.path(), "khalid-docs.tar.gz");
    let listed = Command::new("tar")
        .arg("-tzf")
        .arg(&tarball)
        .output()
        .unwrap();
    let listed = String::from_utf8(listed.stdout).unwrap();
    for file in ["README.md", "docs.json", "pages/deploy/cloudflare.md"] {
        assert!(listed.lines().any(|line| line == file), "{listed}");
    }
}

#[test]
fn every_link_and_picture_in_the_pack_resolves() {
    let temp = packed(None);
    let pack = within(temp.path(), "docs");
    let docs = list(&pack);
    let slugs: BTreeSet<&str> = docs
        .iter()
        .map(|doc| doc["slug"].as_str().unwrap())
        .collect();
    let links = Selector::parse("a[href]").unwrap();
    let pictures = Selector::parse("img[src]").unwrap();
    for doc in &docs {
        let (_, body) = page(&pack, doc["file"].as_str().unwrap());
        let page = html(&body);
        for link in page.select(&links) {
            let href = link.value().attr("href").unwrap();
            let path = href.split('#').next().unwrap();
            let inside = path
                .strip_prefix(BASE)
                .and_then(|rest| rest.strip_suffix('/'))
                .is_some_and(|slug| slugs.contains(slug));
            let full = href.starts_with("https://") || href.starts_with("mailto:");
            assert!(
                inside || full || href.starts_with('#'),
                "{}: {href}",
                doc["slug"]
            );
        }
        for picture in page.select(&pictures) {
            let src = picture.value().attr("src").unwrap();
            let inside = src
                .strip_prefix(BASE)
                .is_some_and(|rest| within(&pack, &format!("pictures/{rest}")).is_file());
            assert!(
                inside || src.starts_with("https://"),
                "{}: {src}",
                doc["slug"]
            );
        }
    }
}

#[test]
fn links_and_pictures_are_rewritten_to_docs_addresses() {
    let root = fixture(
        "See [B](guide/b.md#part), [the readme](../README.md) and [here](#a).\n\n\
         ![A picture](images/p.svg)\n\n\
         Write `[x](nowhere.md)` for a link.\n\n\
         ```\n[y](nowhere.md)\n```",
    );
    fs::create_dir_all(within(root.path(), "docs/images")).unwrap();
    fs::write(within(root.path(), "docs/images/p.svg"), "<svg/>").unwrap();
    let temp = packed(Some(root.path()));
    let pack = within(temp.path(), "docs");
    let (front, body) = page(&pack, "pages/a.md");
    assert_eq!(front.parent, None);
    assert!(body.contains("[B](/khalid/docs/guide/b/#part)"), "{body}");
    let readme = "https://github.com/thebigasif/khalid/blob/main/README.md";
    assert!(body.contains(&format!("[the readme]({readme})")), "{body}");
    assert!(body.contains("[here](#a)"), "{body}");
    assert!(
        body.contains("![A picture](/khalid/docs/images/p.svg)"),
        "{body}"
    );
    assert!(body.contains("`[x](nowhere.md)`"), "{body}");
    assert!(body.contains("\n[y](nowhere.md)\n"), "{body}");
    assert!(within(&pack, "pictures/images/p.svg").is_file());
    let (front, body) = page(&pack, "pages/guide/b.md");
    assert_eq!(front.parent.as_deref(), Some("guide"));
    assert!(body.contains("[A](/khalid/docs/a/)"), "{body}");
    let docs = list(&pack);
    let slugs: Vec<&str> = docs
        .iter()
        .map(|doc| doc["slug"].as_str().unwrap())
        .collect();
    assert_eq!(slugs, ["a", "guide", "guide/b"]);
    assert_eq!(docs[0]["pictures"][0], "pictures/images/p.svg");
}

#[test]
fn a_link_to_a_missing_doc_stops_the_pack() {
    let root = fixture("See [the missing page](missing.md).");
    let (temp, output) = pack(Some(root.path()));
    assert!(!output.status.success());
    let message = stderr(&output);
    assert!(message.contains("docs/a.md line 8"), "{message}");
    assert!(message.contains("missing.md"), "{message}");
    assert!(!within(temp.path(), "khalid-docs.tar.gz").exists());
}

#[test]
fn a_missing_picture_stops_the_pack() {
    let root = fixture("![A chart](images/chart.png)");
    let (_, output) = pack(Some(root.path()));
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("images/chart.png"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn a_link_from_the_site_root_stops_the_pack() {
    let root = fixture("Go [home](/).");
    let (_, output) = pack(Some(root.path()));
    assert!(!output.status.success());
    assert!(stderr(&output).contains("(/)"), "{}", stderr(&output));
}
