//! A site's own CSS rules in `theme/site.css`: `khalid build` adds them to
//! the one stylesheet after the template's rules, so they win, and refuses
//! a file that leads outside the site folder.

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

/// The demo site in Atlas, in a folder of its own with a `theme/` folder.
#[cfg(test)]
fn demo_in_atlas() -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str("\n[template]\nname = \"atlas\"\n");
    fs::write(&settings, toml).unwrap();
    fs::create_dir_all(at(temp.path(), "theme")).unwrap();
    temp
}

#[cfg(test)]
fn build(site: &Path) -> (Output, PathBuf) {
    let out = at(site, "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    (output, out)
}

/// The text of the one stylesheet the build wrote.
#[cfg(test)]
fn stylesheet(out: &Path) -> String {
    let assets = at(out, "assets");
    let css = fs::read_dir(&assets)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().is_some_and(|ext| ext == "css"))
        .unwrap();
    fs::read_to_string(css).unwrap()
}

#[test]
fn the_sites_own_rules_come_after_the_templates() {
    let site = demo_in_atlas();
    fs::write(
        at(site.path(), "theme/site.css"),
        "/* The site's own hero. */\n.hero {\n  color: #123456;\n}\n",
    )
    .unwrap();
    let (output, out) = build(site.path());
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let css = stylesheet(&out);
    let template = css.find(".catbar {").unwrap();
    let own = css.find(".hero {\ncolor: #123456;\n}").unwrap();
    assert!(template < own, "{css}");
    assert!(!css.contains("The site's own hero"), "comments are removed");
}

#[test]
fn a_site_without_the_file_gets_the_template_alone() {
    let site = demo_in_atlas();
    let (output, out) = build(site.path());
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(
        stylesheet(&out)
            .trim_end()
            .ends_with("@media print { .catbar, .masthead::before, .cta-band::before, .site-footer::before { display: none; } }")
    );
}

#[cfg(unix)]
#[test]
fn a_file_linked_outside_the_site_is_refused() {
    let outside = tempfile::tempdir().unwrap();
    let elsewhere = at(outside.path(), "elsewhere.css");
    fs::write(&elsewhere, "body { color: red; }\n").unwrap();
    let site = demo_in_atlas();
    std::os::unix::fs::symlink(&elsewhere, at(site.path(), "theme/site.css")).unwrap();
    let (output, _) = build(site.path());
    assert_ne!(output.status.code(), Some(0));
    let said = String::from_utf8_lossy(&output.stderr);
    assert!(said.contains("theme/site.css"), "{said}");
}
