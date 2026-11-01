//! Khalid's own website in `site/`, published at
//! <https://thebigasif.github.io/khalid/>: every post passes the linter with
//! no finding, the audit finds nothing to fix but the links to the demo
//! sites published beside it, and every page the brief asks for is there.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

/// Where the website lives in the repository.
const SITE: &str = "site";

/// Rules that depend on the day the test runs, not on the content: a post
/// written today is due for review in six months and stale in twelve.
const DATED: [&str; 3] = ["review-due", "stale-post", "stale-page"];

/// The site's home, as its pages link to it.
const HOME: &str = "https://thebigasif.github.io/khalid/";

/// The ten templates, each with a demo at `/khalid/templates/<name>/`.
const TEMPLATES: [&str; 10] = [
    "ledger",
    "atlas",
    "keystone",
    "compass",
    "harbor",
    "wire",
    "workbench",
    "almanac",
    "meridian",
    "folio",
];

/// Every look the template gallery shows, each with a picture of its
/// demo's home page and of a post: Classic, then the ten templates.
const LOOKS: [&str; 11] = [
    "classic",
    "ledger",
    "atlas",
    "keystone",
    "compass",
    "harbor",
    "wire",
    "workbench",
    "almanac",
    "meridian",
    "folio",
];

/// Where the gallery's plain cards sit, beside the Templates page.
const CARDS: &str = "site/content/en/pages/templates";

/// The use case sites, each at `/khalid/examples/<slug>/`, and the page
/// of this site that tells its story.
const EXAMPLES: [(&str, &str); 6] = [
    ("saas", "saas-blog"),
    ("agency", "agency-blog"),
    ("newsletter", "newsletter-writers"),
    ("local-business", "local-business-blog"),
    ("multilingual", "multilingual-blog"),
    ("membership", "membership-blog"),
];

/// One page per feature group, in the order the features list shows them.
const FEATURES: [&str; 12] = [
    "writing-checks",
    "seo-ai-search",
    "blog-editor",
    "schedule-posts",
    "blog-newsletter",
    "blog-comments",
    "ai-visitors",
    "blog-migration",
    "blog-api",
    "blog-hosting",
    "blog-backups",
    "theme-sync",
];

#[cfg(test)]
fn repo_file(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    path
}

#[cfg(test)]
fn within(dir: &Path, relative: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(relative);
    path
}

/// A copy of the website as git holds it, without what a local run left.
#[cfg(test)]
fn site_copy() -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&repo_file(SITE), temp.path());
    temp
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if ["public", "data"].contains(&name.as_str()) {
            continue;
        }
        let target = within(into, &name);
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
        .args(["--site", site.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap()
}

/// The built page at `rest` under the site's `public/` folder.
#[cfg(test)]
fn built_page(site: &Path, rest: &str) -> String {
    let file = within(site, &format!("public/{rest}index.html"));
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("no page at {}", file.display()))
}

/// Whether `target` is a page of the demo sites GitHub Pages publishes
/// beside this site, which an audit of this site alone cannot reach.
#[cfg(test)]
fn is_sibling(target: &str) -> bool {
    let Some(rest) = target.strip_prefix(HOME) else {
        return false;
    };
    rest.starts_with("demo/")
        || TEMPLATES
            .iter()
            .any(|name| rest == format!("templates/{name}/"))
        || EXAMPLES
            .iter()
            .any(|(example, _)| rest == format!("examples/{example}/"))
}

#[test]
fn every_page_of_the_website_passes_the_linter_with_no_finding() {
    let site = site_copy();
    let lint = khalid(site.path(), &["lint", "--json"]);
    let stdout = String::from_utf8_lossy(&lint.stdout);
    let body: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}"));
    let findings: Vec<&Value> = body["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| !DATED.contains(&finding["rule"].as_str().unwrap()))
        .collect();
    assert_eq!(findings, Vec::<&Value>::new(), "{stdout}");
    assert_eq!(lint.status.code(), Some(0), "{stdout}");
    let scored = body["scores"].as_array().unwrap();
    assert_eq!(scored.len(), FEATURES.len() + EXAMPLES.len(), "{stdout}");
    assert!(scored.iter().all(|score| score["value"] == 100), "{stdout}");
}

