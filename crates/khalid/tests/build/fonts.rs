//! Self-hosted fonts in `khalid build`: the font files are written under
//! `assets/`, and every page preloads the body font and only that file.

use std::fs;
use std::path::Path;

use super::{at, demo_site, khalid, page, path, text};

#[test]
fn self_hosted_font_files_are_served_under_assets() {
    let out = tempfile::tempdir().unwrap();
    // A copy, so the shared fixture stays as it is.
    let demo = tempfile::tempdir().unwrap();
    copy_dir(&demo_site(), demo.path());
    let theme = at(demo.path(), "theme");
    let fonts = at(&theme, "fonts");
    fs::create_dir_all(&fonts).unwrap();
    fs::write(at(&fonts, "inter-latin.woff2"), b"font bytes").unwrap();
    fs::write(at(&fonts, "not-a-font.txt"), b"notes").unwrap();
    let tokens = fs::read_to_string(at(&theme, "tokens.toml")).unwrap_or_default();
    let with_font = format!(
        "{tokens}\n[[fonts.files]]\nfamily = \"Inter\"\nstyle = \"normal\"\nweight = \"400\"\npath = \"inter-latin.woff2\"\nrange = \"U+0000-00FF\"\n"
    );
    fs::write(at(&theme, "tokens.toml"), with_font).unwrap();

    let output = khalid(&[
        "--site",
        path(demo.path()),
        "build",
        "--out",
        path(out.path()),
    ]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let served = fs::read(at(out.path(), "assets/fonts/inter-latin.woff2")).unwrap();
    assert_eq!(
        served, b"font bytes",
        "the file the stylesheet names is there"
    );
    assert!(
        !at(out.path(), "assets/fonts/not-a-font.txt").exists(),
        "only woff2 files are served"
    );
}

#[test]
fn every_page_preloads_the_body_font_and_only_that_file() {
    let out = tempfile::tempdir().unwrap();
    let demo = tempfile::tempdir().unwrap();
    copy_dir(&demo_site(), demo.path());
    let theme = at(demo.path(), "theme");
    let fonts = at(&theme, "fonts");
    fs::create_dir_all(&fonts).unwrap();
    for file in [
        "inter-latin-400.woff2",
        "inter-latin-italic-400.woff2",
        "fraunces-latin-600.woff2",
    ] {
        fs::write(at(&fonts, file), b"font bytes").unwrap();
    }
    let face = |family: &str, style: &str, weight: &str, file: &str| {
        format!(
            "\n[[fonts.files]]\nfamily = \"{family}\"\nstyle = \"{style}\"\nweight = \"{weight}\"\npath = \"{file}\"\n"
        )
    };
    let tokens = [
        "[fonts]\nheading = '\"Fraunces\", serif'\nbody = '\"Inter\", system-ui, sans-serif'\n"
            .to_owned(),
        face("Fraunces", "normal", "600", "fraunces-latin-600.woff2"),
        face("Inter", "italic", "400", "inter-latin-italic-400.woff2"),
        face("Inter", "normal", "400", "inter-latin-400.woff2"),
    ]
    .concat();
    fs::write(at(&theme, "tokens.toml"), tokens).unwrap();

    let output = khalid(&[
        "--site",
        path(demo.path()),
        "build",
        "--out",
        path(out.path()),
    ]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    for rel in ["index.html", "why-internal-links-matter/index.html"] {
        let html = page(out.path(), rel);
        let preloads: Vec<&str> = html
            .lines()
            .filter(|line| line.contains(r#"rel="preload""#))
            .collect();
        assert_eq!(preloads.len(), 1, "{rel} preloads one file: {preloads:?}");
        assert!(
            preloads[0].ends_with(
                r#"assets/fonts/inter-latin-400.woff2" as="font" type="font/woff2" crossorigin>"#
            ),
            "{rel} preloads the upright body face where the stylesheet fetches it: {}",
            preloads[0]
        );
    }
}

/// Copies the folder `from` into the empty folder `into`.
#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = at(into, &entry.file_name().to_string_lossy());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}
