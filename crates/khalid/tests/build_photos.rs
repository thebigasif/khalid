//! Author photos and lead images follow their author and post to bylines,
//! cards and author pages (plan section 4.12), checked through `khalid build`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// `rel` under `base`.
#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Copies the folder `from` into `to`, leaving out derived `data/`.
#[cfg(test)]
fn copy(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        let mut target = to.to_path_buf();
        target.push(&name);
        if entry.path().is_dir() {
            if name != "data" {
                copy(&entry.path(), &target);
            }
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// The demo site, with Lena Vogt as a folder author whose photo is beside
/// her file, built into a new folder. Returns that folder.
#[cfg(test)]
fn built() -> (tempfile::TempDir, tempfile::TempDir) {
    let site = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy(&demo, site.path());
    let authors = at(site.path(), "content/en/authors");
    let text = fs::read_to_string(at(&authors, "lena-vogt.md")).unwrap();
    let text = text.replacen("---\n", "---\nphoto: lena.png\n", 1);
    fs::remove_file(at(&authors, "lena-vogt.md")).unwrap();
    let folder = at(&authors, "lena-vogt");
    fs::create_dir_all(&folder).unwrap();
    fs::write(at(&folder, "index.md"), text).unwrap();
    let chart = "content/en/posts/2026-10-05-fixing-slow-pages/page-weight.png";
    fs::copy(at(site.path(), chart), at(&folder, "lena.png")).unwrap();
    // The sitemap post becomes a folder post whose lead image is a drawing.
    let posts = at(site.path(), "content/en/posts");
    let text = fs::read_to_string(at(&posts, "2026-09-28-what-a-sitemap-does.md")).unwrap();
    let text = text.replacen(
        "---\n",
        "---\nimage:\n  src: map.svg\n  alt: A map of pages\n",
        1,
    );
    fs::remove_file(at(&posts, "2026-09-28-what-a-sitemap-does.md")).unwrap();
    let folder = at(&posts, "2026-09-28-what-a-sitemap-does");
    fs::create_dir_all(&folder).unwrap();
    fs::write(at(&folder, "index.md"), text).unwrap();
    let drawing = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 640 300\"><rect width=\"640\" height=\"300\"/></svg>\n";
    fs::write(at(&folder, "map.svg"), drawing).unwrap();
    let out = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "build", "--out"])
        .arg(out.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    (site, out)
}

/// The page at `rel` in the output folder.
#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

#[test]
fn an_authors_photo_and_a_posts_lead_image_show_wherever_they_go() {
    let (_site, out) = built();
    let author = page(out.path(), "authors/lena-vogt/index.html");
    assert!(author.contains("alt=\"Lena Vogt\""), "{author}");
    assert!(author.contains("fetchpriority=\"high\""), "{author}");
    let post = page(out.path(), "fixing-slow-pages/index.html");
    let byline = post.split_once("rel=\"author\"").unwrap().0;
    let avatar = byline.rsplit_once("<img class=\"avatar\"").unwrap().1;
    assert!(avatar.contains("sizes=\"2.75rem\""), "{avatar}");
    assert!(avatar.contains("loading=\"lazy\""), "{avatar}");
    let chart = "alt=\"A bar chart of page weight by part\"";
    let card = card_of(&author, "/blog/fixing-slow-pages/");
    assert!(card.contains(chart), "{card}");
    assert!(card.contains("loading=\"lazy\""), "{card}");
    assert!(!card.contains("<figcaption>"), "{card}");
    let home = page(out.path(), "index.html");
    let card = card_of(&home, "/blog/fixing-slow-pages/");
    assert!(card.contains(chart), "{card}");
    assert!(card.contains("loading=\"lazy\""), "{card}");
}

#[test]
fn a_drawing_as_lead_image_shows_at_its_own_size_on_the_post_and_its_cards() {
    let (_site, out) = built();
    let drawing = "<img src=\"/blog/what-a-sitemap-does/map.svg\" srcset=\"/blog/what-a-sitemap-does/map.svg 640w\"";
    let post = page(out.path(), "what-a-sitemap-does/index.html");
    let lead = post.split_once("<header class=\"post-header\">").unwrap().1;
    let lead = lead.split_once("<figure class=\"image\">").unwrap().1;
    let lead = lead.split_once("</figure>").unwrap().0;
    assert!(lead.contains(drawing), "{lead}");
    assert!(
        lead.contains("width=\"640\" height=\"300\" alt=\"A map of pages\" fetchpriority=\"high\""),
        "{lead}"
    );
    assert!(!lead.contains("image/webp"), "{lead}");
    assert!(
        !post.contains(
            "og:image\" content=\"https://www.example.com/blog/what-a-sitemap-does/map.svg"
        ),
        "a drawing is not a share image"
    );
    let home = page(out.path(), "index.html");
    let card = card_of(&home, "/blog/what-a-sitemap-does/");
    assert!(
        card.contains(drawing) && card.contains("loading=\"lazy\""),
        "{card}"
    );
}

/// The card on `html` that links `href`, up to its link.
#[cfg(test)]
fn card_of<'a>(html: &'a str, href: &str) -> &'a str {
    let before = html.split_once(&format!("href=\"{href}\"")).unwrap().0;
    before.rsplit_once("<article").unwrap().1
}
