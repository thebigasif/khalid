//! A site's own words in `theme/strings/<language>.toml` (KH-CON-8, P3).

use std::fs;
use std::path::{Path, PathBuf};

use khalid_core::Locale;
use khalid_render::{PageBody, PageKind, RenderError, Renderer};

use crate::common::{demo_site, meta};

/// A site folder whose `theme/strings/` holds `files`.
fn site_with_strings(files: &[(&str, &str)]) -> tempfile::TempDir {
    let site = tempfile::tempdir().unwrap();
    let folder = in_folder(site.path(), "theme/strings");
    fs::create_dir_all(&folder).unwrap();
    for (name, text) in files {
        fs::write(in_folder(&folder, name), text).unwrap();
    }
    site
}

fn in_folder(folder: &Path, relative: &str) -> PathBuf {
    let mut path = folder.to_path_buf();
    path.push(relative);
    path
}

fn not_found(site_root: &Path, tag: &str) -> Result<String, RenderError> {
    let meta = meta("Page not found", "/blog/missing/", PageKind::Website);
    Renderer::with_theme(site_root)?.render(&demo_site(tag), &meta, &PageBody::NotFound)
}

fn locale(tag: &str) -> Locale {
    Locale::try_new(tag).unwrap()
}

const FRENCH: &str = r#"
not_found_title = "Cette page n'existe pas"
month_1 = "janvier"
date_format = "{day} {month} {year}"
"#;

#[test]
fn a_language_with_no_built_in_file_takes_the_sites_words() {
    let site = site_with_strings(&[("fr.toml", FRENCH)]);
    let page = not_found(site.path(), "fr").unwrap();
    assert!(page.contains("Cette page n&#39;existe pas"), "{page}");
    // A key the site's file leaves out still reads in English.
    assert!(page.contains("Go to the blog&#39;s front page"), "{page}");
}

#[test]
fn the_sites_words_reach_text_outside_templates() {
    let site = site_with_strings(&[("fr.toml", FRENCH)]);
    let renderer = Renderer::with_theme(site.path()).unwrap();
    assert_eq!(renderer.text(&locale("fr"), "month_1"), Some("janvier"));
    assert_eq!(renderer.text(&locale("fr-CA"), "month_1"), Some("janvier"));
    assert_eq!(renderer.text(&locale("fr"), "month_2"), Some("February"));
    assert_eq!(renderer.text(&locale("en"), "month_1"), Some("January"));
}

#[test]
fn a_site_file_changes_one_built_in_word_and_keeps_the_rest() {
    let site = site_with_strings(&[("de.toml", "not_found_title = \"Nicht gefunden\"\n")]);
    let page = not_found(site.path(), "de").unwrap();
    assert!(page.contains("Nicht gefunden"), "{page}");
    assert!(page.contains("Zur Startseite des Blogs"), "{page}");
    let english = not_found(site.path(), "en").unwrap();
    assert!(english.contains("This page does not exist"), "{english}");
}

#[test]
fn a_language_with_no_built_in_file_starts_from_the_sites_english() {
    let site = site_with_strings(&[
        ("da.toml", "not_found_title = \"Siden findes ikke\"\n"),
        ("en.toml", "not_found_home = \"Back to the notes\"\n"),
    ]);
    let renderer = Renderer::with_theme(site.path()).unwrap();
    let danish = locale("da");
    assert_eq!(
        renderer.text(&danish, "not_found_title"),
        Some("Siden findes ikke")
    );
    assert_eq!(
        renderer.text(&danish, "not_found_home"),
        Some("Back to the notes")
    );
    assert_eq!(
        renderer.text(&locale("it"), "not_found_home"),
        Some("Back to the notes")
    );
}

#[test]
fn a_key_the_templates_never_read_is_refused() {
    let site = site_with_strings(&[("fr.toml", "not_found_headline = \"Introuvable\"\n")]);
    let error = Renderer::with_theme(site.path()).unwrap_err();
    assert!(
        matches!(&error, RenderError::UnknownStringsKey { file, key }
            if file == "fr.toml" && key == "not_found_headline"),
        "{error:?}"
    );
}

#[test]
fn a_file_with_a_toml_mistake_is_refused_with_its_name() {
    let site = site_with_strings(&[("fr.toml", "not_found_title = \n")]);
    let error = Renderer::with_theme(site.path()).unwrap_err();
    assert!(
        matches!(&error, RenderError::SiteStrings { file, .. } if file == "fr.toml"),
        "{error:?}"
    );
    assert!(
        error.to_string().contains("theme/strings/fr.toml"),
        "{error}"
    );
}

#[test]
fn a_value_that_is_not_text_is_refused() {
    let site = site_with_strings(&[("fr.toml", "month_1 = 1\n")]);
    assert!(matches!(
        Renderer::with_theme(site.path()),
        Err(RenderError::SiteStrings { .. })
    ));
}

#[test]
fn a_file_not_named_for_a_language_is_refused() {
    for name in ["french.toml", "de-CH.toml", "FR.toml", "notes.md"] {
        let site = site_with_strings(&[(name, "")]);
        let error = Renderer::with_theme(site.path()).unwrap_err();
        assert!(
            matches!(&error, RenderError::StringsFileName { name: got } if got == name),
            "{name}: {error:?}"
        );
    }
}

#[test]
fn a_hidden_file_is_passed_over() {
    let site = site_with_strings(&[(".DS_Store", "\u{0}\u{1}"), ("fr.toml", FRENCH)]);
    let renderer = Renderer::with_theme(site.path()).unwrap();
    assert_eq!(renderer.text(&locale("fr"), "month_1"), Some("janvier"));
}

#[test]
fn a_site_without_a_strings_folder_uses_the_built_in_words() {
    let site = tempfile::tempdir().unwrap();
    let renderer = Renderer::with_theme(site.path()).unwrap();
    assert_eq!(
        renderer.text(&locale("de"), "not_found_home"),
        Some("Zur Startseite des Blogs")
    );
    assert_eq!(renderer.text(&locale("fr"), "month_1"), Some("January"));
}

#[cfg(unix)]
#[test]
fn a_strings_folder_linked_outside_the_site_is_refused() {
    let outside = tempfile::tempdir().unwrap();
    let site = tempfile::tempdir().unwrap();
    fs::create_dir(in_folder(site.path(), "theme")).unwrap();
    std::os::unix::fs::symlink(outside.path(), in_folder(site.path(), "theme/strings")).unwrap();
    assert!(matches!(
        Renderer::with_theme(site.path()),
        Err(RenderError::ThemePath(_))
    ));
}
