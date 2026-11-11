//! A site's own words in `theme/strings/<language>.toml` end to end:
//! `khalid build` of the demo site shows them on the built pages, and a
//! file with a key no template reads stops the build with a message that
//! names the file (KH-CON-8, backlog 4 row 14).

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

/// Builds a copy of the demo site whose `theme/strings/` holds `files`.
#[cfg(test)]
fn build(files: &[(&str, &str)]) -> (tempfile::TempDir, PathBuf, Output) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let folder = at(temp.path(), "theme/strings");
    fs::create_dir_all(&folder).unwrap();
    for (name, text) in files {
        fs::write(at(&folder, name), text).unwrap();
    }
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    (temp, out, output)
}

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

#[test]
fn the_sites_words_replace_the_built_in_ones_on_built_pages() {
    let german = "not_found_title = \"Seite fehlt\"\n";
    let (_temp, out, output) = build(&[("de.toml", german)]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    let missing = page(&out, "de/404.html");
    assert!(missing.contains("<h1>Seite fehlt</h1>"), "{missing}");
    assert!(missing.contains("Zur Startseite des Blogs"), "{missing}");
    let english = page(&out, "404.html");
    assert!(english.contains("This page does not exist"), "{english}");
}

#[test]
fn a_key_no_template_reads_stops_the_build_and_names_the_file() {
    let (_temp, _out, output) = build(&[("de.toml", "not_found_headline = \"Hier\"\n")]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.contains("theme/strings/de.toml has the key not_found_headline"),
        "{stderr}"
    );
    assert!(stderr.contains("crates/khalid-render/strings"), "{stderr}");
}
