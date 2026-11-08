//! Every site gets a privacy page at `<mount>privacy/`, written from its
//! settings and features, unless the owner writes their own (privacy plan
//! U9, R20 and R21).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const STATIC_SITE: &str = r#"name = "Field Notes"
address = "https://www.example.com"
mount = "/blog/"
mode = "static"
default_language = "en"
languages = ["en", "de"]
"#;

/// The same site run by Khalid, so comments and the newsletter work.
const SERVER_SITE: &str = r#"name = "Field Notes"
address = "https://www.example.com"
mount = "/blog/"
mode = "server"
default_language = "en"
languages = ["en"]
"#;

/// Code that waits for consent, which counts as Marketing.
const MARKETING_CODE: &str = r#"
[code]
head = "<script src=\"https://ads.example.net/tag.js\"></script>"
consent = "ask"
"#;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Writes `text` to `rel` inside `site`, making its folders first.
#[cfg(test)]
fn put(site: &Path, rel: &str, text: &str) {
    let file = at(site, rel);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, text).unwrap();
}

/// A site with `settings` and one page, which no publish gate holds back,
/// whose body ends with `body`.
#[cfg(test)]
fn site(settings: &str, body: &str) -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    put(temp.path(), "site.toml", settings);
    put(
        temp.path(),
        "content/en/pages/about.md",
        &format!(
            "---\ntitle: About\nslug: about\n---\n\n\
             A page that links to [the home page](/blog/).\n\n{body}\n"
        ),
    );
    temp
}

#[cfg(test)]
fn khalid(site: &TempDir, command: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site.path())
        .args(command)
        .output()
        .unwrap()
}

/// Builds `site` and returns what the build said.
#[cfg(test)]
fn build(site: &TempDir) -> String {
    let output = khalid(site, &["build"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.status.code(), Some(0), "{said}");
    said
}

/// The main area of the built page at `rel` inside `public/`.
#[cfg(test)]
fn main_of(site: &TempDir, rel: &str) -> String {
    let page = fs::read_to_string(at(site.path(), &format!("public/{rel}"))).unwrap();
    let (_, main) = page.split_once("<main").unwrap();
    main.split_once("</main>").unwrap().0.to_owned()
}

#[test]
fn a_static_site_gets_a_privacy_page_in_each_language() {
    let site = site(STATIC_SITE, "");
    build(&site);
    let page = main_of(&site, "privacy/index.html");
    assert!(page.contains("<h1>Privacy</h1>"), "{page}");
    assert!(page.contains("Field Notes runs this site"), "{page}");
    assert!(page.contains("<code>khalid-consent</code>"), "{page}");
    assert!(page.contains("This site runs no code from other companies."));
    assert!(page.contains(r##"<a href="#privacy-rights">Your rights</a>"##));
    assert!(page.contains("Global Privacy Control"), "{page}");
    assert!(
        !page.contains("If you comment"),
        "a static site has no comments"
    );
    assert!(!page.contains("If you subscribe"), "{page}");
    let german = main_of(&site, "de/privacy/index.html");
    assert!(german.contains("<h1>Datenschutz</h1>"), "{german}");
}

#[test]
fn an_embed_names_its_player_and_no_other() {
    let site = site(
        STATIC_SITE,
        "::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"A talk\"}",
    );
    let said = build(&site);
    let page = main_of(&site, "privacy/index.html");
    assert!(
        page.contains("A video or audio player from YouTube loads"),
        "{said}{page}"
    );
    assert!(!page.contains("Vimeo"), "{page}");
}

#[test]
fn a_site_with_comments_and_a_newsletter_lists_both_with_their_data_and_retention() {
    let site = site(
        SERVER_SITE,
        "::newsletter{label=\"Your email\" button=\"Join\"}",
    );
    build(&site);
    let page = main_of(&site, "privacy/index.html");
    for said in [
        "If you comment, this site keeps your name and your comment",
        "Comments stay until you ask for them to be removed",
        "If you subscribe, this site keeps your email address",
        "Your email address stays until you unsubscribe.",
        "<code>khalid_reader</code>",
    ] {
        assert!(page.contains(said), "{said}\n{page}");
    }
}

#[test]
fn adding_a_marketing_vendor_lists_it_and_moves_the_last_changed_day() {
    let site = site(SERVER_SITE, "");
    build(&site);
    // A day kept from an earlier build stays while the facts stay the same.
    let kept = at(site.path(), "data/privacy-facts");
    let today = fs::read_to_string(&kept).unwrap();
    let (_, hash) = today.trim().split_once(' ').unwrap();
    fs::write(&kept, format!("2026-01-05 {hash}\n")).unwrap();
    build(&site);
    let page = main_of(&site, "privacy/index.html");
    assert!(page.contains(r#"<time datetime="2026-01-05">5 January 2026</time>"#));
    put(
        site.path(),
        "site.toml",
        &format!("{SERVER_SITE}{MARKETING_CODE}"),
    );
    build(&site);
    let page = main_of(&site, "privacy/index.html");
    assert!(page.contains("<td>Marketing</td>"), "{page}");
    assert!(!page.contains("2026-01-05"), "{page}");
}

#[test]
fn an_owner_page_replaces_the_generated_one_and_places_the_facts() {
    let site = site(STATIC_SITE, "");
    put(
        site.path(),
        "content/en/pages/privacy.md",
        "---\ntitle: How we treat your data\nslug: privacy\n---\n\n\
         We keep as little as we can about {{ site.name }}.\n\n::privacy-facts\n\n\
         Write to us any time.\n",
    );
    build(&site);
    let page = main_of(&site, "privacy/index.html");
    assert!(page.contains("<h1>How we treat your data</h1>"), "{page}");
    assert!(!page.contains("<h1>Privacy</h1>"), "{page}");
    assert!(
        page.contains("about {{ site.name }}."),
        "an owner page is never read as a template: {page}"
    );
    let facts = page.find("<code>khalid-consent</code>").unwrap();
    assert!(page.find("We keep as little").unwrap() < facts, "{page}");
    assert!(
        facts < page.find("Write to us any time.").unwrap(),
        "{page}"
    );
    assert!(!page.contains("khalid-privacy-facts"), "{page}");
}

#[test]
fn missing_contact_details_warn_the_owner_and_never_show_on_the_page() {
    let site = site(STATIC_SITE, "");
    let said = build(&site);
    assert!(
        said.contains("The privacy page has no contact details."),
        "{said}"
    );
    let doctor = khalid(&site, &["doctor", "--offline"]);
    let doctor_said = String::from_utf8_lossy(&doctor.stdout);
    assert_eq!(doctor.status.code(), Some(0), "a warning is no problem");
    assert!(
        doctor_said.contains("The privacy page has no contact details."),
        "{doctor_said}"
    );
    let page = main_of(&site, "privacy/index.html");
    assert!(!page.contains("contact details"), "{page}");
}
