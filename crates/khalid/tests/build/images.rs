//! Images in `khalid build`: a folder post's image is copied beside its
//! page, the lead image loads first and is shared, and every image on every
//! page has its five attributes.

use std::fs;
use std::path::{Path, PathBuf};

use super::{at, demo_site, khalid, page, path, text};

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
