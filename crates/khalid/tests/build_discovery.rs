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

/// A site with one case study and two changelog entries in two months.
#[cfg(test)]
fn with_entries() -> TempDir {
    let site = site();
    put(
        site.path(),
        "content/en/case-studies/northfield-bakery.md",
        "---\ntitle: How a bakery doubled its traffic\nslug: northfield-bakery\n\
         date: 2026-10-20\nclient: Northfield Bakery\noutcome: Search visits doubled.\n\
         description: One topic cluster on sourdough.\n---\n\nThe whole story.\n",
    );
    for (file, title, date, version) in [
        ("2026-09", "Search pages", "2026-09-14", "1.1.0"),
        ("2026-10", "Glossary pages", "2026-10-28", "1.3.0"),
    ] {
        put(
            site.path(),
            &format!("content/en/changelog/{file}.md"),
            &format!(
                "---\ntitle: {title}\ndate: {date}\nversion: {version}\n---\n\n{title} ship.\n"
            ),
        );
    }
    site
}

#[test]
fn case_studies_have_a_list_and_a_page_each() {
    let site = with_entries();
    build(&site);
    let list = page(&site, "case-studies/index.html").unwrap();
    assert!(list.contains("<h1>Case studies</h1>"), "{list}");
    assert!(list.contains(r#"<p class="entry-label">Northfield Bakery</p>"#));
    assert!(list.contains("<p>One topic cluster on sourdough.</p>"));
    assert!(list.contains(r#""@type":"ItemList""#), "{list}");
    let study = page(&site, "case-studies/northfield-bakery/index.html").unwrap();
    assert!(study.contains("<h1>How a bakery doubled its traffic</h1>"));
    assert!(study.contains(r#"<a href="/blog/case-studies/">All case studies</a>"#));
    assert_eq!(page(&site, "de/case-studies/index.html"), None);
}

#[test]
fn the_changelog_lists_each_month_newest_first() {
    let site = with_entries();
    build(&site);
    let list = page(&site, "changelog/index.html").unwrap();
    let main = main_of(&list);
    let october = main.find("October 2026").unwrap();
    let september = main.find("September 2026").unwrap();
    assert!(october < september, "{main}");
    assert!(main.contains("<p>Glossary pages ship.</p>"), "{main}");
    assert!(main.contains(r#"<p class="entry-label">Version 1.3.0</p>"#));
    assert!(page(&site, "changelog/glossary-pages/index.html").is_some());
    let archives = page(&site, "sitemaps/archives-en-1.xml").unwrap();
    assert!(archives.contains("<loc>https://www.example.com/blog/changelog/</loc>"));
}
