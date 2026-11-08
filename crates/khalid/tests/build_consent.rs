//! `khalid build` of a static site asks for consent with a small script
//! (privacy plan U6, KTD7, KTD9): owner code that waits is written as
//! plain text the script turns into code once a reader agrees, the banner
//! is hidden until the script shows it, and with scripts off nothing
//! optional ever loads.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if name == "data" {
            continue;
        }
        let target = at(into, name.to_str().unwrap());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

/// The owner code of the tests: a tag manager in the head and a chat
/// widget at the end of the body.
const CODE: &str = "[code]\nhead = \"<script src='https://tags.example.com/t.js'></script><link rel='stylesheet' href='https://tags.example.com/t.css'>\"\nbody = \"<script>chat()</script>\"\n";

/// An embed of a `YouTube` video at the end of the sitemap post.
const EMBED: &str = "\n::embed{src=\"https://www.youtube.com/watch?v=xfjumh8ySRY\" title=\"New field insights for debugging INP\"}\n";

/// The sitemap post's file in the demo site.
const POST: &str = "content/en/posts/2026-09-28-what-a-sitemap-does.md";

/// The demo site as a static site, with `settings` after its `site.toml`
/// and `more` after the sitemap post, built into `public/`.
#[cfg(test)]
fn build(settings: &str, more: &str) -> (TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let site = at(temp.path(), "site");
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, &site);
    let toml = at(&site, "site.toml");
    let text = fs::read_to_string(&toml).unwrap();
    let text = text.replace("mode = \"server\"", "mode = \"static\"");
    fs::write(&toml, format!("{text}{settings}")).unwrap();
    let post = at(&site, POST);
    let text = fs::read_to_string(&post).unwrap();
    fs::write(&post, format!("{text}{more}")).unwrap();
    let out = at(temp.path(), "public");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(&site)
        .arg("build")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    (temp, out)
}

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap()
}

