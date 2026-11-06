//! The Hub template end to end (docs/templates/hub.md): `khalid build` of
//! the demo site with `[template] name = "hub"`, a call to action, three
//! links and a creator with a drawn photo, then the header and main part
//! of its front page and a post, the link stack in its order, and the
//! main menu that fills the stack when no links are set.

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

/// Hub with a call to action, three links (one with a note and one with
/// no address, which is left out) and Lena as the creator.
const SETTINGS: &str = "\n[template]\nname = \"hub\"\n[template.options]\ncreator = \"lena-vogt\"\ntagline = [\"Notes on search for small teams.\"]\nhero_stats = [\"12k | readers\"]\nlinks = [\"Watch the videos | https://www.example.com/videos | Every Friday\", \"Read the newsletter | https://www.example.com/letter\", \"No address here\", \"About | /blog/about/\"]\n[template.cta]\nlabel = \"Book a call\"\nhref = \"https://www.example.com/call/\"\nnote = \"Thirty minutes, free\"\n";

/// The same site with no links and a main menu of two links, which
/// fill the stack.
const NO_LINKS: &str = "\n[template]\nname = \"hub\"\n\n[[nav.primary]]\nlabel = \"Guides\"\nhref = \"/blog/archive/\"\n\n[[nav.primary]]\nlabel = \"About\"\nhref = \"/blog/about/\"\n";

/// A drawn mark for Lena, beside her author file.
const MARK: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 160 160\" width=\"160\" height=\"160\"><rect width=\"160\" height=\"160\" fill=\"#eee\"/><circle cx=\"80\" cy=\"70\" r=\"30\" fill=\"#333\"/></svg>\n";

/// The demo site built with Hub and `settings`, with a photo for Lena.
#[cfg(test)]
fn built(settings: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let site = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&site).unwrap();
    toml.push_str(settings);
    fs::write(&site, toml).unwrap();
    let lena = at(temp.path(), "content/en/authors/lena-vogt.md");
    let text = fs::read_to_string(&lena).unwrap();
    fs::write(&lena, text.replacen('\n', "\nphoto: lena-vogt.svg\n", 1)).unwrap();
    fs::write(at(temp.path(), "content/en/authors/lena-vogt.svg"), MARK).unwrap();
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

/// The page at `rel`, checked for one `h1` and unique ids, cut to its
/// header and main part: what a template changes.
#[cfg(test)]
fn shown(out: &Path, rel: &str) -> String {
    let page = fs::read_to_string(at(out, rel)).unwrap();
    let html = scraper::Html::parse_document(&page);
    assert!(html.errors.is_empty(), "{rel}: {:?}", html.errors);
    let select = |css: &str| {
        let selector = scraper::Selector::parse(css).unwrap();
        html.select(&selector)
            .filter_map(|element| element.value().attr("id").or(Some("")))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(select("h1").len(), 1, "{rel}: one h1");
    let mut ids = select("[id]");
    let count = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), count, "{rel}: unique ids");
    let start = page.find("<header class=\"site-header\">").unwrap();
    let end = page.find("</main>").unwrap();
    page.get(start..end).unwrap().to_owned()
}

/// The texts of every element `css` selects in `html`.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>())
        .collect()
}

/// The `href` of every element `css` selects in `html`.
#[cfg(test)]
fn hrefs(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .map(str::to_owned)
        .collect()
}

#[test]
fn hub_front_page() {
    let (_temp, out) = built(SETTINGS);
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The call to action leads the stack, then each link in the order
/// written. A line with no address is left out, and a third part shows
/// as the note under the label.
#[test]
fn the_link_stack_leads_with_the_call_to_action_then_the_links() {
    let (_temp, out) = built(SETTINGS);
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".link-stack .link-label"),
        [
            "Book a call",
            "Watch the videos",
            "Read the newsletter",
            "About"
        ]
    );
    assert_eq!(
        texts(&page, ".link-stack .link-note"),
        ["Thirty minutes, free", "Every Friday"]
    );
    assert_eq!(
        hrefs(&page, ".link-tile-lead"),
        ["https://www.example.com/call/"]
    );
    assert_eq!(texts(&page, ".profile-name").len(), 1);
    assert!(page.contains("Notes on search for small teams."));
    assert_eq!(texts(&page, ".hero-stats dd"), ["12k"]);
}

/// The creator the option names leads the front page with their photo,
/// though the newest post is someone else's.
#[test]
fn the_creator_option_picks_the_photo_at_the_top() {
    let (_temp, out) = built(SETTINGS);
    let page = shown(&out, "index.html");
    let photos = scraper::Html::parse_fragment(&page);
    let selector = scraper::Selector::parse(".profile-photo img").unwrap();
    assert_eq!(photos.select(&selector).count(), 1, "{page}");
}

/// Without links, the main menu fills the stack, and a post has no
/// links section at its end.
#[test]
fn the_main_menu_fills_the_stack_when_no_links_are_set() {
    let (_temp, out) = built(NO_LINKS);
    let page = shown(&out, "index.html");
    assert_eq!(texts(&page, ".link-stack .link-label"), ["Guides", "About"]);
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert!(texts(&post, ".post-links").is_empty());
}

#[test]
fn hub_post() {
    let (_temp, out) = built(SETTINGS);
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A post ends with the creator's links again, without the call to
/// action, under a heading with the site's name.
#[test]
fn a_post_ends_with_the_links_again() {
    let (_temp, out) = built(SETTINGS);
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&post, ".post-links h2"),
        ["More from Field Notes on SEO"]
    );
    assert_eq!(
        texts(&post, ".post-links .link-label"),
        ["Watch the videos", "Read the newsletter", "About"]
    );
    assert!(texts(&post, ".post-links .link-tile-lead").is_empty());
}
