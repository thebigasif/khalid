//! `[template]` in `site.toml` end to end: `khalid build` of the demo site
//! with the test-only trial pack differs from classic in its stylesheet,
//! its not-found page and a reader switch, classic builds exactly what a
//! site without the table builds, and a wrong name or option stops the
//! build with its line (templates spec 4.1).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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

/// Builds a copy of the demo site with `table` added to its `site.toml`.
#[cfg(test)]
fn build(table: &str) -> (tempfile::TempDir, PathBuf, Output) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(table);
    fs::write(&settings, toml).unwrap();
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    (temp, out, output)
}

/// Builds like [`build`] and expects success.
#[cfg(test)]
fn built(table: &str) -> (tempfile::TempDir, PathBuf) {
    let (temp, out, output) = build(table);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    (temp, out)
}

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

/// The text of the one stylesheet in `out/assets/`.
#[cfg(test)]
fn stylesheet(out: &Path) -> String {
    let assets = fs::read_dir(at(out, "assets")).unwrap();
    let css = assets
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().is_some_and(|ext| ext == "css"))
        .unwrap();
    fs::read_to_string(css).unwrap()
}

#[test]
fn classic_builds_what_a_site_without_the_table_builds() {
    let (_none, without) = built("");
    let (_classic, classic) = built("\n[template]\nname = \"classic\"\n");
    assert_eq!(stylesheet(&without), stylesheet(&classic));
    for rel in ["index.html", "404.html", "what-a-sitemap-does/index.html"] {
        assert_eq!(page(&without, rel), page(&classic, rel), "{rel}");
    }
}

#[test]
fn the_trial_pack_changes_the_build_and_reads_its_options() {
    let (_classic, classic) = built("");
    let table = "\n[template]\nname = \"trial\"\n[template.options]\ngreeting = \"welcome\"\n";
    let (_trial, trial) = built(table);

    let css = stylesheet(&trial);
    assert!(css.contains(".trial-greeting"), "the pack's rules ship");
    assert!(css.contains("--text-h1: clamp(3rem, 2.6667rem + 1.6667vw, 4rem);"));
    assert!(
        !stylesheet(&classic).contains(".trial-greeting"),
        "classic has none"
    );

    let missing = page(&trial, "404.html");
    assert!(
        missing.contains("<p class=\"trial-greeting\">welcome</p>"),
        "{missing}"
    );

    let post = "what-a-sitemap-does/index.html";
    assert!(
        page(&trial, post).contains("<div class=\"share\">"),
        "trial shows share links"
    );
    assert!(!page(&classic, post).contains("class=\"share\""));
    assert!(page(&classic, post).contains("min read</time>"));
    assert!(
        !page(&trial, post).contains("min read</time>"),
        "trial turns reading time off"
    );
}

#[test]
fn the_sites_own_reader_key_wins_over_the_pack() {
    let table = "\n[reader]\nreading_time = true\n\n[template]\nname = \"trial\"\n";
    let (_temp, out) = built(table);
    assert!(page(&out, "what-a-sitemap-does/index.html").contains("min read</time>"));
}

#[test]
fn a_wrong_name_or_option_stops_the_build_with_its_line() {
    let (temp, _, output) = build("\n[template]\nname = \"ledgr\"\n");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.contains("there is no template named ledgr"),
        "{stderr}"
    );
    let text = fs::read_to_string(at(temp.path(), "site.toml")).unwrap();
    let line = text.lines().count();
    assert!(stderr.contains(&format!("line {line}")), "{stderr}");

    let table = "\n[template]\nname = \"trial\"\n[template.options]\ncolumns = 9\n";
    let (_temp, _, output) = build(table);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.contains("takes a whole number from 2 to 4"),
        "{stderr}"
    );
}