/// The names of the files in `out/assets/` that start with `consent.`.
#[cfg(test)]
fn consent_files(out: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(at(out, "assets"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.starts_with("consent."))
        .collect();
    names.sort();
    names
}

#[test]
fn a_static_build_holds_code_that_waits_as_plain_text() {
    let (_temp, out) = build(CODE, "");
    let html = page(&out, "what-a-sitemap-does/index.html");
    assert!(
        html.contains("<script type=\"text/plain\" data-khalid-consent=\"marketing\" src=\"https://tags.example.com/t.js\"></script>"),
        "{html}"
    );
    assert!(
        html.contains(
            "<script type=\"text/plain\" data-khalid-consent=\"marketing\">chat()</script>"
        ),
        "{html}"
    );
    let template = "<template data-khalid-consent=\"marketing\"><link rel=\"stylesheet\" href=\"https://tags.example.com/t.css\"></template>";
    assert!(html.contains(template), "{html}");
    let outside = html.replace(template, "");
    for live in [
        "<script src=\"https://tags",
        "<script>chat()",
        "<link rel=\"stylesheet\" href=\"https://tags",
    ] {
        assert!(!outside.contains(live), "{live} in {html}");
    }
    let banner = html.split("data-khalid-banner").nth(1).unwrap();
    assert!(banner.starts_with(" hidden>"), "{banner}");
    for part in [
        "value=\"accept-all\">Accept all</button>",
        "value=\"reject-all\">Reject all</button>",
        "href=\"/blog/privacy-choices/\">Choose</a>",
    ] {
        assert!(banner.contains(part), "{part} in {banner}");
    }
    let tag = html.split("data-khalid-rev=").nth(1).unwrap();
    assert!(
        html.contains("<script src=\"/blog/assets/consent."),
        "{html}"
    );
    assert!(tag.contains(" data-khalid-days=\"180\""), "{tag}");
    assert!(tag.contains(" data-khalid-path=\"/blog/\" defer"), "{tag}");
}

#[test]
fn the_consent_script_is_small_and_only_on_sites_that_need_it() {
    let (_temp, out) = build(CODE, "");
    let files = consent_files(&out);
    assert_eq!(files.len(), 3, "{files:?}");
    let brotli = files
        .iter()
        .find(|name| name.rsplit('.').next() == Some("br"))
        .unwrap();
    let size = fs::metadata(at(&out, &format!("assets/{brotli}")))
        .unwrap()
        .len();
    assert!(size <= 2048, "consent.js is {size} bytes compressed");
    let (_temp, plain) = build("", "");
    assert_eq!(consent_files(&plain), Vec::<String>::new());
    let html = page(&plain, "what-a-sitemap-does/index.html");
    assert!(!html.contains("consent."), "{html}");
    assert!(!html.contains("data-khalid-banner"), "{html}");
}

#[test]
fn a_static_post_with_an_embed_loads_the_script_and_shows_no_banner() {
    let (_temp, out) = build("", EMBED);
    let html = page(&out, "what-a-sitemap-does/index.html");
    assert!(
        html.contains("<script src=\"/blog/assets/consent."),
        "{html}"
    );
    assert!(
        html.contains("href=\"https://www.youtube.com/watch?v=xfjumh8ySRY\""),
        "{html}"
    );
    assert!(!html.contains("data-khalid-banner"), "{html}");
    let home = page(&out, "index.html");
    assert!(!home.contains("consent."), "{home}");
}

#[test]
fn the_choices_page_says_nothing_optional_loads_without_scripts() {
    let (_temp, out) = build(CODE, "");
    let html = page(&out, "privacy-choices/index.html");
    assert!(
        html.contains("<meta name=\"robots\" content=\"noindex"),
        "{html}"
    );
    let note = html.split("<noscript><p>").nth(1).unwrap();
    assert!(
        note.starts_with("Your browser runs no scripts, so this blog loads nothing optional."),
        "{note}"
    );
    let form = html.split("data-khalid-choices").nth(1).unwrap();
    assert!(form.starts_with(" hidden>"), "{form}");
    assert!(!html.contains("<form"), "the build locks forms: {html}");
    for part in [
        "name=\"marketing\" value=\"on\">",
        "name=\"external-media\" value=\"on\">",
        "type=\"button\" name=\"choice\" value=\"save\">Save my choices</button>",
    ] {
        assert!(form.contains(part), "{part} in {form}");
    }
    assert!(
        html.contains("<script src=\"/blog/assets/consent."),
        "{html}"
    );
    let german = page(&out, "de/privacy-choices/index.html");
    assert!(
        german.contains("Meine Auswahl speichern</button>"),
        "{german}"
    );
    let mut sitemaps = vec![at(&out, "sitemap.xml")];
    for entry in fs::read_dir(at(&out, "sitemaps")).unwrap() {
        sitemaps.push(entry.unwrap().path());
    }
    for file in sitemaps {
        let sitemap = fs::read_to_string(&file).unwrap();
        assert!(!sitemap.contains("privacy-choices"), "{}", file.display());
    }
}

/// Two entries of `[[code]]`: analytics from a named vendor, and a script
/// that tracks no one.
const ENTRIES: &str = "[[code]]\nhead = \"<script src='https://stats.example.net/a.js'></script>\"\ncategory = \"analytics\"\nvendor = \"Example Stats\"\n\n[[code]]\nbody = \"<script>quiet()</script>\"\ncategory = \"no-tracking\"\n";

#[test]
fn each_entry_waits_for_its_own_category_and_no_tracking_code_runs() {
    let (_temp, out) = build(&format!("[privacy]\nlifetime_months = 3\n\n{ENTRIES}"), "");
    let html = page(&out, "what-a-sitemap-does/index.html");
    assert!(
        html.contains("<script type=\"text/plain\" data-khalid-consent=\"analytics\" src=\"https://stats.example.net/a.js\"></script>"),
        "{html}"
    );
    assert!(html.contains("<script>quiet()</script>"), "{html}");
    assert!(html.contains(" data-khalid-days=\"90\""), "{html}");
}

#[test]
fn a_static_site_that_asks_nothing_says_so_on_its_choices_page() {
    let (_temp, out) = build("", "");
    let html = page(&out, "privacy-choices/index.html");
    assert!(html.contains("so there is nothing to choose."), "{html}");
    assert!(
        !html.contains("<fieldset>") && !html.contains("consent."),
        "{html}"
    );
}

#[test]
fn an_outside_consent_tool_gets_the_code_unheld_and_no_banner() {
    let (_temp, out) = build(
        &format!("[privacy]\nexternal_tool = \"Cookiebot\"\n\n{CODE}"),
        "",
    );
    let html = page(&out, "what-a-sitemap-does/index.html");
    assert!(html.contains("<script>chat()</script>"), "{html}");
    assert!(!html.contains("data-khalid-banner"), "{html}");
    assert!(!html.contains("consent."), "{html}");
}

#[test]
fn the_build_says_global_privacy_control_is_honoured() {
    let (_temp, out) = build("", "");
    let file = page(&out, "_root/.well-known/gpc.json");
    let json: serde_json::Value = serde_json::from_str(&file).unwrap();
    assert_eq!(json["gpc"], true, "{file}");
}
