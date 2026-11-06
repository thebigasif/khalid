//! The Research template end to end (docs/templates/research.md): `khalid
//! build` of the demo site with `[template] name = "research"` and two
//! posts given a paper's facts (venue, year, DOI, citations), then the
//! header and main part of its front page, a paper, a list and the people
//! page, and the cite box built from the paper's own data.

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

const SETTINGS: &str = "\n[template]\nname = \"research\"\n[template.options]\nhero_stats = [\"12 | papers\", \"5 | people\"]\ntagline = [\"We study how the web is read.\"]\n[template.cta]\nlabel = \"Open positions\"\nhref = \"https://www.example.com/jobs/\"\n";

/// Two posts that become papers: their facts, in the order Research
/// reads them.
const PAPERS: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Journal\n    value: \"Web Performance Review\"\n  - label: Year\n    value: \"2026\"\n  - label: DOI\n    value: \"10.5555/demo.2026.001\"\n  - label: Citations\n    value: \"12\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Venue\n    value: \"Search Writing Workshop\"\n  - label: Year\n    value: \"2025\"\n  - label: Link\n    value: \"https://www.example.com/paper.pdf\"\n",
    ),
];

/// The demo site built with Research, with two posts made papers.
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(SETTINGS);
    fs::write(&settings, toml).unwrap();
    for (rel, facts) in PAPERS {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', &format!("\n{facts}"), 1)).unwrap();
    }
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

#[test]
fn research_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// Papers (posts with facts) form the numbered reference list, and every
/// other post the news feed beside it, so no post shows twice.
#[test]
fn research_front_page_splits_papers_from_news() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let papers = texts(&page, ".lab-papers .pub-title");
    assert_eq!(
        papers,
        [
            "Fixing slow pages, starting with the biggest win",
            "How to write a title tag",
        ]
    );
    let news = texts(&page, ".lab-news .feed-item h3");
    assert_ne!(news, Vec::<String>::new(), "the news column is empty");
    assert!(news.iter().all(|title| !papers.contains(title)), "{news:?}");
    assert_eq!(texts(&page, ".figure dd"), ["12", "5"]);
    assert!(page.contains("We study how the web is read."));
}

/// The research-area index lists the categories that hold papers. A
/// category whose posts are all news, here Content strategy, is left out
/// by the same rule as the news column, not by its name.
#[test]
fn the_research_area_index_leaves_out_a_category_of_news_only() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let areas = texts(&page, ".area-index .area-name");
    assert!(areas.contains(&"Technical SEO".to_owned()), "{areas:?}");
    assert!(areas.contains(&"On-page SEO".to_owned()), "{areas:?}");
    assert!(!areas.contains(&"Content strategy".to_owned()), "{areas:?}");
}

#[test]
fn research_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// The cite box is built from the post's own author, title and facts: the
/// DOI links to doi.org, and the BibTeX entry is keyed by the author's
/// surname and the year.
#[test]
fn a_paper_ends_with_a_cite_box_from_its_own_data() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    let cite = texts(&page, ".cite-text").concat();
    assert!(cite.contains("(2026). Fixing slow pages"), "{cite}");
    assert!(cite.contains("Web Performance Review"), "{cite}");
    assert!(
        cite.contains("https://doi.org/10.5555/demo.2026.001"),
        "{cite}"
    );
    let bibtex = texts(&page, ".cite-bibtex code").concat();
    assert!(bibtex.starts_with("@article{vogt2026,"), "{bibtex}");
    assert!(bibtex.contains("doi = {10.5555/demo.2026.001}"), "{bibtex}");
    assert!(page.contains("<a href=\"https://doi.org/10.5555/demo.2026.001\">"));
    let link = shown(&out, "how-to-write-a-title-tag/index.html");
    assert!(
        texts(&link, ".cite-bibtex code")
            .concat()
            .contains("url = {https://www.example.com/paper.pdf}")
    );
    let news = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&news, ".cite-box").is_empty(),
        "a post without facts has no cite box"
    );
}

#[test]
fn research_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn research_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
