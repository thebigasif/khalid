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
    let out = build(site.path());
    (site, out)
}

/// `site` built into a new folder. Returns that folder.
#[cfg(test)]
fn build(site: &Path) -> tempfile::TempDir {
    let out = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "build", "--out"])
        .arg(out.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    out
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
fn a_site_that_asks_for_avif_offers_it_before_webp() {
    let site = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy(&demo, site.path());
    let settings = at(site.path(), "site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    fs::write(&settings, format!("{text}\n[media]\navif = true\n")).unwrap();
    let out = build(site.path());
    let post = page(out.path(), "fixing-slow-pages/index.html");
    let avif = post.find("<source type=\"image/avif\"");
    let webp = post.find("<source type=\"image/webp\"");
    assert!(avif.is_some() && avif < webp, "{post}");
    let address = post
        .split_once("<source type=\"image/avif\" srcset=\"/blog/")
        .and_then(|(_, rest)| rest.split_once(' '))
        .map(|(file, _)| file.to_owned())
        .unwrap_or_default();
    let extension = Path::new(&address).extension();
    assert_eq!(
        extension.and_then(|e| e.to_str()),
        Some("avif"),
        "{address}"
    );
    assert!(at(out.path(), &address).is_file(), "{address} is written");
}

#[test]
fn a_site_that_does_not_ask_makes_no_avif() {
    let (_site, out) = built();
    let post = page(out.path(), "fixing-slow-pages/index.html");
    assert!(!post.contains("image/avif"), "{post}");
}

#[test]
fn an_svg_lead_image_and_photo_are_drawn_at_their_own_size() {
    let (site, _) = built();
    let post = at(site.path(), "content/en/posts/2026-10-05-fixing-slow-pages");
    let text = fs::read_to_string(at(&post, "index.md")).unwrap();
    let text = text.replace("page-weight.png", "cover.svg");
    fs::write(at(&post, "index.md"), text).unwrap();
    let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1600 900\"><rect width=\"1600\" height=\"900\" fill=\"#e8f0ff\"/></svg>\n";
    fs::write(at(&post, "cover.svg"), svg).unwrap();
    let author = at(site.path(), "content/en/authors/lena-vogt");
    let text = fs::read_to_string(at(&author, "index.md")).unwrap();
    fs::write(
        at(&author, "index.md"),
        text.replace("lena.png", "lena.svg"),
    )
    .unwrap();
    let face = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"96\" height=\"96\"><circle cx=\"48\" cy=\"48\" r=\"48\" fill=\"#c2410c\"/></svg>\n";
    fs::write(at(&author, "lena.svg"), face).unwrap();
    let out = build(site.path());
    let post = page(out.path(), "fixing-slow-pages/index.html");
    let lead = "<img src=\"/blog/fixing-slow-pages/cover.svg\" srcset=\"/blog/fixing-slow-pages/cover.svg 1600w\"";
    assert!(post.contains(lead), "{post}");
    assert!(post.contains("width=\"1600\" height=\"900\""), "{post}");
    assert!(post.contains("fetchpriority=\"high\""), "{post}");
    // A drawing is offered as itself, with no WebP copy beside it.
    let picture = post
        .split_once(lead)
        .unwrap()
        .0
        .rsplit_once("<picture>")
        .unwrap()
        .1;
    assert!(!picture.contains("<source"), "{picture}");
    // No network draws an SVG, so the page shares its generated card.
    assert!(!post.contains("content=\"https://www.example.com/blog/fixing-slow-pages/cover.svg"));
    let photo = "<img class=\"avatar\" src=\"/blog/authors/lena-vogt/lena.svg\"";
    assert!(post.contains(photo), "{post}");
    let home = page(out.path(), "index.html");
    let card = card_of(&home, "/blog/fixing-slow-pages/");
    assert!(card.contains(lead) && card.contains(photo), "{card}");
}

#[test]
fn a_photo_from_the_media_library_shows_like_one_beside_the_author() {
    let site = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy(&demo, site.path());
    // The admin's photo picker saves the address the library serves.
    let key = "3f2a9c1e0b57d0aa";
    let library = at(site.path(), &format!("media/{key}"));
    fs::create_dir_all(&library).unwrap();
    let chart = "content/en/posts/2026-10-05-fixing-slow-pages/page-weight.png";
    fs::copy(at(site.path(), chart), at(&library, "lena.png")).unwrap();
    let file = at(site.path(), "content/en/authors/lena-vogt.md");
    let text = fs::read_to_string(&file).unwrap();
    let photo = format!("---\nphoto: /blog/media/library/{key}/lena.png\n");
    fs::write(&file, text.replacen("---\n", &photo, 1)).unwrap();
    let out = build(site.path());
    let author = page(out.path(), "authors/lena-vogt/index.html");
    assert!(author.contains("alt=\"Lena Vogt\""), "{author}");
    assert!(author.contains("fetchpriority=\"high\""), "{author}");
    assert!(author.contains("image/webp"), "{author}");
    let post = page(out.path(), "fixing-slow-pages/index.html");
    let byline = post.split_once("rel=\"author\"").unwrap().0;
    let avatar = byline.rsplit_once("<img class=\"avatar\"").unwrap().1;
    assert!(avatar.contains("sizes=\"2.75rem\""), "{avatar}");
    // The original is published at the address the photo names.
    assert!(at(out.path(), &format!("media/library/{key}/lena.png")).is_file());
}

#[test]
fn a_library_photo_that_is_not_on_disk_is_left_out_without_failing() {
    let site = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy(&demo, site.path());
    let file = at(site.path(), "content/en/authors/lena-vogt.md");
    let text = fs::read_to_string(&file).unwrap();
    let photo = "---\nphoto: /blog/media/library/3f2a9c1e0b57d0aa/gone.png\n";
    fs::write(&file, text.replacen("---\n", photo, 1)).unwrap();
    let out = build(site.path());
    let author = page(out.path(), "authors/lena-vogt/index.html");
    assert!(!author.contains("gone.png"), "{author}");
}

#[test]
fn a_topic_tab_card_shows_the_posts_picture_like_every_other_card() {
    let (site, _) = built();
    let slow = at(site.path(), "content/en/posts/2026-10-05-fixing-slow-pages");
    let text = fs::read_to_string(at(&slow, "index.md")).unwrap();
    fs::write(
        at(&slow, "index.md"),
        text.replace("page-weight.png", "cover.png"),
    )
    .unwrap();
    fs::copy(at(&slow, "page-weight.png"), at(&slow, "cover.png")).unwrap();
    let posts = at(site.path(), "content/en/posts");
    let plain = at(&posts, "2026-10-12-planning-a-topic-cluster.md");
    let text = fs::read_to_string(&plain).unwrap();
    let image = "---\nimage:\n  src: cover.svg\n  alt: A map of a topic cluster\n";
    fs::remove_file(&plain).unwrap();
    let cluster = at(&posts, "2026-10-12-planning-a-topic-cluster");
    fs::create_dir_all(&cluster).unwrap();
    fs::write(at(&cluster, "index.md"), text.replacen("---\n", image, 1)).unwrap();
    let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1600 900\"><rect width=\"1600\" height=\"900\" fill=\"#e8f0ff\"/></svg>\n";
    fs::write(at(&cluster, "cover.svg"), svg).unwrap();
    let settings = at(site.path(), "site.toml");
    let text = fs::read_to_string(&settings).unwrap();
    let keystone = "\n[template]\nname = \"keystone\"\n\n[template.options]\ntopic_tabs = true\n";
    fs::write(&settings, format!("{text}{keystone}")).unwrap();
    let out = build(site.path());
    let home = page(out.path(), "index.html");
    let (rest, tabs) = home.split_once("<section class=\"topic-tabs\"").unwrap();
    let tabs = tabs.split_once("</section>\n</div>").unwrap().0;
    // A PNG is offered in sizes made from it, an SVG as itself.
    for (href, mark) in [
        (
            "/blog/fixing-slow-pages/",
            "alt=\"A bar chart of page weight by part\"",
        ),
        (
            "/blog/planning-a-topic-cluster/",
            "/blog/planning-a-topic-cluster/cover.svg",
        ),
    ] {
        let recent = picture_of(card_of(rest, href));
        assert!(recent.contains(mark), "{recent}");
        let tab = card_of(tabs, href);
        assert!(tab.contains(recent), "the tab card shows {recent}: {tab}");
    }
}

/// The `<picture>` on `card`.
#[cfg(test)]
fn picture_of(card: &str) -> &str {
    let from = card.split_once("<picture>").unwrap().1;
    from.split_once("</picture>").unwrap().0
}

/// The card on `html` that links `href`, up to its link.
#[cfg(test)]
fn card_of<'a>(html: &'a str, href: &str) -> &'a str {
    let before = html.split_once(&format!("href=\"{href}\"")).unwrap().0;
    before.rsplit_once("<article").unwrap().1
}