/// The only errors are the missing demo sites, and only at the addresses
/// the Pages workflow publishes them at; there is no warning at all.
#[test]
fn the_audit_finds_nothing_but_the_demo_sites_published_beside_it() {
    let site = site_copy();
    let audit = khalid(site.path(), &["audit", "--offline", "--json"]);
    let stdout = String::from_utf8_lossy(&audit.stdout);
    let body: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}"));
    let issues = body["issues"].as_array().unwrap();
    let mut siblings = BTreeSet::new();
    for issue in issues {
        let severity = issue["severity"].as_str().unwrap();
        if DATED.contains(&issue["check"].as_str().unwrap()) {
            continue;
        }
        assert_ne!(severity, "warning", "{issue}");
        if severity != "error" {
            continue;
        }
        let target = match issue["check"].as_str().unwrap() {
            "page-4xx" => issue["url"].as_str().unwrap(),
            "broken-link" => issue["detail"].as_str().unwrap(),
            _ => panic!("an error that is not a missing demo page: {issue}"),
        };
        assert!(is_sibling(target), "{issue}");
        siblings.insert(target.to_owned());
    }
    for name in TEMPLATES {
        assert!(
            siblings.contains(&format!("{HOME}templates/{name}/")),
            "{name}"
        );
    }
    for (example, _) in EXAMPLES {
        assert!(
            siblings.contains(&format!("{HOME}examples/{example}/")),
            "{example}"
        );
    }
}

#[test]
fn every_page_the_brief_asks_for_is_built_and_links_its_live_demo() {
    let site = site_copy();
    let built = khalid(site.path(), &["build"]);
    assert!(built.status.success(), "{built:?}");

    let home = built_page(site.path(), "");
    assert_eq!(home.matches("<h1").count(), 1);
    for part in [
        "Who it is for",
        "The problem it solves",
        "What you get",
        "Three ways to start",
        "deploy.workers.cloudflare.com",
        "Write, check, publish",
        "class=\"closing\"",
    ] {
        assert!(home.contains(part), "{part}");
    }
    for slug in FEATURES {
        assert!(
            home.contains(&format!("href=\"/khalid/features/{slug}/\"")),
            "{slug}"
        );
    }

    let features = built_page(site.path(), "features/");
    assert_eq!(
        features.matches("<span class=\"icon\"><svg").count(),
        FEATURES.len()
    );
    assert!(
        !features.contains("<path d=\"\""),
        "every feature has an icon"
    );
    let places: Vec<usize> = FEATURES
        .iter()
        .map(|slug| {
            features
                .find(&format!("href=\"/khalid/features/{slug}/\""))
                .unwrap_or_else(|| panic!("{slug} is not listed"))
        })
        .collect();
    assert!(places.is_sorted(), "{places:?}");
    for slug in FEATURES {
        let page = built_page(site.path(), &format!("features/{slug}/"));
        assert!(page.contains("See it live"), "{slug}");
    }

    for (example, slug) in EXAMPLES {
        let page = built_page(site.path(), &format!("use-cases/{slug}/"));
        assert!(
            page.contains(&format!("href=\"/khalid/examples/{example}/\"")),
            "{slug}"
        );
    }

    let templates = built_page(site.path(), "templates/");
    for name in TEMPLATES {
        assert!(
            templates.contains(&format!("href=\"/khalid/templates/{name}/\"")),
            "{name}"
        );
    }

    let start = built_page(site.path(), "start/");
    for way in [
        "install.sh",
        "ghcr.io/thebigasif/khalid",
        "deploy.workers.cloudflare.com",
        "khalid.git",
    ] {
        assert!(start.contains(way), "{way}");
    }

    let compare = built_page(site.path(), "compare/");
    for source in ["wordpress.org", "ghost.org", "developers.webflow.com"] {
        assert!(compare.contains(source), "{source}");
    }
}

/// The plain cards beside the Templates page are what the preview script
/// writes where no Chromium runs, so the website shows the same card
/// before Pages draws the screenshots over it.
#[test]
fn the_gallery_cards_are_the_preview_scripts_plain_cards() {
    let read = Command::new("bash")
        .arg("-n")
        .arg(repo_file("scripts/template-previews.sh"))
        .output()
        .unwrap();
    assert!(read.status.success(), "{read:?}");
    let drawn = tempfile::tempdir().unwrap();
    let script = Command::new("bash")
        .arg(repo_file("scripts/template-previews.sh"))
        .arg("--cards")
        .arg(drawn.path())
        .output()
        .unwrap();
    assert!(script.status.success(), "{script:?}");
    for name in LOOKS {
        for kind in ["home", "post"] {
            let file = format!("{name}-{kind}.svg");
            let committed = fs::read_to_string(within(&repo_file(CARDS), &file)).unwrap();
            let fresh = fs::read_to_string(within(drawn.path(), &file)).unwrap();
            assert_eq!(committed, fresh, "{file}");
        }
    }
    let beside = fs::read_dir(repo_file(CARDS)).unwrap().count();
    assert_eq!(beside, LOOKS.len() * 2 + 1, "the cards and index.md");
}
