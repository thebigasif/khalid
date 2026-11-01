//! A template pack's token defaults under the site's own
//! `theme/tokens.toml`: the pack fills what the site leaves out, and the
//! site's value always wins (templates spec 4.3).

use std::fs;
use std::path::Path;

use khalid_render::{RenderError, Tokens};

/// Writes `text` as the tokens file of `site`.
#[cfg(test)]
fn write_tokens(site: &Path, text: &str) {
    let mut theme = site.to_path_buf();
    theme.push("theme");
    fs::create_dir_all(&theme).unwrap();
    theme.push("tokens.toml");
    fs::write(&theme, text).unwrap();
}

#[test]
fn a_pack_default_fills_what_the_site_leaves_out_and_the_site_wins() {
    let site = tempfile::tempdir().unwrap();
    write_tokens(
        site.path(),
        "[type]\nh1 = \"2rem 3rem\"\n[shape]\nradius = \"0px\"\n",
    );
    let pack = "[type]\nh1 = \"3rem 4rem\"\nh2 = \"2rem 2.5rem\"\n[shape]\nradius = \"1rem\"\n";
    let css = Tokens::load_over(site.path(), pack).unwrap().to_css();
    for line in [
        "--text-h1: clamp(2rem, 1.6667rem + 1.6667vw, 3rem);",
        "--text-h2: clamp(2rem, 1.8333rem + 0.8333vw, 2.5rem);",
        "--radius: 0px;",
        "--measure: 68ch;",
    ] {
        assert!(css.contains(line), "{line} in {css}");
    }
}

#[test]
fn a_mistake_in_the_site_file_keeps_its_own_line_under_a_pack() {
    let site = tempfile::tempdir().unwrap();
    write_tokens(site.path(), "# mine\n[type]\nh1 = \"big\"\n");
    let error = Tokens::load_over(site.path(), "[type]\nh2 = \"2rem\"\n").unwrap_err();
    let line = matches!(error, RenderError::Tokens { line: Some(3), .. });
    assert!(line, "{error:?}");
}

#[test]
fn a_site_without_a_file_takes_the_pack_defaults() {
    let site = tempfile::tempdir().unwrap();
    let tokens = Tokens::load_over(site.path(), "[space]\nsection = \"4rem\"\n").unwrap();
    assert!(tokens.to_css().contains("--space-section: 4rem;"));
    let classic = Tokens::load_over(site.path(), "").unwrap();
    assert_eq!(classic, Tokens::default());
    assert_eq!(classic, Tokens::load(site.path()).unwrap());
}
