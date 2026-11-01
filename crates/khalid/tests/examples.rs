//! The example sites in `fixtures/examples/`, end to end: each one builds,
//! passes `khalid lint` and `khalid audit` with no finding, has its README,
//! and uses the features its use case is there to show.

use std::fs;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

/// Rules that depend on the day the test runs, not on the content: a post
/// written today is due for review in six months and stale in twelve.
const DATED: [&str; 3] = ["review-due", "stale-post", "stale-page"];

/// The fewest and the most posts an example has in each language.
const POSTS: RangeInclusive<usize> = 6..=10;

/// What each example is there to show: text its `site.toml` holds, and
/// text its content holds.
const FEATURES: [(&str, &[&str], &[&str]); 1] = [(
    "local-business",
    &["[[seo.locations]]", "[[seo.locations.hours]]"],
    &["::::faq", "::::howto", "::business{"],
)];

/// `base` with each of `parts` added, all of them fixed names.
#[cfg(test)]
fn within(base: &Path, parts: &[&str]) -> PathBuf {
    let mut path = base.to_path_buf();
    for part in parts {
        path.push(part);
    }
    path
}

/// The folder of the example `slug`, or of all of them for `""`.
#[cfg(test)]
fn example(slug: &str) -> PathBuf {
    within(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        &["../../fixtures/examples", slug],
    )
}

/// Every example site's folder name, sorted.
#[cfg(test)]
fn slugs() -> Vec<String> {
    let mut slugs: Vec<String> = fs::read_dir(example(""))
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    slugs.sort();
    slugs
}

/// A copy of one example, so the build and the audit's `data/` stay out of
/// the fixture. An example may name a template that is still on its way
/// into Khalid; until it is built in, the copy uses `classic`, so the
/// content is still checked.
#[cfg(test)]
fn example_copy(slug: &str) -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&example(slug), temp.path());
    let settings = within(temp.path(), &["site.toml"]);
    let mut site: toml_edit::DocumentMut = fs::read_to_string(&settings).unwrap().parse().unwrap();
    let name = site["template"]["name"].as_str().unwrap().to_owned();
    if !khalid_theme::packs().iter().any(|pack| pack.name == name) {
        eprintln!("{slug}: the {name} template is not built in yet, so this run uses classic");
        site["template"]["name"] = toml_edit::value("classic");
        fs::write(&settings, site.to_string()).unwrap();
    }
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

/// The text of every markdown file under `dir`, by path.
#[cfg(test)]
fn markdown(dir: &Path, found: &mut Vec<(PathBuf, String)>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            markdown(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            let text = fs::read_to_string(&path).unwrap();
            found.push((path, text));
        }
    }
}

#[test]
fn every_example_this_test_knows_is_there() {
    let slugs = slugs();
    for (slug, _, _) in FEATURES {
        assert!(slugs.iter().any(|found| found == slug), "{slug}: {slugs:?}");
    }
}

#[test]
fn every_example_passes_lint_with_no_finding() {
    for slug in slugs() {
        let site = example_copy(&slug);
        let output = khalid(site.path(), &["lint", "--json"]);
        let body = json(&output);
        let findings: Vec<&Value> = body["findings"]
            .as_array()
            .unwrap_or_else(|| panic!("{slug}: {body}"))
            .iter()
            .filter(|finding| !DATED.contains(&finding["rule"].as_str().unwrap()))
            .collect();
        assert_eq!(findings, Vec::<&Value>::new(), "{slug}: {body}");
        assert_eq!(output.status.code(), Some(0), "{slug}");
    }
}

#[test]
fn every_built_example_passes_the_audit_with_no_warning() {
    for slug in slugs() {
        let site = example_copy(&slug);
        let output = khalid(site.path(), &["audit", "--offline", "--json"]);
        let body = json(&output);
        let problems: Vec<&Value> = body["issues"]
            .as_array()
            .unwrap_or_else(|| panic!("{slug}: {body}"))
            .iter()
            .filter(|issue| ["error", "warning"].contains(&issue["severity"].as_str().unwrap()))
            .filter(|issue| !DATED.contains(&issue["check"].as_str().unwrap()))
            .collect();
        assert_eq!(problems, Vec::<&Value>::new(), "{slug}: {body}");
        assert_eq!(body["score"], 100, "{slug}: {body}");
        assert_eq!(output.status.code(), Some(0), "{slug}");
    }
}

#[test]
fn every_example_has_a_readme_a_template_and_six_to_ten_posts() {
    for slug in slugs() {
        let site = example(&slug);
        assert!(
            within(&site, &["README.md"]).is_file(),
            "{slug} has no README.md"
        );
        let settings: toml::Table = fs::read_to_string(within(&site, &["site.toml"]))
            .unwrap()
            .parse()
            .unwrap();
        assert!(
            settings["template"]["name"].is_str(),
            "{slug} names no template"
        );
        for language in fs::read_dir(within(&site, &["content"])).unwrap() {
            let mut posts = Vec::new();
            markdown(&within(&language.unwrap().path(), &["posts"]), &mut posts);
            let count = posts
                .iter()
                .filter(|(path, _)| !path.ends_with("_index.md"))
                .count();
            assert!(POSTS.contains(&count), "{slug} has {count} posts");
        }
    }
}

#[test]
fn every_example_shows_the_features_of_its_use_case() {
    for (slug, settings, content) in FEATURES {
        let site = example(slug);
        let toml = fs::read_to_string(within(&site, &["site.toml"])).unwrap();
        for needle in settings {
            assert!(toml.contains(needle), "{slug}: site.toml has no {needle}");
        }
        let mut texts = Vec::new();
        markdown(&within(&site, &["content"]), &mut texts);
        for needle in content {
            assert!(
                texts.iter().any(|(_, text)| text.contains(needle)),
                "{slug}: no post or page uses {needle}"
            );
        }
    }
}

#[test]
fn no_example_has_an_em_dash_or_filler_text() {
    for slug in slugs() {
        let mut texts = Vec::new();
        markdown(&example(&slug), &mut texts);
        for (path, text) in texts {
            let lower = text.to_lowercase();
            for banned in ["\u{2014}", "lorem", "placeholder"] {
                assert!(!lower.contains(banned), "{}: {banned}", path.display());
            }
        }
    }
}
