//! The showcase site, end to end: every post passes `khalid lint` with no
//! finding, the built site passes `khalid audit` with no warning but the
//! known home page pair, and the posts use every block the showcase
//! promises to show.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

/// Rules that depend on the day the test runs, not on the content: a post
/// written today is due for review in six months and stale in twelve.
const DATED: [&str; 3] = ["review-due", "stale-post", "stale-page"];

/// Every block the showcase shows. Audio, video files and recipes are left
/// out: the fixture keeps no sound or film files, and recipe markup on a
/// page that is not a recipe breaks the search engines' rules.
const BLOCKS: [&str; 24] = [
    "bookmark",
    "business",
    "button",
    "callout",
    "columns",
    "embed",
    "faq",
    "figure",
    "file",
    "form",
    "gallery",
    "gated",
    "header",
    "howto",
    "html",
    "maths",
    "newsletter",
    "product",
    "proscons",
    "pullquote",
    "review",
    "snippet",
    "table",
    "toc",
];

/// The one known warning: each language's home page takes its title and
/// description from the site's name and tagline, so the English and German
/// home pages match. A fix to the home page leaves this test green.
const HOMES: [&str; 2] = [
    "https://www.example.com/blog/",
    "https://www.example.com/blog/de/",
];

/// Whether `issue` is the known duplicate between the two home pages.
#[cfg(test)]
fn home_pair(issue: &Value) -> bool {
    ["title-duplicate", "description-duplicate"].contains(&issue["check"].as_str().unwrap())
        && HOMES.contains(&issue["url"].as_str().unwrap())
        && HOMES.contains(&issue["detail"].as_str().unwrap())
}

#[cfg(test)]
fn showcase() -> PathBuf {
    let mut site = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    site.push("../../fixtures/showcase-site");
    site
}

/// A copy of the showcase site, so the build and the audit's `data/` stay
/// out of the fixture.
#[cfg(test)]
fn showcase_copy() -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&showcase(), temp.path());
    temp
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        if ["data", "public"].contains(&entry.file_name().to_str().unwrap()) {
            continue;
        }
        let mut target = into.to_path_buf();
        target.push(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

#[cfg(test)]
fn khalid(site: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .arg("--site")
        .arg(site)
        .args(args)
        .output()
        .unwrap()
}

#[cfg(test)]
fn json(output: &Output) -> Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}"))
}

/// Every markdown file under `dir`, with its text.
#[cfg(test)]
fn markdown(dir: &Path, found: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            markdown(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            found.push(fs::read_to_string(&path).unwrap());
        }
    }
}

#[test]
fn every_showcase_post_passes_lint_with_no_finding() {
    let site = showcase_copy();
    let output = khalid(site.path(), &["lint", "--json"]);
    let body = json(&output);
    let findings: Vec<&Value> = body["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| !DATED.contains(&finding["rule"].as_str().unwrap()))
        .collect();
    assert_eq!(findings, Vec::<&Value>::new(), "{body}");
    assert_eq!(output.status.code(), Some(0));
    let scores = body["scores"].as_array().unwrap();
    assert!(scores.len() >= 15, "{body}");
}

#[test]
fn the_built_showcase_passes_the_audit_with_no_new_warning() {
    let site = showcase_copy();
    let output = khalid(site.path(), &["audit", "--offline", "--json"]);
    let body = json(&output);
    let problems: Vec<&Value> = body["issues"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|issue| ["error", "warning"].contains(&issue["severity"].as_str().unwrap()))
        .filter(|issue| !DATED.contains(&issue["check"].as_str().unwrap()))
        .filter(|issue| !home_pair(issue))
        .collect();
    assert_eq!(problems, Vec::<&Value>::new(), "{body}");
    assert_eq!(body["score"], 100, "{body}");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_showcase_uses_every_block_it_promises() {
    let mut content = showcase();
    content.push("content");
    let mut texts = Vec::new();
    markdown(&content, &mut texts);
    let all = texts.concat();
    let missing: Vec<&str> = BLOCKS
        .into_iter()
        .filter(|name| !all.contains(&format!("::{name}")))
        .collect();
    assert_eq!(missing, Vec::<&str>::new());
    for kind in ["note", "tip", "warning", "summary", "read", "stat"] {
        assert!(
            all.contains(&format!(":::callout{{kind={kind}}}")),
            "{kind}"
        );
    }
    for feature in ["[[", "updated_note:", "access: members", "role: pillar"] {
        assert!(all.contains(feature), "{feature}");
    }
}

#[test]
fn every_template_demo_shows_the_showcase_call_to_action() {
    let packs = [
        "almanac",
        "atlas",
        "compass",
        "folio",
        "harbor",
        "keystone",
        "ledger",
        "meridian",
        "wire",
        "workbench",
    ];
    for pack in packs {
        // As scripts/pages.sh makes each template demo: the template's name
        // and the band's place go after the showcase's own [template.cta].
        let site = showcase_copy();
        let mut toml = site.path().to_path_buf();
        toml.push("site.toml");
        let text = fs::read_to_string(&toml).unwrap();
        let demo = format!(
            "{text}\n[template]\nname = \"{pack}\"\n\n[template.options]\ncta_band = \"both\"\n"
        );
        fs::write(&toml, demo).unwrap();
        let out = tempfile::tempdir().unwrap();
        let output = khalid(
            site.path(),
            &["build", "--out", out.path().to_str().unwrap()],
        );
        assert_eq!(output.status.code(), Some(0), "{pack}: {output:?}");
        for page in ["index.html", "seo-writing-guide/index.html"] {
            let mut path = out.path().to_path_buf();
            path.push(page);
            let html = fs::read_to_string(&path).unwrap();
            let band = html.split_once("<div class=\"cta-band\">");
            let band = band.unwrap_or_else(|| panic!("{pack} {page}: no band")).1;
            assert!(band.contains("Read the 2026 content benchmark"), "{pack} {page}");
        }
    }
}
