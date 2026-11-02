//! `khalid build` writes the pages that help readers find their way: the
//! glossary from A to Z, series, hubs, case studies and the changelog.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const SITE_TOML: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
mount = "/blog/"
mode = "static"
default_language = "en"
languages = ["en", "de"]
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

/// A site with one post in each language.
#[cfg(test)]
fn site() -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    put(temp.path(), "site.toml", SITE_TOML);
    for (language, title) in [("en", "Why links matter"), ("de", "Warum Links zählen")] {
        let text = format!(
            "---\ntitle: {title}\nslug: why-links-matter\ndate: 2026-09-01\n---\n\n\
             A post that links to [the home page](/blog/).\n"
        );
        put(
            temp.path(),
            &format!("content/{language}/posts/why-links-matter.md"),
            &text,
        );
    }
    temp
}

/// Writes a glossary term in `language`.
#[cfg(test)]
fn term(site: &TempDir, language: &str, (term, slug): (&str, &str), definition: &str) {
    put(
        site.path(),
        &format!("content/{language}/glossary/{slug}.md"),
        &format!("---\nterm: {term}\nslug: {slug}\n---\n\n{definition}\n"),
    );
}

#[cfg(test)]
fn build(site: &TempDir) {
    let output: Output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site.path())
        .arg("build")
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{said}");
}

/// The built file at `rel` inside `public/`, or `None` when there is none.
#[cfg(test)]
fn page(site: &TempDir, rel: &str) -> Option<String> {
    fs::read_to_string(at(site.path(), &format!("public/{rel}"))).ok()
}

/// The page's main area.
#[cfg(test)]
fn main_of(page: &str) -> &str {
    let (_, main) = page.split_once("<main").unwrap();
    main.split_once("</main>").unwrap().0
}

/// A site with three English terms under two letters and one German term.
#[cfg(test)]
fn with_terms() -> TempDir {
    let site = site();
    let terms = [
        (
            "en",
            "Topical authority",
            "How much a search engine trusts a site.",
        ),
        (
            "en",
            "Anchor text",
            "The words a reader clicks to follow a link.",
        ),
        (
            "en",
            "Thin content",
            "A page with too little on it to help anyone.",
        ),
        ("de", "Ankertext", "Die Wörter, auf die ein Leser klickt."),
    ];
    for (language, name, definition) in terms {
        let slug = name.to_lowercase().replace(' ', "-");
        term(&site, language, (name, &slug), definition);
    }
    site
}

#[test]
fn the_glossary_lists_every_term_under_its_letter_with_its_definition() {
    let site = with_terms();
    build(&site);
    let glossary = page(&site, "glossary/index.html").unwrap();
    insta::assert_snapshot!(main_of(&glossary));
    let german = page(&site, "de/glossary/index.html").unwrap();
    assert!(
        german.contains(r##"<a href="#letter-a">A</a>"##),
        "{german}"
    );
    assert!(german.contains("Die Wörter, auf die ein Leser klickt."));
}

#[test]
fn the_glossary_is_a_defined_term_set_in_the_sitemaps() {
    let site = with_terms();
    build(&site);
    let glossary = page(&site, "glossary/index.html").unwrap();
    let term = r#""hasDefinedTerm":[{"@type":"DefinedTerm","@id":"https://www.example.com/blog/glossary/anchor-text/#term""#;
    assert!(glossary.contains(term), "{glossary}");
    let archives = page(&site, "sitemaps/archives-en-1.xml").unwrap();
    assert!(
        archives.contains("<loc>https://www.example.com/blog/glossary/</loc>"),
        "{archives}"
    );
    let html = page(&site, "sitemap/index.html").unwrap();
    assert!(
        html.contains(r#"<a href="/blog/glossary/">Glossary</a>"#),
        "{html}"
    );
}

#[test]
fn a_language_with_no_terms_has_no_glossary_page() {
    let site = site();
    term(
        &site,
        "en",
        ("Anchor text", "anchor-text"),
        "The words a reader clicks to follow a link.",
    );
    build(&site);
    assert!(page(&site, "glossary/index.html").is_some());
    assert_eq!(page(&site, "de/glossary/index.html"), None);
}

#[test]
fn the_glossary_index_file_names_and_introduces_the_glossary() {
    let site = with_terms();
    put(
        site.path(),
        "content/en/glossary/_index.md",
        "---\ntitle: SEO glossary\ndescription: Every SEO term we use, in plain words.\n---\n\n\
         Start with the terms you meet most.\n",
    );
    build(&site);
    let glossary = page(&site, "glossary/index.html").unwrap();
    assert!(glossary.contains("<h1>SEO glossary</h1>"), "{glossary}");
    assert!(glossary.contains("<p>Start with the terms you meet most.</p>"));
    assert!(
        glossary.contains(
            r#"<meta name="description" content="Every SEO term we use, in plain words.">"#
        )
    );
}
