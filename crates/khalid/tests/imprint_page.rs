//! A site with `[privacy.imprint]` gets an imprint, the Impressum German
//! law asks for, at `<mount>impressum/` in each language, linked from the
//! footer of every page, unless the owner writes their own (backlog row 36).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const SITE: &str = r#"name = "Field Notes"
address = "https://www.example.com"
mount = "/blog/"
mode = "static"
default_language = "en"
languages = ["en", "de"]

[privacy.contact]
name = "Notes GmbH"
email = "hello@example.com"
address = "Hauptstraße 1, 10115 Berlin"
"#;

const IMPRINT: &str = r#"
[privacy.imprint]
phone = "+49 30 1234567"
register = "Amtsgericht Berlin, HRB 12345"
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

/// A site with `settings` and one page no publish gate holds back.
#[cfg(test)]
fn site(settings: &str) -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    put(temp.path(), "site.toml", settings);
    put(
        temp.path(),
        "content/en/pages/about.md",
        "---\ntitle: About\nslug: about\n---\n\nA page that links to [the home page](/blog/).\n",
    );
    temp
}

/// Runs `khalid build` on `site` and returns its exit code and what it said.
#[cfg(test)]
fn build(site: &TempDir) -> (Option<i32>, String) {
    let output: Output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site.path())
        .arg("build")
        .output()
        .unwrap();
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    (output.status.code(), said)
}

/// The built page at `rel` inside `public/`.
#[cfg(test)]
fn page(site: &TempDir, rel: &str) -> String {
    fs::read_to_string(at(site.path(), &format!("public/{rel}"))).unwrap()
}

/// The footer of `html`.
#[cfg(test)]
fn footer_of(html: &str) -> &str {
    let (_, footer) = html.split_once("<footer").unwrap();
    footer
}

#[test]
fn a_site_with_an_imprint_table_gets_the_page_in_each_language() {
    let site = site(&format!("{SITE}{IMPRINT}"));
    let (code, said) = build(&site);
    assert_eq!(code, Some(0), "{said}");
    let english = page(&site, "impressum/index.html");
    for line in [
        "<h1>Imprint</h1>",
        "<address>Notes GmbH<br>Hauptstraße 1, 10115 Berlin</address>",
        "<a href=\"tel:+49301234567\">+49 30 1234567</a>",
        "Amtsgericht Berlin, HRB 12345",
        "<a href=\"/blog/privacy/\">Privacy</a>",
        "<meta name=\"description\" content=\"Who runs Field Notes and how to reach them.\">",
    ] {
        assert!(english.contains(line), "{line} in {english}");
    }
    let german = page(&site, "de/impressum/index.html");
    assert!(german.contains("<h1>Impressum</h1>"), "{german}");
    assert!(
        german.contains("<a href=\"/blog/de/privacy/\">"),
        "{german}"
    );
}

#[test]
fn every_page_links_to_the_imprint_from_its_footer() {
    let site = site(&format!("{SITE}{IMPRINT}"));
    let (code, said) = build(&site);
    assert_eq!(code, Some(0), "{said}");
    let about = page(&site, "about/index.html");
    assert!(
        footer_of(&about).contains("<a href=\"/blog/impressum/\">Imprint</a>"),
        "{about}"
    );
    let german_home = page(&site, "de/index.html");
    assert!(
        footer_of(&german_home).contains("<a href=\"/blog/de/impressum/\">Impressum</a>"),
        "{german_home}"
    );
}

#[test]
fn a_site_without_the_table_has_no_imprint_and_no_link() {
    let site = site(SITE);
    let (code, said) = build(&site);
    assert_eq!(code, Some(0), "{said}");
    assert!(!at(site.path(), "public/impressum/index.html").exists());
    assert!(!page(&site, "about/index.html").contains("impressum/"));
}

#[test]
fn an_owner_page_replaces_the_generated_one_and_is_linked_once() {
    let settings = format!(
        "{SITE}\n[[nav.footer]]\nlabel = \"Legal notice\"\nhref = \"/blog/impressum/\"\n{IMPRINT}"
    );
    let site = site(&settings);
    put(
        site.path(),
        "content/en/pages/impressum.md",
        "---\ntitle: Legal notice\nslug: impressum\n---\n\nNotes GmbH, written by hand.\n",
    );
    let (code, said) = build(&site);
    assert_eq!(code, Some(0), "{said}");
    let own = page(&site, "impressum/index.html");
    assert!(own.contains("<h1>Legal notice</h1>"), "{own}");
    assert!(!own.contains("<h1>Imprint</h1>"), "{own}");
    let footer = footer_of(&own);
    assert_eq!(
        footer.matches("href=\"/blog/impressum/\"").count(),
        1,
        "{footer}"
    );
}

#[test]
fn an_imprint_without_an_address_stops_the_build_with_the_fix() {
    let settings = SITE.replace("address = \"Hauptstraße 1, 10115 Berlin\"\n", "");
    let site = site(&format!("{settings}{IMPRINT}"));
    let (code, said) = build(&site);
    assert_ne!(code, Some(0), "{said}");
    assert!(said.contains("[privacy.contact]"), "{said}");
    assert!(said.contains("postal address"), "{said}");
}
