//! Inside a site folder, `--site` may be left out: every command that
//! works on one site uses the current folder when it holds site.toml, and
//! answers the same as with `--site` (BL-CLI-1). Outside one, the command
//! still asks for the folder.

#[expect(
    dead_code,
    reason = "these tests only copy the demo site, so the server helpers stay unused"
)]
mod common;

use std::path::Path;
use std::process::Command;

/// The commands that work on one site folder, with arguments that need
/// nothing from the internet.
const COMMANDS: &[&[&str]] = &[
    &["lint"],
    &["build"],
    &["audit", "--offline"],
    &["redirects", "export"],
    &["search-console", "check"],
    &["doctor"],
    &["import", "wordpress", "EXPORT", "--dry-run"],
];

/// Runs `khalid` with `args` in the folder `at`, with `EXPORT` standing for
/// the importer's `WordPress` sample, and returns the exit code and what it
/// printed.
#[cfg(test)]
fn khalid(at: &Path, args: &[&str]) -> (i32, String) {
    let export = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../khalid-import/fixtures/wordpress/basic.xml"
    );
    let args = args
        .iter()
        .map(|arg| if *arg == "EXPORT" { export } else { arg });
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .current_dir(at)
        .args(args)
        .output()
        .unwrap();
    let mut printed = String::from_utf8(output.stdout).unwrap();
    printed.push_str(&String::from_utf8(output.stderr).unwrap());
    (output.status.code().unwrap(), printed)
}

#[test]
fn every_site_command_works_inside_the_site_folder_without_site() {
    let site = common::demo_copy();
    let elsewhere = tempfile::tempdir().unwrap();
    for args in COMMANDS {
        let named: Vec<&str> = ["--site", site.path().to_str().unwrap()]
            .into_iter()
            .chain(args.iter().copied())
            .collect();
        let (code_named, said_named) = khalid(elsewhere.path(), &named);
        let (code_here, said_here) = khalid(site.path(), args);
        let context = args.join(" ");
        assert!(
            !said_here.contains("needs a site folder"),
            "{context}: {said_here}"
        );
        assert_eq!(
            code_here, code_named,
            "{context}:\n{said_here}\n{said_named}"
        );
    }
    let mut home = site.path().to_path_buf();
    home.push("public/index.html");
    assert!(home.is_file());
}

#[test]
fn outside_a_site_folder_a_command_still_asks_for_one() {
    let elsewhere = tempfile::tempdir().unwrap();
    let (code, said) = khalid(elsewhere.path(), &["lint"]);
    assert_eq!(code, 2, "{said}");
    assert!(
        said.contains("The command `khalid lint` needs a site folder."),
        "{said}"
    );
    assert!(
        said.contains("inside the folder that holds site.toml"),
        "{said}"
    );
}

#[test]
fn bare_khalid_inside_a_broken_site_folder_still_prints_the_help() {
    let site = tempfile::tempdir().unwrap();
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    std::fs::write(&settings, "this is not = = toml").unwrap();
    let (code, said) = khalid(site.path(), &[]);
    assert_eq!(code, 0, "{said}");
    assert!(said.contains("Usage:"), "{said}");
}
