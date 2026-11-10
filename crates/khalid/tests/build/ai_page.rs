//! The page for AI assistants that `khalid build` writes: a site can switch
//! it off, and its settings show as text, never as markup.

use std::fs;

use khalid_core::SitePath;

use super::{demo_site, khalid, page, path, text};

#[test]
fn a_site_can_switch_the_ai_page_off() {
    let out = tempfile::tempdir().unwrap();
    let site = tempfile::tempdir().unwrap();
    let demo = demo_site();
    std::process::Command::new("cp")
        .arg("-R")
        .arg(format!("{}/.", path(&demo)))
        .arg(site.path())
        .status()
        .unwrap();
    let settings = SitePath::within(site.path(), "site.toml")
        .unwrap()
        .as_path()
        .to_path_buf();
    let mut file = fs::read_to_string(&settings).unwrap();
    file.push_str("\n[seo.ai_page]\noff = true\n");
    fs::write(&settings, file).unwrap();
    let output = khalid(&[
        "--site",
        path(site.path()),
        "build",
        "--out",
        path(out.path()),
    ]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    // Without the page: 46 pages, no ai/ folder, no footer link.
    assert!(
        text(&output.stdout).contains("46 pages"),
        "{}",
        text(&output.stdout)
    );
    assert!(
        SitePath::within(out.path(), "ai")
            .unwrap()
            .as_path()
            .read_dir()
            .is_err()
    );
    let home_body = page(out.path(), "index.html");
    assert!(!home_body.contains("href=\"/blog/ai/\""), "{home_body}");
}

#[test]
fn the_ai_page_shows_its_settings_as_text_not_markup() {
    let out = tempfile::tempdir().unwrap();
    let site = tempfile::tempdir().unwrap();
    let demo = demo_site();
    std::process::Command::new("cp")
        .arg("-R")
        .arg(format!("{}/.", path(&demo)))
        .arg(site.path())
        .status()
        .unwrap();
    let settings = SitePath::within(site.path(), "site.toml")
        .unwrap()
        .as_path()
        .to_path_buf();
    let mut file = fs::read_to_string(&settings).unwrap();
    file.push_str("\n[seo.ai_page]\ncite = \"Quote Tom & Jerry, never <b>bold</b>.\"\n");
    fs::write(&settings, file).unwrap();
    let output = khalid(&[
        "--site",
        path(site.path()),
        "build",
        "--out",
        path(out.path()),
    ]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let ai = page(out.path(), "ai/index.html");
    assert!(
        ai.contains("Quote Tom &amp; Jerry, never &lt;b&gt;bold&lt;/b&gt;."),
        "{ai}"
    );
}
