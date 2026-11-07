//! `khalid init --template`: a new blog starts with the template a person
//! picked from the demos, or stops with the list of names (plan KTD10, R9).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use khalid_core::{Config, Environment};

/// Runs the binary with stdin closed, which is not a terminal.
#[cfg(test)]
fn khalid(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(args)
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

#[cfg(test)]
fn path(folder: &Path) -> &str {
    folder.to_str().unwrap()
}

/// `khalid init <site> --yes --address ... ` with `extra` after it.
#[cfg(test)]
fn init(site: &Path, extra: &[&str]) -> Output {
    let start = [
        "init",
        path(site),
        "--yes",
        "--address",
        "https://blog.example.com",
    ];
    khalid(&[&start[..], extra].concat())
}

#[test]
fn a_site_made_with_a_template_names_it_and_builds_in_it() {
    let temp = tempfile::tempdir().unwrap();
    let site = at(temp.path(), "blog");
    let output = init(&site, &["--template", "atlas"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("atlas"), "says the template");

    let settings = fs::read_to_string(at(&site, "site.toml")).unwrap();
    assert!(
        settings.contains("[template]\nname = \"atlas\"\n"),
        "{settings}"
    );
    let config = Config::load(&site, &Environment::empty()).unwrap();
    assert_eq!(config.template().name().as_str(), "atlas");
    // The template's own colours, so the blog looks like its demo.
    let tokens = fs::read_to_string(at(&site, "theme/tokens.toml")).unwrap();
    assert!(tokens.contains("link = \"#4338ca\""), "{tokens}");

    let out = at(temp.path(), "out");
    let built = khalid(&["--site", path(&site), "build", "--out", path(&out)]);
    assert_eq!(built.status.code(), Some(0), "{}", text(&built.stderr));
    let home = fs::read_to_string(at(&out, "index.html")).unwrap();
    assert!(
        home.contains("<div class=\"masthead\">"),
        "Atlas's front:\n{home}"
    );
    let css: String = fs::read_dir(at(&out, "assets"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|file| file.extension().is_some_and(|ext| ext == "css"))
        .map(|file| fs::read_to_string(file).unwrap())
        .collect();
    assert!(css.contains("#4338ca"), "the pages use Atlas's link colour");
}

#[test]
fn an_unknown_template_stops_with_the_names_and_makes_no_folder() {
    let temp = tempfile::tempdir().unwrap();
    let site = at(temp.path(), "blog");
    let output = init(&site, &["--template", "velvet"]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = text(&output.stderr);
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 3, "want what, why, fix:\n{stderr}");
    assert!(stderr.contains("velvet"), "{stderr}");
    for name in ["classic", "atlas", "workbench"] {
        assert!(stderr.contains(name), "{name} is listed:\n{stderr}");
    }
    assert!(!stderr.contains('\u{2014}'), "{stderr}");
    assert!(!site.exists(), "no folder is made");
}

#[test]
fn without_the_flag_the_site_is_classic_as_before() {
    let temp = tempfile::tempdir().unwrap();
    let site = at(temp.path(), "blog");
    let output = init(&site, &[]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let settings = fs::read_to_string(at(&site, "site.toml")).unwrap();
    assert!(!settings.contains("[template]"), "{settings}");
    let config = Config::load(&site, &Environment::empty()).unwrap();
    assert_eq!(config.template().name().as_str(), "classic");
    let tokens = fs::read_to_string(at(&site, "theme/tokens.toml")).unwrap();
    assert!(tokens.contains("link = \"#0b57d0\""), "{tokens}");
}

#[test]
fn the_help_shows_the_template_flag() {
    let output = khalid(&["init", "--help"]);
    assert_eq!(output.status.code(), Some(0));
    let help = text(&output.stdout);
    assert!(help.contains("--template <NAME>"), "{help}");
    assert!(help.contains("atlas"), "{help}");
}
