//! `khalid man <FOLDER>`, in builds with the `man` feature: one manual page
//! for khalid and one for each command, as `just man` and the release write
//! them (RULES.md BL-CLI-1, BL-CLI-2).

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use serde_json::Value;

#[cfg(test)]
fn khalid(args: &[&str], out: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(args)
        .arg(out)
        .output()
        .unwrap()
}

#[cfg(test)]
fn page(folder: &Path, name: &str) -> String {
    let mut path = folder.to_path_buf();
    path.push(name);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn every_command_gets_a_page() {
    let temp = tempfile::tempdir().unwrap();
    let mut out = temp.path().to_path_buf();
    out.push("man");
    let output = khalid(&["man"], &out);
    assert!(output.status.success(), "{output:?}");
    let top = page(&out, "khalid.1");
    assert!(top.contains(".TH khalid 1"), "{top}");
    assert!(top.contains("completions"), "{top}");
    let init = page(&out, "khalid-init.1");
    assert!(init.contains("\\-\\-address"), "{init}");
    page(&out, "khalid-keys-create.1");
    let completions = page(&out, "khalid-completions.1");
    let source = concat!("\"khalid ", env!("CARGO_PKG_VERSION"), "\"");
    assert!(completions.contains(source), "{completions}");
    let mut hidden = out.clone();
    hidden.push("khalid-man.1");
    assert!(!hidden.exists(), "the hidden man command got a page");
    let said = String::from_utf8(output.stdout).unwrap();
    assert!(said.contains("manual pages"), "{said}");
}

#[test]
fn json_lists_the_pages() {
    let temp = tempfile::tempdir().unwrap();
    let output = khalid(&["man", "--json"], temp.path());
    assert!(output.status.success(), "{output:?}");
    let body: Value = serde_json::from_slice(&output.stdout).unwrap();
    let pages: Vec<&str> = body["pages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|page| page.as_str().unwrap())
        .collect();
    assert_eq!(pages.first(), Some(&"khalid-agents.1"));
    assert!(pages.contains(&"khalid.1"));
    assert!(pages.is_sorted());
}

#[test]
fn a_file_in_the_way_is_a_failure_with_a_fix() {
    let temp = tempfile::tempdir().unwrap();
    let mut file = temp.path().to_path_buf();
    file.push("taken");
    fs::write(&file, "not a folder").unwrap();
    let output = khalid(&["man"], &file);
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let message = String::from_utf8(output.stderr).unwrap();
    assert!(message.contains("taken"), "{message}");
}
